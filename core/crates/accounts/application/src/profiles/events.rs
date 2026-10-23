//! Profile events written to `app.outbox` in the same transaction as the change, in the accounts
//! audit envelope `{actor, details, refs}` (contract `DetailsProfileChanged`, s6-profiles §3).

use chrono::{DateTime, Utc};
use serde_json::{Map, Value, json};
use uuid::Uuid;

use accounts_enterprise::profile::{DistanceProfile, ProfileSide};
use accounts_enterprise::{AccountId, AccountName, SizeAmount};

/// `profiles.account_changed` `action`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileAction {
    InitialCopy,
    Edited,
    Restored,
}

impl ProfileAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InitialCopy => "initial_copy",
            Self::Edited => "edited",
            Self::Restored => "restored",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileEvent {
    DefaultChanged {
        at: DateTime<Utc>,
        instrument_id: Uuid,
        size_amount: SizeAmount,
        previous: Option<DistanceProfile>,
        new: DistanceProfile,
    },
    AccountChanged {
        at: DateTime<Utc>,
        account_id: AccountId,
        account_name: AccountName,
        instrument_id: Uuid,
        action: ProfileAction,
        copied_from_size: Option<SizeAmount>,
        previous: Option<DistanceProfile>,
        new: DistanceProfile,
    },
}

/// Audited event types (the audit consumer registers these; also emitted to the outbox).
pub const PROFILE_EVENT_TYPES: [&str; 2] = ["profiles.default_changed", "profiles.account_changed"];

/// Contract `DistanceProfile` as the audit shows it.
pub fn distance_json(p: &DistanceProfile) -> Value {
    json!({
        "distance_unit": p.unit().as_str(),
        "stop_distance": p.text(ProfileSide::Stop),
        "target_distance": p.text(ProfileSide::Target),
    })
}

impl ProfileEvent {
    pub const fn event_type(&self) -> &'static str {
        match self {
            Self::DefaultChanged { .. } => "profiles.default_changed",
            Self::AccountChanged { .. } => "profiles.account_changed",
        }
    }

    pub const fn at(&self) -> DateTime<Utc> {
        match self {
            Self::DefaultChanged { at, .. } | Self::AccountChanged { at, .. } => *at,
        }
    }

    /// Outbox payload. A default belongs to no account, so its `refs` are empty.
    pub fn payload(&self) -> Value {
        let mut details = Map::new();
        let refs = match self {
            Self::DefaultChanged {
                instrument_id,
                size_amount,
                previous,
                new,
                ..
            } => {
                details.insert("instrument_id".into(), json!(instrument_id));
                details.insert("size_amount".into(), json!(size_amount.to_wire()));
                if let Some(p) = previous {
                    details.insert("previous".into(), distance_json(p));
                }
                details.insert("new".into(), distance_json(new));
                json!({})
            }
            Self::AccountChanged {
                account_id,
                account_name,
                instrument_id,
                action,
                copied_from_size,
                previous,
                new,
                ..
            } => {
                details.insert("instrument_id".into(), json!(instrument_id));
                details.insert("action".into(), json!(action.as_str()));
                if let Some(s) = copied_from_size {
                    details.insert("copied_from_size".into(), json!(s.to_wire()));
                }
                if let Some(p) = previous {
                    details.insert("previous".into(), distance_json(p));
                }
                details.insert("new".into(), distance_json(new));
                json!({"account_id": account_id.as_uuid(), "account_name": account_name.as_str()})
            }
        };
        json!({"actor": "user", "details": Value::Object(details), "refs": refs})
    }
}
