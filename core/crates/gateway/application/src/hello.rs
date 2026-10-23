//! Handshake (ADR-0004 §3): `hello` → `welcome` → `limits` for every mapped account.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use gateway_enterprise::ids::{AccountId, SessionId};
use gateway_enterprise::protocol::{ProtocolVersion, VersionError};
use gateway_enterprise::sync::SyncState;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::events::{self, DisconnectReason, SessionStatus};
use crate::ports::{
    AccountBindings, AdapterPrincipal, GatewayStore, OutboundMessage, PortError, RiskLimits,
    StoreError, UnitOfWork, VisiblePlatformAccount,
};
use crate::session::{MappedAccount, Session, SessionSettings};
use crate::wire::{self, Hello, Welcome, WelcomeAccount};

#[derive(Debug, thiserror::Error)]
pub enum HelloError {
    #[error(transparent)]
    Port(#[from] PortError),
    #[error(transparent)]
    Store(#[from] StoreError),
}

pub enum HelloOutcome {
    /// Write `welcome`, then the `limits` frame (already stored as reliable), then start
    /// delivery; the session is registered by the caller.
    Welcome {
        session: Arc<Session>,
        welcome_frame: String,
        limits: Option<OutboundMessage>,
    },
    /// Close 4003; the connection shows as outdated.
    Outdated,
}

#[derive(Clone)]
pub struct HandleHello {
    store: Arc<dyn GatewayStore>,
    bindings: Arc<dyn AccountBindings>,
    risk: Arc<dyn RiskLimits>,
    settings: SessionSettings,
}

impl HandleHello {
    pub fn new(
        store: Arc<dyn GatewayStore>,
        bindings: Arc<dyn AccountBindings>,
        risk: Arc<dyn RiskLimits>,
        settings: SessionSettings,
    ) -> Self {
        Self {
            store,
            bindings,
            risk,
            settings,
        }
    }

    pub async fn handle(
        &self,
        principal: &AdapterPrincipal,
        hello: &Hello,
        now: DateTime<Utc>,
    ) -> Result<HelloOutcome, HelloError> {
        let offered: Vec<ProtocolVersion> = hello
            .supported_protocol_versions
            .iter()
            .filter_map(|v| ProtocolVersion::parse(v).ok())
            .collect();
        let session_id = SessionId::from_uuid(Uuid::now_v7());
        let version = match ProtocolVersion::negotiate(&offered) {
            Ok(v) => v,
            Err(VersionError::Unsupported | VersionError::Invalid) => {
                let event = events::connection_session_changed(
                    principal.connection_id,
                    session_id,
                    SessionStatus::Outdated,
                    Some(DisconnectReason::Outdated),
                    None,
                    &hello.adapter.version,
                    hello.instance_id,
                    now,
                );
                self.store
                    .commit(
                        principal.user_id,
                        UnitOfWork {
                            events: vec![event],
                            ..UnitOfWork::default()
                        },
                    )
                    .await?;
                return Ok(HelloOutcome::Outdated);
            }
        };

        let visible: Vec<VisiblePlatformAccount> = hello
            .accounts
            .iter()
            .map(|a| VisiblePlatformAccount {
                platform_account: a.platform_account.clone(),
                kind: a.kind.clone(),
                oco_location: a.oco_location.clone(),
            })
            .collect();
        let venues = self
            .store
            .upsert_venue_accounts(principal.user_id, principal.connection_id, &visible, now)
            .await?;
        let ids: Vec<_> = venues.iter().map(|v| v.id).collect();
        let bound = self
            .bindings
            .accounts_for_venues(principal.user_id, &ids)
            .await?;

        let mut events: Vec<crate::events::GatewayEvent> = Vec::new();
        let mut mapped = Vec::new();
        let mut refused = Vec::new();
        let mut welcome_accounts = Vec::new();
        for venue in venues {
            let Some(account) = bound.get(&venue.id) else {
                continue;
            };
            if let Some(previous) = &venue.previous_kind {
                // A bound account whose platform kind changed (ADR-0008 §3.5, QA Security I1).
                events.push(crate::events::audit(
                    "connection.account_kind_changed",
                    now,
                    json!({
                        "account_id": account.account_id,
                        "connection_id": principal.connection_id,
                        "platform_account": venue.platform_account,
                        "previous_kind": previous,
                        "kind": venue.kind,
                    }),
                    crate::audit::refs(account.account_id, Some(&account.display_name), None),
                ));
            }
            // Environment guard (ADR-0004 §8): outside production only Sim/Playback accounts are
            // mapped; the adapter then ignores the others (no commands, no reports).
            if self.settings.simulation_only
                && !matches!(venue.kind.as_str(), "simulation" | "playback")
            {
                refused.push(account.account_id);
                continue;
            }
            let sync = self
                .store
                .account_sync(principal.user_id, account.account_id)
                .await?;
            welcome_accounts.push(WelcomeAccount {
                account_id: account.account_id.as_uuid(),
                platform_account: venue.platform_account.clone(),
                roles: vec!["destination"],
                sync_state: match sync.state() {
                    SyncState::InSync => "in_sync",
                    SyncState::OutOfSync => "out_of_sync",
                },
                sync_reasons: sync
                    .wire_codes()
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                display_name: version
                    .supports_notifications()
                    .then(|| account.display_name.clone()),
            });
            mapped.push(MappedAccount {
                account_id: account.account_id,
                display_name: account.display_name.clone(),
                venue,
            });
        }

        let session = Arc::new(
            Session::new(
                session_id,
                principal.user_id,
                principal.connection_id,
                principal.key_id.clone(),
                version,
                hello.instance_id,
                hello.adapter.version.clone(),
                mapped,
            )
            .with_refused(refused),
        );

        let welcome = Welcome {
            session_id: session_id.as_uuid(),
            connection_id: principal.connection_id.as_uuid(),
            protocol_version: version.to_string(),
            heartbeat_interval_ms: self.settings.heartbeat_interval_ms,
            ack_timeout_ms: self.settings.ack_timeout_ms,
            simulation_only: self.settings.simulation_only,
            locale: principal.locale.clone(),
            accounts: welcome_accounts,
        };
        let welcome_payload =
            serde_json::to_value(&welcome).map_err(|e| PortError(e.to_string()))?;
        let welcome_frame =
            wire::envelope(version, Uuid::now_v7(), "welcome", now, &welcome_payload);

        let limits = self.limits_frame(&session, now).await?;
        if let Some(message) = &limits {
            self.store.enqueue_outbound(message).await?;
        }

        events.push(events::connection_session_changed(
            principal.connection_id,
            session_id,
            SessionStatus::Connected,
            None,
            Some(version.to_string()),
            &hello.adapter.version,
            hello.instance_id,
            now,
        ));
        self.store
            .commit(
                principal.user_id,
                UnitOfWork {
                    events,
                    ..UnitOfWork::default()
                },
            )
            .await?;

        Ok(HelloOutcome::Welcome {
            session,
            welcome_frame,
            limits,
        })
    }

    /// `limits` for every mapped account that risk has derived limits for. An account without
    /// derived limits is left out: the plugin keeps its cached ones and the Core's
    /// `routing.limits_pending` gate holds entries (Esquadro L1). Never invented.
    async fn limits_frame(
        &self,
        session: &Session,
        now: DateTime<Utc>,
    ) -> Result<Option<OutboundMessage>, HelloError> {
        let ids: Vec<AccountId> = session.accounts().map(|a| a.account_id).collect();
        if ids.is_empty() {
            return Ok(None);
        }
        let derived = self.risk.current_limits(session.user_id, &ids).await?;
        let items: Vec<Value> = session
            .accounts()
            .filter_map(|a| {
                let limits = derived.get(&a.account_id)?;
                session.should_send_limits(a.account_id, limits.version);
                Some(limits_item(a, limits))
            })
            .collect();
        if items.is_empty() {
            return Ok(None);
        }
        Ok(Some(outbound(
            session,
            "limits",
            json!({ "accounts": items }),
            now,
        )))
    }
}

pub(crate) fn limits_item(account: &MappedAccount, limits: &crate::ports::DerivedLimits) -> Value {
    let mut item = limits.body.clone();
    if let Value::Object(map) = &mut item {
        map.insert("account_id".into(), json!(account.account_id));
        map.insert(
            "platform_account".into(),
            json!(account.venue.platform_account),
        );
        map.insert("version".into(), json!(limits.version));
    }
    item
}

pub(crate) fn outbound(
    session: &Session,
    kind: &str,
    payload: Value,
    now: DateTime<Utc>,
) -> OutboundMessage {
    let msg_id = Uuid::now_v7();
    OutboundMessage {
        msg_id,
        user_id: session.user_id,
        connection_id: session.connection_id,
        kind: kind.to_owned(),
        frame: wire::envelope(session.version, msg_id, kind, now, &payload),
        payload,
        protocol_version: session.version,
        created_at: now,
    }
}
