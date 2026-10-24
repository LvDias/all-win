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
    Actor, AuditAction, AuditRecord, ChangeAudit, EndedSessions, IdentityEvent, Notice,
    OperatorOutcome, OperatorRecord, StepUpFailure, Trail,
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
        // The admin's id stays out of the user's trail (HTTP 5.4.0).
        Actor::PlatformAdmin(_) => "platform_admin",
    }
}

/// `kind`, and for an admin recovery `verified_by` and `break_glass` on every entry, plus
/// `verification_ref` on `requested` and `on_hold` (HTTP 5.4.0 `DetailsSecurityChannelChange`).
fn change_details(mut d: Value, kind: &ChangeAudit, with_ref: bool) -> Value {
    d["kind"] = json!(kind.kind().as_str());
    if let ChangeAudit::AdminRecovery {
        verified_by,
        verification_ref,
        break_glass,
    } = kind
    {
        d["verified_by"] = json!(verified_by.as_str());
        d["break_glass"] = json!(break_glass);
        if with_ref {
            d["verification_ref"] = json!(verification_ref.as_str());
        }
    }
    d
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
            kind,
            channel,
            masked_new_value,
        } => change_details(
            json!({ "change_id": change_id.as_uuid(), "channel": channel.as_str(), "masked_new_value": masked_new_value }),
            kind,
            true,
        ),
        AuditAction::ChannelChangeOnHold {
            change_id,
            kind,
            channel,
            hold_until,
        } => change_details(
            json!({ "change_id": change_id.as_uuid(), "channel": channel.as_str(), "hold_until": ts(hold_until) }),
            kind,
            true,
        ),
        AuditAction::ChannelChangeCompleted {
            change_id,
            kind,
            channel,
            masked_old_value,
            masked_new_value,
        } => {
            let mut d = json!({ "change_id": change_id.as_uuid(), "channel": channel.as_str(), "masked_new_value": masked_new_value });
            if let Some(old) = masked_old_value {
                d["masked_old_value"] = json!(old);
            }
            change_details(d, kind, false)
        }
        AuditAction::ChannelChangeCancelled {
            change_id,
            kind,
            channel,
            by_user,
        } => change_details(
            json!({ "change_id": change_id.as_uuid(), "channel": channel.as_str(), "by": if *by_user { "user" } else { "system" } }),
            kind,
            false,
        ),
        AuditAction::ChannelChangeNotCompleted {
            change_id,
            kind,
            channel,
        } => change_details(
            json!({ "change_id": change_id.as_uuid(), "channel": channel.as_str() }),
            kind,
            false,
        ),
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
        Notice::AccountRecovered {
            to: d,
            sources,
            pairings,
        } => {
            json!({ "kind": "account_recovered", "to": to(d), "sources": sources, "pairings": pairings })
        }
        Notice::RecoveredByAdmin {
            to: d,
            channel,
            masked_new_value,
            at,
        } => {
            json!({ "kind": "recovered_by_admin", "to": to(d), "channel": channel.as_str(), "masked_new_value": masked_new_value, "at": ts(at) })
        }
        Notice::BreakGlassStarted {
            to: d,
            masked_account,
            at,
        } => {
            json!({ "kind": "break_glass_started", "to": to(d), "masked_account": masked_account, "at": ts(at) })
        }
        Notice::AdminBreakGlassStarted {
            to: d,
            masked_account,
            at,
        } => {
            json!({ "kind": "admin_break_glass_started", "to": to(d), "masked_account": masked_account, "at": ts(at) })
        }
        Notice::OperatorAlert {
            to: d,
            change_id,
            step,
            outcome,
            at,
        } => {
            json!({ "kind": "operator_alert", "to": to(d), "change_id": change_id, "step": step, "outcome": outcome, "at": ts(at) })
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
        "account_recovered" => Notice::AccountRecovered {
            to: to()?,
            sources: usize::try_from(payload.get("sources")?.as_u64()?).ok()?,
            pairings: usize::try_from(payload.get("pairings")?.as_u64()?).ok()?,
        },
        "break_glass_started" => Notice::BreakGlassStarted {
            to: to()?,
            masked_account: text("masked_account")?,
            at: DateTime::parse_from_rfc3339(&text("at")?)
                .ok()?
                .with_timezone(&Utc),
        },
        "operator_alert" => Notice::OperatorAlert {
            to: to()?,
            change_id: text("change_id"),
            step: text("step")?,
            outcome: text("outcome")?,
            at: DateTime::parse_from_rfc3339(&text("at")?)
                .ok()?
                .with_timezone(&Utc),
        },
        "admin_break_glass_started" => Notice::AdminBreakGlassStarted {
            to: to()?,
            masked_account: text("masked_account")?,
            at: DateTime::parse_from_rfc3339(&text("at")?)
                .ok()?
                .with_timezone(&Utc),
        },
        "recovered_by_admin" => Notice::RecoveredByAdmin {
            to: to()?,
            channel: channel("channel")?,
            masked_new_value: text("masked_new_value")?,
            at: DateTime::parse_from_rfc3339(&text("at")?)
                .ok()?
                .with_timezone(&Utc),
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
        IdentityEvent::OperatorRecovery(r) => Envelope {
            user_id: r.user_id,
            event_type: event.event_type(),
            occurred_at: r.at,
            payload: operator_payload(r),
        },
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

/// `identity.operator_recovery` payload: ids, the step and outcome, and the host-reported
/// session values under `host_reported` (formats validated by `HostReport::parse`). Never a code.
fn operator_payload(r: &OperatorRecord) -> Value {
    let (outcome, reason) = match r.outcome {
        OperatorOutcome::Done => ("done", None),
        OperatorOutcome::Refused(why) => ("refused", Some(why)),
        OperatorOutcome::StepUpFailed => ("step_up_failed", None),
        OperatorOutcome::AttemptsExhausted => ("attempts_exhausted", None),
    };
    json!({
        "admin_user_id": r.admin_user_id.map(|u| u.as_uuid()),
        "change_id": r.change_id.map(|c| c.as_uuid()),
        "step": r.step.as_str(),
        "outcome": outcome,
        "reason": reason,
        "break_glass": r.break_glass,
        "after_cancel": r.after_cancel,
        "host_reported": {
            "login": r.host.login(),
            "ssh_connection": r.host.ssh_connection(),
            "key_fingerprint": r.host.key_fingerprint(),
        },
    })
}

/// The operator log line of an `identity.operator_recovery` row, read back by `app`'s consumer.
#[derive(Debug, PartialEq, Eq)]
pub struct OperatorLogLine {
    pub admin_user_id: Option<String>,
    pub change_id: Option<String>,
    pub step: String,
    pub outcome: String,
    pub reason: Option<String>,
    pub break_glass: bool,
    pub after_cancel: bool,
    pub host_login: Option<String>,
    pub host_ssh_connection: Option<String>,
    pub host_key_fingerprint: Option<String>,
}

pub fn operator_from_payload(payload: &Value) -> Option<OperatorLogLine> {
    let text = |v: &Value, k: &str| v.get(k).and_then(Value::as_str).map(str::to_owned);
    let host = payload.get("host_reported")?;
    Some(OperatorLogLine {
        admin_user_id: text(payload, "admin_user_id"),
        change_id: text(payload, "change_id"),
        step: text(payload, "step")?,
        outcome: text(payload, "outcome")?,
        reason: text(payload, "reason"),
        break_glass: payload.get("break_glass")?.as_bool()?,
        after_cancel: payload.get("after_cancel")?.as_bool()?,
        host_login: text(host, "login"),
        host_ssh_connection: text(host, "ssh_connection"),
        host_key_fingerprint: text(host, "key_fingerprint"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use identity_application::changes::Notice;
    use identity_enterprise::email::EmailAddress;

    #[test]
    fn operator_events_carry_marked_host_values_and_read_back() {
        use identity_application::changes::{HostReport, OperatorStep};
        let user = shared_kernel::UserId::new_v7();
        let r = OperatorRecord {
            user_id: user,
            admin_user_id: Some(user),
            change_id: None,
            step: OperatorStep::Resolve,
            outcome: OperatorOutcome::Refused("other_admin_available"),
            break_glass: true,
            after_cancel: false,
            host: HostReport::parse(
                Some("ops"),
                Some("203.0.113.5 51234 10.0.0.2 22"),
                Some("bad"),
            ),
            at: Utc::now(),
        };
        let e = event(&IdentityEvent::OperatorRecovery(r));
        assert_eq!(e.event_type, "identity.operator_recovery");
        assert_eq!(e.user_id, user);
        let line = operator_from_payload(&e.payload).unwrap();
        assert_eq!(line.outcome, "refused");
        assert_eq!(line.reason.as_deref(), Some("other_admin_available"));
        assert!(line.break_glass);
        assert_eq!(line.host_login.as_deref(), Some("ops"));
        assert_eq!(line.host_key_fingerprint, None, "invalid format dropped");
        assert!(e.payload["host_reported"].is_object());
    }

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
        let n = Notice::AccountRecovered {
            to: Destination::Email(EmailAddress::parse("n@example.com").unwrap()),
            sources: 2,
            pairings: 1,
        };
        let p = notice_payload(Locale::PtBr, &n);
        assert_eq!(notice_from_payload(&p), Some((Locale::PtBr, n)));
        let n = Notice::RecoveredByAdmin {
            to: Destination::Email(EmailAddress::parse("a@example.com").unwrap()),
            channel: Channel::Email,
            masked_new_value: "n***@e***.com".into(),
            at: DateTime::parse_from_rfc3339("2026-10-26T09:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        };
        let p = notice_payload(Locale::En, &n);
        assert_eq!(p.get("sources"), None, "no count to the old channel");
        assert_eq!(notice_from_payload(&p), Some((Locale::En, n)));
        let at = DateTime::parse_from_rfc3339("2026-10-26T09:00:00.000Z")
            .unwrap()
            .with_timezone(&Utc);
        for n in [
            Notice::BreakGlassStarted {
                to: Destination::Email(EmailAddress::parse("a@example.com").unwrap()),
                masked_account: "a***@e***.com".into(),
                at,
            },
            Notice::AdminBreakGlassStarted {
                to: Destination::Email(EmailAddress::parse("b@example.com").unwrap()),
                masked_account: "a***@e***.com".into(),
                at,
            },
        ] {
            let p = notice_payload(Locale::PtBr, &n);
            assert_eq!(notice_from_payload(&p), Some((Locale::PtBr, n)));
        }
    }
}
