//! Routing's ports that `routing.rs` takes from the caller: entry gates 7–8 (the test-build
//! stand-ins behind `dev-stubs`, or fail-closed until S13/S16), the connection state for the
//! route-status view, and the outbox types routing writes. The startup check refuses a stand-in
//! in production and a real gate next to `dev-stubs` (Vigia C5).

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use accounts_application::enterprise::AccountId as AccountsAccountId;
use async_trait::async_trait;
use gateway_application::ports::GatewayStore;
use gateway_enterprise::ids::VenueAccountId;
use identity_application::ports::IdentityStore;
use identity_enterprise::connection::ConnectionStatus;
use routing_application::decisions::{AccountKindOf, Contract, EntryGate, EntryGatePorts};
use routing_application::enterprise::AccountId;
use routing_application::routes::PortError;
use shared_kernel::UserId;
use uuid::Uuid;

use crate::config::AppEnv;
use crate::gateway::AccountsReads;
use crate::routing::AccountConnections;

/// Audit codes routing writes to the outbox: every decision code the audit registry knows, the
/// route change and the exits' `execution.command_requested`. Registering a code routing never
/// writes is harmless; a written code without a consumer fails startup.
/// TODO(Ferro): move into `routing_application` as its emitted list.
pub const ROUTING_AUDIT_CODES: &[&str] = &[
    "routing.subscription_changed",
    "execution.command_requested",
    "routing.routed",
    "routing.quantity_capped",
    "routing.out_of_sync",
    "routing.risk_blocked",
    "routing.max_position_reached",
    "routing.limits_pending",
    "routing.no_bracket_profile",
    "routing.expired",
    "routing.account_inactive",
    "routing.instrument_unavailable",
    "routing.platform_unbound",
    "routing.automation_not_accepted",
    "routing.copy_not_accepted",
    "routing.signal_type_not_selected",
    "routing.nothing_to_close",
    "routing.nothing_to_reduce",
    "routing.position_unknown",
    "routing.adapter_protocol_outdated",
    "routing.no_correlated_order",
    "routing.position_conflict",
];

/// Every outbox type routing writes (the gateway consumes the commands).
pub fn emitted() -> Vec<&'static str> {
    let mut all = ROUTING_AUDIT_CODES.to_vec();
    all.push(routing_application::commands::COMMANDS_REQUESTED);
    all
}

/// Gates 7 (risk) and 8 (limits) as wired in this build.
pub struct RiskGates {
    pub risk_blocked: Arc<dyn EntryGate>,
    pub limits_pending: Arc<dyn EntryGate>,
}

/// The test-build stand-ins: they let an entry through only outside production and for a
/// simulation or playback account read at the decision (`routing_application::dev_stubs`).
#[cfg(feature = "dev-stubs")]
pub fn risk_gates(app_env: AppEnv, kinds: Arc<dyn AccountKindOf>) -> RiskGates {
    use routing_application::dev_stubs::{Environment, SimulationOnlyGate, StandInGate};
    let environment = Environment::from_app_env(Some(app_env_str(app_env)));
    RiskGates {
        risk_blocked: Arc::new(SimulationOnlyGate::new(
            StandInGate::RiskBlocked,
            environment,
            kinds.clone(),
        )),
        limits_pending: Arc::new(SimulationOnlyGate::new(
            StandInGate::LimitsPending,
            environment,
            kinds,
        )),
    }
}

/// Without S13/S16 every entry waits for gates 7–8 and is never routed; exits do not ask them.
#[cfg(not(feature = "dev-stubs"))]
pub fn risk_gates(app_env: AppEnv, kinds: Arc<dyn AccountKindOf>) -> RiskGates {
    let _ = (app_env, kinds);
    RiskGates {
        risk_blocked: Arc::new(GateUnavailable),
        limits_pending: Arc::new(GateUnavailable),
    }
}

/// A gate whose module is not built: it never answers, so the decision fails closed.
pub struct GateUnavailable;

#[async_trait]
impl EntryGate for GateUnavailable {
    async fn barred(
        &self,
        _: UserId,
        _: &Contract,
        _: &[AccountId],
    ) -> Result<HashMap<AccountId, bool>, PortError> {
        Err(PortError("entry gate not available".into()))
    }
}

/// `APP_ENV` as the stand-in reads it (`Environment::from_app_env`).
pub fn app_env_str(app_env: AppEnv) -> &'static str {
    match app_env {
        AppEnv::Development => "development",
        AppEnv::Test => "test",
        AppEnv::Production => "production",
    }
}

/// The gates' stand-ins as this build must have them: exactly 7 and 8 with `dev-stubs`, none
/// without. A stand-in in production, or a real gate 7/8 next to `dev-stubs`, fails startup.
pub fn check_stand_ins(gates: &EntryGatePorts, app_env: AppEnv) -> Result<(), String> {
    let stand_ins = gates.stand_ins();
    let expected: &[&str] = if cfg!(feature = "dev-stubs") {
        &["risk_blocked", "limits_pending"]
    } else {
        &[]
    };
    if stand_ins != expected {
        return Err(format!(
            "routing entry gates: stand-ins {stand_ins:?}, expected {expected:?} in this build; a real gate replaces the dev-stubs wiring"
        ));
    }
    if !stand_ins.is_empty() && app_env.is_production() {
        return Err("routing entry gates: a test-build stand-in in production".into());
    }
    Ok(())
}

/// The route-status connection state: an account is connected when its bound platform account
/// belongs to a connection identity marks `connected` (the gateway's live session). Unbound,
/// unknown or archived accounts are not connected; a failed read is an error.
pub struct GatewayConnections {
    pub accounts: AccountsReads,
    pub store: Arc<dyn GatewayStore>,
    pub identity: Arc<dyn IdentityStore>,
}

#[async_trait]
impl AccountConnections for GatewayConnections {
    async fn connected(&self, user: UserId, accounts: &[Uuid]) -> Result<HashSet<Uuid>, String> {
        let reads = self.accounts.get().ok_or("accounts not wired")?;
        let mut out = HashSet::new();
        for id in accounts {
            let Some(venue) = reads
                .binding_of(user, AccountsAccountId::from_uuid(*id))
                .await
                .map_err(|e| e.to_string())?
            else {
                continue;
            };
            let Some((connection, _)) = self
                .store
                .venue_account(user, VenueAccountId::from_uuid(venue.as_uuid()))
                .await
                .map_err(|e| e.to_string())?
            else {
                continue;
            };
            let connected = self
                .identity
                .connection(
                    user,
                    identity_enterprise::ids::ConnectionId::from_uuid(connection.as_uuid()),
                )
                .await
                .map_err(|e| e.to_string())?
                .is_some_and(|r| r.connection.status == ConnectionStatus::Connected);
            if connected {
                out.insert(*id);
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_routing_audit_code_has_an_audit_rule() {
        for code in ROUTING_AUDIT_CODES {
            assert!(
                audit_application::rule_for(code).is_some(),
                "{code} has no audit rule"
            );
        }
    }

    #[test]
    fn only_development_and_test_are_outside_production() {
        assert_eq!(app_env_str(AppEnv::Development), "development");
        assert_eq!(app_env_str(AppEnv::Test), "test");
        assert_eq!(app_env_str(AppEnv::Production), "production");
    }
}
