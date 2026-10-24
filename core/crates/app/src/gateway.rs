//! Wiring of the gateway (S2): the `/adapter/v1` WebSocket, the query ports the gateway serves
//! to accounts (platform accounts, ADR-0008) and routing (position, account kind, out-of-sync
//! gate), the ports it consumes (bindings from accounts, limits from risk) and its outbox
//! consumers (C1 commands, C2 token revocation, connection presence for identity).
//!
//! Risk (S13) is not built: without the `dev-stubs` feature the gateway refuses every adapter
//! session (fail closed). With it, a simulation-only stand-in answers "no limits, no commission
//! floor" outside production; it shows only in the operator log, never in the audit.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

use accounts_application::enterprise::{
    ConnectionId as AccountsConnectionId, VenueAccountId as AccountsVenueId,
};
use accounts_application::ports::{
    BindingView, PortError as AccountsPortError, VenueAccountFacts, VenueAccounts, VenueKind,
};
use accounts_application::usecases::ReadAccounts;
use async_trait::async_trait;
use axum::Router;
use gateway_application::consume::{
    CommandsOutcome, CommandsRequested, ConsumeCommandsRequested, ConsumeError, DeviceTokenRevoked,
    on_token_revoked,
};
use gateway_application::deliver::DeliverCommands;
use gateway_application::disconnect::HandleDisconnect;
use gateway_application::events::{AUDIT_CODES, CONNECTION_SESSION_CHANGED};
use gateway_application::frames::{CloseCode, HandleFrame};
use gateway_application::hello::HandleHello;
use gateway_application::intake::HandleExecutionReport;
use gateway_application::ports::{
    AccountBindings, BoundAccount, DerivedLimits, DeviceTokens, GatewayStore,
    PortError as GatewayPortError, RiskLimits,
};
use gateway_application::session::{SessionSettings, Sessions};
use gateway_application::snapshot::HandlePositionSnapshot;
use gateway_enterprise::ids::{AccountId as GatewayAccountId, ConnectionId, VenueAccountId};
use gateway_enterprise::instrument::Instrument;
use gateway_enterprise::position::ExpectedPosition as GatewayPosition;
use gateway_enterprise::sync::SyncState;
use gateway_infra::ws::Control;
use gateway_infra::{GatewayRuntime, PgGatewayStore, ProtocolSchema};
use identity_application::ports::IdentityStore;
use identity_application::usecases::connections::RecordConnectionLiveState;
use identity_enterprise::connection::LiveState;
use routing_application::decisions::{
    AccountKind, AccountKindOf, Contract, EntryGate, ExpectedPosition, PositionFor,
};
use routing_application::enterprise::AccountId as RoutingAccountId;
use routing_application::routes::PortError as RoutingPortError;
use shared_kernel::{Clock, Money, UserId};
use sqlx::PgPool;

use crate::outbox::{Consumers, EventConsumer, OutboxEvent, StartPolicy};

/// How long an opening leg waits for the flatten it depends on, from that flatten's `sent_at`
/// (ADR-0004 §5: "within its normal report timeout").
const PREREQUISITE_TIMEOUT_S: i64 = 30;

/// The gateway store, shared by the accounts adapter (built before accounts) and the gateway.
pub fn store(db: &PgPool) -> Arc<dyn GatewayStore> {
    Arc::new(PgGatewayStore::new(db.clone()))
}

fn gw_err(e: impl std::fmt::Display) -> GatewayPortError {
    GatewayPortError(e.to_string())
}

/// accounts' `ReadAccounts`, set once accounts is wired (accounts needs [`GatewayVenues`]
/// first). Unset means accounts is not wired: every read fails closed.
pub type AccountsReads = Arc<OnceLock<ReadAccounts>>;

// ---- served to accounts (ADR-0008) ---------------------------------------------------------

/// `VenueAccounts` for accounts: the platform accounts the gateway has seen in `hello`, with the
/// connection's name and status from identity.
pub struct GatewayVenues {
    pub store: Arc<dyn GatewayStore>,
    pub identity: Arc<dyn IdentityStore>,
    pub accounts: AccountsReads,
}

fn venue_kind(kind: &str) -> VenueKind {
    match kind {
        "simulation" => VenueKind::Simulation,
        "playback" => VenueKind::Playback,
        "live" => VenueKind::Live,
        _ => VenueKind::Unknown,
    }
}

