//! A preset's rules as published (HTTP `PresetRules`, S16): each field is either verified (one value
//! with its sources) or not confirmed (≥ 2 published values, never a value; S16 AC-5/AC-6). Typed
//! here so the rule resolver and the risk inputs (ADR-0006) never parse documents. A document that
//! does not convert is an error (fail closed), never an empty rule.
//!
//! The `Raw*` types mirror the wire shape for deserialization; [`PresetRules::try_from`] checks the
//! provenance shape and the money strings exactly (X-3).

use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;

pub mod account;

use crate::rule_value::{RuleField, RuleOrigin};
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
    /// Why the title is blank (presets 1.1 / HTTP 6.0.0), only without `page_title`.
    pub title_note: Option<String>,
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
    /// Not confirmed by the firm's official pages (never prefilled; a required field needs the
    /// user's value). `published_values`: the conflicting values (>= 2), empty for a partial or a
    /// missing publication (presets 1.2, whose quotes and research are shown from the wire only).
    NotConfirmed {
        published_values: Vec<Published<T>>,
        reason: NotConfirmedReason,
    },
}

/// Why a field is not confirmed (presets 1.2 / HTTP 6.3.0 `not_confirmed_reason`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotConfirmedReason {
    /// The pages disagree: the user picks a published value or types their own.
    Conflicting,
    /// The firm publishes only part of it (verbatim quotes): the user types the value.
    Partial,
    /// The firm does not publish it: the user types the value.
    NotPublished,
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
            Self::NotConfirmed {
                published_values, ..
            } => published_values.iter().map(|p| &p.value).collect(),
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
    /// The firm's published text (Q90: information; the structure is `trailing_lock`).
    pub stops_trailing_at: Option<String>,
    /// Required for `other`.
    pub description: Option<String>,
    /// Where a trailing drawdown stops trailing (presets 1.1, HTTP 5.4.0; Q90 a). `None`: not
    /// modelled (the interim: it keeps trailing, with a warning).
    pub trailing_lock: Option<TrailingLock>,
}

