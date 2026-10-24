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

    async fn anchor(
        &self,
        _: UserId,
        _: Uuid,
        _: DateTime<Utc>,
    ) -> Result<(Option<Decimal>, Option<risk_enterprise::DrawdownTracker>), StoreError> {
        Ok((None, None))
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
                max_position_qty: 2,
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

// ---- limits body (protocol 1.4) -------------------------------------------------------------

#[tokio::test]
async fn the_limits_body_carries_distances_from_the_realized_day_start() {
    use crate::wire::{DayAnchor, limits_body};
    use risk_enterprise::{Drawdown, DrawdownType, OnReach, TrailingLock};
    let w = world();
    w.set("500", 1, NOW);
    DeriveLimits::new(w.deps())
        .handle(w.user, w.account)
        .await
        .unwrap();
    let mut l = w.store.current(w.user, w.account).unwrap();
    l.in_force.drawdown = Some(Drawdown {
        kind: DrawdownType::TrailingIntraday,
        amount: LimitAmount::parse("2000").unwrap(),
        lock: Some(TrailingLock::FloorReachesStart {
            offset: risk_enterprise::NonNegativeMoney::parse("100").unwrap(),
        }),
    });
    l.in_force.daily_profit_target = Some(LimitAmount::parse("800").unwrap());
    l.in_force.on_reach = OnReach::FlattenAndBlock;
    // The 52k/50k case: the size is 50 000 and today started at 50 300 realized.
    let body = limits_body(
        &l,
        &DayAnchor {
            current_start_at: utc("2026-11-02T22:00:00Z"),
            e_start: dec("50300"),
            tracker: None,
        },
    );
    assert_eq!(
        body,
        serde_json::json!({
            "enforcement_enabled": false,
            "max_position": 2,
            "warning_threshold_pct": 80,
            "trading_day": {"reset_local_time": "17:00", "time_zone": "America/New_York",
                            "current_start_at": "2026-11-02T22:00:00Z"},
            "daily_loss_limit": "500.00",
            // Enforcement off: flatten_and_block acts as notify (Q63).
            "daily_profit_target": {"amount": "800.00", "on_reach": "notify"},
            "drawdown": {"type": "trailing_intraday", "amount": "2000.00",
                         "floor_balance": "48000.00", "high_water_balance": "50000.00",
                         "room_to_floor": "2300.00", "high_water_above_start": "-300.00",
                         "lock_at_balance": "50100.00", "lock_above_start": "-200.00"}
        })
    );
}

#[test]
fn money_keeps_two_decimals_and_exact_scale() {
    use crate::wire::money;
    assert_eq!(money(dec("500")), "500.00");
    assert_eq!(money(dec("0.3125")), "0.3125");
    assert_eq!(money(dec("-120.5")), "-120.50");
    assert_eq!(money(dec("48000.0000")), "48000.00");
}

#[tokio::test]
async fn current_limits_returns_only_derived_accounts_with_the_plugin_version() {
    use crate::limits::CurrentLimits;
    let w = world();
    w.set("500", 1, NOW);
    DeriveLimits::new(w.deps())
        .handle(w.user, w.account)
        .await
        .unwrap();
    let missing = Uuid::now_v7();
    let found = CurrentLimits::new(w.deps())
        .handle(w.user, &[w.account, missing])
        .await
        .unwrap();
    assert_eq!(found.len(), 1);
    let l = &found[&w.account];
    assert_eq!((l.version, l.derived_from_version), (1, 1));
    assert_eq!(l.body["daily_loss_limit"], "500.00");
    assert_eq!(
        l.body["trading_day"]["current_start_at"],
        "2026-11-02T22:00:00Z"
    );
    // Another user's id list never reaches this user's limits.
    assert!(
        CurrentLimits::new(w.deps())
            .handle(UserId::new_v7(), &[w.account])
            .await
            .unwrap()
            .is_empty()
    );
}

// ---- fills (E1 gateway.execution_reported) ------------------------------------------------

mod fills {
    use super::*;
    use chrono::NaiveDate;
    use risk_enterprise::{Position, Side};

    use crate::fills::{
        AppliedFill, ApplyFill, FillOperation, FillOutcome, FillStore, FillWrite, FillsDeps,
        InstrumentFacts, InstrumentPoints, ReportedFill,
    };

    #[derive(Default)]
    struct Fills {
        rows: Mutex<HashMap<(Uuid, String), AppliedFill>>,
        positions: Mutex<HashMap<(Uuid, String), Position>>,
        days: Mutex<HashMap<(Uuid, NaiveDate), (Decimal, Decimal, Decimal)>>,
        stale: Mutex<bool>,
    }

    #[async_trait]
    impl FillStore for Fills {
        async fn applied_fill(
            &self,
            _: UserId,
            account: Uuid,
            fill_id: &str,
        ) -> Result<Option<AppliedFill>, StoreError> {
            Ok(self
                .rows
                .lock()
                .unwrap()
                .get(&(account, fill_id.to_owned()))
                .cloned())
        }

        async fn position(
            &self,
            _: UserId,
            account: Uuid,
            symbol: &str,
            _: NaiveDate,
        ) -> Result<Option<Position>, StoreError> {
            Ok(self
                .positions
                .lock()
                .unwrap()
                .get(&(account, symbol.to_owned()))
                .copied())
        }

        async fn write_fill(
            &self,
            _: UserId,
            account: Uuid,
            w: &FillWrite,
        ) -> Result<(), StoreError> {
            let key = (account, w.row.fill_id.clone());
            let mut rows = self.rows.lock().unwrap();
            if w.new_day.is_some() && rows.contains_key(&key) {
                return Err(StoreError::Stale);
            }
            rows.insert(key, w.row.clone());
            let mut days = self.days.lock().unwrap();
            let day = days.entry((account, w.row.trading_date)).or_default();
            day.0 += w.delta_gross;
            day.1 += w.delta_commission;
            day.2 += w.delta_gap;
            if let Some(p) = w.position {
                self.positions
                    .lock()
                    .unwrap()
                    .insert((account, w.row.instrument_symbol.clone()), p);
            }
            *self.stale.lock().unwrap() = w.stale;
            Ok(())
        }
    }

    struct Points;

    #[async_trait]
    impl InstrumentPoints for Points {
        async fn facts(&self, symbol: &str) -> Result<Option<InstrumentFacts>, PortError> {
            Ok((symbol == "MNQ").then_some(InstrumentFacts {
                point_value: dec("2"),
                tick_size: dec("0.25"),
            }))
        }
    }

    struct Env {
        w: World,
        fills: Arc<Fills>,
        uc: ApplyFill,
    }

    async fn env() -> Env {
        let w = world();
        w.set("500", 1, NOW);
        DeriveLimits::new(w.deps())
            .handle(w.user, w.account)
            .await
            .unwrap();
        let fills = Arc::new(Fills::default());
        let uc = ApplyFill::new(FillsDeps {
            limits: w.store.clone(),
            fills: fills.clone(),
            instruments: Arc::new(Points),
        });
        Env { w, fills, uc }
    }

    fn fill(
        id: &str,
        side: Side,
        qty: i64,
        price: &str,
        after: (i64, Option<&str>),
    ) -> ReportedFill {
        ReportedFill {
            event_id: Uuid::now_v7(),
            account_id: Uuid::nil(),
            fill_id: id.into(),
            operation: FillOperation::Add,
            instrument_symbol: "MNQ".into(),
            contract_month: NaiveDate::from_ymd_opt(2026, 12, 1).unwrap(),
            side,
            qty,
            price: dec(price),
            reported_commission: Some(dec("0.50")),
            commission_effective: dec("0.62") * Decimal::from(qty),
            position_after: Some(after.0),
            avg_price_after: after.1.map(dec),
            at: utc(NOW),
        }
    }

    impl Env {
        async fn apply(&self, mut f: ReportedFill) -> FillOutcome {
            f.account_id = self.w.account;
            self.uc.handle(self.w.user, &f).await.unwrap()
        }

        fn day(&self) -> (Decimal, Decimal, Decimal) {
            let date = NaiveDate::from_ymd_opt(2026, 11, 3).unwrap();
            self.fills.days.lock().unwrap()[&(self.w.account, date)]
        }

        fn stale(&self) -> bool {
            *self.fills.stale.lock().unwrap()
        }
    }

    #[tokio::test]
    async fn a_round_trip_realizes_on_average_cost_once() {
        let e = env().await;
        let open = fill("F1", Side::Buy, 2, "21000", (2, Some("21000")));
        assert_eq!(
            e.apply(open.clone()).await,
            FillOutcome::Applied { stale: false }
        );
        assert_eq!(e.apply(open).await, FillOutcome::Duplicate, "redelivery");
        let close = fill("F2", Side::Sell, 2, "21010", (0, None));
        assert_eq!(e.apply(close).await, FillOutcome::Applied { stale: false });
        // 2 × 10 points × 2 USD. Effective commission 0.62 × 2 per fill (2.48 in all); the
        // platform reported 0.50 per fill, so the gap is (1.24 − 0.50) × 2 = 1.48.
        let (gross, commission, gap) = e.day();
        assert_eq!(gross, dec("40"));
        assert_eq!(commission, dec("2.48"));
        assert_eq!(gap, dec("1.48"));
        let row = e.fills.rows.lock().unwrap()[&(e.w.account, "F2".to_owned())].clone();
        assert_eq!(
            (row.position_before, row.avg_price_before, row.point_value),
            (Some(2), Some(dec("21000")), dec("2"))
        );
    }

    #[tokio::test]
    async fn an_amend_recomputes_from_stored_inputs_and_a_remove_reverses() {
        let e = env().await;
        e.apply(fill("F1", Side::Buy, 2, "21000", (2, Some("21000"))))
            .await;
        e.apply(fill("F2", Side::Sell, 2, "21010", (0, None))).await;
        let mut amend = fill("F2", Side::Sell, 2, "21005", (0, None));
        amend.operation = FillOperation::Amend;
        assert_eq!(e.apply(amend).await, FillOutcome::Applied { stale: false });
        assert_eq!(e.day().0, dec("20"));
        let mut remove = fill("F2", Side::Sell, 2, "21005", (2, Some("21000")));
        remove.operation = FillOperation::Remove;
        e.apply(remove).await;
        assert_eq!(e.day().0, dec("0"));
        let row = e.fills.rows.lock().unwrap()[&(e.w.account, "F2".to_owned())].clone();
        assert!(row.removed && row.qty == 0 && row.gross.is_zero());
    }

    #[tokio::test]
    async fn an_unknown_basis_realizes_nothing_and_is_stale() {
        let e = env().await;
        // No stored row; the platform shows a flip: the old average cannot be known.
        let f = fill("F1", Side::Sell, 3, "21000", (-2, Some("21000")));
        assert_eq!(e.apply(f).await, FillOutcome::Applied { stale: true });
        assert_eq!(e.day().0, dec("0"));
        assert!(e.stale());
    }

    #[tokio::test]
    async fn unknown_cases_write_nothing() {
        let e = env().await;
        let mut amend = fill("F9", Side::Sell, 1, "21000", (0, None));
        amend.operation = FillOperation::Amend;
        assert_eq!(e.apply(amend).await, FillOutcome::UnknownFill);
        let mut other = fill("F1", Side::Buy, 1, "100", (1, Some("100")));
        other.instrument_symbol = "ZZZ".into();
        assert_eq!(e.apply(other).await, FillOutcome::UnknownInstrument);
        // An account without derived limits yet.
        let mut no_limits = fill("F1", Side::Buy, 1, "21000", (1, Some("21000")));
        no_limits.account_id = Uuid::now_v7();
        assert_eq!(
            e.uc.handle(e.w.user, &no_limits).await.unwrap(),
            FillOutcome::NoLimits
        );
        assert!(e.fills.rows.lock().unwrap().is_empty());
    }
}
