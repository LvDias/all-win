//! `audit` module — application layer: turns published domain events into audit
//! entries. Input is the outbox envelope, so `audit` depends on no other module.
//!
//! Every audited event carries the same payload envelope:
//!
//! ```json
//! { "actor": "user" | "system" | "platform_admin",
//!   "details": { ... },            // exactly AuditDetailsByCode[code]
//!   "refs": { ... } }              // optional: column-only values
//! ```
//!
//! `refs` holds values that become columns but are not part of the code's closed `details`
//! schema (`instruction_id`, `source_id`, `source_name`, `source_key_id`, `account_id`,
//! `account_name`, `message_id`, `signal_type`, `is_exit`). Column values that the `details`
//! schema itself has (`source_id`, `source_name`, `key_id`, `account_id`, `account_name`) are
//! copied from `details` and stay there. Outcome and `safe_to_display` are decided here, per
//! code (s10 §2.1/§5), except a routing decision's outcome, which the producer sets in
//! `refs.outcome` from a closed list.

use std::sync::Arc;

use async_trait::async_trait;
use audit_enterprise::{Actor, AuditEntry, Category, Outcome};
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use shared_kernel::UserId;
use uuid::Uuid;

pub use audit_enterprise as enterprise;

/// A domain event as published through `app.outbox`.
#[derive(Debug, Clone, PartialEq)]
pub struct PublishedEvent {
    pub id: Uuid,
    pub user_id: UserId,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    /// `app.outbox.seq`.
    pub event_seq: i64,
    pub payload: Value,
}

#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("audit store: {0}")]
    Store(String),
    /// A wiring or producer error; the message names the code and field, never a value.
    #[error("not auditable: {0}")]
    NotAuditable(String),
}

#[async_trait]
pub trait AuditStore: Send + Sync {
    /// Appends once per `(occurred_at, source_event_id)`; a redelivery is a no-op.
    async fn append(&self, entry: &AuditEntry) -> Result<(), AuditError>;
}

/// Records a published event in the affected user's audit log.
#[derive(Clone)]
pub struct RecordAuditEntry {
    store: Arc<dyn AuditStore>,
}

impl RecordAuditEntry {
    pub fn new(store: Arc<dyn AuditStore>) -> Self {
        Self { store }
    }

    /// `Ok(true)` once recorded. An event audit cannot record is an error, never "done".
    pub async fn handle(&self, event: &PublishedEvent) -> Result<bool, AuditError> {
        let entry = entry_for(event)?;
        self.store.append(&entry).await.map(|()| true)
    }
}

