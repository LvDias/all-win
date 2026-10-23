//! Wire shapes of the accounts operations and their mapping to use-case commands and views.
//! Bodies are deserialized only after schema validation, so serde errors here mean drift.

use std::str::FromStr;

use access::StepUpProof;
use chrono::{DateTime, NaiveDate, SecondsFormat, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use uuid::Uuid;

use accounts_application::commands::{
    BindingRef, CommissionInput, CreateAccount, OptInChoices, PropFieldsSent, SelectionInput,
    UpdateAccount,
};
use accounts_application::enterprise::{
    AccountId, AccountKind, AccountName, Choice, ChoiceRequest, ConnectionId, FirmPlatform,
    ManualText, MaxPositionQty, MoneyAmount, Multiplier, OptInState, PresetVersionId,
    RuleVersionId, SizeAmount,
};
use accounts_application::ports::RouteRef;
use accounts_application::views::{AccountView, ArchiveImpact, OptInView};

/// A field whose value the schema accepts but a value object refuses (e.g. a blank name).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadField(pub &'static str);

pub fn timestamp(t: DateTime<Utc>) -> String {
    t.to_rfc3339_opts(SecondsFormat::AutoSi, true)
}

// ---- requests -----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct BindingBody {
    pub connection_id: Uuid,
    pub platform_account: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChoiceBody {
    pub choice: String,
    pub rule_version_id: Uuid,
}

#[derive(Debug, Default, Deserialize)]
pub struct OptInsBody {
    pub automation: Option<ChoiceBody>,
    pub copy: Option<ChoiceBody>,
}

#[derive(Debug, Deserialize)]
pub struct StepUpBody {
    pub challenge_id: String,
    pub code: String,
}

impl StepUpBody {
    fn proof(self) -> StepUpProof {
        StepUpProof {
            challenge_id: self.challenge_id,
            code: self.code,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateBody {
    pub kind: String,
    pub display_name: String,
    pub preset_version_id: Option<Uuid>,
    pub firm_platform: Option<String>,
    pub firm_name: Option<String>,
    pub plan_name: Option<String>,
    pub phase: Option<String>,
    pub size_amount: Option<String>,
    pub multiplier: Option<i64>,
    pub max_position_qty: i64,
    pub commission_per_contract_side: Option<String>,
    pub evaluation_start: Option<String>,
    pub evaluation_earned_before: Option<String>,
    pub binding: Option<BindingBody>,
    pub rule_values: Option<Value>,
    pub profile_size: Option<String>,
    pub opt_ins: Option<OptInsBody>,
    pub step_up: Option<StepUpBody>,
}

/// `Some(None)` is an explicit `null`; `None` is absent.
fn explicit_null<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Option<String>>, D::Error> {
    Option::<String>::deserialize(d).map(Some)
}

#[derive(Debug, Deserialize)]
pub struct UpdateBody {
    pub row_version: i32,
    pub display_name: String,
    pub preset_version_id: Option<Uuid>,
    pub firm_platform: Option<String>,
    pub firm_name: Option<String>,
    pub plan_name: Option<String>,
    pub phase: Option<String>,
    pub size_amount: Option<String>,
    pub multiplier: Option<i64>,
    pub max_position_qty: i64,
    #[serde(default, deserialize_with = "explicit_null")]
    pub commission_per_contract_side: Option<Option<String>>,
    pub binding: Option<BindingBody>,
    pub rule_values: Option<Value>,
    pub opt_ins: Option<OptInsBody>,
    pub step_up: Option<StepUpBody>,
}

#[derive(Debug, Deserialize)]
pub struct LifecycleBody {
    pub state: String,
    pub row_version: i32,
}

#[derive(Debug, Deserialize)]
pub struct OptInBody {
    pub choice: String,
    pub rule_version_id: Uuid,
    pub step_up: Option<StepUpBody>,
}

#[derive(Debug, Deserialize)]
pub struct RestartBody {
    pub evaluation_start: String,
    pub evaluation_earned_before: Option<String>,
    pub row_version: i32,
}

fn dec(raw: &str, field: &'static str) -> Result<Decimal, BadField> {
    Decimal::from_str(raw).map_err(|_| BadField(field))
}

pub fn size(raw: &str, field: &'static str) -> Result<SizeAmount, BadField> {
    SizeAmount::new(dec(raw, field)?).map_err(|_| BadField(field))
}

pub fn money(raw: &str, field: &'static str) -> Result<MoneyAmount, BadField> {
    MoneyAmount::new(dec(raw, field)?).map_err(|_| BadField(field))
}

pub fn date(raw: &str, field: &'static str) -> Result<NaiveDate, BadField> {
    NaiveDate::parse_from_str(raw, "%Y-%m-%d").map_err(|_| BadField(field))
}

fn text(raw: &str, field: &'static str) -> Result<ManualText, BadField> {
    ManualText::parse(raw).map_err(|_| BadField(field))
}

pub fn choice(body: &ChoiceBody, field: &'static str) -> Result<ChoiceRequest, BadField> {
    Ok(ChoiceRequest {
        choice: Choice::parse(&body.choice).ok_or(BadField(field))?,
        rule_version_id: RuleVersionId::from_uuid(body.rule_version_id),
    })
}

fn opt_ins(body: Option<&OptInsBody>) -> Result<OptInChoices, BadField> {
    let Some(b) = body else {
        return Ok(OptInChoices::default());
    };
    Ok(OptInChoices {
        automation: b
            .automation
            .as_ref()
            .map(|c| choice(c, "/opt_ins/automation/choice"))
            .transpose()?,
        copy: b
            .copy
            .as_ref()
            .map(|c| choice(c, "/opt_ins/copy/choice"))
            .transpose()?,
    })
}

#[allow(clippy::too_many_arguments)]
fn selection(
    preset_version_id: Option<Uuid>,
    firm_platform: Option<&str>,
    firm_name: Option<&str>,
    plan_name: Option<&str>,
    phase: Option<&str>,
) -> Result<SelectionInput, BadField> {
    match (preset_version_id, firm_name, plan_name, phase) {
        (Some(id), ..) => Ok(SelectionInput::Preset {
            preset_version_id: PresetVersionId::from_uuid(id),
            firm_platform: firm_platform
                .map(FirmPlatform::parse)
                .transpose()
                .map_err(|_| BadField("/firm_platform"))?,
        }),
        (None, Some(f), Some(p), Some(ph)) => Ok(SelectionInput::Manual {
            firm_name: text(f, "/firm_name")?,
            plan_name: text(p, "/plan_name")?,
            phase: text(ph, "/phase")?,
        }),
        _ => Ok(SelectionInput::None),
    }
}

fn binding(b: Option<BindingBody>) -> Option<BindingRef> {
    b.map(|b| BindingRef {
        connection_id: ConnectionId::from_uuid(b.connection_id),
        platform_account: b.platform_account,
    })
}

fn multiplier(m: Option<i64>) -> Result<Multiplier, BadField> {
    m.map_or(Ok(Multiplier::ONE), |m| {
        Multiplier::new(m).map_err(|_| BadField("/multiplier"))
    })
}

pub fn create_command(
    b: CreateBody,
    idempotency_key: Uuid,
    request_hash: [u8; 32],
) -> Result<CreateAccount, BadField> {
    let kind = AccountKind::parse(&b.kind).ok_or(BadField("/kind"))?;
    Ok(CreateAccount {
        idempotency_key,
        request_hash,
        kind,
        display_name: AccountName::parse(&b.display_name).map_err(|_| BadField("/display_name"))?,
        selection: match kind {
            AccountKind::Real => SelectionInput::None,
            AccountKind::Prop => selection(
                b.preset_version_id,
                b.firm_platform.as_deref(),
                b.firm_name.as_deref(),
                b.plan_name.as_deref(),
                b.phase.as_deref(),
            )?,
        },
        size_amount: b
            .size_amount
            .as_deref()
            .map(|s| size(s, "/size_amount"))
            .transpose()?,
        multiplier: multiplier(b.multiplier)?,
        max_position_qty: MaxPositionQty::new(b.max_position_qty)
            .map_err(|_| BadField("/max_position_qty"))?,
        commission_per_contract_side: b
            .commission_per_contract_side
            .as_deref()
            .map(|s| money(s, "/commission_per_contract_side"))
            .transpose()?,
        evaluation_start: b
            .evaluation_start
            .as_deref()
            .map(|s| date(s, "/evaluation_start"))
            .transpose()?,
        evaluation_earned_before: b
            .evaluation_earned_before
            .as_deref()
            .map(|s| money(s, "/evaluation_earned_before"))
            .transpose()?,
        binding: binding(b.binding),
        rule_values: b.rule_values,
        // Any kind (5.x, Q73 b): the profile planner decides whether the size is a tie and ignores
        // it otherwise.
        profile_size: b
            .profile_size
            .as_deref()
            .map(|s| size(s, "/profile_size"))
            .transpose()?,
        opt_ins: opt_ins(b.opt_ins.as_ref())?,
        step_up: b.step_up.map(StepUpBody::proof),
    })
}

pub fn update_command(account_id: AccountId, b: UpdateBody) -> Result<UpdateAccount, BadField> {
    let sent = PropFieldsSent {
        preset_version_id: b.preset_version_id.is_some(),
        firm_platform: b.firm_platform.is_some(),
        firm_name: b.firm_name.is_some(),
        plan_name: b.plan_name.is_some(),
        phase: b.phase.is_some(),
        size_amount: b.size_amount.is_some(),
        rule_values: b.rule_values.is_some(),
    };
    Ok(UpdateAccount {
        account_id,
        row_version: b.row_version,
        display_name: AccountName::parse(&b.display_name).map_err(|_| BadField("/display_name"))?,
        selection: selection(
            b.preset_version_id,
            b.firm_platform.as_deref(),
            b.firm_name.as_deref(),
            b.plan_name.as_deref(),
            b.phase.as_deref(),
        )?,
        size_amount: b
            .size_amount
            .as_deref()
            .map(|s| size(s, "/size_amount"))
            .transpose()?,
        multiplier: multiplier(b.multiplier)?,
        max_position_qty: MaxPositionQty::new(b.max_position_qty)
            .map_err(|_| BadField("/max_position_qty"))?,
        commission_per_contract_side: match b.commission_per_contract_side {
            None => CommissionInput::Unchanged,
            Some(None) => CommissionInput::Remove,
            Some(Some(s)) => CommissionInput::Set(money(&s, "/commission_per_contract_side")?),
        },
        binding: binding(b.binding),
        rule_values: b.rule_values,
        opt_ins_sent: b.opt_ins.is_some(),
        opt_ins: opt_ins(b.opt_ins.as_ref())?,
        sent,
        step_up: b.step_up.map(StepUpBody::proof),
    })
}

/// SHA-256 of the canonical body (keys sorted, `step_up` removed): the create replay key.
pub fn request_hash(body: &Value) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut v = body.clone();
    if let Some(o) = v.as_object_mut() {
        o.remove("step_up");
    }
    let mut out = Vec::new();
    canonical(&v, &mut out);
    Sha256::digest(&out).into()
}

fn canonical(v: &Value, out: &mut Vec<u8>) {
    match v {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            out.push(b'{');
            for (i, k) in keys.into_iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                out.extend(Value::String(k.clone()).to_string().as_bytes());
                out.push(b':');
                if let Some(child) = map.get(k) {
                    canonical(child, out);
                }
            }
            out.push(b'}');
        }
        Value::Array(items) => {
            out.push(b'[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                canonical(item, out);
            }
            out.push(b']');
        }
        other => out.extend(other.to_string().as_bytes()),
    }
}

// ---- responses ----------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct OptInStateDto {
    pub kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firm_stance: Option<&'static str>,
    pub state: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_reason: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_version_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_rule_version_id: Option<Uuid>,
}

