//! Wiring of the identity module (S3): its dependencies, router, outbox consumers, the source-key
//! listener into the ingress negative cache, the detached code delivery, the channel-hold worker
//! and the startup reset of live connections.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use access::{SessionAuthenticator, StepUpGate};
use adapter_tradingview::IngressState;
use async_trait::async_trait;
use axum::Router;
use identity_application::changes::{
    AUDIT_EVENT_TYPES, NOTICE_EVENT_TYPE, Notice, OPERATOR_EVENT_TYPE, SSE_EVENT_TYPES,
};
use identity_application::deps::{IdentityDeps, IdentitySettings};
use identity_application::ports::{
    AccountTargets, CodeSender, ConnectionAccounts, ConnectionPresence, DeliveryQueue, Mailer,
    SourceActivity, SourceKeyListener,
};
use identity_application::usecases::access_port::IdentityAccess;
use identity_application::usecases::channel_change::ApplyDueChannelHolds;
use identity_application::usecases::connections::DisconnectLiveConnectionsAtStartup;
use identity_enterprise::ids::{ConnectionId, SourceId};
use identity_enterprise::tokens::KeyId;
use identity_infra::crypto::{OsRandom, PepperedHasher, VersionedKeys};
use identity_infra::delivery::{
    DeliveryWorker, DisabledSender, NoticeSender, SmtpDevSender, delivery_channel,
};
use identity_infra::http::{IdentityHttpState, SseHub};
use identity_infra::ports::{AdapterVersionPolicy, DisabledMailer};
use identity_infra::store::PgIdentityStore;
use identity_infra::throttle::{
    CodeThrottlePolicy, LogPairingAlerts, LogSecurityAlerts, PairingThrottlePolicy,
    PerIpPublicThrottle, PrefixCodeThrottle, PrefixPairingThrottle,
};
use shared_kernel::{Clock, UserId};
use sqlx::PgPool;

use crate::config::{CodeSenderConfig, Config, IdentityConfig};
use crate::outbox::{Consumers, EventConsumer, OutboxEvent};

/// Assumed limit of the public token endpoints (invitation lookup, sign-up start) per IP and
/// minute; not set by the policy yet (open question to the PO).
const PUBLIC_PER_MINUTE: usize = 10;
/// How often the worker applies due channel-change holds.
const HOLD_SWEEP: Duration = Duration::from_secs(60);

/// Everything `app` needs from identity after wiring.
pub struct IdentityWiring {
    pub deps: IdentityDeps,
    pub router: Router,
    /// For other modules' handlers (ADR-0007).
    pub authenticator: Arc<dyn SessionAuthenticator>,
    pub step_up: Arc<dyn StepUpGate>,
    delivery: DeliveryWorker,
    sender: Arc<dyn CodeSender>,
}

impl IdentityWiring {
    /// Startup work before adapters are accepted, then the background tasks.
    pub async fn start(self) -> anyhow::Result<()> {
        let reset = DisconnectLiveConnectionsAtStartup::new(self.deps.clone())
            .handle()
            .await
            .map_err(|e| anyhow::anyhow!("startup reset of live connections: {e}"))?;
        tracing::info!(connections = reset, "live connections reset at startup");
        tokio::spawn(self.delivery.run(self.sender.clone(), self.deps.clone()));
        let holds = ApplyDueChannelHolds::new(self.deps.clone());
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(HOLD_SWEEP);
            loop {
                tick.tick().await;
                match holds.handle().await {
                    Ok(0) => {}
                    Ok(n) => tracing::info!(holds = n, "channel-change holds applied"),
                    Err(e) => tracing::error!(error = %e, "applying channel-change holds failed"),
                }
            }
        });
        Ok(())
    }
}

/// Tells the ingress negative cache about every new source key (Vigia S1 I1).
struct IngressKeyListener(Arc<IngressState>);

impl SourceKeyListener for IngressKeyListener {
    fn key_activated(&self, key_id: &KeyId) {
        self.0.unknown_keys.invalidate(key_id.as_str());
    }
}

/// The gateway (S2) reports visible accounts; until it is wired, none.
struct NoPresence;

#[async_trait]
impl ConnectionPresence for NoPresence {
    async fn accounts(
        &self,
        _: UserId,
        _: &[ConnectionId],
    ) -> Result<std::collections::HashMap<ConnectionId, ConnectionAccounts>, String> {
        Ok(std::collections::HashMap::new())
    }
}

/// Last instruction per source, read from routing's instruction table through its own query.
struct NoSourceActivity;

