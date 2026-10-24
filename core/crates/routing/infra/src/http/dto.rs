//! Wire shapes of the route operations and their mapping to use-case commands and views.
//! Bodies are deserialized only after schema validation, so serde errors here mean drift.

use access::StepUpProof;
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use uuid::Uuid;

use routing_application::enterprise::{
    AccountId, EffectiveStatus, RouteReason, RouteSignalType, SignalTypes, SourceId,
};
use routing_application::routes::{
    Candidate, Candidates, CreateRouteCommand, RouteView, UpdateRouteCommand,
};

/// A field whose value the schema accepts but the domain refuses (400 with this pointer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadField(pub String);

pub fn timestamp(t: DateTime<Utc>) -> String {
    t.to_rfc3339_opts(SecondsFormat::AutoSi, true)
}

// ---- requests -----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct StepUpBody {
    pub challenge_id: String,
    pub code: String,
}

impl StepUpBody {
    pub fn proof(self) -> StepUpProof {
        StepUpProof {
            challenge_id: self.challenge_id,
            code: self.code,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateBody {
    pub source_id: Uuid,
    pub account_id: Uuid,
    pub signal_types: Vec<String>,
    pub is_enabled: Option<bool>,
    pub step_up: Option<StepUpBody>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBody {
    pub signal_types: Vec<String>,
    pub is_enabled: bool,
    /// The schema bounds it below only; a value the Core cannot store is a 400.
    pub row_version: Number,
    pub step_up: Option<StepUpBody>,
}

/// The schema already refuses empty lists, duplicates and unknown types; the value object's
/// refusal is reported at the same pointers.
pub fn signal_types(raw: &[String]) -> Result<SignalTypes, BadField> {
    let types = raw
        .iter()
        .enumerate()
        .map(|(i, t)| {
            RouteSignalType::parse(t).ok_or_else(|| BadField(format!("/signal_types/{i}")))
        })
        .collect::<Result<Vec<_>, _>>()?;
    SignalTypes::new(types).map_err(|_| BadField("/signal_types".into()))
}

/// The create command and the optional step-up proof (consumed by the HTTP adapter's
/// `EnableAuthorizer`).
pub fn create_command(
    b: CreateBody,
) -> Result<(CreateRouteCommand, Option<StepUpProof>), BadField> {
    let cmd = CreateRouteCommand {
        source_id: SourceId::from_uuid(b.source_id),
        account_id: AccountId::from_uuid(b.account_id),
        signal_types: signal_types(&b.signal_types)?,
        // Contract: defaults to true.
        is_enabled: b.is_enabled.unwrap_or(true),
    };
    Ok((cmd, b.step_up.map(StepUpBody::proof)))
}

pub fn update_command(
    b: UpdateBody,
) -> Result<(UpdateRouteCommand, Option<StepUpProof>), BadField> {
    let row_version = b
        .row_version
        .as_i64()
        .and_then(|v| i32::try_from(v).ok())
        .filter(|v| *v >= 1)
        .ok_or_else(|| BadField("/row_version".into()))?;
    let cmd = UpdateRouteCommand {
        signal_types: signal_types(&b.signal_types)?,
        is_enabled: b.is_enabled,
        row_version,
    };
    Ok((cmd, b.step_up.map(StepUpBody::proof)))
}

// ---- responses ----------------------------------------------------------------------------

/// `RouteReason`: the detail field that goes with the code, and no other.
#[derive(Debug, Serialize)]
pub struct RouteReasonDto {
    pub code: &'static str,
    pub blocks: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_state: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_ids: Option<Vec<Uuid>>,
}

impl From<&RouteReason> for RouteReasonDto {
    fn from(r: &RouteReason) -> Self {
        let mut dto = Self {
            code: r.code(),
            blocks: r.blocks().as_str(),
            lifecycle_state: None,
            opt_in_state: None,
            instrument_ids: None,
        };
        match r {
            RouteReason::AccountInactive(state) => dto.lifecycle_state = Some(state.as_str()),
            RouteReason::AutomationNotAccepted(n) | RouteReason::CopyNotAccepted(n) => {
                dto.opt_in_state = Some(n.as_str());
            }
            RouteReason::NoBracketProfile(ids) => dto.instrument_ids = Some(ids.clone()),
            _ => {}
        }
        dto
    }
}

fn reasons(rs: &[RouteReason]) -> Vec<RouteReasonDto> {
    rs.iter().map(RouteReasonDto::from).collect()
}

/// `RouteStatus`.
#[derive(Debug, Serialize)]
pub struct RouteStatusDto {
    pub status: &'static str,
    pub reasons: Vec<RouteReasonDto>,
    pub computed_at: String,
}

impl RouteStatusDto {
    pub fn new(e: &EffectiveStatus, computed_at: DateTime<Utc>) -> Self {
        Self {
            status: e.status.as_str(),
            reasons: reasons(&e.reasons),
            computed_at: timestamp(computed_at),
        }
    }
}

/// `Route`.
#[derive(Debug, Serialize)]
pub struct RouteDto {
    pub id: Uuid,
    pub source_id: Uuid,
    pub account_id: Uuid,
    pub signal_types: Vec<&'static str>,
    pub is_enabled: bool,
    pub effective: RouteStatusDto,
    pub row_version: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&RouteView> for RouteDto {
    fn from(v: &RouteView) -> Self {
        let r = &v.route;
        Self {
            id: r.id.as_uuid(),
            source_id: r.source_id.as_uuid(),
            account_id: r.account_id.as_uuid(),
            signal_types: r.signal_types.iter().map(RouteSignalType::as_str).collect(),
            is_enabled: r.is_enabled,
            effective: RouteStatusDto::new(&v.effective, v.computed_at),
            row_version: r.row_version,
            created_at: timestamp(r.created_at),
            updated_at: timestamp(r.updated_at),
        }
    }
}

/// `RouteList`.
#[derive(Debug, Serialize)]
pub struct RouteListDto {
    pub routes: Vec<RouteDto>,
}

impl From<&[RouteView]> for RouteListDto {
    fn from(views: &[RouteView]) -> Self {
        Self {
            routes: views.iter().map(RouteDto::from).collect(),
        }
    }
}

/// `RouteCandidate`.
#[derive(Debug, Serialize)]
pub struct RouteCandidateDto {
    pub account_id: Uuid,
    pub blocking_reasons: Vec<RouteReasonDto>,
    pub status_if_routed: RouteStatusDto,
}

impl RouteCandidateDto {
    fn new(c: &Candidate, computed_at: DateTime<Utc>) -> Self {
        Self {
            account_id: c.account_id.as_uuid(),
            blocking_reasons: reasons(&c.blocking_reasons),
            status_if_routed: RouteStatusDto::new(&c.status_if_routed, computed_at),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ManualOnlyDto {
    pub account_id: Uuid,
    /// The preset's `RuleQuote`, verbatim.
    pub reason: Value,
}

/// `RouteCandidates`.
#[derive(Debug, Serialize)]
pub struct RouteCandidatesDto {
    pub source_id: Uuid,
    pub candidates: Vec<RouteCandidateDto>,
    pub manual_only: Vec<ManualOnlyDto>,
}

impl From<&Candidates> for RouteCandidatesDto {
    fn from(c: &Candidates) -> Self {
        Self {
            source_id: c.source_id.as_uuid(),
            candidates: c
                .candidates
                .iter()
                .map(|x| RouteCandidateDto::new(x, c.computed_at))
                .collect(),
            manual_only: c
                .manual_only
                .iter()
                .map(|(id, reason)| ManualOnlyDto {
                    account_id: id.as_uuid(),
                    reason: reason.clone(),
                })
                .collect(),
        }
    }
}