fn acc_err(e: impl std::fmt::Display) -> AccountsPortError {
    AccountsPortError(e.to_string())
}

#[async_trait]
impl VenueAccounts for GatewayVenues {
    async fn resolve(
        &self,
        user_id: UserId,
        connection_id: AccountsConnectionId,
        platform_account: &str,
    ) -> Result<Option<VenueAccountFacts>, AccountsPortError> {
        // Only a paired connection of this user resolves (unpaired = absent).
        let paired = self
            .identity
            .connection(
                user_id,
                identity_enterprise::ids::ConnectionId::from_uuid(connection_id.as_uuid()),
            )
            .await
            .map_err(acc_err)?
            .is_some_and(|r| r.connection.unpaired_at.is_none());
        if !paired {
            return Ok(None);
        }
        let venue = self
            .store
            .venue_by_name(
                user_id,
                ConnectionId::from_uuid(connection_id.as_uuid()),
                platform_account,
            )
            .await
            .map_err(acc_err)?;
        Ok(venue.map(|v| VenueAccountFacts {
            venue_account_id: AccountsVenueId::from_uuid(v.id.as_uuid()),
            kind: venue_kind(&v.kind),
        }))
    }

    async fn describe(
        &self,
        user_id: UserId,
        ids: &[AccountsVenueId],
    ) -> Result<Vec<BindingView>, AccountsPortError> {
        let ids: Vec<VenueAccountId> = ids
            .iter()
            .map(|i| VenueAccountId::from_uuid(i.as_uuid()))
            .collect();
        let venues = self.store.venues(user_id, &ids).await.map_err(acc_err)?;
        let mut out = Vec::with_capacity(venues.len());
        for (connection, venue) in venues {
            let Some(record) = self
                .identity
                .connection(
                    user_id,
                    identity_enterprise::ids::ConnectionId::from_uuid(connection.as_uuid()),
                )
                .await
                .map_err(acc_err)?
            else {
                continue;
            };
            out.push(BindingView {
                venue_account_id: AccountsVenueId::from_uuid(venue.id.as_uuid()),
                connection_id: AccountsConnectionId::from_uuid(connection.as_uuid()),
                connection_name: record.connection.name.as_str().to_owned(),
                connection_status: record.connection.status.as_str().to_owned(),
                platform_account: venue.platform_account,
                kind: venue.kind,
                oco_location: venue.oco_location,
            });
        }
        Ok(out)
    }

    async fn has_open_position(
        &self,
        user_id: UserId,
        venue_account_id: AccountsVenueId,
    ) -> Result<bool, AccountsPortError> {
        let reads = self
            .accounts
            .get()
            .ok_or_else(|| AccountsPortError("accounts not wired".into()))?;
        let bound = reads
            .accounts_for_venues(user_id, &[venue_account_id])
            .await
            .map_err(acc_err)?;
        let Some(account) = bound.get(&venue_account_id) else {
            return Ok(open_position_answer(None, &[]));
        };
        let account = GatewayAccountId::from_uuid(account.account_id.as_uuid());
        let sync = self
            .store
            .account_sync(user_id, account)
            .await
            .map_err(acc_err)?;
        let open = self
            .store
            .open_positions(user_id, account)
            .await
            .map_err(acc_err)?;
        Ok(open_position_answer(Some(sync.state()), &open))
    }
}

/// `has_open_position` never answers "flat" on uncertainty (Brasa M2) and never fails on it
/// either (Grosa H-1: an error fails `listAccounts` for every account and blocks the pause):
/// a venue no longer bound to an account (a rebind race) or an account out of sync reads
/// "possibly open". Rebind, archive and delete stay refused; pause and disable work. A
/// three-state port is a follow-up (Brasa/Esquadro).
fn open_position_answer(sync: Option<SyncState>, open: &[(Instrument, i64)]) -> bool {
    match sync {
        Some(SyncState::InSync) => open.iter().any(|(_, q)| *q != 0),
        Some(SyncState::OutOfSync) | None => true,
    }
}

// ---- consumed from accounts ----------------------------------------------------------------

/// `AccountBindings` over accounts' `ReadAccounts` (Malha, 0e5b6bd).
pub struct AccountsBindings(pub AccountsReads);

impl AccountsBindings {
    fn reads(&self) -> Result<&ReadAccounts, GatewayPortError> {
        self.0
            .get()
            .ok_or_else(|| GatewayPortError("accounts not wired".into()))
    }
}