#[async_trait]
impl SourceActivity for NoSourceActivity {
    async fn last_received(
        &self,
        _: UserId,
        _: &[SourceId],
    ) -> Result<std::collections::HashMap<SourceId, chrono::DateTime<chrono::Utc>>, String> {
        Ok(std::collections::HashMap::new())
    }
}

/// Accounts (S5) answers "is this account the user's?"; until its query port is wired every
/// account target is 404 (fail closed).
struct NoAccountTargets;

#[async_trait]
impl AccountTargets for NoAccountTargets {
    async fn owned_by(&self, _: UserId, _: uuid::Uuid) -> Result<bool, String> {
        Ok(false)
    }
}

/// SSE projection of `identity.*` integration events.
struct SseConsumer(Arc<SseHub>);

#[async_trait]
impl EventConsumer for SseConsumer {
    fn name(&self) -> &'static str {
        "identity-sse"
    }

    async fn handle(&self, e: &OutboxEvent) -> Result<(), String> {
        if self
            .0
            .publish(e.user_id, &e.event_type, e.occurred_at, &e.payload)
        {
            Ok(())
        } else {
            Err(format!("event type {} is not streamed", e.event_type))
        }
    }
}

/// E-mail notices (new device, channel changes). Without a provider they are dropped with a
/// warning: nothing could ever send them, and retrying would only dead-letter them.
struct NoticeConsumer {
    sender: Arc<dyn NoticeSender>,
    disabled: bool,
}

#[async_trait]
impl EventConsumer for NoticeConsumer {
    fn name(&self) -> &'static str {
        "identity-notice"
    }

    async fn handle(&self, e: &OutboxEvent) -> Result<(), String> {
        let (locale, notice) = identity_infra::outbox::notice_from_payload(&e.payload)
            .ok_or_else(|| "malformed notice payload".to_owned())?;
        if self.disabled {
            tracing::warn!(event_id = %e.id, "notice not delivered: no e-mail provider configured");
            return Ok(());
        }
        self.sender
            .send_notice(locale, &notice)
            .await
            .map_err(|err| format!("notice delivery failed: {}", err.class()))
    }
}

/// `identity.operator_recovery` (S3 AC-12, Esquadro N-1): the operator command runs in its own
/// process, whose stdout is the operator's terminal, so its actions reach the collected server
/// log only through this consumer. Normal actions: warn. Break-glass: error, a `security_alert`
/// line and an e-mail to `OPERATOR_ALERT_EMAIL` (ids only). Who ran it (host-reported) goes to
/// this log only; the audit consumer never registers the type.
struct OperatorConsumer {
    sender: Arc<dyn NoticeSender>,
    disabled: bool,
    alert_to: Option<identity_enterprise::channel::Destination>,
    /// Break-glass events already logged whose alert e-mail is still to be sent (Brasa R3-2):
    /// a retry re-sends the e-mail without repeating the log lines.
    awaiting_mail: std::sync::Mutex<std::collections::HashSet<uuid::Uuid>>,
}

impl OperatorConsumer {
    fn new(
        sender: Arc<dyn NoticeSender>,
        disabled: bool,
        alert_to: Option<identity_enterprise::channel::Destination>,
    ) -> Self {
        Self {
            sender,
            disabled,
            alert_to,
            awaiting_mail: std::sync::Mutex::new(std::collections::HashSet::new()),
        }
    }

    /// Marks `id` as logged; `false` when it already was (a retry of its e-mail).
    fn first_time(&self, id: uuid::Uuid) -> bool {
        self.awaiting_mail.lock().map_or(true, |mut s| s.insert(id))
    }

    fn mailed(&self, id: uuid::Uuid) {
        if let Ok(mut s) = self.awaiting_mail.lock() {
            s.remove(&id);
        }
    }
}

