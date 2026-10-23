//! A preset's rules as published (HTTP `PresetRules`, S16): each field is either verified (one value
//! with its sources) or not confirmed (≥ 2 published values, never a value; S16 AC-5/AC-6). Typed
//! here so the rule resolver and the risk inputs (ADR-0006) never parse documents. A document that
//! does not convert is an error (fail closed), never an empty rule.
//!
//! The `Raw*` types mirror the wire shape for deserialization; [`PresetRules::try_from`] checks the
//! provenance shape and the money strings exactly (X-3).

use std::str::FromStr;

use chrono::{NaiveDate, NaiveTime};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::values::MoneyAmount;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("preset rules: invalid {0}")]
pub struct PresetRulesError(pub String);

fn bad(what: impl Into<String>) -> PresetRulesError {
    PresetRulesError(what.into())
}

/// `SourceRef`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRef {
    pub url: String,
    pub accessed_at: NaiveDate,
    pub page_title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Published<T> {
    pub value: T,
    pub sources: Vec<SourceRef>,
}

/// One rule field's provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Provenance<T> {
    Verified {
        value: T,
        sources: Vec<SourceRef>,
    },
    /// The firm's official pages disagree: the user picks one or types their own (never prefilled).
    NotConfirmed {
        published_values: Vec<Published<T>>,
    },
}

impl<T> Provenance<T> {
    pub fn verified(&self) -> Option<&T> {
        match self {
            Self::Verified { value, .. } => Some(value),
            Self::NotConfirmed { .. } => None,
        }
    }

    pub fn is_not_confirmed(&self) -> bool {
        matches!(self, Self::NotConfirmed { .. })
    }

