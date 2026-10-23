//! TradingView source policy (ADR-0003 §6; s1-webhook-ingest §2 step 7).

use contracts::instruction::v1::{Action, Instruction, OrderType, PositionEffect, SignalType};
use routing_application::enterprise::RejectionCode;

use crate::SOURCE_TYPE;

/// A policy rejection: code and the JSON pointer of the field that caused it.
pub type Violation = (RejectionCode, &'static str);

pub fn check(i: &Instruction) -> Result<(), Violation> {
    if i.source.kind != SOURCE_TYPE {
        return Err((RejectionCode::SourceMismatch, "/source/type"));
    }
    if i.signal_type == SignalType::MasterAccount {
        return Err((RejectionCode::SignalTypeNotAllowedForSource, "/signal_type"));
    }
    if matches!(i.action, Action::Modify | Action::Cancel) {
        return Err((RejectionCode::ActionNotAllowedForSource, "/action"));
    }
    if i.position_effect == Some(PositionEffect::Reduce) {
        return Err((RejectionCode::ActionNotAllowedForSource, "/position_effect"));
    }
    if i.order
        .as_ref()
        .is_some_and(|o| o.kind != OrderType::Market)
    {
        return Err((RejectionCode::OrderTypeNotAllowedForSource, "/order/type"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use contracts::instruction::v1;

    fn instr(extra: &str) -> Instruction {
        let raw = format!(
            r#"{{"schema_version":"1.0","message_id":"m","sent_at":"2026-10-22T16:03:20Z",
               "source":{{"type":"tradingview"}},"signal_type":"trigger","instrument":{{"symbol":"MNQ"}}{extra}}}"#
        );
        v1::parse(raw.as_bytes()).unwrap()
    }

    #[test]
    fn allowed_tradingview_actions() {
        for extra in [
            r#","action":"buy","quantity":1"#,
            r#","action":"sell","quantity":1,"position_effect":"open""#,
            r#","action":"close""#,
            r#","action":"reverse","side":"short","quantity":2"#,
            r#","action":"buy","quantity":1,"order":{"type":"market"}"#,
        ] {
            assert_eq!(check(&instr(extra)), Ok(()), "{extra}");
        }
    }

    #[test]
    fn rejected_by_policy() {
        let cases = [
            (
                r#","action":"sell","quantity":1,"position_effect":"reduce""#,
                RejectionCode::ActionNotAllowedForSource,
            ),
            (
                r#","action":"cancel","refs":{"order_ref":"o1"}"#,
                RejectionCode::ActionNotAllowedForSource,
            ),
            (
                r#","action":"modify","refs":{"order_ref":"o1"},"changes":{"quantity":2}"#,
                RejectionCode::ActionNotAllowedForSource,
            ),
            (
                r#","action":"buy","quantity":1,"order":{"type":"limit","limit_price":"1.25"}"#,
                RejectionCode::OrderTypeNotAllowedForSource,
            ),
        ];
        for (extra, code) in cases {
            assert_eq!(check(&instr(extra)).map_err(|v| v.0), Err(code), "{extra}");
        }
    }

    #[test]
    fn source_and_signal_type_must_match_the_credential() {
        let mut i = instr(r#","action":"buy","quantity":1"#);
        i.signal_type = SignalType::MasterAccount;
        assert_eq!(
            check(&i),
            Err((RejectionCode::SignalTypeNotAllowedForSource, "/signal_type"))
        );
        i.source.kind = "master_account".into();
        assert_eq!(
            check(&i),
            Err((RejectionCode::SourceMismatch, "/source/type"))
        );
    }
}