#[async_trait]
impl AccountBindings for AccountsBindings {
    async fn accounts_for_venues(
        &self,
        user_id: UserId,
        venues: &[VenueAccountId],
    ) -> Result<HashMap<VenueAccountId, BoundAccount>, GatewayPortError> {
        let ids: Vec<AccountsVenueId> = venues
            .iter()
            .map(|v| AccountsVenueId::from_uuid(v.as_uuid()))
            .collect();
        let bound = self
            .reads()?
            .accounts_for_venues(user_id, &ids)
            .await
            .map_err(gw_err)?;
        Ok(bound
            .into_iter()
            .map(|(v, a)| {
                (
                    VenueAccountId::from_uuid(v.as_uuid()),
                    BoundAccount {
                        account_id: GatewayAccountId::from_uuid(a.account_id.as_uuid()),
                        display_name: a.display_name.as_str().to_owned(),
                    },
                )
            })
            .collect())
    }

    async fn binding_of(
        &self,
        user_id: UserId,
        account: GatewayAccountId,
    ) -> Result<Option<VenueAccountId>, GatewayPortError> {
        let venue = self
            .reads()?
            .binding_of(
                user_id,
                accounts_application::enterprise::AccountId::from_uuid(account.as_uuid()),
            )
            .await
            .map_err(gw_err)?;
        Ok(venue.map(|v| VenueAccountId::from_uuid(v.as_uuid())))
    }
}

// ---- served to routing (S7) ----------------------------------------------------------------

fn rt_err(e: impl std::fmt::Display) -> RoutingPortError {
    RoutingPortError(e.to_string())
}

fn gateway_account(account: RoutingAccountId) -> GatewayAccountId {
    GatewayAccountId::from_uuid(account.as_uuid())
}

/// Routing's reads of the gateway's state: expected position, platform account kind and the
/// out-of-sync gate (6). Each is read at the decision, never cached (Vigia C2).
#[derive(Clone)]
pub struct GatewayReads {
    store: Arc<dyn GatewayStore>,
    bindings: Arc<AccountsBindings>,
}

#[async_trait]
impl PositionFor for GatewayReads {
    async fn position(
        &self,
        user: UserId,
        account: RoutingAccountId,
        contract: &Contract,
    ) -> Result<ExpectedPosition, RoutingPortError> {
        let instrument = Instrument::new(&contract.symbol, &contract.month).map_err(rt_err)?;
        let history = self
            .store
            .position_history(user, gateway_account(account), &instrument)
            .await
            .map_err(rt_err)?;
        let expected = GatewayPosition::flat().replay(&history);
        // An unknown position is never read as flat: the decision waits (fail closed).
        let quantity = expected
            .quantity()
            .ok_or_else(|| RoutingPortError("position unknown".into()))?;
        Ok(ExpectedPosition {
            quantity,
            position_ref: expected.position_ref().map(|c| c.as_uuid().to_string()),
        })
    }
}

#[async_trait]
impl AccountKindOf for GatewayReads {
    async fn kind(
        &self,
        user: UserId,
        account: RoutingAccountId,
    ) -> Result<AccountKind, RoutingPortError> {
        let Some(venue) = self
            .bindings
            .binding_of(user, gateway_account(account))
            .await
            .map_err(rt_err)?
        else {
            return Ok(AccountKind::Unknown);
        };
        let Some((_, v)) = self
            .store
            .venue_account(user, venue)
            .await
            .map_err(rt_err)?
        else {
            return Ok(AccountKind::Unknown);
        };
        Ok(match v.kind.as_str() {
            "simulation" => AccountKind::Simulation,
            "playback" => AccountKind::Playback,
            "live" => AccountKind::Live,
            _ => AccountKind::Unknown,
        })
    }
}

#[async_trait]
impl EntryGate for GatewayReads {
    /// Gate 6: barred while the account is out of sync (ADR-0004 §6).
    async fn barred(
        &self,
        user: UserId,
        _contract: &Contract,
        accounts: &[RoutingAccountId],
    ) -> Result<HashMap<RoutingAccountId, bool>, RoutingPortError> {
        let mut out = HashMap::with_capacity(accounts.len());
        for a in accounts {
            let sync = self
                .store
                .account_sync(user, gateway_account(*a))
                .await
                .map_err(rt_err)?;
            out.insert(*a, sync.state() == SyncState::OutOfSync);
        }
        Ok(out)
    }
}