    /// Every published value (one when verified).
    pub fn values(&self) -> Vec<&T> {
        match self {
            Self::Verified { value, .. } => vec![value],
            Self::NotConfirmed { published_values } => {
                published_values.iter().map(|p| &p.value).collect()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaxPosition {
    /// MNQ (micro) contracts, 1..=10 000.
    pub contracts: u32,
    pub published_text: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawdownType {
    TrailingIntraday,
    TrailingEod,
    Static,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Drawdown {
    pub kind: DrawdownType,
    pub amount: MoneyAmount,
    pub stops_trailing_at: Option<String>,
    /// Required for `other`.
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DailyLoss {
    Hard(MoneyAmount),
    SoftPause(MoneyAmount),
    /// The firm states there is none (a verified value, not an absent field).
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradingDayReset {
    pub local_time: NaiveTime,
    /// IANA name as published (validated by the contract pattern; resolved by risk).
    pub time_zone: String,
    pub published_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Consistency {
    pub text: String,
    pub threshold_pct: Option<Decimal>,
}

/// HTTP `PresetRules` (S16; commission U20 a; evaluation target Q46).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresetRules {
    pub max_position_qty: Provenance<MaxPosition>,
    pub drawdown: Provenance<Drawdown>,
    pub daily_loss_limit: Provenance<DailyLoss>,
    pub daily_profit_target: Option<Provenance<MoneyAmount>>,
    pub trading_day_reset: Provenance<TradingDayReset>,
    pub consistency_rule: Option<Provenance<Consistency>>,
    pub commission_per_contract_side: Option<Provenance<MoneyAmount>>,
    pub evaluation_profit_target: Option<Provenance<MoneyAmount>>,
}

impl PresetRules {
    /// The not-confirmed fields, by contract name (they need the user's choice before saving).
    pub fn not_confirmed_fields(&self) -> Vec<&'static str> {
        let mut out = Vec::new();
        let mut check = |name, nc: bool| {
            if nc {
                out.push(name);
            }
        };
        check("max_position_qty", self.max_position_qty.is_not_confirmed());
        check("drawdown", self.drawdown.is_not_confirmed());
        check("daily_loss_limit", self.daily_loss_limit.is_not_confirmed());
        check(
            "daily_profit_target",
            self.daily_profit_target
                .as_ref()
                .is_some_and(Provenance::is_not_confirmed),
        );
        check(
            "trading_day_reset",
            self.trading_day_reset.is_not_confirmed(),
        );
        check(
            "consistency_rule",
            self.consistency_rule
                .as_ref()
                .is_some_and(Provenance::is_not_confirmed),
        );
        check(
            "commission_per_contract_side",
            self.commission_per_contract_side
                .as_ref()
                .is_some_and(Provenance::is_not_confirmed),
        );
        check(
            "evaluation_profit_target",
            self.evaluation_profit_target
                .as_ref()
                .is_some_and(Provenance::is_not_confirmed),
        );
        out
    }
}

// ---- wire shape ------------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawSource {
    pub url: String,
    pub accessed_at: String,
    #[serde(default)]
    pub page_title: Option<String>,
    /// The release file's import evidence; not part of `SourceRef`, dropped.
    #[serde(default)]
    pub quote: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawPublished<T> {
    pub value: T,
    pub sources: Vec<RawSource>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, bound(deserialize = "T: Deserialize<'de>"))]
pub struct RawField<T> {
    pub status: String,
    #[serde(default)]
    pub value: Option<T>,
    #[serde(default)]
    pub sources: Option<Vec<RawSource>>,
    #[serde(default)]
    pub published_values: Option<Vec<RawPublished<T>>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawMaxPosition {
    pub contracts: i64,
    #[serde(default)]
    pub published_text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawDrawdown {
    #[serde(rename = "type")]
    pub kind: String,
    pub amount: String,
    #[serde(default)]
    pub stops_trailing_at: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawDailyLoss {
    pub kind: String,
    #[serde(default)]
    pub amount: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawAmount {
    pub amount: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawReset {
    pub local_time: String,
    pub time_zone: String,
    #[serde(default)]
    pub published_text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawConsistency {
    pub text: String,
    #[serde(default)]
    pub threshold_pct: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawPresetRules {
    pub max_position_qty: RawField<RawMaxPosition>,
    pub drawdown: RawField<RawDrawdown>,
    pub daily_loss_limit: RawField<RawDailyLoss>,
    #[serde(default)]
    pub daily_profit_target: Option<RawField<RawAmount>>,
    pub trading_day_reset: RawField<RawReset>,
    #[serde(default)]
    pub consistency_rule: Option<RawField<RawConsistency>>,
    #[serde(default)]
    pub commission_per_contract_side: Option<RawField<RawAmount>>,
    #[serde(default)]
    pub evaluation_profit_target: Option<RawField<RawAmount>>,
}

// ---- conversion ------------------------------------------------------------------------------

/// `MoneyAmount` text (`^[0-9]{1,14}(\.[0-9]{1,4})?$`), parsed exactly.
pub fn money(text: &str, what: &str) -> Result<MoneyAmount, PresetRulesError> {
    let (int, frac) = text.split_once('.').unwrap_or((text, ""));
    let digits = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
    let ok = (1..=14).contains(&int.len())
        && digits(int)
        && digits(frac)
        && frac.len() <= 4
        && !(text.contains('.') && frac.is_empty());
    if !ok {
        return Err(bad(what));
    }
    Decimal::from_str(text)
        .ok()
        .and_then(|d| MoneyAmount::new(d).ok())
        .ok_or_else(|| bad(what))
}

fn source(raw: RawSource, what: &str) -> Result<SourceRef, PresetRulesError> {
    if !raw.url.starts_with("https://") {
        return Err(bad(format!("{what} source url")));
    }
    Ok(SourceRef {
        url: raw.url,
        accessed_at: NaiveDate::parse_from_str(&raw.accessed_at, "%Y-%m-%d")
            .map_err(|_| bad(format!("{what} accessed_at")))?,
        page_title: raw.page_title,
    })
}

fn sources(raw: Vec<RawSource>, what: &str) -> Result<Vec<SourceRef>, PresetRulesError> {
    if raw.is_empty() {
        return Err(bad(format!("{what} sources")));
    }
    raw.into_iter().map(|s| source(s, what)).collect()
}

fn field<R, T>(
    raw: RawField<R>,
    what: &str,
    convert: impl Fn(R) -> Result<T, PresetRulesError>,
) -> Result<Provenance<T>, PresetRulesError> {
    match (raw.status.as_str(), raw.value, raw.published_values) {
        ("verified", Some(value), None) => Ok(Provenance::Verified {
            value: convert(value)?,
            sources: sources(raw.sources.unwrap_or_default(), what)?,
        }),
        ("not_confirmed", None, Some(published))
            if published.len() >= 2 && raw.sources.is_none() =>
        {
            Ok(Provenance::NotConfirmed {
                published_values: published
                    .into_iter()
                    .map(|p| {
                        Ok(Published {
                            value: convert(p.value)?,
                            sources: sources(p.sources, what)?,
                        })
                    })
                    .collect::<Result<_, PresetRulesError>>()?,
            })
        }
        _ => Err(bad(format!("{what} provenance"))),
    }
}

fn max_position(r: RawMaxPosition) -> Result<MaxPosition, PresetRulesError> {
    let contracts = u32::try_from(r.contracts)
        .ok()
        .filter(|c| (1..=10_000).contains(c))
        .ok_or_else(|| bad("max_position_qty contracts"))?;
    Ok(MaxPosition {
        contracts,
        published_text: r.published_text,
    })
}

fn drawdown(r: RawDrawdown) -> Result<Drawdown, PresetRulesError> {
    let kind = match r.kind.as_str() {
        "trailing_intraday" => DrawdownType::TrailingIntraday,
        "trailing_eod" => DrawdownType::TrailingEod,
        "static" => DrawdownType::Static,
        "other" if r.description.is_some() => DrawdownType::Other,
        _ => return Err(bad("drawdown type")),
    };
    Ok(Drawdown {
        kind,
        amount: money(&r.amount, "drawdown amount")?,
        stops_trailing_at: r.stops_trailing_at,
        description: r.description,
    })
}

fn daily_loss(r: RawDailyLoss) -> Result<DailyLoss, PresetRulesError> {
    match (r.kind.as_str(), r.amount) {
        ("hard", Some(a)) => Ok(DailyLoss::Hard(money(&a, "daily_loss_limit amount")?)),
        ("soft_pause", Some(a)) => Ok(DailyLoss::SoftPause(money(&a, "daily_loss_limit amount")?)),
        ("none", None) => Ok(DailyLoss::None),
        _ => Err(bad("daily_loss_limit")),
    }
}

fn amount(what: &'static str) -> impl Fn(RawAmount) -> Result<MoneyAmount, PresetRulesError> {
    move |r| money(&r.amount, what)
}

fn reset(r: RawReset) -> Result<TradingDayReset, PresetRulesError> {
    let valid_time = r.local_time.len() == 5 && r.local_time.as_bytes()[2] == b':';
    let local_time = NaiveTime::parse_from_str(&r.local_time, "%H:%M")
        .ok()
        .filter(|_| valid_time)
        .ok_or_else(|| bad("trading_day_reset local_time"))?;
    if r.time_zone.is_empty() || r.time_zone.len() > 64 {
        return Err(bad("trading_day_reset time_zone"));
    }
    Ok(TradingDayReset {
        local_time,
        time_zone: r.time_zone,
        published_text: r.published_text,
    })
}

fn consistency(r: RawConsistency) -> Result<Consistency, PresetRulesError> {
    if r.text.is_empty() {
        return Err(bad("consistency_rule text"));
    }
    let threshold_pct = r
        .threshold_pct
        .map(|t| {
            Decimal::from_str(&t)
                .ok()
                .filter(|d| !d.is_sign_negative())
                .ok_or_else(|| bad("consistency threshold_pct"))
        })
        .transpose()?;
    Ok(Consistency {
        text: r.text,
        threshold_pct,
    })
}

impl TryFrom<RawPresetRules> for PresetRules {
    type Error = PresetRulesError;

    fn try_from(r: RawPresetRules) -> Result<Self, PresetRulesError> {
        Ok(Self {
            max_position_qty: field(r.max_position_qty, "max_position_qty", max_position)?,
            drawdown: field(r.drawdown, "drawdown", drawdown)?,
            daily_loss_limit: field(r.daily_loss_limit, "daily_loss_limit", daily_loss)?,
            daily_profit_target: r
                .daily_profit_target
                .map(|f| {
                    field(
                        f,
                        "daily_profit_target",
                        amount("daily_profit_target amount"),
                    )
                })
                .transpose()?,
            trading_day_reset: field(r.trading_day_reset, "trading_day_reset", reset)?,
            consistency_rule: r
                .consistency_rule
                .map(|f| field(f, "consistency_rule", consistency))
                .transpose()?,
            commission_per_contract_side: r
                .commission_per_contract_side
                .map(|f| {
                    field(
                        f,
                        "commission_per_contract_side",
                        amount("commission amount"),
                    )
                })
                .transpose()?,
            evaluation_profit_target: r
                .evaluation_profit_target
                .map(|f| {
                    field(
                        f,
                        "evaluation_profit_target",
                        amount("evaluation target amount"),
                    )
                })
                .transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;

    fn src() -> Value {
        json!([{"url": "https://firm.example/rules", "accessed_at": "2026-10-20"}])
    }

    fn verified(value: Value) -> Value {
        json!({"status": "verified", "value": value, "sources": src()})
    }

    fn base() -> Value {
        json!({
            "max_position_qty": verified(json!({"contracts": 50, "published_text": "5 minis / 50 micros"})),
            "drawdown": {"status": "not_confirmed", "published_values": [
                {"value": {"type": "trailing_eod", "amount": "2000.00"}, "sources": src()},
                {"value": {"type": "trailing_intraday", "amount": "2500.00"}, "sources": src()}]},
            "daily_loss_limit": verified(json!({"kind": "none"})),
            "trading_day_reset": verified(json!({"local_time": "17:00", "time_zone": "America/New_York"})),
            "commission_per_contract_side": verified(json!({"amount": "0.37"})),
            "evaluation_profit_target": verified(json!({"amount": "3000.00"})),
        })
    }

    fn parse(v: Value) -> Result<PresetRules, PresetRulesError> {
        let raw: RawPresetRules = serde_json::from_value(v).map_err(|e| bad(e.to_string()))?;
        PresetRules::try_from(raw)
    }

    #[test]
    fn a_published_document_converts() {
        let r = parse(base()).unwrap();
        assert_eq!(r.max_position_qty.verified().map(|m| m.contracts), Some(50));
        assert_eq!(r.not_confirmed_fields(), ["drawdown"]);
        assert_eq!(r.drawdown.values().len(), 2);
        assert_eq!(r.daily_loss_limit.verified(), Some(&DailyLoss::None));
        assert_eq!(
            r.commission_per_contract_side
                .as_ref()
                .and_then(Provenance::verified)
                .map(MoneyAmount::value),
            Some(Decimal::new(37, 2))
        );
        assert!(r.daily_profit_target.is_none() && r.consistency_rule.is_none());
    }

    macro_rules! example {
        ($name:literal) => {
            include_str!(concat!(
                "../../../../../contracts/presets/v1/examples/valid/",
                $name
            ))
        };
    }

    #[test]
    fn every_published_example_converts() {
        for raw in [
            example!("release-v1-new-firm.json"),
            example!("release-v2-stance-changed.json"),
            example!("release-manual-only.json"),
            example!("release-not-confirmed-per-platform.json"),
            example!("release-stored-only-platform-manual-device-not-confirmed.json"),
        ] {
            let file: Value = serde_json::from_str(raw).unwrap();
            for plan in file["plans"].as_array().unwrap() {
                for phase in plan["phases"].as_array().unwrap() {
                    for size in phase["sizes"].as_array().unwrap() {
                        for set in size["rule_sets"].as_array().unwrap() {
                            parse(set["rules"].clone()).unwrap();
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn bad_documents_fail_closed() {
        let cases: Vec<(&str, Value)> = vec![
            ("money as number", json!({"amount": 0.37})),
            ("exponent", json!({"amount": "1e3"})),
            ("five decimals", json!({"amount": "0.12345"})),
            ("trailing dot", json!({"amount": "1."})),
            ("negative", json!({"amount": "-1"})),
        ];
        for (name, value) in cases {
            let mut v = base();
            v["commission_per_contract_side"] = verified(value);
            assert!(parse(v).is_err(), "{name}");
        }
        let mut v = base();
        v["drawdown"]["published_values"]
            .as_array_mut()
            .unwrap()
            .pop();
        assert!(parse(v).is_err(), "not confirmed with one value");
        let mut v = base();
        v["max_position_qty"]["published_values"] = json!([]);
        assert!(parse(v).is_err(), "verified with published values");
        let mut v = base();
        v["daily_loss_limit"] = verified(json!({"kind": "none", "amount": "100"}));
        assert!(parse(v).is_err(), "none with an amount");
        let mut v = base();
        v["max_position_qty"] = verified(json!({"contracts": 0}));
        assert!(parse(v).is_err(), "zero contracts");
        let mut v = base();
        v["trading_day_reset"] =
            verified(json!({"local_time": "5:00", "time_zone": "America/New_York"}));
        assert!(parse(v).is_err(), "time shape");
        let mut v = base();
        v["max_position_qty"]["sources"] = json!([]);
        assert!(parse(v).is_err(), "verified without sources");
        let mut v = base();
        v["surprise"] = json!(1);
        assert!(parse(v).is_err(), "unknown field");
        let mut v = base();
        v.as_object_mut().unwrap().remove("trading_day_reset");
        assert!(parse(v).is_err(), "required field missing");
    }
}
