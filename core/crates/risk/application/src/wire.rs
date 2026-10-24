//! The body of one account's `limits` item for the plugin (adapter protocol 1.4,
//! `limits.accounts[]` without `account_id`, `platform_account` and `version`, which the gateway
//! adds). Money travels as decimal strings. The drawdown is enforced on distances from the
//! trading-day start (Esquadro FINAL, 2026-10-24); the absolutes are for display only.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::{Map, Value, json};

use risk_enterprise::{DailyLoss, DrawdownTracker, DrawdownType, OnReach, ResetSchedule};

use crate::ports::StoredLimits;

/// The warning level, fixed at 80 % (D18).
const WARNING_THRESHOLD_PCT: u32 = 80;

/// Decimal string with at least 2 decimals (`"500.00"`, `"0.3125"`, `"-120.50"`).
pub fn money(v: Decimal) -> String {
    let mut v = v.normalize();
    if v.scale() < 2 {
        v.rescale(2);
    }
    v.to_string()
}

/// What the derivation alone does not know: the trading day's start, the realized equity at that
/// start (`E_start` = size + effective net realized up to the boundary; the size before any fill)
/// and the drawdown tracker (`None` before the first observation: it starts at the account size).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayAnchor {
    pub current_start_at: DateTime<Utc>,
    pub e_start: Decimal,
    pub tracker: Option<DrawdownTracker>,
}

pub fn limits_body(limits: &StoredLimits, anchor: &DayAnchor) -> Value {
    let rules = &limits.in_force;
    let mut body = Map::new();
    body.insert(
        "enforcement_enabled".into(),
        json!(limits.enforcement_enabled),
    );
    body.insert("max_position".into(), json!(limits.max_position_qty));
    body.insert("warning_threshold_pct".into(), json!(WARNING_THRESHOLD_PCT));
    body.insert("trading_day".into(), trading_day(limits.reset(), anchor));
    // `soft_pause` is sent like `hard`: the plugin's local action is the same; the kind only
    // changes what the firm does with the account (Core and web). Protocol 1.4.3 (Compasso).
    if let Some(DailyLoss::Hard(a) | DailyLoss::SoftPause(a)) = rules.daily_loss {
        body.insert("daily_loss_limit".into(), json!(money(a.value())));
    }
    if let Some(target) = rules.daily_profit_target {
        // Q63: `flatten_and_block` without enforcement acts as `notify`.
        let on_reach = match (rules.on_reach, limits.enforcement_enabled) {
            (OnReach::FlattenAndBlock, true) => OnReach::FlattenAndBlock,
            _ => OnReach::Notify,
        };
        body.insert(
            "daily_profit_target".into(),
            json!({"amount": money(target.value()), "on_reach": on_reach.as_str()}),
        );
    }
    if let Some(rule) = rules.drawdown
        && let Some(kind) = wire_type(rule.kind)
    {
        let lock_at = rule.lock_at(limits.size_amount);
        let t = anchor
            .tracker
            .unwrap_or_else(|| DrawdownTracker::start(rule, limits.size_amount, lock_at));
        let mut d = Map::new();
        d.insert("type".into(), json!(kind));
        d.insert("amount".into(), json!(money(rule.amount.value())));
        d.insert("floor_balance".into(), json!(money(t.floor)));
        d.insert("high_water_balance".into(), json!(money(t.high_water)));
        d.insert(
            "room_to_floor".into(),
            json!(money(anchor.e_start - t.floor)),
        );
        if rule.kind == DrawdownType::TrailingIntraday {
            d.insert(
                "high_water_above_start".into(),
                json!(money(t.high_water - anchor.e_start)),
            );
        }
        if let Some(lock) = lock_at {
            d.insert("lock_at_balance".into(), json!(money(lock)));
            d.insert(
                "lock_above_start".into(),
                json!(money(lock - anchor.e_start)),
            );
        }
        body.insert("drawdown".into(), Value::Object(d));
    }
    if let Some(rate) = rules.commission_per_contract_side {
        body.insert(
            "commission_per_contract_side".into(),
            json!(money(rate.value())),
        );
    }
    Value::Object(body)
}

const fn wire_type(kind: DrawdownType) -> Option<&'static str> {
    match kind {
        DrawdownType::TrailingIntraday => Some("trailing_intraday"),
        DrawdownType::TrailingEod => Some("trailing_end_of_day"),
        DrawdownType::Static => Some("static"),
        DrawdownType::Other => None,
    }
}

fn trading_day(reset: ResetSchedule, anchor: &DayAnchor) -> Value {
    let (time, zone) = match reset {
        ResetSchedule::Firm(r) => (r.local_time, r.time_zone.name().to_owned()),
        ResetSchedule::CmeSessionClose => (
            chrono::NaiveTime::from_hms_opt(16, 0, 0).unwrap_or(chrono::NaiveTime::MIN),
            "America/Chicago".to_owned(),
        ),
    };
    json!({
        "reset_local_time": time.format("%H:%M").to_string(),
        "time_zone": zone,
        "current_start_at": anchor.current_start_at.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    })
}