// ---- consumed from risk (S13 not built) ----------------------------------------------------

/// Without `dev-stubs`: a session with a bound account gets no `welcome` (it needs the limits)
/// and no fill is recorded (it needs the commission floor). Never "no limits" by default.
pub struct RiskUnavailable;

#[async_trait]
impl RiskLimits for RiskUnavailable {
    async fn current_limits(
        &self,
        _: UserId,
        _: &[GatewayAccountId],
    ) -> Result<HashMap<GatewayAccountId, DerivedLimits>, GatewayPortError> {
        Err(GatewayPortError("risk unavailable".into()))
    }

    async fn commission_rate(
        &self,
        _: UserId,
        _: GatewayAccountId,
    ) -> Result<Option<Money>, GatewayPortError> {
        Err(GatewayPortError("risk unavailable".into()))
    }
}

#[cfg(feature = "dev-stubs")]
pub mod dev_stubs {
    //! Test-build stand-in for the gateway's risk port until S13, under the same rules as
    //! `routing_application::dev_stubs` (Esquadro; Vigia C2): it answers only outside
    //! production AND for an account whose platform account is simulation or playback, read at
    //! each call (never cached). Anything else, an unknown kind or a failed read included,
    //! fails closed. It shows only in the operator log, never in the audit.

    use routing_application::dev_stubs::Environment;

    use super::*;

    /// In every operator log line and in the binary: release checks look for it.
    pub const STAND_IN_MARKER: &str = "allwin-dev-stub:gateway-risk";

    /// No limits frame (the plugin keeps its cache; routing's `limits_pending` stand-in gates
    /// entries) and no commission floor (net = gross, `commission_source: none`).
    pub struct SimulationRisk {
        pub environment: Environment,
        pub kinds: Arc<dyn AccountKindOf>,
    }

    impl SimulationRisk {
        async fn allowed(&self, user: UserId, account: GatewayAccountId) -> Result<(), String> {
            if self.environment != Environment::NonProduction {
                return Err("risk unavailable".into());
            }
            let kind = self
                .kinds
                .kind(user, RoutingAccountId::from_uuid(account.as_uuid()))
                .await
                .map_err(|e| e.0)?;
            match kind {
                AccountKind::Simulation | AccountKind::Playback => Ok(()),
                AccountKind::Live | AccountKind::Unknown => {
                    Err("risk unavailable: not a simulation account".into())
                }
            }
        }
    }

    #[async_trait]
    impl RiskLimits for SimulationRisk {
        async fn current_limits(
            &self,
            user: UserId,
            accounts: &[GatewayAccountId],
        ) -> Result<HashMap<GatewayAccountId, DerivedLimits>, GatewayPortError> {
            // Production fails before any read, also for an empty account list (Grosa L-2).
            if self.environment != Environment::NonProduction {
                return Err(GatewayPortError("risk unavailable".into()));
            }
            for a in accounts {
                self.allowed(user, *a).await.map_err(GatewayPortError)?;
            }
            tracing::warn!(marker = STAND_IN_MARKER, "gateway limits from the stand-in");
            Ok(HashMap::new())
        }

        async fn commission_rate(
            &self,
            user: UserId,
            account: GatewayAccountId,
        ) -> Result<Option<Money>, GatewayPortError> {
            self.allowed(user, account)
                .await
                .map_err(GatewayPortError)?;
            tracing::warn!(
                marker = STAND_IN_MARKER,
                "gateway commission floor from the stand-in"
            );
            Ok(None)
        }
    }
}

fn risk(app_env: Option<&str>, kinds: Arc<dyn AccountKindOf>) -> Arc<dyn RiskLimits> {
    #[cfg(feature = "dev-stubs")]
    {
        let environment = routing_application::dev_stubs::Environment::from_app_env(app_env);
        tracing::warn!(
            marker = dev_stubs::STAND_IN_MARKER,
            ?environment,
            "gateway risk port is a test-build stand-in"
        );
        Arc::new(dev_stubs::SimulationRisk { environment, kinds })
    }
    #[cfg(not(feature = "dev-stubs"))]
    {
        let _ = (app_env, kinds);
        Arc::new(RiskUnavailable)
    }
}

// ---- operational metrics -------------------------------------------------------------------

