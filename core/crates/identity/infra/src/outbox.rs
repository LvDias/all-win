//! Outbox envelopes for identity's audit records and integration events (ALIGN-2).
//!
//! `event_type` is the stable code. Audit codes start with `security.`, `config.` or
//! `connection.`, so the audit module records them in the affected user's log; integration
//! events start with `identity.` and are not audited. Payloads carry ids and public `key_id`s
//! only (the outbox CHECK also rejects anything shaped like a token).

use chrono::{DateTime, Utc};
use identity_application::changes::{
    Actor, AuditAction, AuditRecord, EndedSessions, IdentityEvent, Trail,
};
use serde_json::{Map, Value, json};
use shared_kernel::UserId;

pub struct Envelope {
    pub user_id: UserId,
    pub event_type: &'static str,
    pub occurred_at: DateTime<Utc>,
    pub payload: Value,
}

fn actor_fields(actor: Actor, out: &mut Map<String, Value>) {
    let (kind, id) = match actor {
        Actor::User(id) => ("user", Some(id.to_string())),
        Actor::Admin(id) => ("admin", Some(id.to_string())),
        Actor::System => ("system", None),
        Actor::Adapter(c) => ("adapter", Some(c.as_uuid().to_string())),
        Actor::TokenHolder => ("token_holder", None),
    };
    out.insert("actor".into(), json!(kind));
    if let Some(id) = id {
        out.insert("actor_id".into(), json!(id));
    }
}

pub fn audit(record: &AuditRecord) -> Envelope {
    let fields = match &record.action {
        AuditAction::AccountCreated { invitation_id } => {
            json!({ "invitation_id": invitation_id.as_uuid() })
        }
        AuditAction::PasswordChanged => json!({}),
        AuditAction::PasswordResetCompleted => json!({}),
        AuditAction::PasswordResetLinkIssued { target } => json!({ "target_user_id": target }),
        AuditAction::TotpEnabled => json!({}),
        AuditAction::TotpDisabled => json!({}),
        AuditAction::TotpResetByAdmin { target } => json!({ "target_user_id": target }),
        AuditAction::RecoveryCodesRegenerated => json!({}),
        AuditAction::RecoveryCodeUsed { remaining } => json!({ "remaining": remaining }),
        AuditAction::InvitationIssued { invitation_id } => {
            json!({ "invitation_id": invitation_id.as_uuid() })
        }
        AuditAction::InvitationRevoked { invitation_id } => {
            json!({ "invitation_id": invitation_id.as_uuid() })
        }
        AuditAction::SourceCreated { source_id, key_id } => {
            json!({ "source_id": source_id.as_uuid(), "key_id": key_id.as_str() })
        }
        AuditAction::SourceRenamed { source_id } => json!({ "source_id": source_id.as_uuid() }),
        AuditAction::SourceRotated {
            source_id,
            old_key_id,
            new_key_id,
        } => {
            json!({ "source_id": source_id.as_uuid(), "key_id": new_key_id.as_str(), "previous_key_id": old_key_id.as_str() })
        }
        AuditAction::SourceRevoked { source_id, key_id } => {
            json!({ "source_id": source_id.as_uuid(), "key_id": key_id.as_str() })
        }
        AuditAction::ConnectionCreated { connection_id } => {
            json!({ "connection_id": connection_id.as_uuid() })
        }
        AuditAction::ConnectionRenamed { connection_id } => {
            json!({ "connection_id": connection_id.as_uuid() })
        }
        AuditAction::PairingCodeIssued {
            connection_id,
            repairing,
        } => json!({ "connection_id": connection_id.as_uuid(), "repairing": repairing }),
        AuditAction::ConnectionPaired {
            connection_id,
            instance_id,
            device_key_id,
            replaced_previous_instance,
        } => json!({
            "connection_id": connection_id.as_uuid(),
            "instance_id": instance_id,
            "device_key_id": device_key_id.as_str(),
            "replaced_previous_instance": replaced_previous_instance,
        }),
        AuditAction::ConnectionUnpaired {
            connection_id,
            open_positions_acknowledged,
        } => {
            json!({ "connection_id": connection_id.as_uuid(), "open_positions_acknowledged": open_positions_acknowledged })
        }
        AuditAction::ConnectionDisconnectedByRestart { connection_id } => {
            json!({ "connection_id": connection_id.as_uuid() })
        }
    };
    let mut payload = fields.as_object().cloned().unwrap_or_default();
    actor_fields(record.actor, &mut payload);
    if let Some(ip) = record.ip {
        payload.insert("ip".into(), json!(ip.to_string()));
    }
    let user_id = match record.trail {
        Trail::User(user) => user,
        Trail::Admin(admin) => {
            payload.insert("trail".into(), json!("admin"));
            admin
        }
    };
    Envelope {
        user_id,
        event_type: record.action.event_type(),
        occurred_at: record.at,
        payload: Value::Object(payload),
    }
}

pub fn event(event: &IdentityEvent) -> Envelope {
    match event {
        IdentityEvent::SessionsEnded {
            user_id,
            sessions,
            reason,
            at,
        } => {
            let scope = match sessions {
                EndedSessions::One(id) => json!({ "session_id": id.as_uuid() }),
                EndedSessions::AllExcept(except) => {
                    json!({ "all_except": except.iter().map(|s| s.as_uuid()).collect::<Vec<_>>() })
                }
            };
            let mut payload = scope.as_object().cloned().unwrap_or_default();
            payload.insert("reason".into(), json!(reason.as_str()));
            Envelope {
                user_id: *user_id,
                event_type: event.event_type(),
                occurred_at: *at,
                payload: Value::Object(payload),
            }
        }
        IdentityEvent::ConnectionPaired {
            user_id,
            connection_id,
            instance_name,
            at,
        } => Envelope {
            user_id: *user_id,
            event_type: event.event_type(),
            occurred_at: *at,
            payload: json!({ "connection_id": connection_id.as_uuid(), "instance_name": instance_name }),
        },
        IdentityEvent::ConnectionStatusChanged {
            user_id,
            connection_id,
            status,
            at,
        } => Envelope {
            user_id: *user_id,
            event_type: event.event_type(),
            occurred_at: *at,
            payload: json!({ "connection_id": connection_id.as_uuid(), "status": status.as_str() }),
        },
        IdentityEvent::PairingCodeSuperseded {
            user_id,
            connection_id,
            at,
        } => Envelope {
            user_id: *user_id,
            event_type: event.event_type(),
            occurred_at: *at,
            payload: json!({ "connection_id": connection_id.as_uuid(), "reason": "superseded" }),
        },
    }
}
