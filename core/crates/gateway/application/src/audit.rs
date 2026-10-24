//! Audit details for the codes the gateway produces (HTTP 5.3.0 `AuditDetailsByCode`). Each
//! builder emits only the fields of its closed schema; a producer test validates them.

use chrono::{DateTime, Utc};
use gateway_enterprise::command::CommandKind;
use gateway_enterprise::ids::AccountId;
use gateway_enterprise::report::{ChildRole, FillOperation, ReportFact, ReportStatus};
use gateway_enterprise::sync::{ReasonEntry, SyncChange};
use serde_json::{Map, Value, json};

use crate::events::{self, GatewayEvent};
use crate::wire::{ExecutionReport, timestamp};

/// `refs` columns of an audit entry.
pub fn refs(
    account_id: impl serde::Serialize,
    account_name: Option<&str>,
    instruction_id: Option<uuid::Uuid>,
) -> Value {
    let mut r = Map::new();
    r.insert("account_id".into(), json!(account_id));
    if let Some(n) = account_name {
        r.insert("account_name".into(), json!(n));
    }
    if let Some(i) = instruction_id {
        r.insert("instruction_id".into(), json!(i));
    }
    Value::Object(r)
}

fn put(m: &mut Map<String, Value>, k: &str, v: Option<Value>) {
    if let Some(v) = v {
        m.insert(k.into(), v);
    }
}

/// `execution.command_sent` (`DetailsCommandSent`).
pub fn command_sent(
    kind: CommandKind,
    step: &str,
    expires_at: Option<DateTime<Utc>>,
    at: DateTime<Utc>,
    refs: Value,
) -> GatewayEvent {
    let mut d = Map::new();
    d.insert("step".into(), json!(step));
    d.insert("kind".into(), json!(kind));
    put(
        &mut d,
        "expires_at",
        expires_at.map(|e| json!(timestamp(e))),
    );
    events::audit("execution.command_sent", at, Value::Object(d), refs)
}

/// The `execution.*` code of one report; `None` for `duplicate` (a non-transition).
pub fn execution_code(fact: &ReportFact) -> Option<&'static str> {
    if fact
        .fill
        .as_ref()
        .is_some_and(|f| f.operation != FillOperation::Add)
    {
        return Some("execution.fill_corrected");
    }
    Some(match fact.status {
        ReportStatus::Received => "execution.order_received",
        ReportStatus::Working => "execution.order_working",
        ReportStatus::PartiallyFilled => "execution.partially_filled",
        ReportStatus::Filled => "execution.filled",
        ReportStatus::Cancelled => "execution.cancelled",
        ReportStatus::Rejected => "execution.rejected",
        ReportStatus::Expired => "execution.expired",
        ReportStatus::Unknown => "execution.state_unknown",
        ReportStatus::Duplicate => return None,
    })
}

/// `DetailsExecution` for one report. `child_role` is limited to the schema's entry/stop/target
/// (exit/flatten are open with the Architect).
pub fn execution_details(
    r: &ExecutionReport,
    fact: &ReportFact,
    effective_commission: Option<String>,
) -> Value {
    let mut d = Map::new();
    d.insert("origin".into(), json!(r.origin));
    put(
        &mut d,
        "child_role",
        fact.child_role
            .filter(|c| matches!(c, ChildRole::Entry | ChildRole::Stop | ChildRole::Target))
            .map(|c| json!(c)),
    );
    put(
        &mut d,
        "platform_order_id",
        r.platform_order_id.as_ref().map(|v| json!(v)),
    );
    put(&mut d, "quantity", r.quantity.map(|v| json!(v)));
    if let Some(f) = &r.fill {
        d.insert("fill_qty".into(), json!(f.quantity));
        d.insert("fill_price".into(), json!(f.price));
        d.insert(
            "fill_operation".into(),
            json!(f.operation.as_deref().unwrap_or("add")),
        );
        put(&mut d, "commission", effective_commission.map(|c| json!(c)));
    }
    put(
        &mut d,
        "position_after",
        fact.position_after.map(|v| json!(v)),
    );
    put(
        &mut d,
        "avg_price_after",
        r.position_after
            .as_ref()
            .and_then(|p| p.average_price.as_ref())
            .map(|v| json!(v)),
    );
    put(
        &mut d,
        "platform_reason",
        r.reason_code.as_ref().map(|v| json!(v)),
    );
    put(
        &mut d,
        "platform_time",
        r.platform_time.map(|t| json!(timestamp(t))),
    );
    Value::Object(d)
}

/// HTTP `SyncReason` view of an entry.
fn reason(r: &ReasonEntry) -> Value {
    let mut v = serde_json::to_value(&r.details).unwrap_or_else(|_| json!({}));
    if let Value::Object(m) = &mut v {
        m.insert("code".into(), json!(r.code.as_str()));
        m.insert("detected_at".into(), json!(timestamp(r.detected_at)));
        if let Some(at) = r.resolved_at {
            m.insert("resolved_at".into(), json!(timestamp(at)));
        }
    }
    v
}

/// `sync.flagged` per added reason and `sync.reason_resolved` per resolved one (`DetailsSync`).
/// Per added reason: the audit entry `sync.flagged` and, for the bracket reasons and a flatten
/// that lost protection, in the same transaction, a notification event typed with the reason's
/// own registry code (S18 from_now). Per resolved reason: `sync.reason_resolved` (audit only).
pub fn sync_entries(change: &SyncChange, at: DateTime<Utc>, refs: &Value) -> Vec<GatewayEvent> {
    let account = refs
        .get("account_id")
        .and_then(Value::as_str)
        .and_then(|s| uuid::Uuid::parse_str(s).ok())
        .map(AccountId::from_uuid);
    let mut out = Vec::new();
    for r in &change.added {
        let details = json!({ "sync_version": change.version, "reason": reason(r) });
        let audit = events::audit("sync.flagged", at, details.clone(), refs.clone());
        // A reason notified as `sync.flagged` has no separate event: the audit entry of the same
        // type is the notification source (S18 reads it; Brasa/Esquadro option (a)). Two
        // payload shapes under one event type would break the audit consumer.
        let own_code = r.code.notification_code() != "sync.flagged";
        let note = account.filter(|_| own_code).map(|a| {
            events::notification(
                r.code.notification_code(),
                at,
                a,
                json!({ "kind": "account", "id": a }),
                audit.id,
                details,
            )
        });
        out.push(audit);
        out.extend(note);
    }
    for r in &change.resolved {
        out.push(events::audit(
            "sync.reason_resolved",
            at,
            json!({ "sync_version": change.version, "reason": reason(r) }),
            refs.clone(),
        ));
    }
    out
}

/// `sync.restored` (ADR-0004 §6 row 3): the audit entry, which is also the S18 notification
/// source (no separate event of the same type).
pub fn restored(
    sync_version: i64,
    connection: impl serde::Serialize,
    at: DateTime<Utc>,
    refs: Value,
) -> Vec<GatewayEvent> {
    let details = json!({ "sync_version": sync_version, "connection_id": connection });
    vec![events::audit("sync.restored", at, details, refs)]
}
