#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, NaiveTime, Utc};
use rust_decimal::Decimal;
use shared_kernel::{Clock, UserId};
use uuid::Uuid;

use risk_enterprise::{
    DailyLoss, LimitAmount, LimitKind, Pending, ResetTime, RiskRules, RuleInputs,
};

use crate::limits::{DeriveLimits, LimitsDeps, Outcome, ResetTradingDay};
use crate::ports::{
    AccountInputs, AccountKind, AccountRiskInputs, PortError, RiskStore, StoreError, StoredLimits,
};

// ---- fakes --------------------------------------------------------------------------------

#[derive(Default)]
struct Inputs(Mutex<HashMap<(UserId, Uuid), AccountInputs>>);

#[async_trait]
impl AccountRiskInputs for Inputs {
    async fn read(&self, user: UserId, id: Uuid) -> Result<Option<AccountInputs>, PortError> {
        Ok(self.0.lock().unwrap().get(&(user, id)).cloned())
    }

    async fn limits_versions(&self) -> Result<Vec<(UserId, Uuid, i32)>, PortError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .iter()
            .map(|((u, a), i)| (*u, *a, i.limits_version))
            .collect())
    }
}

/// Keeps every written derivation (the last one is current) with the store's newer-wins rule.
#[derive(Default)]
struct Store(Mutex<HashMap<(UserId, Uuid), Vec<StoredLimits>>>);

impl Store {
    fn current(&self, user: UserId, id: Uuid) -> Option<StoredLimits> {
        self.0
            .lock()
            .unwrap()
            .get(&(user, id))
            .and_then(|v| v.last().cloned())
    }

    fn writes(&self, user: UserId, id: Uuid) -> usize {
        self.0.lock().unwrap().get(&(user, id)).map_or(0, Vec::len)
    }
}

#[async_trait]
impl RiskStore for Store {
    async fn limits(&self, user: UserId, id: Uuid) -> Result<Option<StoredLimits>, StoreError> {
        Ok(self.current(user, id))
    }

    async fn save_derivation(
        &self,
        user: UserId,
        id: Uuid,
        limits: &StoredLimits,
    ) -> Result<(), StoreError> {
        let mut all = self.0.lock().unwrap();
        let rows = all.entry((user, id)).or_default();
        if let Some(last) = rows.last()
            && (limits.derived_from_version, limits.effective_as_of)
                <= (last.derived_from_version, last.effective_as_of)
        {
            return Err(StoreError::Stale);
        }
        rows.push(limits.clone());
        Ok(())
    }
}

struct Now(Mutex<DateTime<Utc>>);

impl Clock for Now {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
}

struct World {
    inputs: Arc<Inputs>,
    store: Arc<Store>,
    clock: Arc<Now>,
    user: UserId,
    account: Uuid,
}

impl World {
    fn deps(&self) -> LimitsDeps {
        LimitsDeps {
            inputs: self.inputs.clone(),
            store: self.store.clone(),
            clock: self.clock.clone(),
        }
    }

    fn set(&self, loss: &str, version: i32, saved_at: &str) {
        self.inputs.0.lock().unwrap().insert(
            (self.user, self.account),
            AccountInputs {
                kind: AccountKind::Prop,
                size_amount: Decimal::from(50_000),
                limits_version: version,
                rules: RuleInputs {
                    rules: RiskRules {
                        daily_loss: Some(DailyLoss::Hard(LimitAmount::parse(loss).unwrap())),
                        reset: Some(reset()),
                        ..RiskRules::default()
                    },
                    saved_at: [(LimitKind::DailyLoss, utc(saved_at))].into(),
                    baseline_at: utc("2026-10-01T00:00:00Z"),
                },
                evaluation_start: None,
                evaluation_earned_before: None,
            },
        );
    }

    fn at(&self, t: &str) {
        *self.clock.0.lock().unwrap() = utc(t);
    }

    fn loss(&self) -> Option<Decimal> {
        self.store
            .current(self.user, self.account)
            .and_then(|l| l.in_force.daily_loss)
            .and_then(|l| l.amount())
            .map(|a| a.value())
    }
}

fn utc(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
}

fn reset() -> ResetTime {
    ResetTime {
        local_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
        time_zone: chrono_tz::America::New_York,
    }
}

fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).unwrap()
}

// Tue 2026-11-03 10:00 New York; the trading day runs 2026-11-02T22:00Z .. 2026-11-03T22:00Z.
const NOW: &str = "2026-11-03T15:00:00Z";