#[async_trait]
impl EventConsumer for OperatorConsumer {
    fn name(&self) -> &'static str {
        "identity-operator"
    }

    async fn handle(&self, e: &OutboxEvent) -> Result<(), String> {
        let l = identity_infra::outbox::operator_from_payload(&e.payload)
            .ok_or_else(|| "malformed operator payload".to_owned())?;
        let dash = |v: &Option<String>| v.clone().unwrap_or_else(|| "-".into());
        if !l.break_glass {
            tracing::warn!(
                target: "operator",
                event_id = %e.id,
                user_id = %e.user_id.as_uuid(),
                admin_user_id = dash(&l.admin_user_id),
                change_id = dash(&l.change_id),
                step = l.step,
                outcome = l.outcome,
                reason = dash(&l.reason),
                after_cancel = l.after_cancel,
                host_login = dash(&l.host_login),
                host_ssh_connection = dash(&l.host_ssh_connection),
                host_ssh_key = dash(&l.host_key_fingerprint),
                "admin recovery (host-reported session values)"
            );
            if l.after_cancel {
                tracing::warn!(target: "security_alert", user_id = %e.user_id.as_uuid(), "channel change requested again after a cancellation");
            }
            return Ok(());
        }
        if self.first_time(e.id) {
            self.log_break_glass(e, &l);
        }
        let Some(to) = &self.alert_to else {
            self.mailed(e.id);
            return Ok(());
        };
        if self.disabled {
            self.mailed(e.id);
            tracing::warn!(event_id = %e.id, "break-glass alert e-mail not sent: no e-mail provider configured");
            return Ok(());
        }
        let notice = Notice::OperatorAlert {
            to: to.clone(),
            change_id: l.change_id.clone(),
            step: l.step.clone(),
            outcome: l.outcome.clone(),
            at: e.occurred_at,
        };
        match self
            .sender
            .send_notice(identity_enterprise::profile::Locale::PtBr, &notice)
            .await
        {
            Ok(()) => {
                self.mailed(e.id);
                Ok(())
            }
            Err(err) => {
                tracing::error!(target: "operator", event_id = %e.id, class = err.class(), "break-glass alert e-mail failed; it will be retried");
                Err(format!("operator alert delivery failed: {}", err.class()))
            }
        }
    }
}

impl OperatorConsumer {
    fn log_break_glass(&self, e: &OutboxEvent, l: &identity_infra::outbox::OperatorLogLine) {
        let dash = |v: &Option<String>| v.clone().unwrap_or_else(|| "-".into());
        tracing::error!(
            target: "operator",
            event_id = %e.id,
            user_id = %e.user_id.as_uuid(),
            admin_user_id = dash(&l.admin_user_id),
            change_id = dash(&l.change_id),
            step = l.step,
            outcome = l.outcome,
            reason = dash(&l.reason),
            after_cancel = l.after_cancel,
            host_login = dash(&l.host_login),
            host_ssh_connection = dash(&l.host_ssh_connection),
            host_ssh_key = dash(&l.host_key_fingerprint),
            "admin recovery BREAK-GLASS (host-reported session values)"
        );
        tracing::error!(target: "security_alert", event_id = %e.id, step = l.step, outcome = l.outcome, "break-glass used");
    }
}

pub fn versioned_keys(cfg: &IdentityConfig) -> anyhow::Result<VersionedKeys> {
    let keys: BTreeMap<i16, Vec<u8>> = cfg
        .peppers
        .iter()
        .map(|(v, k)| (*v, k.expose().to_vec()))
        .collect();
    VersionedKeys::new(cfg.pepper_current, keys)
        .map_err(|e| anyhow::anyhow!("IDENTITY_PEPPERS: {e}"))
}

/// Identity's dependencies and the senders `app` drives, shared by the server and the operator
/// command (`admin recovery`).
pub(crate) struct IdentityParts {
    pub deps: IdentityDeps,
    pub sender: Arc<dyn CodeSender>,
    pub notices: Arc<dyn NoticeSender>,
    pub public_origin: String,
}

