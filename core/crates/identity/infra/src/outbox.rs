//! Outbox envelopes for identity's audit records and integration events (ALIGN-2).
//!
//! `event_type` is the stable code. Audit codes are the contract `AuditCode`s (`security.*`,
//! `sources.*`, `connections.*`, `connection.status_changed`), recorded by the audit module in the
//! affected user's log; integration events start with `identity.` and are not audited. Payloads
//! carry ids, labels, masked values and public `key_id`s only (the outbox CHECK also rejects
//! anything shaped like a token). The notice payload holds the recipient's own address, which
//! the notice consumer needs; it never holds a code.

use chrono::{DateTime, Utc};
use identity_application::changes::{
    Actor, AuditAction, AuditRecord, EndedSessions, IdentityEvent, Notice, StepUpFailure, Trail,
};
use identity_enterprise::channel::{Channel, Destination};
use identity_enterprise::profile::Locale;
use serde_json::{Value, json};
use shared_kernel::UserId;

pub struct Envelope {
    pub user_id: UserId,
    pub event_type: &'static str,
    pub occurred_at: DateTime<Utc>,
    pub payload: Value,
}

/// `actor` (rev. 12, contract `AuditEntry.actor`): user, system or platform_admin. It is an
/// envelope field next to the `details`, never part of them.
fn actor(actor: Actor) -> &'static str {
    match actor {
        Actor::User(_) | Actor::Admin(_) | Actor::TokenHolder => "user",
        Actor::System | Actor::Adapter(_) => "system",
    }
}