impl From<&OptInView> for OptInStateDto {
    fn from(v: &OptInView) -> Self {
        let recorded = v.state.recorded();
        Self {
            kind: v.kind.as_str(),
            firm_stance: v.firm_stance.map(|s| s.as_str()),
            state: v.state.state_str(),
            pending_reason: match v.state {
                OptInState::PendingReview { reason, .. } => Some(reason.as_str()),
                _ => None,
            },
            choice: recorded.map(|c| c.choice.as_str()),
            chosen_at: recorded.map(|c| timestamp(c.chosen_at)),
            rule_version_id: recorded.map(|c| c.rule_version_id.as_uuid()),
            current_rule_version_id: v.current_rule_version_id.map(|id| id.as_uuid()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AccountBindingDto {
    pub connection_id: Uuid,
    pub connection_name: String,
    pub connection_status: String,
    pub platform_account: String,
    pub kind: String,
    pub oco_location: String,
    pub bound_at: String,
}

#[derive(Debug, Serialize)]
pub struct AccountOptInsDto {
    pub automation: OptInStateDto,
    pub copy: OptInStateDto,
}

#[derive(Debug, Serialize)]
pub struct AccountDto {
    pub id: Uuid,
    pub kind: &'static str,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firm_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset_update_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firm_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset_version_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firm_platform: Option<String>,
    pub size_amount: String,
    pub currency: &'static str,
    pub multiplier: u32,
    pub max_position_qty: u32,
    pub lifecycle_state: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding: Option<AccountBindingDto>,
    pub opt_ins: AccountOptInsDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_per_contract_side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_earned_before: Option<String>,
    pub has_open_position: bool,
    pub row_version: i32,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
}

impl From<&AccountView> for AccountDto {
    fn from(v: &AccountView) -> Self {
        let a = &v.account;
        let prop = v.prop.as_ref();
        Self {
            id: a.id.as_uuid(),
            kind: a.kind.as_str(),
            display_name: a.display_name.as_str().to_owned(),
            firm_name: prop.map(|p| p.firm_name.clone()),
            plan_name: prop.map(|p| p.plan_name.clone()),
            phase: prop.map(|p| p.phase.clone()),
            preset_update_available: prop.and_then(|p| p.preset_update_available),
            firm_id: prop.and_then(|p| p.firm_id).map(|f| f.as_uuid()),
            preset_version_id: prop.and_then(|p| p.preset_version_id).map(|p| p.as_uuid()),
            firm_platform: prop
                .and_then(|p| p.firm_platform.as_ref())
                .map(|p| p.as_str().to_owned()),
            size_amount: a.size_amount.to_wire(),
            currency: "USD",
            multiplier: a.multiplier.value(),
            max_position_qty: a.max_position_qty.value(),
            lifecycle_state: a.lifecycle.as_str(),
            binding: match (&v.binding, a.binding) {
                (Some(b), Some(stored)) => Some(AccountBindingDto {
                    connection_id: b.connection_id.as_uuid(),
                    connection_name: b.connection_name.clone(),
                    connection_status: b.connection_status.clone(),
                    platform_account: b.platform_account.clone(),
                    kind: b.kind.clone(),
                    oco_location: b.oco_location.clone(),
                    bound_at: timestamp(stored.bound_at),
                }),
                _ => None,
            },
            opt_ins: AccountOptInsDto {
                automation: (&v.automation).into(),
                copy: (&v.copy).into(),
            },
            commission_per_contract_side: a.commission_per_contract_side.map(|m| m.to_wire()),
            evaluation_start: a.evaluation_start.map(|d| d.to_string()),
            evaluation_earned_before: a.evaluation_earned_before.map(|m| m.to_wire()),
            has_open_position: v.has_open_position,
            row_version: a.row_version,
            created_at: timestamp(a.created_at),
            updated_at: timestamp(a.updated_at),
            archived_at: a.archived_at.map(timestamp),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AccountListDto {
    pub accounts: Vec<AccountDto>,
}

#[derive(Debug, Serialize)]
pub struct RouteDto {
    pub source_id: Uuid,
    pub source_name: String,
}

#[derive(Debug, Serialize)]
pub struct ArchiveImpactDto {
    pub has_open_position: bool,
    pub routes: Vec<RouteDto>,
    /// Master/follower relations (S19); always empty before it.
    pub follower_links: Vec<Value>,
}

impl From<&ArchiveImpact> for ArchiveImpactDto {
    fn from(i: &ArchiveImpact) -> Self {
        Self {
            has_open_position: i.has_open_position,
            routes: i
                .routes
                .iter()
                .map(|r: &RouteRef| RouteDto {
                    source_id: r.source_id,
                    source_name: r.source_name.clone(),
                })
                .collect(),
            follower_links: Vec::new(),
        }
    }
}