/// `DrawdownValue.trailing_lock` (Q90 a).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrailingLock {
    /// The floor locks at the account's `size_amount` + `offset` (the real size, Q73).
    FloorReachesStart { offset: Option<MoneyAmount> },
    /// It never stops trailing.
    Never,
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
    /// A published rule set (the contract's `PresetRules`, e.g. a `&serde_json::Value`).
    pub fn from_wire<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Self, PresetRulesError> {
        let raw = RawPresetRules::deserialize(d).map_err(|e| bad(e.to_string()))?;
        Self::try_from(raw)
    }

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
    #[serde(default)]
    pub title_note: Option<String>,
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
    /// Presets 1.2: `conflicting` (absent = conflicting) | `partial` | `not_published`.
    #[serde(default)]
    pub not_confirmed_reason: Option<String>,
    /// Presets 1.2 (partial): shown from the wire; only their presence is checked here.
    #[serde(default)]
    pub quotes: Option<serde::de::IgnoredAny>,
    #[serde(default)]
    pub research: Option<serde::de::IgnoredAny>,
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
    #[serde(default)]
    pub trailing_lock: Option<RawTrailingLock>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawTrailingLock {
    pub mode: String,
    #[serde(default)]
    pub offset: Option<String>,
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
    if raw.page_title.is_some() && raw.title_note.is_some() {
        return Err(bad(format!("{what} source title_note with page_title")));
    }
    Ok(SourceRef {
        title_note: raw.title_note,
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
    let (quotes, research) = (raw.quotes.is_some(), raw.research.is_some());
    let reason = match raw.not_confirmed_reason.as_deref() {
        None | Some("conflicting") => NotConfirmedReason::Conflicting,
        Some("partial") => NotConfirmedReason::Partial,
        Some("not_published") => NotConfirmedReason::NotPublished,
        Some(_) => return Err(bad(format!("{what} not_confirmed_reason"))),
    };
    let bare = raw.sources.is_none();
    match (raw.status.as_str(), raw.value, raw.published_values) {
        ("verified", Some(value), None)
            if raw.not_confirmed_reason.is_none() && !quotes && !research =>
        {
            Ok(Provenance::Verified {
                value: convert(value)?,
                sources: sources(raw.sources.unwrap_or_default(), what)?,
            })
        }
        ("not_confirmed", None, None)
            if bare
                && ((reason == NotConfirmedReason::Partial && quotes && research)
                    || (reason == NotConfirmedReason::NotPublished && !quotes && research)) =>
        {
            Ok(Provenance::NotConfirmed {
                published_values: Vec::new(),
                reason,
            })
        }
        ("not_confirmed", None, Some(published))
            if reason == NotConfirmedReason::Conflicting
                && published.len() >= 2
                && bare
                && !quotes
                && !research =>
        {
            Ok(Provenance::NotConfirmed {
                reason,
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
    let trailing_lock = match r.trailing_lock {
        None => None,
        Some(_)
            if !matches!(
                kind,
                DrawdownType::TrailingIntraday | DrawdownType::TrailingEod
            ) =>
        {
            return Err(bad("trailing_lock on a drawdown that does not trail"));
        }
        Some(l) => Some(match (l.mode.as_str(), l.offset) {
            ("never", None) => TrailingLock::Never,
            ("floor_reaches_start", offset) => TrailingLock::FloorReachesStart {
                offset: offset
                    .map(|o| money(&o, "trailing_lock offset"))
                    .transpose()?,
            },
            _ => return Err(bad("trailing_lock")),
        }),
    };
    Ok(Drawdown {
        kind,
        amount: money(&r.amount, "drawdown amount")?,
        stops_trailing_at: r.stops_trailing_at,
        description: r.description,
        trailing_lock,
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

// ---- the account's own rule values (accounts.account_rule_value) ------------------------------

/// What happens when the profit target is reached (S13; Q3, Q49, Q63).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfitTargetOnReach {
    Notify,
    FlattenAndBlock,
}

impl ProfitTargetOnReach {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notify => "notify",
            Self::FlattenAndBlock => "flatten_and_block",
        }
    }
}

/// One stored rule value, typed by its field. `max_position_qty` has none (the account's
/// top-level column holds it; its row only records the origin).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleValue {
    Drawdown(Drawdown),
    DailyLossLimit(DailyLoss),
    DailyProfitTarget(MoneyAmount),
    TradingDayReset(TradingDayReset),
    ConsistencyRule(Consistency),
    ProfitTargetOnReach(ProfitTargetOnReach),
    /// Greater than 0, at most 100, one decimal (Q-S13-7).
    ConsistencyThresholdPct(Decimal),
    CommissionPerContractSide(MoneyAmount),
    EvaluationProfitTarget(MoneyAmount),
}

impl RuleValue {
    pub fn field(&self) -> RuleField {
        match self {
            Self::Drawdown(_) => RuleField::Drawdown,
            Self::DailyLossLimit(_) => RuleField::DailyLossLimit,
            Self::DailyProfitTarget(_) => RuleField::DailyProfitTarget,
            Self::TradingDayReset(_) => RuleField::TradingDayReset,
            Self::ConsistencyRule(_) => RuleField::ConsistencyRule,
            Self::ProfitTargetOnReach(_) => RuleField::ProfitTargetOnReach,
            Self::ConsistencyThresholdPct(_) => RuleField::ConsistencyThresholdPct,
            Self::CommissionPerContractSide(_) => RuleField::CommissionPerContractSide,
            Self::EvaluationProfitTarget(_) => RuleField::EvaluationProfitTarget,
        }
    }

    /// Parses the stored (or requested) JSON object of `field`. `max_position_qty` has no value.
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        field: RuleField,
        d: D,
    ) -> Result<Self, PresetRulesError> {
        let e = |m: D::Error| bad(format!("{} value: {m}", field.as_str()));
        Ok(match field {
            RuleField::MaxPositionQty => return Err(bad("max_position_qty has no stored value")),
            RuleField::Drawdown => {
                Self::Drawdown(drawdown(RawDrawdown::deserialize(d).map_err(e)?)?)
            }
            RuleField::DailyLossLimit => {
                Self::DailyLossLimit(daily_loss(RawDailyLoss::deserialize(d).map_err(e)?)?)
            }
            RuleField::DailyProfitTarget => Self::DailyProfitTarget(money(
                &RawAmount::deserialize(d).map_err(e)?.amount,
                "daily_profit_target",
            )?),
            RuleField::TradingDayReset => {
                Self::TradingDayReset(reset(RawReset::deserialize(d).map_err(e)?)?)
            }
            RuleField::ConsistencyRule => {
                Self::ConsistencyRule(consistency(RawConsistency::deserialize(d).map_err(e)?)?)
            }
            RuleField::ProfitTargetOnReach => {
                Self::ProfitTargetOnReach(match RawMode::deserialize(d).map_err(e)?.mode.as_str() {
                    "notify" => ProfitTargetOnReach::Notify,
                    "flatten_and_block" => ProfitTargetOnReach::FlattenAndBlock,
                    _ => return Err(bad("profit_target_on_reach mode")),
                })
            }
            RuleField::ConsistencyThresholdPct => Self::ConsistencyThresholdPct(threshold_pct(
                &RawPct::deserialize(d).map_err(e)?.pct,
            )?),
            RuleField::CommissionPerContractSide => Self::CommissionPerContractSide(money(
                &RawAmount::deserialize(d).map_err(e)?.amount,
                "commission",
            )?),
            RuleField::EvaluationProfitTarget => Self::EvaluationProfitTarget(money(
                &RawAmount::deserialize(d).map_err(e)?.amount,
                "evaluation target",
            )?),
        })
    }

    /// The JSON object as stored and shown (money as strings, X-3).
    pub fn to_wire(&self) -> WireValue {
        let amount = |m: &MoneyAmount| WireValue::Amount {
            amount: m.to_wire(),
        };
        match self {
            Self::Drawdown(d) => WireValue::Drawdown {
                kind: match d.kind {
                    DrawdownType::TrailingIntraday => "trailing_intraday",
                    DrawdownType::TrailingEod => "trailing_eod",
                    DrawdownType::Static => "static",
                    DrawdownType::Other => "other",
                },
                amount: d.amount.to_wire(),
                stops_trailing_at: d.stops_trailing_at.clone(),
                description: d.description.clone(),
                trailing_lock: d.trailing_lock.as_ref().map(|l| match l {
                    TrailingLock::Never => WireLock {
                        mode: "never",
                        offset: None,
                    },
                    TrailingLock::FloorReachesStart { offset } => WireLock {
                        mode: "floor_reaches_start",
                        offset: offset.as_ref().map(MoneyAmount::to_wire),
                    },
                }),
            },
            Self::DailyLossLimit(l) => match l {
                DailyLoss::Hard(m) => WireValue::DailyLoss {
                    kind: "hard",
                    amount: Some(m.to_wire()),
                },
                DailyLoss::SoftPause(m) => WireValue::DailyLoss {
                    kind: "soft_pause",
                    amount: Some(m.to_wire()),
                },
                DailyLoss::None => WireValue::DailyLoss {
                    kind: "none",
                    amount: None,
                },
            },
            Self::DailyProfitTarget(m)
            | Self::CommissionPerContractSide(m)
            | Self::EvaluationProfitTarget(m) => amount(m),
            Self::TradingDayReset(r) => WireValue::Reset {
                local_time: r.local_time.format("%H:%M").to_string(),
                time_zone: r.time_zone.clone(),
                published_text: r.published_text.clone(),
            },
            Self::ConsistencyRule(c) => WireValue::Consistency {
                text: c.text.clone(),
                threshold_pct: c.threshold_pct.map(|t| t.normalize().to_string()),
            },
            Self::ProfitTargetOnReach(m) => WireValue::Mode { mode: m.as_str() },
            Self::ConsistencyThresholdPct(p) => WireValue::Pct {
                pct: {
                    let mut v = p.normalize();
                    v.rescale(1);
                    v.to_string()
                },
            },
        }
    }
}

/// `RuleValue` as its JSON object (serialize with serde).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(untagged)]
pub enum WireValue {
    Drawdown {
        #[serde(rename = "type")]
        kind: &'static str,
        amount: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        stops_trailing_at: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        trailing_lock: Option<WireLock>,
    },
    DailyLoss {
        kind: &'static str,
        #[serde(skip_serializing_if = "Option::is_none")]
        amount: Option<String>,
    },
    Amount {
        amount: String,
    },
    Reset {
        local_time: String,
        time_zone: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        published_text: Option<String>,
    },
    Consistency {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        threshold_pct: Option<String>,
    },
    Mode {
        mode: &'static str,
    },
    Pct {
        pct: String,
    },
}

/// `trailing_lock` as its JSON object.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct WireLock {
    pub mode: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMode {
    mode: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPct {
    pct: String,
}

/// `^(100(\.0)?|[0-9]{1,2}(\.[0-9])?)$`, greater than 0 (Q-S13-7).
fn threshold_pct(text: &str) -> Result<Decimal, PresetRulesError> {
    let (int, frac) = text.split_once('.').unwrap_or((text, ""));
    let shape = !int.is_empty()
        && int.len() <= 3
        && int.bytes().all(|b| b.is_ascii_digit())
        && frac.len() <= 1
        && frac.bytes().all(|b| b.is_ascii_digit())
        && !(text.contains('.') && frac.is_empty());
    let value = Decimal::from_str(text)
        .ok()
        .filter(|_| shape)
        .ok_or_else(|| bad("consistency_threshold_pct"))?;
    if value <= Decimal::ZERO || value > Decimal::from(100) {
        return Err(bad("consistency_threshold_pct range"));
    }
    Ok(value)
}

/// `chose` of a value resolving a not-confirmed field (Q-S16-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chose {
    PublishedValue,
    OtherValue,
}

impl Chose {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublishedValue => "published_value",
            Self::OtherValue => "other_value",
        }
    }
}

/// One `accounts.account_rule_value` row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountRuleValue {
    pub field: RuleField,
    pub origin: RuleOrigin,
    /// `None` for `max_position_qty` and for a tombstone (`is_unset`).
    pub value: Option<RuleValue>,
    pub is_unset: bool,
    /// With `chosen_from_conflict` only, with `chosen_at`.
    pub chose: Option<(Chose, DateTime<Utc>)>,
    /// This field's own save time ('latest save wins', ADR-0006 §3.7).
    pub saved_at: DateTime<Utc>,
}

impl AccountRuleValue {
    /// Same stored meaning (origin, value, unset, choice) regardless of when it was saved: a
    /// write of the same thing changes nothing (Q48 effective_from must not move by itself).
    pub fn same_as(&self, other: &Self) -> bool {
        self.field == other.field
            && self.origin == other.origin
            && self.value == other.value
            && self.is_unset == other.is_unset
            && self.chose.map(|(c, _)| c) == other.chose.map(|(c, _)| c)
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
                "../../../../../../contracts/presets/v1/examples/valid/",
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

    /// Presets 1.2 shapes: the reason decides what must and must not be there.
    #[test]
    fn not_confirmed_reasons() {
        let quotes = json!([{"text": "6PM", "language": "en", "url": "https://firm.example/rules",
                             "accessed_at": "2026-10-20", "page_title": "Rules"}]);
        let research = json!({"searched_on": "2026-10-20", "sources": src()});
        let nc = |extra: Value| {
            let mut f = json!({"status": "not_confirmed"});
            for (k, v) in extra.as_object().unwrap() {
                f[k] = v.clone();
            }
            let mut v = base();
            v["trading_day_reset"] = f;
            parse(v)
        };
        let ok =
            nc(json!({"not_confirmed_reason": "partial", "quotes": quotes, "research": research}))
                .unwrap();
        assert_eq!(
            ok.trading_day_reset,
            Provenance::NotConfirmed {
                published_values: vec![],
                reason: NotConfirmedReason::Partial
            }
        );
        assert!(nc(json!({"not_confirmed_reason": "not_published", "research": research})).is_ok());
        for (name, bad_field) in [
            (
                "partial without research",
                json!({"not_confirmed_reason": "partial", "quotes": quotes}),
            ),
            (
                "partial without quotes",
                json!({"not_confirmed_reason": "partial", "research": research}),
            ),
            (
                "not_published with quotes",
                json!({"not_confirmed_reason": "not_published", "quotes": quotes, "research": research}),
            ),
            (
                "unknown reason",
                json!({"not_confirmed_reason": "maybe", "research": research}),
            ),
            (
                "conflicting with nothing",
                json!({"not_confirmed_reason": "conflicting"}),
            ),
        ] {
            assert!(nc(bad_field).is_err(), "{name}");
        }
        let mut v = base();
        v["drawdown"]["not_confirmed_reason"] = json!("conflicting");
        assert!(parse(v.clone()).is_ok(), "explicit conflicting = absent");
        v["drawdown"]["research"] = research.clone();
        assert!(parse(v).is_err(), "conflicting with research");
        let mut v = base();
        v["max_position_qty"]["not_confirmed_reason"] = json!("partial");
        assert!(parse(v).is_err(), "verified with a reason");
    }

    #[test]
    fn trailing_lock_shapes() {
        let mut v = base();
        v["drawdown"] = verified(json!({"type": "trailing_eod", "amount": "2000.00",
            "trailing_lock": {"mode": "floor_reaches_start", "offset": "100.00"}}));
        let r = parse(v).unwrap();
        assert!(matches!(
            r.drawdown.verified().and_then(|d| d.trailing_lock.clone()),
            Some(TrailingLock::FloorReachesStart { offset: Some(_) })
        ));
        let lock = |l: Value, kind: &str| {
            let mut v = base();
            v["drawdown"] =
                verified(json!({"type": kind, "amount": "2000.00", "trailing_lock": l}));
            parse(v)
        };
        assert!(lock(json!({"mode": "never"}), "trailing_intraday").is_ok());
        assert!(
            lock(
                json!({"mode": "never", "offset": "1.00"}),
                "trailing_intraday"
            )
            .is_err(),
            "never has no offset"
        );
        assert!(
            lock(json!({"mode": "floor_reaches_start"}), "static").is_err(),
            "static does not trail"
        );
        assert!(lock(json!({"mode": "sometimes"}), "trailing_eod").is_err());
        assert!(
            lock(
                json!({"mode": "floor_reaches_start", "offset": 100}),
                "trailing_eod"
            )
            .is_err(),
            "money as number"
        );
        // Round trip through the stored form.
        let value = RuleValue::deserialize(
            RuleField::Drawdown,
            json!({"type": "trailing_eod", "amount": "2000.00",
                    "trailing_lock": {"mode": "floor_reaches_start", "offset": "100.00"}}),
        )
        .unwrap();
        let wire = serde_json::to_value(value.to_wire()).unwrap();
        assert_eq!(
            wire["trailing_lock"],
            json!({"mode": "floor_reaches_start", "offset": "100.00"})
        );
        assert_eq!(
            RuleValue::deserialize(RuleField::Drawdown, &wire).unwrap(),
            value
        );
    }

    #[test]
    fn a_source_may_carry_a_title_note_instead_of_a_title() {
        let mut v = base();
        v["max_position_qty"]["sources"] = json!([{"url": "https://firm.example/rules", "accessed_at": "2026-10-20",
            "title_note": "title not retrieved: site blocks automated access, 2026-10-24"}]);
        assert!(parse(v.clone()).is_ok());
        v["max_position_qty"]["sources"][0]["page_title"] = json!("Rules");
        assert!(parse(v).is_err(), "a note only without a title");
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