/// Why C1 dropped a `routing.commands_requested` without storing it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropReason {
    /// The payload does not parse as `CommandsRequested`.
    Malformed,
    /// It parses but the gateway refuses it (`ConsumeError::Invalid`).
    Invalid,
}

impl DropReason {
    const ALL: [Self; 2] = [Self::Malformed, Self::Invalid];

    const fn label(self) -> &'static str {
        match self {
            Self::Malformed => "malformed",
            Self::Invalid => "invalid",
        }
    }
}

/// Gateway counters for `/metrics` (no users, accounts or tokens). Alert on any increase of
/// `allwin_gateway_commands_dropped_total`: a routed command that will never reach the plugin
/// (Esquadro, M3 condition (a)).
#[derive(Debug, Default)]
pub struct GatewayMetrics {
    malformed: AtomicU64,
    invalid: AtomicU64,
}

impl GatewayMetrics {
    fn counter(&self, reason: DropReason) -> &AtomicU64 {
        match reason {
            DropReason::Malformed => &self.malformed,
            DropReason::Invalid => &self.invalid,
        }
    }

    pub fn dropped(&self, reason: DropReason) -> u64 {
        self.counter(reason).load(Ordering::Relaxed)
    }

    fn record_drop(&self, reason: DropReason) {
        self.counter(reason).fetch_add(1, Ordering::Relaxed);
    }

    /// Prometheus text format, appended to the app's `/metrics`.
    pub fn render(&self, out: &mut String) {
        out.push_str("# TYPE allwin_gateway_commands_dropped_total counter\n");
        for reason in DropReason::ALL {
            out.push_str(&format!(
                "allwin_gateway_commands_dropped_total{{reason=\"{}\"}} {}\n",
                reason.label(),
                self.dropped(reason)
            ));
        }
    }
}

/// The `command_id`s an event names, read leniently (the event may not parse).
fn named_command_ids(payload: &serde_json::Value) -> Vec<String> {
    payload["commands"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|c| c["command_id"].as_str().map(str::to_owned))
        .collect()
}

/// The operational alert of a dropped C1 event: counted, and logged as an ERROR with the
/// `alert` key the log alerting matches, the event id and the command ids (never the body).
fn alert_dropped(
    metrics: &GatewayMetrics,
    event_id: uuid::Uuid,
    payload: &serde_json::Value,
    reason: DropReason,
    why: &str,
) {
    metrics.record_drop(reason);
    tracing::error!(
        alert = "gateway.commands_dropped",
        reason = reason.label(),
        %event_id,
        command_ids = ?named_command_ids(payload),
        %why,
        "routing.commands_requested dropped: these commands will not reach the plugin"
    );
}

// ---- outbox consumers ----------------------------------------------------------------------

/// C1: `routing.commands_requested` → command rows, then wake the connection's session.
/// Order across instructions is the outbox's, which is not strict during a retry backoff
/// (Brasa M3, PROVISIONAL for Sunday: Sim only; a per-account ordered loop is the follow-up).
struct CommandsConsumer {
    consume: ConsumeCommandsRequested,
    runtime: Arc<GatewayRuntime>,
    metrics: Arc<GatewayMetrics>,
}

