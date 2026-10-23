//! Events of the release job, written to `app.outbox` in the user's transaction:
//! - `accounts.opt_in_suspended` / `accounts.opt_in_rule_updated`: the user-visible audit, actor
//!   `system` (an automatic change of consent), common envelope `{actor, details, refs}`;
//! - `account.changed`: the SSE refetch notice for each changed account;
//! - `presets.version_available`: the SSE notice, one per user and firm release, with a
//!   deterministic outbox id so a reprocess never duplicates it (Lastro, data model 16.12 L2):
//!   UUIDv5([`VERSION_AVAILABLE_NAMESPACE`], 16 raw bytes of `release_id` ‖ 16 raw bytes of
//!   `user_id`), inserted with `ON CONFLICT (id) DO NOTHING`.

use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use uuid::Uuid;

use accounts_enterprise::{
    AccountId, AccountName, FirmId, OptInKind, PendingReason, RuleVersionId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseJobEvent {
    OptInSuspended {
        at: DateTime<Utc>,
        account_id: AccountId,
        account_name: AccountName,
        kind: OptInKind,
        cause: PendingReason,
        previous_rule_version_id: RuleVersionId,
        rule_version_id: RuleVersionId,
        rule_version_number: i32,
        firm_name: String,
    },
    OptInRuleUpdated {
        at: DateTime<Utc>,
        account_id: AccountId,
        account_name: AccountName,
        kind: OptInKind,
        previous_rule_version_id: RuleVersionId,
        rule_version_id: RuleVersionId,
    },
    AccountChanged {
        at: DateTime<Utc>,
        account_id: AccountId,
    },
    VersionAvailable {
        at: DateTime<Utc>,
        release_id: Uuid,
        firm_id: FirmId,
        account_ids: Vec<AccountId>,
    },
}

/// Audited types (the audit consumer registers these) and every type the job emits.
pub const RELEASE_JOB_AUDIT_TYPES: [&str; 2] =
    ["accounts.opt_in_suspended", "accounts.opt_in_rule_updated"];
pub const RELEASE_JOB_EVENT_TYPES: [&str; 4] = [
    "accounts.opt_in_suspended",
    "accounts.opt_in_rule_updated",
    "account.changed",
    "presets.version_available",
];

impl ReleaseJobEvent {
    pub const fn event_type(&self) -> &'static str {
        match self {
            Self::OptInSuspended { .. } => "accounts.opt_in_suspended",
            Self::OptInRuleUpdated { .. } => "accounts.opt_in_rule_updated",
            Self::AccountChanged { .. } => "account.changed",
            Self::VersionAvailable { .. } => "presets.version_available",
        }
    }

    pub const fn at(&self) -> DateTime<Utc> {
        match self {
            Self::OptInSuspended { at, .. }
            | Self::OptInRuleUpdated { at, .. }
            | Self::AccountChanged { at, .. }
            | Self::VersionAvailable { at, .. } => *at,
        }
    }

    pub fn payload(&self) -> Value {
        let audit = |details: Value, account_id: &AccountId, name: &AccountName| {
            json!({"actor": "system", "details": details,
                   "refs": {"account_id": account_id.as_uuid(), "account_name": name.as_str()}})
        };
        match self {
            Self::OptInSuspended {
                account_id,
                account_name,
                kind,
                cause,
                previous_rule_version_id,
                rule_version_id,
                rule_version_number,
                firm_name,
                ..
            } => audit(
                json!({
                    "kind": kind.as_str(), "cause": cause.as_str(),
                    "previous_rule_version_id": previous_rule_version_id.as_uuid(),
                    "rule_version_id": rule_version_id.as_uuid(),
                    "rule_version_number": rule_version_number, "firm_name": firm_name,
                }),
                account_id,
                account_name,
            ),
            Self::OptInRuleUpdated {
                account_id,
                account_name,
                kind,
                previous_rule_version_id,
                rule_version_id,
                ..
            } => audit(
                json!({
                    "account_id": account_id.as_uuid(), "kind": kind.as_str(),
                    "previous_rule_version_id": previous_rule_version_id.as_uuid(),
                    "rule_version_id": rule_version_id.as_uuid(), "cause": "not_material",
                }),
                account_id,
                account_name,
            ),
            Self::AccountChanged { account_id, .. } => {
                json!({"kind": "account.changed", "account_id": account_id.as_uuid()})
            }
            Self::VersionAvailable {
                firm_id,
                account_ids,
                ..
            } => json!({
                "kind": "presets.version_available",
                "firm_id": firm_id.as_uuid(),
                "account_ids": account_ids.iter().map(AccountId::as_uuid).collect::<Vec<_>>(),
            }),
        }
    }

    /// The outbox id when the event must be written at most once: `presets.version_available`
    /// per (release, user).
    pub fn fixed_id(&self, user_id: Uuid) -> Option<Uuid> {
        match self {
            Self::VersionAvailable { release_id, .. } => {
                Some(version_available_id(*release_id, user_id))
            }
            _ => None,
        }
    }
}

/// The namespace of `presets.version_available` outbox ids:
/// UUIDv5(NAMESPACE_URL, "https://allwin/ns/presets.version_available").
pub const VERSION_AVAILABLE_NAMESPACE: Uuid =
    Uuid::from_u128(0xeb75edeb_abd3_545e_b41a_4607ae326bd3);

/// UUIDv5 (RFC 9562) of `release_id` ‖ `user_id` (16 + 16 raw bytes) in the namespace above.
pub fn version_available_id(release_id: Uuid, user_id: Uuid) -> Uuid {
    use sha1::{Digest, Sha1};
    let mut h = Sha1::new();
    h.update(VERSION_AVAILABLE_NAMESPACE.as_bytes());
    h.update(release_id.as_bytes());
    h.update(user_id.as_bytes());
    let digest = h.finalize();
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    uuid::Builder::from_sha1_bytes(bytes).into_uuid()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_available_has_a_stable_id_per_release_and_user() {
        let e = |release: u128| ReleaseJobEvent::VersionAvailable {
            at: Utc::now(),
            release_id: Uuid::from_u128(release),
            firm_id: FirmId::from_uuid(Uuid::nil()),
            account_ids: vec![],
        };
        let user = Uuid::from_u128(5);
        assert_eq!(e(1).fixed_id(user), e(1).fixed_id(user));
        assert_ne!(e(1).fixed_id(user), e(2).fixed_id(user));
        assert_ne!(e(1).fixed_id(user), e(1).fixed_id(Uuid::from_u128(6)));
        assert_eq!(e(1).fixed_id(user).map(|u| u.get_version_num()), Some(5));
        // Test vector (Python: uuid.uuid5(NS, bytes(release) + bytes(user))), so every
        // implementation derives the same id.
        assert_eq!(
            version_available_id(Uuid::from_u128(1), Uuid::from_u128(5)).to_string(),
            "684f037a-6b8f-5e90-b627-e95e3ed3fe1e"
        );
        let changed = ReleaseJobEvent::AccountChanged {
            at: Utc::now(),
            account_id: AccountId::new_v7(),
        };
        assert_eq!(changed.fixed_id(user), None);
    }
}
