//! Test-build stand-ins for entry gates 7 (risk) and 8 (limits) until S13/S16 wire the real
//! ones (Architect, ADR-0006 rule 8; Vigia C1–C5). Compiled only with the `dev-stubs`
//! feature, which no release or deploy build enables; the app deletes the wiring when the
//! real gate merges.
//!
//! The stand-in lets an entry through only outside production AND for an account whose
//! platform account is a simulation or playback account, read at this decision (never a
//! cached binding). Anything else, including an unknown kind or a failed read, answers
//! "barred", so the decision skips the entry with the code the real gate would give
//! (`routing.risk_blocked` / `routing.limits_pending`): never routed. Exits never consult
//! the gate. The stand-in shows only in the operator log, never in the audit (Vigia C3).

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use routing_enterprise::AccountId;
use shared_kernel::UserId;

use crate::decisions::{AccountKind, AccountKindOf, Contract, EntryGate};
use crate::routes::PortError;

/// In every operator log line and in the binary: release checks look for it (Vigia C1/C5).
pub const STAND_IN_MARKER: &str = "allwin-dev-stub:routing-entry-gate";

/// `APP_ENV`: only `development` and `test` are outside production; absent, empty or any
/// other value is production (Vigia C2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Production,
    NonProduction,
}

impl Environment {
    pub fn from_app_env(value: Option<&str>) -> Self {
        match value {
            Some("development" | "test") => Self::NonProduction,
            _ => Self::Production,
        }
    }
}

/// Which entry gate the stand-in answers for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandInGate {
    /// 7: `routing.risk_blocked`.
    RiskBlocked,
    /// 8: `routing.limits_pending`.
    LimitsPending,
}

impl StandInGate {
    const fn name(self) -> &'static str {
        match self {
            Self::RiskBlocked => "risk_blocked",
            Self::LimitsPending => "limits_pending",
        }
    }
}

/// "No block, limits derived" for simulation accounts outside production; barred otherwise.
pub struct SimulationOnlyGate {
    gate: StandInGate,
    environment: Environment,
    kinds: Arc<dyn AccountKindOf>,
}

impl SimulationOnlyGate {
    pub fn new(gate: StandInGate, environment: Environment, kinds: Arc<dyn AccountKindOf>) -> Self {
        Self {
            gate,
            environment,
            kinds,
        }
    }
}

#[async_trait]
impl EntryGate for SimulationOnlyGate {
    async fn barred(
        &self,
        user: UserId,
        _: &Contract,
        accounts: &[AccountId],
    ) -> Result<HashMap<AccountId, bool>, PortError> {
        let mut answers = HashMap::with_capacity(accounts.len());
        for account in accounts {
            let kind = if self.environment == Environment::Production {
                None
            } else {
                match self.kinds.kind(user, *account).await {
                    Ok(kind) => Some(kind),
                    Err(e) => {
                        tracing::warn!(marker = STAND_IN_MARKER, gate = self.gate.name(), account_id = %account, error = %e, "account kind unreadable: entry barred");
                        None
                    }
                }
            };
            let passes = matches!(kind, Some(AccountKind::Simulation | AccountKind::Playback));
            tracing::warn!(
                marker = STAND_IN_MARKER,
                gates_stand_in = true,
                gate = self.gate.name(),
                account_id = %account,
                environment = ?self.environment,
                kind = ?kind,
                passes,
                "entry gate answered by a test-build stand-in"
            );
            answers.insert(*account, !passes);
        }
        Ok(answers)
    }

    fn stand_in(&self) -> Option<&'static str> {
        Some(STAND_IN_MARKER)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct Kinds {
        of: HashMap<AccountId, AccountKind>,
        down: bool,
        reads: Mutex<usize>,
    }

    #[async_trait]
    impl AccountKindOf for Kinds {
        async fn kind(&self, _: UserId, account: AccountId) -> Result<AccountKind, PortError> {
            *self.reads.lock().unwrap() += 1;
            if self.down {
                return Err(PortError("gateway store down".into()));
            }
            Ok(self
                .of
                .get(&account)
                .copied()
                .unwrap_or(AccountKind::Unknown))
        }
    }

    fn contract() -> Contract {
        Contract {
            symbol: "MNQ".into(),
            month: "2026-12".into(),
        }
    }

    fn world(down: bool) -> (Arc<Kinds>, [AccountId; 5]) {
        let ids = [(); 5].map(|()| AccountId::new_v7());
        let kinds = [
            AccountKind::Simulation,
            AccountKind::Playback,
            AccountKind::Live,
            AccountKind::Unknown,
        ];
        let of = ids.iter().copied().zip(kinds).collect();
        let kinds = Arc::new(Kinds {
            of,
            down,
            reads: Mutex::new(0),
        });
        (kinds, ids)
    }

    #[tokio::test]
    async fn only_simulation_and_playback_pass_outside_production() {
        let (kinds, ids) = world(false);
        let gate = SimulationOnlyGate::new(
            StandInGate::RiskBlocked,
            Environment::NonProduction,
            kinds.clone(),
        );
        let answers = gate
            .barred(UserId::new_v7(), &contract(), &ids)
            .await
            .unwrap();
        // simulation, playback, live, unknown, no venue account.
        let barred: Vec<bool> = ids.iter().map(|a| answers[a]).collect();
        assert_eq!(barred, [false, false, true, true, true]);
        // Read at the decision, for every account, each time.
        gate.barred(UserId::new_v7(), &contract(), &ids)
            .await
            .unwrap();
        assert_eq!(*kinds.reads.lock().unwrap(), 10);
        assert_eq!(gate.stand_in(), Some(STAND_IN_MARKER));
    }

    #[tokio::test]
    async fn production_or_an_unreadable_kind_bars_every_entry() {
        let (kinds, ids) = world(false);
        let gate =
            SimulationOnlyGate::new(StandInGate::LimitsPending, Environment::Production, kinds);
        let answers = gate
            .barred(UserId::new_v7(), &contract(), &ids)
            .await
            .unwrap();
        assert!(ids.iter().all(|a| answers[a]), "{answers:?}");

        let (kinds, ids) = world(true);
        let gate = SimulationOnlyGate::new(
            StandInGate::LimitsPending,
            Environment::NonProduction,
            kinds,
        );
        let answers = gate
            .barred(UserId::new_v7(), &contract(), &ids)
            .await
            .unwrap();
        assert!(ids.iter().all(|a| answers[a]), "{answers:?}");
    }

    #[test]
    fn app_env_is_production_unless_development_or_test() {
        for (value, env) in [
            (None, Environment::Production),
            (Some(""), Environment::Production),
            (Some("production"), Environment::Production),
            (Some("Test"), Environment::Production),
            (Some("staging"), Environment::Production),
            (Some("development"), Environment::NonProduction),
            (Some("test"), Environment::NonProduction),
        ] {
            assert_eq!(Environment::from_app_env(value), env, "{value:?}");
        }
    }
}