#[async_trait]
impl EventConsumer for CommandsConsumer {
    fn name(&self) -> &'static str {
        "gateway.commands"
    }

    /// Side effect (session wake) after each event.
    fn batch(&self) -> i64 {
        1
    }

    async fn handle(&self, event: &OutboxEvent) -> Result<(), String> {
        let parsed: CommandsRequested = match serde_json::from_value(event.payload.clone()) {
            Ok(p) => p,
            Err(e) => {
                // A malformed event never becomes valid: not retried (Brasa M3).
                alert_dropped(
                    &self.metrics,
                    event.id,
                    &event.payload,
                    DropReason::Malformed,
                    &e.to_string(),
                );
                return Ok(());
            }
        };
        match self.consume.handle(event.user_id, &parsed).await {
            Ok(CommandsOutcome::Enqueued { connection, .. }) => {
                self.runtime.signal(connection, Control::Wake);
                Ok(())
            }
            Ok(CommandsOutcome::Unbound) => {
                tracing::warn!(event_id = %event.id, instruction_id = %parsed.instruction_id, "commands for an unbound account; not stored");
                Ok(())
            }
            Err(ConsumeError::Invalid(why)) => {
                alert_dropped(
                    &self.metrics,
                    event.id,
                    &event.payload,
                    DropReason::Invalid,
                    &why,
                );
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

/// Whether a revocation closes this live session (Brasa M1): only a session of the same user
/// whose own token no longer validates. After a re-pair the plugin may already be back with the
/// new token; that session stays. A failed check is retried.
fn revocation_closes(
    same_user: bool,
    still_valid: Result<bool, GatewayPortError>,
) -> Result<bool, String> {
    if !same_user {
        return Ok(false);
    }
    match still_valid {
        Ok(valid) => Ok(!valid),
        Err(e) => Err(e.0),
    }
}

/// C2: `identity.device_token_revoked` → close the connection's live session with 4002, when
/// its token is the revoked one.
struct RevocationConsumer {
    runtime: Arc<GatewayRuntime>,
    sessions: Sessions,
    tokens: Arc<dyn DeviceTokens>,
}

#[async_trait]
impl EventConsumer for RevocationConsumer {
    fn name(&self) -> &'static str {
        "gateway.token_revocations"
    }

    /// A revocation only matters to a live session.
    fn start(&self) -> StartPolicy {
        StartPolicy::FromNow
    }

    async fn handle(&self, event: &OutboxEvent) -> Result<(), String> {
        let parsed: DeviceTokenRevoked =
            serde_json::from_value(event.payload.clone()).map_err(|e| e.to_string())?;
        let Some(session) = on_token_revoked(&self.sessions, &parsed) else {
            return Ok(());
        };
        let still_valid = self.tokens.still_valid(&session.key_id).await;
        if revocation_closes(session.user_id == event.user_id, still_valid)? {
            self.runtime
                .signal(session.connection_id, Control::Close(CloseCode::Revoked));
        }
        Ok(())
    }
}

/// C3: `account.binding_changed` → nothing on the live session: a bind or rebind applies at the
/// next handshake (Esquadro X2). Registered so the type is consumed, not allowlisted.
struct BindingConsumer;

#[async_trait]
impl EventConsumer for BindingConsumer {
    fn name(&self) -> &'static str {
        "gateway.bindings"
    }

    fn start(&self) -> StartPolicy {
        StartPolicy::FromNow
    }

    async fn handle(&self, event: &OutboxEvent) -> Result<(), String> {
        gateway_application::consume::on_binding_changed(&event.payload);
        Ok(())
    }
}

/// `gateway.connection_session_changed` → identity's connection status (connected,
/// disconnected, outdated) through `RecordConnectionLiveState`.
struct PresenceConsumer(RecordConnectionLiveState);

#[async_trait]
impl EventConsumer for PresenceConsumer {
    fn name(&self) -> &'static str {
        "identity.connection_presence"
    }

    fn start(&self) -> StartPolicy {
        StartPolicy::FromNow
    }

    async fn handle(&self, event: &OutboxEvent) -> Result<(), String> {
        let p = &event.payload;
        let connection = p["connection_id"]
            .as_str()
            .and_then(|s| uuid::Uuid::parse_str(s).ok())
            .ok_or("connection_id")?;
        let state = match p["status"].as_str() {
            Some("connected") => LiveState::Connected,
            Some("disconnected") => LiveState::Disconnected,
            Some("outdated") => LiveState::Outdated,
            _ => return Err("status".into()),
        };
        self.0
            .handle(
                event.user_id,
                identity_enterprise::ids::ConnectionId::from_uuid(connection),
                state,
                event.occurred_at,
            )
            .await
            .map_err(|e| e.to_string())
    }
}

// ---- wiring --------------------------------------------------------------------------------

pub struct GatewayDeps {
    pub store: Arc<dyn GatewayStore>,
    pub tokens: Arc<dyn DeviceTokens>,
    /// Set by the caller once accounts is wired.
    pub accounts: AccountsReads,
    /// identity's handler of adapter-session events.
    pub presence: RecordConnectionLiveState,
    pub clock: Arc<dyn Clock>,
    /// `APP_ENV`: the dev-stubs stand-in answers only for `development` / `test` (absent or
    /// anything else is production; `routing_application::dev_stubs::Environment`).
    pub app_env: Option<String>,
    /// Failed device-token authentications per IP and minute: the ingress's
    /// `ip_token_failures_per_min` (Crivo g1).
    pub token_failures_per_min: u32,
    /// The last `X-Forwarded-For` hop is the client (behind the trusted proxy).
    pub trust_forwarded_for: bool,
}