/// How the Core records one code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeRule {
    pub outcome: OutcomeRule,
    /// Whether the raw `details` may be shown; each code's `details` schema is closed and
    /// checked by a contract test.
    pub safe_to_display: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeRule {
    Fixed(Outcome),
    /// A routing decision code: the producer sets `refs.outcome`, because the same code can
    /// be a skip or the open leg of a close-only `reverse` (Q74). Only the listed outcomes are
    /// accepted for that code (Architect, 2026-10-23; decision `capped` is audited `partial`).
    PerDecision(&'static [Outcome]),
}

const fn decision(allowed: &'static [Outcome]) -> Option<CodeRule> {
    Some(CodeRule {
        outcome: OutcomeRule::PerDecision(allowed),
        safe_to_display: true,
    })
}

const fn rule(outcome: Outcome, safe_to_display: bool) -> Option<CodeRule> {
    Some(CodeRule {
        outcome: OutcomeRule::Fixed(outcome),
        safe_to_display,
    })
}

/// The codes the Core records today, with their outcome (s10 §2.1) and `safe_to_display`.
/// A code missing here is not auditable: adding a producer means adding its rule.
pub fn rule_for(code: &str) -> Option<CodeRule> {
    use Outcome::*;
    match code {
        // S1 (Architect, 2026-10-23: safe, details are closed and redacted).
        "ingress.received" => rule(Received, true),
        "ingress.duplicate_ignored" => rule(Skipped, true),
        "ingress.rejected" | "ingress.rate_limited" => rule(Rejected, true),
        // S7.
        "routing.subscription_changed" => rule(Info, true),
        // S7 C1 (HTTP 5.2.0 `DetailsCommandSent`): an exit command routing committed and the
        // gateway has not sent yet. Closed details {step, kind}.
        "execution.command_requested" => rule(Info, true),
        // S7 decisions (`DetailsRoutingDecision`), one per (instruction, account), with the
        // outcomes each code may carry. Safe to display (Architect, 2026-10-23): the producer's
        // tests validate every emitted code against the contract, and the schema has no free
        // string (routing-application `decisions_tests`).
        "routing.routed" => decision(&[Routed]),
        "routing.quantity_capped" => decision(&[Partial]),
        // Not sent, or the close-only open leg of a `reverse` (Q74; M = 0 for
        // max_position). Protective gates refuse, configuration rules skip (HTTP 5.1.1
        // `AuditOutcome`, which wins over the first table).
        "routing.out_of_sync" | "routing.risk_blocked" => decision(&[Refused, Partial]),
        "routing.max_position_reached"
        | "routing.limits_pending"
        | "routing.no_bracket_profile" => decision(&[Skipped, Partial]),
        "routing.expired" => decision(&[Expired, Partial]),
        "routing.account_inactive"
        | "routing.instrument_unavailable"
        | "routing.platform_unbound"
        | "routing.automation_not_accepted"
        | "routing.copy_not_accepted"
        | "routing.signal_type_not_selected"
        | "routing.nothing_to_close"
        | "routing.nothing_to_reduce"
        | "routing.position_unknown"
        | "routing.adapter_protocol_outdated"
        | "routing.no_correlated_order" => decision(&[Skipped]),
        "routing.position_conflict" => decision(&[Refused]),
        // S3 (28 codes): refused attempts are `rejected`, the rest `info`.
        "security.sign_in_failed" | "security.step_up_failed" | "security.code_rate_limited" => {
            rule(Rejected, true)
        }
        "security.signed_in"
        | "security.code_delivery_failed"
        | "security.new_device_notice_sent"
        | "security.step_up_succeeded"
        | "security.channel_change_requested"
        | "security.channel_change_on_hold"
        | "security.channel_change_completed"
        | "security.channel_change_cancelled"
        | "security.channel_change_not_completed"
        | "security.device_forgotten"
        | "security.signed_out_everywhere"
        | "security.invitation_issued"
        | "security.invitation_revoked"
        | "security.account_created"
        | "sources.created"
        | "sources.renamed"
        | "sources.rotated"
        | "sources.revoked"
        | "connections.created"
        | "connections.renamed"
        | "connections.pairing_code_issued"
        | "connections.paired"
        | "connections.repaired"
        | "connections.unpaired"
        | "connection.status_changed" => rule(Info, true),
        // S5: safe, with the producer's tests (details against the schema, FieldChange.field
        // from a closed list; Architect, 2026-10-23).
        "accounts.created"
        | "accounts.updated"
        | "accounts.archived"
        | "accounts.lifecycle_changed"
        | "accounts.opt_in_chosen"
        | "accounts.evaluation_restarted" => rule(Info, true),
        // No producer (and no producer test) yet: fail-closed.
        "accounts.opt_in_suspended" => rule(Info, false),
        // S2 gateway (HTTP 5.3.0; s10 §2.1 report-status mapping). Not displayable until the
        // producer's details contract test lands (Sonda E-S2-1): fail-closed.
        "execution.filled" => rule(Executed, false),
        "execution.partially_filled" => rule(Partial, false),
        "execution.rejected" => rule(Rejected, false),
        "execution.expired" => rule(Expired, false),
        "execution.state_unknown" | "execution.undelivered" => rule(Unknown, false),
        "execution.command_sent"
        | "execution.order_received"
        | "execution.order_working"
        | "execution.cancelled"
        | "execution.fill_corrected"
        | "execution.unbound_activity"
        | "connection.account_kind_changed"
        | "sync.flagged"
        | "sync.reason_resolved"
        | "sync.restored" => rule(Info, false),
        _ => None,
    }
}

/// `audit_entry.source_name` holds at most 60 characters (the contract's `DisplayName`). The
/// database does not limit `identity.source.name`, so a longer name is cut to 60 characters
/// here instead of failing every entry of that source.
fn snapshot_name(name: String) -> String {
    const MAX: usize = 60;
    match name.char_indices().nth(MAX) {
        Some((cut, _)) => name[..cut].to_owned(),
        None => name,
    }
}

fn not_auditable(code: &str, why: &str) -> AuditError {
    AuditError::NotAuditable(format!("{code}: {why}"))
}

/// Maps one published event to its audit entry.
pub fn entry_for(event: &PublishedEvent) -> Result<AuditEntry, AuditError> {
    let code = event.event_type.as_str();
    let category = Category::of_code(code).ok_or_else(|| not_auditable(code, "no category"))?;
    let rule = rule_for(code).ok_or_else(|| not_auditable(code, "no rule"))?;
    let payload = event
        .payload
        .as_object()
        .ok_or_else(|| not_auditable(code, "payload is not an object"))?;
    let actor = payload
        .get("actor")
        .and_then(Value::as_str)
        .and_then(Actor::parse)
        .ok_or_else(|| not_auditable(code, "missing or invalid actor"))?;
    let details = payload
        .get("details")
        .and_then(Value::as_object)
        .ok_or_else(|| not_auditable(code, "missing details"))?;
    let refs = match payload.get("refs") {
        None => &Map::new(),
        Some(Value::Object(r)) => r,
        Some(_) => return Err(not_auditable(code, "refs is not an object")),
    };
    // refs first, then the column values the details schema itself carries.
    let text = |k: &str, alt: &str| {
        refs.get(k)
            .or_else(|| details.get(alt))
            .and_then(Value::as_str)
            .map(str::to_owned)
    };
    let ref_text = |k: &str| refs.get(k).and_then(Value::as_str).map(str::to_owned);
    let outcome = match rule.outcome {
        OutcomeRule::Fixed(o) => o,
        OutcomeRule::PerDecision(allowed) => refs
            .get("outcome")
            .and_then(Value::as_str)
            .and_then(Outcome::parse)
            .filter(|o| allowed.contains(o))
            .ok_or_else(|| not_auditable(code, "missing or invalid refs.outcome"))?,
    };
    let id = |k: &str| {
        refs.get(k)
            .or_else(|| details.get(k))
            .and_then(Value::as_str)
            .and_then(|s| Uuid::parse_str(s).ok())
    };
    Ok(AuditEntry {
        id: Uuid::now_v7(),
        user_id: event.user_id,
        occurred_at: event.occurred_at,
        source_event_id: event.id,
        event_seq: event.event_seq,
        category,
        code: code.to_owned(),
        outcome,
        actor,
        safe_to_display: rule.safe_to_display,
        instruction_id: id("instruction_id"),
        source_id: id("source_id"),
        source_name: text("source_name", "source_name").map(snapshot_name),
        source_key_id: text("source_key_id", "key_id"),
        account_id: id("account_id"),
        account_name: text("account_name", "account_name"),
        message_id: ref_text("message_id"),
        signal_type: ref_text("signal_type"),
        is_exit: refs.get("is_exit").and_then(Value::as_bool),
        details: Value::Object(details.clone()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn event(code: &str, payload: Value) -> PublishedEvent {
        PublishedEvent {
            id: Uuid::now_v7(),
            user_id: UserId::new_v7(),
            event_type: code.into(),
            occurred_at: Utc::now(),
            event_seq: 42,
            payload,
        }
    }

    #[test]
    fn ingress_envelope_maps_refs_to_columns_and_keeps_details_closed() {
        let iid = Uuid::now_v7();
        let sid = Uuid::now_v7();
        let details = json!({"action":"buy","instrument_symbol":"MNQ","quantity":1,"sent_at":"2026-10-23T12:00:00Z"});
        let e = event(
            "ingress.received",
            json!({"actor":"system","details":details,
                   "refs":{"instruction_id":iid,"source_id":sid,"source_name":"TV main",
                           "source_key_id":"AbCdEf123456","message_id":"m-1","signal_type":"trigger"}}),
        );
        let a = entry_for(&e).unwrap();
        assert_eq!(a.category, Category::Ingress);
        assert_eq!(a.outcome, Outcome::Received);
        assert_eq!(a.actor, Actor::System);
        assert!(a.safe_to_display);
        assert_eq!(a.event_seq, 42);
        assert_eq!(a.instruction_id, Some(iid));
        assert_eq!(a.source_id, Some(sid));
        assert_eq!(a.source_name.as_deref(), Some("TV main"));
        assert_eq!(a.source_key_id.as_deref(), Some("AbCdEf123456"));
        assert_eq!(a.message_id.as_deref(), Some("m-1"));
        assert_eq!(a.signal_type.as_deref(), Some("trigger"));
        assert_eq!(a.details, details);
        assert_eq!(a.source_event_id, e.id);
    }

    #[test]
    fn a_routing_decision_takes_its_outcome_from_refs() {
        let decision = |outcome: Value| {
            event(
                "routing.out_of_sync",
                json!({"actor":"system",
                       "details":{"sizing":{"multiplier":1,"max_position_qty":2},"reverse_closed_qty":1},
                       "refs":{"outcome":outcome,"account_id":Uuid::now_v7(),"account_name":"Apex",
                               "is_exit":false}}),
            )
        };
        let a = entry_for(&decision(json!("partial"))).unwrap();
        assert_eq!(
            (a.category, a.outcome),
            (Category::Routing, Outcome::Partial)
        );
        assert!(a.safe_to_display);
        assert_eq!(a.is_exit, Some(false));
        assert_eq!(
            entry_for(&decision(json!("refused"))).unwrap().outcome,
            Outcome::Refused
        );
        // Missing, unknown, or not allowed for this code: fail closed.
        for bad in [
            Value::Null,
            json!("bogus"),
            json!("info"),
            json!("routed"),
            json!("expired"),
            json!("skipped"),
        ] {
            assert!(matches!(
                entry_for(&decision(bad)),
                Err(AuditError::NotAuditable(_))
            ));
        }
        // The closed per-code table.
        let allowed = |code: &str| match rule_for(code).map(|r| r.outcome) {
            Some(OutcomeRule::PerDecision(a)) => a.to_vec(),
            other => panic!("{code}: {other:?}"),
        };
        use Outcome::*;
        assert_eq!(allowed("routing.routed"), [Routed]);
        assert_eq!(allowed("routing.quantity_capped"), [Partial]);
        for gate in ["routing.out_of_sync", "routing.risk_blocked"] {
            assert_eq!(allowed(gate), [Refused, Partial], "{gate}");
        }
        for gate in [
            "routing.max_position_reached",
            "routing.limits_pending",
            "routing.no_bracket_profile",
        ] {
            assert_eq!(allowed(gate), [Skipped, Partial], "{gate}");
        }
        assert_eq!(allowed("routing.expired"), [Expired, Partial]);
        for skip in [
            "routing.account_inactive",
            "routing.instrument_unavailable",
            "routing.platform_unbound",
            "routing.automation_not_accepted",
            "routing.copy_not_accepted",
            "routing.signal_type_not_selected",
            "routing.nothing_to_close",
            "routing.nothing_to_reduce",
            "routing.position_unknown",
            "routing.adapter_protocol_outdated",
            "routing.no_correlated_order",
        ] {
            assert_eq!(allowed(skip), [Skipped], "{skip}");
        }
        assert_eq!(allowed("routing.position_conflict"), [Refused]);
        let routed_as_skip = event(
            "routing.routed",
            json!({"actor":"system","details":{},"refs":{"outcome":"skipped"}}),
        );
        assert!(entry_for(&routed_as_skip).is_err());
        // A fixed-outcome code ignores refs.outcome.
        let a = entry_for(&event(
            "sources.created",
            json!({"actor":"user","details":{},"refs":{"outcome":"skipped"}}),
        ))
        .unwrap();
        assert_eq!(a.outcome, Outcome::Info);
    }

    #[test]
    fn s3_envelope_copies_source_columns_from_details() {
        let sid = Uuid::now_v7();
        let details = json!({"source_id":sid,"source_name":"TV","key_id":"AbCdEf123456"});
        let a = entry_for(&event(
            "sources.created",
            json!({"actor":"user","details":details}),
        ))
        .unwrap();
        assert_eq!(a.category, Category::Config);
        assert_eq!(a.outcome, Outcome::Info);
        assert_eq!(a.actor, Actor::User);
        assert_eq!(a.source_id, Some(sid));
        assert_eq!(a.source_name.as_deref(), Some("TV"));
        assert_eq!(a.source_key_id.as_deref(), Some("AbCdEf123456"));
        assert_eq!(a.details, details);
        assert_eq!(a.message_id, None);
    }

    #[test]
    fn an_over_long_source_name_is_cut_to_sixty_characters() {
        let long = "á".repeat(61);
        let a = entry_for(&event(
            "sources.renamed",
            json!({"actor":"user","details":{"source_id":Uuid::now_v7(),"source_name":long,
                   "previous_name":"x","key_id":"AbCdEf123456"}}),
        ))
        .unwrap();
        assert_eq!(a.source_name.as_deref(), Some("á".repeat(60).as_str()));
        // details are kept as the producer wrote them
        assert_eq!(
            a.details["source_name"].as_str().unwrap().chars().count(),
            61
        );
    }

    #[test]
    fn refused_security_attempts_are_rejected() {
        for code in [
            "security.sign_in_failed",
            "security.step_up_failed",
            "security.code_rate_limited",
        ] {
            let a = entry_for(&event(code, json!({"actor":"user","details":{}}))).unwrap();
            assert_eq!(
                (a.category, a.outcome),
                (Category::Security, Outcome::Rejected),
                "{code}"
            );
        }
        let a = entry_for(&event(
            "security.code_delivery_failed",
            json!({"actor":"system","details":{}}),
        ))
        .unwrap();
        assert_eq!(a.outcome, Outcome::Info);
    }

    #[test]
    fn s3_codes_all_have_rules() {
        let codes = [
            "security.signed_in",
            "security.sign_in_failed",
            "security.code_rate_limited",
            "security.code_delivery_failed",
            "security.new_device_notice_sent",
            "security.step_up_succeeded",
            "security.step_up_failed",
            "security.channel_change_requested",
            "security.channel_change_on_hold",
            "security.channel_change_completed",
            "security.channel_change_cancelled",
            "security.channel_change_not_completed",
            "security.device_forgotten",
            "security.signed_out_everywhere",
            "security.invitation_issued",
            "security.invitation_revoked",
            "security.account_created",
            "sources.created",
            "sources.renamed",
            "sources.rotated",
            "sources.revoked",
            "connections.created",
            "connections.renamed",
            "connections.pairing_code_issued",
            "connections.paired",
            "connections.repaired",
            "connections.unpaired",
            "connection.status_changed",
        ];
        assert_eq!(codes.len(), 28);
        for code in codes {
            let r = rule_for(code).unwrap_or_else(|| panic!("{code}"));
            assert!(r.safe_to_display, "{code}");
            assert!(Category::of_code(code).is_some(), "{code}");
        }
    }

    #[test]
    fn gateway_codes_have_rules_categories_and_fixed_outcomes() {
        use Outcome::*;
        let codes = [
            ("execution.command_sent", Category::Execution, Info),
            ("execution.order_received", Category::Execution, Info),
            ("execution.order_working", Category::Execution, Info),
            ("execution.filled", Category::Execution, Executed),
            ("execution.partially_filled", Category::Execution, Partial),
            ("execution.cancelled", Category::Execution, Info),
            ("execution.rejected", Category::Execution, Rejected),
            ("execution.expired", Category::Execution, Expired),
            ("execution.state_unknown", Category::Execution, Unknown),
            ("execution.undelivered", Category::Execution, Unknown),
            ("execution.fill_corrected", Category::Execution, Info),
            ("execution.unbound_activity", Category::Execution, Info),
            (
                "connection.account_kind_changed",
                Category::Connection,
                Info,
            ),
            ("sync.flagged", Category::Connection, Info),
            ("sync.reason_resolved", Category::Connection, Info),
            ("sync.restored", Category::Connection, Info),
        ];
        for (code, category, outcome) in codes {
            let r = rule_for(code).unwrap_or_else(|| panic!("{code}"));
            assert_eq!(r.outcome, OutcomeRule::Fixed(outcome), "{code}");
            assert!(!r.safe_to_display, "{code}: fail-closed until E-S2-1");
            assert_eq!(Category::of_code(code), Some(category), "{code}");
        }
    }

    #[test]
    fn accounts_codes_are_config_with_account_columns() {
        let aid = Uuid::now_v7();
        let a = entry_for(&event(
            "accounts.lifecycle_changed",
            json!({"actor":"user","details":{"previous_state":"active","new_state":"paused","had_open_position":false},
                   "refs":{"account_id":aid,"account_name":"Apex 50k"}}),
        ))
        .unwrap();
        assert_eq!(a.category, Category::Config);
        assert_eq!(a.account_id, Some(aid));
        assert_eq!(a.account_name.as_deref(), Some("Apex 50k"));
        assert!(a.safe_to_display);
        assert!(
            !rule_for("accounts.opt_in_suspended")
                .unwrap()
                .safe_to_display
        );
    }

    #[test]
    fn malformed_envelopes_and_unknown_codes_fail_loudly_without_values() {
        let cases = [
            (
                "ingress.received",
                json!({"details":{}}),
                "missing or invalid actor",
            ),
            (
                "ingress.received",
                json!({"actor":"robot","details":{}}),
                "missing or invalid actor",
            ),
            (
                "ingress.received",
                json!({"actor":"system"}),
                "missing details",
            ),
            (
                "ingress.received",
                json!({"actor":"system","details":{},"refs":[]}),
                "refs is not an object",
            ),
            ("ingress.received", json!("x"), "payload is not an object"),
            (
                "other.code",
                json!({"actor":"system","details":{}}),
                "no category",
            ),
            // `sync.*` has a category since 5.3.0; a code without a rule still fails.
            (
                "sync.cleared",
                json!({"actor":"system","details":{}}),
                "no rule",
            ),
            (
                "routing.not_a_code",
                json!({"actor":"system","details":{}}),
                "no rule",
            ),
        ];
        for (code, payload, why) in cases {
            let err = entry_for(&event(code, payload)).unwrap_err().to_string();
            assert_eq!(err, format!("not auditable: {code}: {why}"));
        }
    }
}