fn world() -> World {
    World {
        inputs: Arc::default(),
        store: Arc::default(),
        clock: Arc::new(Now(Mutex::new(utc(NOW)))),
        user: UserId::new_v7(),
        account: Uuid::now_v7(),
    }
}

// ---- derivation ---------------------------------------------------------------------------

#[tokio::test]
async fn the_first_derivation_is_in_force_from_the_current_day() {
    let w = world();
    w.set("500", 1, NOW);
    let out = DeriveLimits::new(w.deps())
        .handle(w.user, w.account)
        .await
        .unwrap();
    assert_eq!(out, Outcome::Derived { version: 1 });
    let l = w.store.current(w.user, w.account).unwrap();
    assert_eq!((l.derived_from_version, l.version), (1, 1));
    assert_eq!(l.effective_as_of, utc("2026-11-02T22:00:00Z"));
    assert_eq!(w.loss(), Some(dec("500")));
}

#[tokio::test]
async fn a_redelivered_or_older_event_never_downgrades() {
    let w = world();
    w.set("500", 2, NOW);
    let uc = DeriveLimits::new(w.deps());
    uc.handle(w.user, w.account).await.unwrap();
    // The same version again (redelivery) and an older read are both no-ops.
    assert_eq!(
        uc.handle(w.user, w.account).await.unwrap(),
        Outcome::UpToDate
    );
    w.set("900", 1, NOW);
    assert_eq!(
        uc.handle(w.user, w.account).await.unwrap(),
        Outcome::UpToDate
    );
    assert_eq!(w.loss(), Some(dec("500")));
    assert_eq!(w.store.writes(w.user, w.account), 1);
}

#[tokio::test]
async fn a_looser_save_waits_for_the_reset_and_a_stricter_one_applies_now() {
    let w = world();
    w.set("500", 1, NOW);
    let derive = DeriveLimits::new(w.deps());
    derive.handle(w.user, w.account).await.unwrap();

    w.set("800", 2, NOW);
    derive.handle(w.user, w.account).await.unwrap();
    let l = w.store.current(w.user, w.account).unwrap();
    assert_eq!(w.loss(), Some(dec("500")), "looser: today's limit stays");
    assert_eq!(l.scheduled.len(), 1);
    assert_eq!(l.scheduled[0].change, Pending::LimitRaised);
    assert_eq!(l.scheduled[0].effective_from, utc("2026-11-03T22:00:00Z"));

    // The reset at 17:00 New York applies it, without a limits_version bump.
    let reset = ResetTradingDay::new(w.deps());
    w.at("2026-11-03T21:59:59Z");
    assert_eq!(
        reset.handle(w.user, w.account).await.unwrap(),
        Outcome::UpToDate
    );
    w.at("2026-11-03T22:00:30Z");
    assert_eq!(
        reset.handle(w.user, w.account).await.unwrap(),
        Outcome::Derived { version: 3 }
    );
    let l = w.store.current(w.user, w.account).unwrap();
    assert_eq!(w.loss(), Some(dec("800")));
    assert!(l.scheduled.is_empty());
    assert_eq!(
        (l.derived_from_version, l.effective_as_of),
        (2, utc("2026-11-03T22:00:00Z"))
    );
    // A second run of the same reset is a no-op.
    assert_eq!(
        reset.handle(w.user, w.account).await.unwrap(),
        Outcome::UpToDate
    );

    // Next day, a stricter save is in force at once.
    w.set("300", 3, "2026-11-04T15:00:00Z");
    w.at("2026-11-04T15:00:00Z");
    derive.handle(w.user, w.account).await.unwrap();
    assert_eq!(w.loss(), Some(dec("300")));
}

#[tokio::test]
async fn unknown_accounts_write_nothing() {
    let w = world();
    assert_eq!(
        DeriveLimits::new(w.deps())
            .handle(w.user, w.account)
            .await
            .unwrap(),
        Outcome::NoAccount
    );
    assert_eq!(
        ResetTradingDay::new(w.deps())
            .handle(w.user, w.account)
            .await
            .unwrap(),
        Outcome::UpToDate
    );
    // Another user never reads this user's inputs.
    w.set("500", 1, NOW);
    assert_eq!(
        DeriveLimits::new(w.deps())
            .handle(UserId::new_v7(), w.account)
            .await
            .unwrap(),
        Outcome::NoAccount
    );
    assert_eq!(w.store.writes(w.user, w.account), 0);
}