/// What `app` keeps from the gateway after wiring.
pub struct GatewayWiring {
    /// `GET /adapter/v1`, outside the cookie-session routes (a device token, not a session).
    pub router: Router,
    /// For routing (S7): `PositionFor`, `AccountKindOf` and entry gate 6.
    pub reads: GatewayReads,
    /// Rendered by the app's `/metrics`.
    pub metrics: Arc<GatewayMetrics>,
}

/// Builds the gateway and registers its consumers and its audited event types.
pub fn wire(
    deps: GatewayDeps,
    audit: Arc<dyn EventConsumer>,
    consumers: &mut Consumers,
) -> Result<GatewayWiring, String> {
    let schema = Arc::new(ProtocolSchema::load()?);
    let store = deps.store;
    let bindings = Arc::new(AccountsBindings(deps.accounts));
    let reads = GatewayReads {
        store: store.clone(),
        bindings: bindings.clone(),
    };
    let risk = risk(deps.app_env.as_deref(), Arc::new(reads.clone()));
    let sessions = Sessions::default();
    // V1 is simulation only (ADR-0004 §8 environment guard), in every environment.
    let settings = SessionSettings::default();
    let intake = HandleExecutionReport::new(store.clone(), bindings.clone(), risk.clone());
    let snapshot = HandlePositionSnapshot::new(store.clone(), bindings.clone(), sessions.clone());
    let runtime = Arc::new(
        GatewayRuntime::new(
            sessions.clone(),
            deps.tokens.clone(),
            schema.clone(),
            HandleHello::new(store.clone(), bindings.clone(), risk, settings.clone()),
            HandleFrame::new(schema, store.clone(), intake, snapshot),
            DeliverCommands::new(
                store.clone(),
                settings.clone(),
                chrono::Duration::seconds(PREREQUISITE_TIMEOUT_S),
            ),
            HandleDisconnect::new(store.clone(), sessions.clone()),
            settings,
            deps.clock,
        )
        .with_failure_budget(deps.token_failures_per_min, deps.trust_forwarded_for),
    );

    let metrics = Arc::new(GatewayMetrics::default());
    consumers.register(
        "routing.commands_requested",
        Arc::new(CommandsConsumer {
            consume: ConsumeCommandsRequested::new(store.clone(), bindings.clone()),
            runtime: runtime.clone(),
            metrics: metrics.clone(),
        }),
    );
    consumers.register(
        "identity.device_token_revoked",
        Arc::new(RevocationConsumer {
            runtime: runtime.clone(),
            sessions,
            tokens: deps.tokens,
        }),
    );
    consumers.register("account.binding_changed", Arc::new(BindingConsumer));
    consumers.register(
        CONNECTION_SESSION_CHANGED,
        Arc::new(PresenceConsumer(deps.presence)),
    );
    for code in AUDIT_CODES {
        consumers.register(code, audit.clone());
    }

    Ok(GatewayWiring {
        router: gateway_infra::router(runtime),
        reads,
        metrics,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn venue_kinds_never_default_to_simulation() {
        assert_eq!(venue_kind("simulation"), VenueKind::Simulation);
        assert_eq!(venue_kind("playback"), VenueKind::Playback);
        assert_eq!(venue_kind("live"), VenueKind::Live);
        for other in ["unknown", "", "Simulation", "sim"] {
            assert_eq!(venue_kind(other), VenueKind::Unknown, "{other}");
        }
    }

    #[tokio::test]
    async fn without_risk_every_risk_read_fails_closed() {
        let r = RiskUnavailable;
        assert!(
            r.current_limits(UserId::from_uuid(uuid::Uuid::nil()), &[])
                .await
                .is_err()
        );
        assert!(
            r.commission_rate(
                UserId::from_uuid(uuid::Uuid::nil()),
                GatewayAccountId::from_uuid(uuid::Uuid::nil())
            )
            .await
            .is_err()
        );
    }

    #[tokio::test]
    async fn unwired_accounts_fail_closed() {
        let b = AccountsBindings(Arc::new(OnceLock::new()));
        assert!(
            b.binding_of(
                UserId::from_uuid(uuid::Uuid::nil()),
                GatewayAccountId::from_uuid(uuid::Uuid::nil())
            )
            .await
            .is_err()
        );
    }

    #[test]
    fn a_revocation_closes_only_a_session_whose_token_is_revoked() {
        // Re-paired: the live session already uses the new, valid token.
        assert_eq!(revocation_closes(true, Ok(true)), Ok(false));
        assert_eq!(revocation_closes(true, Ok(false)), Ok(true));
        // Another user's event never closes this session.
        assert_eq!(revocation_closes(false, Ok(false)), Ok(false));
        // The check failed: retried, nothing closed now.
        assert!(revocation_closes(true, Err(GatewayPortError("down".into()))).is_err());
    }

    #[cfg(feature = "dev-stubs")]
    mod stand_in {
        use routing_application::dev_stubs::Environment;

        use super::super::dev_stubs::SimulationRisk;
        use super::*;

        struct Kinds(Result<AccountKind, String>);

        #[async_trait]
        impl AccountKindOf for Kinds {
            async fn kind(
                &self,
                _: UserId,
                _: RoutingAccountId,
            ) -> Result<AccountKind, RoutingPortError> {
                self.0.clone().map_err(RoutingPortError)
            }
        }

        fn risk(environment: Environment, kind: Result<AccountKind, String>) -> SimulationRisk {
            SimulationRisk {
                environment,
                kinds: Arc::new(Kinds(kind)),
            }
        }

        /// Brasa H1 / Vigia C2: only simulation or playback, only outside production.
        #[tokio::test]
        async fn the_stand_in_answers_only_for_simulation_accounts_outside_production() {
            let user = UserId::from_uuid(uuid::Uuid::nil());
            let account = GatewayAccountId::from_uuid(uuid::Uuid::nil());
            for kind in [AccountKind::Simulation, AccountKind::Playback] {
                let r = risk(Environment::NonProduction, Ok(kind));
                assert!(r.current_limits(user, &[account]).await.is_ok(), "{kind:?}");
                assert_eq!(r.commission_rate(user, account).await.ok(), Some(None));
            }
            for kind in [
                Ok(AccountKind::Live),
                Ok(AccountKind::Unknown),
                Err("down".into()),
            ] {
                let r = risk(Environment::NonProduction, kind.clone());
                assert!(
                    r.current_limits(user, &[account]).await.is_err(),
                    "{kind:?}"
                );
                assert!(r.commission_rate(user, account).await.is_err(), "{kind:?}");
            }
            let r = risk(Environment::Production, Ok(AccountKind::Simulation));
            assert!(r.current_limits(user, &[account]).await.is_err());
            assert!(r.commission_rate(user, account).await.is_err());
        }
    }

    /// Esquadro M3 (a): a dropped C1 event is counted per reason and named in the alert.
    #[test]
    fn dropped_commands_are_counted_and_rendered() {
        let m = GatewayMetrics::default();
        let payload = serde_json::json!({"commands": [{"command_id": "c-1"}, {"x": 1}]});
        assert_eq!(named_command_ids(&payload), ["c-1"]);
        let id = uuid::Uuid::nil();
        alert_dropped(&m, id, &payload, DropReason::Malformed, "bad");
        alert_dropped(&m, id, &payload, DropReason::Invalid, "bad");
        alert_dropped(&m, id, &payload, DropReason::Invalid, "bad");
        assert_eq!(m.dropped(DropReason::Malformed), 1);
        assert_eq!(m.dropped(DropReason::Invalid), 2);
        let mut out = String::new();
        m.render(&mut out);
        assert!(out.contains("allwin_gateway_commands_dropped_total{reason=\"malformed\"} 1"));
        assert!(out.contains("allwin_gateway_commands_dropped_total{reason=\"invalid\"} 2"));
        assert!(named_command_ids(&serde_json::json!("not an object")).is_empty());
    }

    /// Grosa H-1: uncertainty reads "possibly open", never an error and never "flat".
    #[test]
    fn has_open_position_is_possibly_open_when_unsure() {
        let mnq = Instrument::new("MNQ", "2026-12").unwrap();
        assert!(!open_position_answer(Some(SyncState::InSync), &[]));
        assert!(!open_position_answer(
            Some(SyncState::InSync),
            &[(mnq.clone(), 0)]
        ));
        assert!(open_position_answer(
            Some(SyncState::InSync),
            &[(mnq.clone(), -1)]
        ));
        assert!(open_position_answer(Some(SyncState::OutOfSync), &[]));
        assert!(open_position_answer(None, &[]), "venue no longer bound");
    }
}
