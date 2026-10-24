//! Every audit code identity emits serializes `details` that match the contract
//! (`AuditDetailsByCode[code]` in `contracts/http/openapi.bundled.yaml`; codes without an entry
//! have `{}`). The `Details*` schemas are closed, so a new field fails here instead of leaking
//! (Esquadro, condition of `safe_to_display = true`).

#![allow(clippy::unwrap_used, clippy::panic)]

use std::collections::BTreeSet;
use std::path::Path;

use chrono::{TimeZone, Utc};
use identity_application::changes::{
    AUDIT_EVENT_TYPES, Actor, AuditAction, AuditRecord, ChangeAudit, StepUpFailure, Trail,
};
use identity_enterprise::channel::{Channel, PhoneNumber, mask_email};
use identity_enterprise::channel_change::{VerificationRef, VerifiedBy};
use identity_enterprise::connection::ConnectionStatus;
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::{ChannelChangeId, ConnectionId, DeviceId, InvitationId, SourceId};
use identity_enterprise::otp::{Purpose, StepUpAction};
use identity_enterprise::tokens::KeyId;
use identity_infra::outbox;
use serde_json::{Value, json};
use shared_kernel::UserId;
use uuid::Uuid;

fn contract() -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../contracts/http/openapi.bundled.yaml");
    yaml_serde::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

/// One sample per variant; optional fields appear both set and unset.
fn samples() -> Vec<AuditAction> {
    let at = Utc.with_ymd_and_hms(2026, 10, 23, 9, 0, 0).unwrap();
    let change = ChannelChangeId::new_v7();
    let source = SourceId::new_v7();
    let connection = ConnectionId::new_v7();
    let invitation = InvitationId::new_v7();
    let key = KeyId::parse("AbCdEf123456").unwrap();
    let previous_key = KeyId::parse("ZyXwVu987654").unwrap();
    let email = |raw: &str| mask_email(&EmailAddress::parse(raw).unwrap());
    let phone = PhoneNumber::parse("+5511987651234").unwrap().masked();
    let recovery = |break_glass| ChangeAudit::AdminRecovery {
        verified_by: VerifiedBy::VideoCall,
        verification_ref: VerificationRef::parse("ticket 4521").unwrap(),
        break_glass,
    };
    vec![
        AuditAction::SignedIn {
            channel: Channel::Email,
            new_device: true,
            device_label: "Firefox on Linux".into(),
        },
        AuditAction::SignInFailed {
            channel: Channel::Whatsapp,
        },
        AuditAction::CodeRateLimited {
            purpose: Purpose::Login,
        },
        AuditAction::CodeDeliveryFailed {
            channel: Channel::Email,
            purpose: Purpose::StepUp,
        },
        AuditAction::NewDeviceNoticeSent {
            device_label: "Safari on macOS".into(),
        },
        AuditAction::StepUpSucceeded {
            action: StepUpAction::RouteEnable,
            target_id: Some(Uuid::now_v7()),
        },
        AuditAction::StepUpSucceeded {
            action: StepUpAction::SourceCreate,
            target_id: None,
        },
        AuditAction::StepUpFailed {
            action: StepUpAction::ConnectionUnpair,
            target_id: Some(Uuid::now_v7()),
            reason: StepUpFailure::CodeInvalid,
        },
        AuditAction::StepUpFailed {
            action: StepUpAction::SourceCreate,
            target_id: None,
            reason: StepUpFailure::Missing,
        },
        AuditAction::ChannelChangeRequested {
            change_id: change,
            kind: ChangeAudit::UserChange,
            channel: Channel::Email,
            masked_new_value: email("ana@example.com"),
        },
        AuditAction::ChannelChangeOnHold {
            change_id: change,
            kind: ChangeAudit::UserChange,
            channel: Channel::Email,
            hold_until: at,
        },
        AuditAction::ChannelChangeCompleted {
            change_id: change,
            kind: ChangeAudit::UserChange,
            channel: Channel::Email,
            masked_old_value: Some(email("bia@example.com")),
            masked_new_value: email("ana@example.com"),
        },
        AuditAction::ChannelChangeCompleted {
            change_id: change,
            kind: ChangeAudit::UserChange,
            channel: Channel::Whatsapp,
            masked_old_value: None,
            masked_new_value: phone,
        },
        AuditAction::ChannelChangeCancelled {
            change_id: change,
            kind: ChangeAudit::UserChange,
            channel: Channel::Email,
            by_user: true,
        },
        AuditAction::ChannelChangeCancelled {
            change_id: change,
            kind: ChangeAudit::UserChange,
            channel: Channel::Email,
            by_user: false,
        },
        AuditAction::ChannelChangeNotCompleted {
            change_id: change,
            kind: ChangeAudit::UserChange,
            channel: Channel::Whatsapp,
        },
        // Admin recovery (AC-12, HTTP 5.4.0): every code with the recovery fields.
        AuditAction::ChannelChangeRequested {
            change_id: change,
            kind: recovery(false),
            channel: Channel::Email,
            masked_new_value: email("ana@example.com"),
        },
        AuditAction::ChannelChangeOnHold {
            change_id: change,
            kind: recovery(true),
            channel: Channel::Email,
            hold_until: at,
        },
        AuditAction::ChannelChangeCompleted {
            change_id: change,
            kind: recovery(false),
            channel: Channel::Email,
            masked_old_value: Some(email("bia@example.com")),
            masked_new_value: email("ana@example.com"),
        },
        AuditAction::ChannelChangeCancelled {
            change_id: change,
            kind: recovery(false),
            channel: Channel::Email,
            by_user: false,
        },
        AuditAction::ChannelChangeNotCompleted {
            change_id: change,
            kind: recovery(true),
            channel: Channel::Email,
        },
        AuditAction::StepUpSucceeded {
            action: StepUpAction::AdminRecovery,
            target_id: Some(Uuid::now_v7()),
        },
        AuditAction::DeviceForgotten {
            device_id: DeviceId::new_v7(),
            device_label: "Chrome on Windows".into(),
        },
        AuditAction::SignedOutEverywhere,
        AuditAction::InvitationIssued {
            invitation_id: invitation,
            masked_email: email("caio@example.com"),
            email_sent: false,
        },
        AuditAction::InvitationRevoked {
            invitation_id: invitation,
        },
        AuditAction::AccountCreated {
            invitation_id: invitation,
        },
        AuditAction::SourceCreated {
            source_id: source,
            source_name: "Main".into(),
            key_id: key.clone(),
        },
        AuditAction::SourceRenamed {
            source_id: source,
            source_name: "Main 2".into(),
            previous_name: "Main".into(),
            key_id: key.clone(),
        },
        AuditAction::SourceRotated {
            source_id: source,
            source_name: "Main".into(),
            key_id: key.clone(),
            previous_key_id: previous_key,
        },
        AuditAction::SourceRevoked {
            source_id: source,
            source_name: "Main".into(),
            key_id: key,
        },
        AuditAction::ConnectionCreated {
            connection_id: connection,
            connection_name: "Desk".into(),
        },
        AuditAction::ConnectionRenamed {
            connection_id: connection,
            connection_name: "Desk 2".into(),
            previous_name: "Desk".into(),
        },
        AuditAction::PairingCodeIssued {
            connection_id: connection,
            connection_name: "Desk".into(),
            code_expires_at: at,
        },
        AuditAction::ConnectionPaired {
            connection_id: connection,
            connection_name: "Desk".into(),
            instance_name: "TRADING-PC".into(),
            adapter_version: "1.0.0".into(),
            repaired: false,
        },
        AuditAction::ConnectionPaired {
            connection_id: connection,
            connection_name: "Desk".into(),
            instance_name: "TRADING-PC".into(),
            adapter_version: "1.0.1".into(),
            repaired: true,
        },
        AuditAction::ConnectionUnpaired {
            connection_id: connection,
            connection_name: "Desk".into(),
            open_positions_acknowledged: true,
        },
        AuditAction::ConnectionStatusChanged {
            connection_id: connection,
            connection_name: "Desk".into(),
            previous_status: ConnectionStatus::Connected,
            status: ConnectionStatus::Disconnected,
        },
    ]
}

