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
    AUDIT_EVENT_TYPES, DEVICE_TOKEN_REVOKED_EVENT_TYPE, NOTICE_EVENT_TYPE, SSE_EVENT_TYPES,
};
use identity_application::deps::{IdentityDeps, IdentitySettings};
use identity_application::ports::{
    AccountTargets, CodeSender, ConnectionAccounts, ConnectionPresence, Mailer, SourceActivity,
    SourceKeyListener,
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
use crate::outbox::{Consumers, EventConsumer, OutboxEvent, StartPolicy};

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
    /// `APP_PUBLIC_ORIGIN`, for the other modules' Origin check.
    pub public_origin: String,
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

/// The gateway's adapter authentication (S2) over identity's device tokens: token parsing and
/// the hash compare stay in identity; the gateway only sees the principal.
pub struct IdentityDeviceTokens(pub IdentityDeps);

#[async_trait]
impl gateway_application::ports::DeviceTokens for IdentityDeviceTokens {
    async fn authenticate(
        &self,
        presented: &gateway_application::ports::PresentedToken,
    ) -> Result<
        Option<gateway_application::ports::AdapterPrincipal>,
        gateway_application::ports::PortError,
    > {
        let principal = identity_application::usecases::adapter_auth::AuthenticateDeviceToken::new(
            self.0.clone(),
        )
        .handle(&identity_enterprise::secret::SecretString::new(
            presented.expose(),
        ))
        .await
        .map_err(|e| gateway_application::ports::PortError(e.to_string()))?;
        Ok(
            principal.map(|p| gateway_application::ports::AdapterPrincipal {
                user_id: p.user_id,
                connection_id: gateway_enterprise::ids::ConnectionId::from_uuid(
                    p.connection_id.as_uuid(),
                ),
                key_id: p.key_id.as_str().to_owned(),
                locale: p.locale.as_str().to_owned(),
            }),
        )
    }

    async fn still_valid(
        &self,
        key_id: &str,
    ) -> Result<bool, gateway_application::ports::PortError> {
        let Some(key) = KeyId::parse(key_id) else {
            return Ok(false);
        };
        identity_application::usecases::adapter_auth::DeviceTokenStatus::new(self.0.clone())
            .handle(&key)
            .await
            .map_err(|e| gateway_application::ports::PortError(e.to_string()))
    }
}

/// SSE projection of `identity.*` integration events.
struct SseConsumer(Arc<SseHub>);

#[async_trait]
impl EventConsumer for SseConsumer {
    fn name(&self) -> &'static str {
        "identity.sse"
    }

    /// Live projection: clients refetch on reconnect, so past rows are never replayed.
    fn start(&self) -> StartPolicy {
        StartPolicy::FromNow
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
        "identity.notice"
    }

    /// Never re-send old e-mails when the consumer is first registered (cutover, #14).
    fn start(&self) -> StartPolicy {
        StartPolicy::FromNow
    }

    /// One e-mail per transaction: a failed commit re-sends at most one (at least once delivery).
    fn batch(&self) -> i64 {
        1
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

pub fn versioned_keys(cfg: &IdentityConfig) -> anyhow::Result<VersionedKeys> {
    let keys: BTreeMap<i16, Vec<u8>> = cfg
        .peppers
        .iter()
        .map(|(v, k)| (*v, k.expose().to_vec()))
        .collect();
    VersionedKeys::new(cfg.pepper_current, keys)
        .map_err(|e| anyhow::anyhow!("IDENTITY_PEPPERS: {e}"))
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
    account_targets: Arc<dyn AccountTargets>,
    audit: Arc<dyn EventConsumer>,
    consumers: &mut Consumers,
) -> anyhow::Result<IdentityWiring> {
    let alerts = Arc::new(LogSecurityAlerts);
    let (queue, delivery) = delivery_channel();
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
        source_keys: Arc::new(IngressKeyListener(ingress)),
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
        delivery: Arc::new(queue),
        alerts,
        account_targets,
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
    let events = Arc::new(SseHub::new());
    let sse: Arc<dyn EventConsumer> = Arc::new(SseConsumer(events.clone()));
    let notice: Arc<dyn EventConsumer> = Arc::new(NoticeConsumer {
        sender: notices,
        disabled: cfg.code_sender == CodeSenderConfig::Disabled,
    });
    for t in AUDIT_EVENT_TYPES {
        consumers.register(t, audit.clone());
    }
    for t in SSE_EVENT_TYPES {
        consumers.register(t, sse.clone());
    }
    consumers.register(NOTICE_EVENT_TYPE, notice);
    // The revocation's consumer is the gateway's, wired after identity: `App::wire` requires it.
    let emitted: Vec<&str> = identity_application::changes::emitted_event_types()
        .filter(|t| *t != DEVICE_TOKEN_REVOKED_EVENT_TYPE)
        .collect();
    consumers.require(&emitted, crate::unconsumed::UNCONSUMED)?;
    let router = identity_infra::http::router(Arc::new(IdentityHttpState {
        deps: deps.clone(),
        public_origin: public_origin.clone(),
        trust_forwarded_for: config.trust_forwarded_for,
        events,
    }));
    let access = Arc::new(IdentityAccess::new(deps.clone()));
    Ok(IdentityWiring {
        deps,
        router,
        authenticator: access.clone(),
        step_up: access,
        public_origin,
        delivery,
        sender,
    })
}