fn ts(at: &DateTime<Utc>) -> String {
    at.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

/// The contract `details` of each audit code (modules/audit.yaml `AuditDetailsByCode`).
fn details(action: &AuditAction) -> Value {
    match action {
        AuditAction::SignedIn {
            channel,
            new_device,
            device_label,
        } => {
            json!({ "channel": channel.as_str(), "new_device": new_device, "device_label": device_label })
        }
        AuditAction::SignInFailed { channel } => json!({ "channel": channel.as_str() }),
        AuditAction::CodeRateLimited { purpose } => json!({ "purpose": purpose.as_str() }),
        AuditAction::CodeDeliveryFailed { channel, purpose } => {
            json!({ "channel": channel.as_str(), "purpose": purpose.as_str() })
        }
        AuditAction::NewDeviceNoticeSent { device_label } => {
            json!({ "device_label": device_label })
        }
        AuditAction::StepUpSucceeded { action, target_id } => {
            let mut d = json!({ "action": action.as_str() });
            if let Some(t) = target_id {
                d["target_id"] = json!(t);
            }
            d
        }
        AuditAction::StepUpFailed {
            action,
            target_id,
            reason,
        } => {
            let mut d = json!({
                "action": action.as_str(),
                "reason": match reason {
                    StepUpFailure::Missing => "missing",
                    StepUpFailure::CodeInvalid => "code_invalid",
                },
            });
            if let Some(t) = target_id {
                d["target_id"] = json!(t);
            }
            d
        }
        AuditAction::ChannelChangeRequested {
            change_id,
            channel,
            masked_new_value,
        } => {
            json!({ "change_id": change_id.as_uuid(), "channel": channel.as_str(), "masked_new_value": masked_new_value })
        }
        AuditAction::ChannelChangeOnHold {
            change_id,
            channel,
            hold_until,
        } => {
            json!({ "change_id": change_id.as_uuid(), "channel": channel.as_str(), "hold_until": ts(hold_until) })
        }
        AuditAction::ChannelChangeCompleted {
            change_id,
            channel,
            masked_old_value,
            masked_new_value,
        } => {
            let mut d = json!({ "change_id": change_id.as_uuid(), "channel": channel.as_str(), "masked_new_value": masked_new_value });
            if let Some(old) = masked_old_value {
                d["masked_old_value"] = json!(old);
            }
            d
        }
        AuditAction::ChannelChangeCancelled {
            change_id,
            channel,
            by_user,
        } => {
            json!({ "change_id": change_id.as_uuid(), "channel": channel.as_str(), "by": if *by_user { "user" } else { "system" } })
        }
        AuditAction::ChannelChangeNotCompleted { change_id, channel } => {
            json!({ "change_id": change_id.as_uuid(), "channel": channel.as_str() })
        }
        AuditAction::DeviceForgotten {
            device_id,
            device_label,
        } => json!({ "device_id": device_id.as_uuid(), "device_label": device_label }),
        AuditAction::SignedOutEverywhere => json!({}),
        AuditAction::InvitationIssued {
            invitation_id,
            masked_email,
            email_sent,
        } => {
            json!({ "invitation_id": invitation_id.as_uuid(), "masked_email": masked_email, "email_sent": email_sent })
        }
        AuditAction::InvitationRevoked { invitation_id }
        | AuditAction::AccountCreated { invitation_id } => {
            json!({ "invitation_id": invitation_id.as_uuid() })
        }
        AuditAction::SourceCreated {
            source_id,
            source_name,
            key_id,
        }
        | AuditAction::SourceRevoked {
            source_id,
            source_name,
            key_id,
        } => {
            json!({ "source_id": source_id.as_uuid(), "source_name": source_name, "key_id": key_id.as_str() })
        }
        AuditAction::SourceRenamed {
            source_id,
            source_name,
            previous_name,
            key_id,
        } => {
            json!({ "source_id": source_id.as_uuid(), "source_name": source_name, "previous_name": previous_name, "key_id": key_id.as_str() })
        }
        AuditAction::SourceRotated {
            source_id,
            source_name,
            key_id,
            previous_key_id,
        } => {
            json!({ "source_id": source_id.as_uuid(), "source_name": source_name, "key_id": key_id.as_str(), "previous_key_id": previous_key_id.as_str() })
        }
        AuditAction::ConnectionCreated {
            connection_id,
            connection_name,
        } => {
            json!({ "connection_id": connection_id.as_uuid(), "connection_name": connection_name })
        }
        AuditAction::ConnectionRenamed {
            connection_id,
            connection_name,
            previous_name,
        } => {
            json!({ "connection_id": connection_id.as_uuid(), "connection_name": connection_name, "previous_name": previous_name })
        }
        AuditAction::PairingCodeIssued {
            connection_id,
            connection_name,
            code_expires_at,
        } => {
            json!({ "connection_id": connection_id.as_uuid(), "connection_name": connection_name, "code_expires_at": ts(code_expires_at) })
        }
        AuditAction::ConnectionPaired {
            connection_id,
            connection_name,
            instance_name,
            adapter_version,
            ..
        } => json!({
            "connection_id": connection_id.as_uuid(),
            "connection_name": connection_name,
            "instance_name": instance_name.chars().take(64).collect::<String>(),
            "adapter_version": adapter_version.chars().take(32).collect::<String>(),
        }),
        AuditAction::ConnectionUnpaired {
            connection_id,
            connection_name,
            open_positions_acknowledged,
        } => {
            json!({ "connection_id": connection_id.as_uuid(), "connection_name": connection_name, "open_positions_acknowledged": open_positions_acknowledged })
        }
        AuditAction::ConnectionStatusChanged {
            connection_id,
            connection_name,
            previous_status,
            status,
        } => {
            json!({ "connection_id": connection_id.as_uuid(), "connection_name": connection_name, "previous_status": previous_status.as_str(), "status": status.as_str() })
        }
    }
}

/// Audit envelope: `{ "actor": …, "details": {…} }`. The audit module stores `details` as the
/// entry's details and `actor` in its column; nothing else (no IP, no token) travels.
pub fn audit(record: &AuditRecord) -> Envelope {
    let user_id = match record.trail {
        Trail::User(user) | Trail::Admin(user) => user,
    };
    Envelope {
        user_id,
        event_type: record.action.event_type(),
        occurred_at: record.at,
        payload: json!({ "actor": actor(record.actor), "details": details(&record.action) }),
    }
}

fn notice_payload(locale: Locale, notice: &Notice) -> Value {
    let to = |d: &Destination| json!({ "channel": d.channel().as_str(), "value": d.normalized() });
    let body = match notice {
        Notice::NewDevice {
            to: d,
            device_label,
            ip,
            channel,
        } => {
            json!({ "kind": "new_device", "to": to(d), "device_label": device_label, "ip": ip.map(|i| i.to_string()), "channel": channel.as_str() })
        }
        Notice::ChannelChanged {
            to: d,
            channel,
            masked_new_value,
        } => {
            json!({ "kind": "channel_changed", "to": to(d), "channel": channel.as_str(), "masked_new_value": masked_new_value })
        }
        Notice::ChannelChangeOnHold {
            to: d,
            channel,
            masked_new_value,
            hold_until,
        } => {
            json!({ "kind": "channel_change_on_hold", "to": to(d), "channel": channel.as_str(), "masked_new_value": masked_new_value, "hold_until": ts(hold_until) })
        }
        Notice::ChannelChangeCancelled { to: d, channel } => {
            json!({ "kind": "channel_change_cancelled", "to": to(d), "channel": channel.as_str() })
        }
    };
    let mut body = body;
    body["locale"] = json!(locale.as_str());
    body
}

/// Rebuilds a notice from its outbox payload (the notice consumer in `app`).
pub fn notice_from_payload(payload: &Value) -> Option<(Locale, Notice)> {
    let locale = Locale::parse(payload.get("locale")?.as_str()?)?;
    let channel = |key: &str| Channel::parse(payload.get(key)?.as_str()?);
    let text = |key: &str| payload.get(key).and_then(Value::as_str).map(str::to_owned);
    let to = || {
        let t = payload.get("to")?;
        let c = Channel::parse(t.get("channel")?.as_str()?)?;
        Some(Destination::from_stored(
            c,
            t.get("value")?.as_str()?.to_owned(),
        ))
    };
    let notice = match payload.get("kind")?.as_str()? {
        "new_device" => Notice::NewDevice {
            to: to()?,
            device_label: text("device_label")?,
            ip: text("ip").and_then(|i| i.parse().ok()),
            channel: channel("channel")?,
        },
        "channel_changed" => Notice::ChannelChanged {
            to: to()?,
            channel: channel("channel")?,
            masked_new_value: text("masked_new_value")?,
        },
        "channel_change_on_hold" => Notice::ChannelChangeOnHold {
            to: to()?,
            channel: channel("channel")?,
            masked_new_value: text("masked_new_value")?,
            hold_until: DateTime::parse_from_rfc3339(&text("hold_until")?)
                .ok()?
                .with_timezone(&Utc),
        },
        "channel_change_cancelled" => Notice::ChannelChangeCancelled {
            to: to()?,
            channel: channel("channel")?,
        },
        _ => return None,
    };
    Some((locale, notice))
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
                EndedSessions::Device(device) => json!({ "device_id": device.as_uuid() }),
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
        IdentityEvent::NoticeRequested {
            user_id,
            locale,
            notice,
            at,
        } => Envelope {
            user_id: *user_id,
            event_type: event.event_type(),
            occurred_at: *at,
            payload: notice_payload(*locale, notice),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use identity_application::changes::Notice;
    use identity_enterprise::email::EmailAddress;

    #[test]
    fn notices_round_trip_through_the_outbox() {
        let n = Notice::ChannelChangeOnHold {
            to: Destination::Email(EmailAddress::parse("a@example.com").unwrap()),
            channel: Channel::Email,
            masked_new_value: "n***@e***.com".into(),
            hold_until: DateTime::parse_from_rfc3339("2026-10-26T09:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        };
        let p = notice_payload(Locale::En, &n);
        assert_eq!(notice_from_payload(&p), Some((Locale::En, n)));
        let n = Notice::NewDevice {
            to: Destination::Email(EmailAddress::parse("a@example.com").unwrap()),
            device_label: "Firefox on Linux".into(),
            ip: Some("203.0.113.7".parse().unwrap()),
            channel: Channel::Email,
        };
        let p = notice_payload(Locale::PtBr, &n);
        assert_eq!(notice_from_payload(&p), Some((Locale::PtBr, n)));
    }
}