#[test]
fn every_audit_code_serializes_contract_details() {
    let doc = contract();
    let components = doc.pointer("/components").unwrap().clone();
    let by_code = doc
        .pointer("/components/schemas/AuditDetailsByCode/properties")
        .and_then(Value::as_object)
        .unwrap();
    let known: BTreeSet<&str> = doc
        .pointer("/components/schemas/KnownAuditCode/enum")
        .and_then(Value::as_array)
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .collect();
    let user = UserId::new_v7();
    let mut covered = BTreeSet::new();
    let mut failures = Vec::new();
    for action in samples() {
        let code = action.event_type();
        covered.insert(code);
        if !known.contains(code) {
            failures.push(format!("{code}: not a contract KnownAuditCode"));
            continue;
        }
        let envelope = outbox::audit(&AuditRecord {
            trail: Trail::User(user),
            actor: Actor::User(user),
            action,
            at: Utc::now(),
            ip: None,
        });
        assert_eq!(envelope.event_type, code);
        let keys: BTreeSet<&str> = envelope
            .payload
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, BTreeSet::from(["actor", "details"]), "{code}");
        let details = &envelope.payload["details"];
        if !by_code.contains_key(code) {
            if details != &json!({}) {
                failures.push(format!("{code}: {details} without a details schema"));
            }
            continue;
        }
        let schema = json!({
            "$ref": format!("#/components/schemas/AuditDetailsByCode/properties/{code}"),
            "components": components,
        });
        let validator = jsonschema::validator_for(&schema).unwrap();
        failures.extend(
            validator
                .iter_errors(details)
                .map(|e| format!("{code}: {e} at {}", e.instance_path())),
        );
    }
    let listed: BTreeSet<&str> = AUDIT_EVENT_TYPES.iter().copied().collect();
    assert_eq!(covered, listed, "one sample per emitted audit code");
    assert!(failures.is_empty(), "{failures:#?}");
}