pub(crate) fn parts(
    config: &Config,
    cfg: &IdentityConfig,
    db: &PgPool,
    worker: &PgPool,
    clock: Arc<dyn Clock>,
    source_keys: Arc<dyn SourceKeyListener>,
    queue: Arc<dyn DeliveryQueue>,
) -> anyhow::Result<IdentityParts> {
    let alerts = Arc::new(LogSecurityAlerts);
    let (sender, notices, mailer): (Arc<dyn CodeSender>, Arc<dyn NoticeSender>, Arc<dyn Mailer>) =
        match &cfg.code_sender {
            CodeSenderConfig::Disabled => (
                Arc::new(DisabledSender),
                Arc::new(DisabledSender),
                Arc::new(DisabledMailer),
            ),
            CodeSenderConfig::DevSmtp { host, port, from } => {
                let smtp = Arc::new(SmtpDevSender::new(host.clone(), *port, from.clone()));
                (smtp.clone(), smtp.clone(), smtp)
            }
        };
    let minimums: Vec<(&str, &str)> = cfg
        .adapter_min_versions
        .iter()
        .map(|(t, v)| (t.as_str(), v.as_str()))
        .collect();
    let adapters = AdapterVersionPolicy::new(&minimums, !config.app_env.is_production())
        .map_err(|e| anyhow::anyhow!("IDENTITY_ADAPTER_MIN_VERSIONS: {e}"))?;
    let public_origin = config
        .public_origin
        .clone()
        .ok_or_else(|| anyhow::anyhow!("APP_PUBLIC_ORIGIN is required with identity"))?;
    let deps = IdentityDeps {
        store: Arc::new(PgIdentityStore::new(db.clone(), worker.clone())),
        clock,
        hasher: Arc::new(PepperedHasher::new(versioned_keys(cfg)?)),
        random: Arc::new(OsRandom),
        mailer,
        presence: Arc::new(NoPresence),
        source_activity: Arc::new(NoSourceActivity),
        source_keys,
        adapters: Arc::new(adapters),
        pairing_throttle: Arc::new(PrefixPairingThrottle::new(
            PairingThrottlePolicy::default(),
            Arc::new(LogPairingAlerts),
        )),
        public_throttle: Arc::new(PerIpPublicThrottle::new(PUBLIC_PER_MINUTE)),
        code_throttle: Arc::new(PrefixCodeThrottle::new(
            CodeThrottlePolicy::default(),
            alerts.clone(),
        )),
        delivery: queue,
        alerts,
        account_targets: Arc::new(NoAccountTargets),
        settings: IdentitySettings {
            public_origin: public_origin.clone(),
            step_up_q35_persistence_actions: cfg.step_up_q35_persistence_actions,
            whatsapp_enabled: false,
        },
    };
    tracing::info!(
        step_up_q35_persistence_actions = cfg.step_up_q35_persistence_actions,
        code_sender = match cfg.code_sender {
            CodeSenderConfig::Disabled => "disabled",
            CodeSenderConfig::DevSmtp { .. } => "dev_smtp",
        },
        "identity settings"
    );
    Ok(IdentityParts {
        deps,
        sender,
        notices,
        public_origin,
    })
}

/// Builds identity and registers its outbox consumers (`audit` for the audit codes).
#[allow(clippy::too_many_arguments)]
pub fn wire(
    config: &Config,
    cfg: &IdentityConfig,
    db: &PgPool,
    worker: &PgPool,
    clock: Arc<dyn Clock>,
    ingress: Arc<IngressState>,
    audit: Arc<dyn EventConsumer>,
    consumers: &mut Consumers,
) -> anyhow::Result<IdentityWiring> {
    let (queue, delivery) = delivery_channel();
    let IdentityParts {
        deps,
        sender,
        notices,
        public_origin,
    } = parts(
        config,
        cfg,
        db,
        worker,
        clock,
        Arc::new(IngressKeyListener(ingress)),
        Arc::new(queue),
    )?;
    let events = Arc::new(SseHub::new());
    let sse: Arc<dyn EventConsumer> = Arc::new(SseConsumer(events.clone()));
    let notice: Arc<dyn EventConsumer> = Arc::new(NoticeConsumer {
        sender: notices.clone(),
        disabled: cfg.code_sender == CodeSenderConfig::Disabled,
    });
    for t in AUDIT_EVENT_TYPES {
        consumers.register(t, audit.clone());
    }
    for t in SSE_EVENT_TYPES {
        consumers.register(t, sse.clone());
    }
    consumers.register(NOTICE_EVENT_TYPE, notice);
    let alert_to = cfg.operator_alert_email.as_deref().and_then(|raw| {
        identity_enterprise::email::EmailAddress::parse(raw)
            .ok()
            .map(identity_enterprise::channel::Destination::Email)
    });
    if alert_to.is_none() {
        tracing::warn!(
            "OPERATOR_ALERT_EMAIL is unset: break-glass alerts reach the server log only"
        );
    }
    let operator: Arc<dyn EventConsumer> = Arc::new(OperatorConsumer::new(
        notices.clone(),
        cfg.code_sender == CodeSenderConfig::Disabled,
        alert_to,
    ));
    consumers.register(OPERATOR_EVENT_TYPE, operator);
    let emitted: Vec<&str> = identity_application::changes::emitted_event_types().collect();
    consumers.require(&emitted, crate::unconsumed::UNCONSUMED)?;
    let router = identity_infra::http::router(Arc::new(IdentityHttpState {
        deps: deps.clone(),
        public_origin,
        trust_forwarded_for: config.trust_forwarded_for,
        events,
    }));
    let access = Arc::new(IdentityAccess::new(deps.clone()));
    Ok(IdentityWiring {
        deps,
        router,
        authenticator: access.clone(),
        step_up: access,
        delivery,
        sender,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]
    use super::*;
    use identity_application::changes::{
        HostReport, IdentityEvent, OperatorOutcome, OperatorRecord, OperatorStep,
    };
    use identity_application::ports::DeliveryError;
    use identity_enterprise::channel::Destination;
    use identity_enterprise::email::EmailAddress;
    use identity_enterprise::profile::Locale;
    use std::sync::Mutex;

    #[derive(Default)]
    struct Sent(Mutex<Vec<Notice>>);

    #[async_trait]
    impl NoticeSender for Sent {
        async fn send_notice(&self, _: Locale, notice: &Notice) -> Result<(), DeliveryError> {
            self.0.lock().unwrap().push(notice.clone());
            Ok(())
        }
    }

    fn row(break_glass: bool) -> OutboxEvent {
        let user = UserId::new_v7();
        let envelope =
            identity_infra::outbox::event(&IdentityEvent::OperatorRecovery(OperatorRecord {
                user_id: user,
                admin_user_id: Some(user),
                change_id: None,
                step: OperatorStep::Start,
                outcome: OperatorOutcome::Done,
                break_glass,
                after_cancel: false,
                host: HostReport::parse(Some("ops"), Some("203.0.113.5 1 10.0.0.2 22"), None),
                at: chrono::Utc::now(),
            }));
        OutboxEvent {
            id: uuid::Uuid::now_v7(),
            user_id: envelope.user_id,
            event_type: envelope.event_type.to_owned(),
            occurred_at: envelope.occurred_at,
            payload: envelope.payload,
        }
    }

    fn consumer(sent: Arc<Sent>, disabled: bool, to: Option<&str>) -> OperatorConsumer {
        OperatorConsumer::new(
            sent,
            disabled,
            to.map(|t| Destination::Email(EmailAddress::parse(t).unwrap())),
        )
    }

    #[tokio::test]
    async fn break_glass_e_mails_the_operator_with_ids_only() {
        let sent = Arc::new(Sent::default());
        let c = consumer(sent.clone(), false, Some("ops@example.com"));
        c.handle(&row(true)).await.unwrap();
        let notices = sent.0.lock().unwrap().clone();
        assert_eq!(notices.len(), 1);
        let Notice::OperatorAlert {
            to, step, outcome, ..
        } = &notices[0]
        else {
            panic!("{notices:?}");
        };
        assert_eq!(to.normalized(), "ops@example.com");
        assert_eq!((step.as_str(), outcome.as_str()), ("start", "done"));
        assert!(
            !format!("{notices:?}").contains("203.0.113.5"),
            "who ran it stays in the log"
        );
    }

    #[tokio::test]
    async fn normal_actions_and_unconfigured_or_disabled_senders_send_nothing() {
        let sent = Arc::new(Sent::default());
        consumer(sent.clone(), false, Some("ops@example.com"))
            .handle(&row(false))
            .await
            .unwrap();
        consumer(sent.clone(), false, None)
            .handle(&row(true))
            .await
            .unwrap();
        consumer(sent.clone(), true, Some("ops@example.com"))
            .handle(&row(true))
            .await
            .unwrap();
        assert!(sent.0.lock().unwrap().is_empty());
        let mut bad = row(true);
        bad.payload = serde_json::json!({});
        assert!(
            consumer(sent, false, None).handle(&bad).await.is_err(),
            "a malformed row is retried, never dropped silently"
        );
    }

    /// Fails the first send, then delivers.
    #[derive(Default)]
    struct FlakyOnce(Mutex<u32>);

    #[async_trait]
    impl NoticeSender for FlakyOnce {
        async fn send_notice(&self, _: Locale, _: &Notice) -> Result<(), DeliveryError> {
            let mut calls = self.0.lock().unwrap();
            *calls += 1;
            if *calls == 1 {
                Err(DeliveryError::Failed("smtp".into()))
            } else {
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn an_e_mail_retry_does_not_repeat_the_log_lines() {
        let flaky = Arc::new(FlakyOnce::default());
        let c = OperatorConsumer::new(
            flaky.clone(),
            false,
            Some(Destination::Email(
                EmailAddress::parse("ops@example.com").unwrap(),
            )),
        );
        let ev = row(true);
        assert!(c.handle(&ev).await.is_err(), "the failed e-mail is retried");
        assert!(!c.first_time(ev.id), "logged once, before the retry");
        // Still pending (that check changed nothing): the retry only mails.
        assert!(c.handle(&ev).await.is_ok());
        assert_eq!(*flaky.0.lock().unwrap(), 2);
        assert!(c.first_time(ev.id), "cleared once the e-mail went out");
    }

    #[test]
    fn the_operator_event_is_never_an_audit_code() {
        assert!(!AUDIT_EVENT_TYPES.contains(&OPERATOR_EVENT_TYPE));
        assert!(
            identity_application::changes::emitted_event_types().any(|t| t == OPERATOR_EVENT_TYPE),
            "app must register a consumer for it"
        );
    }
}
