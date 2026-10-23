//! Command delivery rules (ADR-0004 §4–§5, r4).
//!
//! `gateway.command` is also the delivery outbox. The decision for each command is pure; the
//! delivery loop must commit `sent`/`sent_at` **before** writing the frame to the socket, so
//! `Pending` proves nothing left the Core (the reverse expiry exception depends on it).

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::ids::CommandId;
use crate::protocol::ProtocolVersion;

/// A command not acked within 72 h of its creation is never retransmitted (ADR-0004 §4).
/// Must stay below the adapter's 6-day staleness guard and 7-day dedupe retention.
pub const RETRANSMISSION_HORIZON: Duration = Duration::hours(72);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandKind {
    Place,
    Modify,
    Cancel,
    Flatten,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PositionEffect {
    Open,
    Reduce,
}

/// `gateway.command.state` (data model rev. 16.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandState {
    Pending,
    Sent,
    Acked,
    Completed,
    Rejected,
    Expired,
    Failed,
    Unknown,
    Undelivered,
}

impl CommandState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Sent => "sent",
            Self::Acked => "acked",
            Self::Completed => "completed",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
            Self::Failed => "failed",
            Self::Unknown => "unknown",
            Self::Undelivered => "undelivered",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        [
            Self::Pending,
            Self::Sent,
            Self::Acked,
            Self::Completed,
            Self::Rejected,
            Self::Expired,
            Self::Failed,
            Self::Unknown,
            Self::Undelivered,
        ]
        .into_iter()
        .find(|s| s.as_str() == value)
    }

    /// Final states never change again (a later report for the command is history only).
    pub const fn is_final(self) -> bool {
        !matches!(self, Self::Pending | Self::Sent | Self::Acked)
    }

    /// Still owed to the adapter: in the outbox until acked.
    pub const fn awaits_ack(self) -> bool {
        matches!(self, Self::Pending | Self::Sent)
    }
}

/// Outcome of the command's `depends_on` prerequisite, as seen by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prerequisite {
    /// No `depends_on`.
    None,
    /// Not yet terminal and still within its normal report timeout.
    NotTerminal,
    /// Terminal success (`filled`, or `cancelled` + `command.already_flat` for a flatten);
    /// `flat_proven` per ADR-0004 §5 r4 (see `position::flat_proven`).
    Succeeded { flat_proven: bool },
    /// Terminal failure (rejected, expired, unknown…).
    Failed,
    /// Never became terminal within its normal report timeout.
    TimedOut,
}

/// Session-side facts for one command's account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeliveryContext {
    /// `None` when no session of the connection is live.
    pub negotiated: Option<ProtocolVersion>,
    /// The account is in the current session's `welcome` mapping (Esquadro X2: a bind made
    /// during the session applies at the next handshake).
    pub account_in_welcome: bool,
    /// The account's reconnect `position_snapshot` was evaluated in this session (§3 step 3).
    pub snapshot_evaluated: bool,
    pub prerequisite: Prerequisite,
    /// The account has an open sync reason other than one this command would raise.
    pub other_sync_reason_open: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitReason {
    NoSession,
    AccountNotInWelcome,
    SnapshotPending,
    /// A `reduce` never goes to a session below 1.2 (§9).
    ProtocolBelowReduce,
    /// Expired opening leg of a reverse, never sent: decided when its flatten is terminal.
    HeldForPrerequisite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    /// Commit `sent` (first send) then write; for `Sent`, retransmit the stored frame unchanged.
    Send,
    Wait(WaitReason),
    /// Becomes `expired`, audited (`execution.expired`). `flag` raises `sync.command_expired`;
    /// `reverse_close_only` notifies `routing.reverse_close_only` with `routing.expired`.
    Expire {
        flag: bool,
        reverse_close_only: bool,
    },
    /// 72-h horizon: `undelivered`, removed from the outbox, `sync.command_undelivered`.
    Undeliver,
    /// Acked or final: nothing to deliver.
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub id: CommandId,
    pub kind: CommandKind,
    pub position_effect: Option<PositionEffect>,
    pub depends_on: Option<CommandId>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub state: CommandState,
    pub sent_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CommandError {
    #[error("illegal command transition from {from:?}")]
    IllegalTransition { from: CommandState },
    #[error("a position-increasing command requires expires_at")]
    MissingExpiry,
    #[error("only a place carries a position effect")]
    PositionEffectOnNonPlace,
}

impl Command {
    /// A new outbox row, as consumed from `routing.commands_requested`.
    pub fn new_pending(
        id: CommandId,
        kind: CommandKind,
        position_effect: Option<PositionEffect>,
        depends_on: Option<CommandId>,
        expires_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
    ) -> Result<Self, CommandError> {
        match (kind, position_effect) {
            (CommandKind::Place, None) => return Err(CommandError::PositionEffectOnNonPlace),
            (CommandKind::Place, Some(PositionEffect::Open)) if expires_at.is_none() => {
                return Err(CommandError::MissingExpiry);
            }
            (CommandKind::Place, _) => {}
            (_, Some(_)) => return Err(CommandError::PositionEffectOnNonPlace),
            (_, None) => {}
        }
        Ok(Self {
            id,
            kind,
            position_effect,
            depends_on,
            expires_at,
            created_at,
            state: CommandState::Pending,
            sent_at: None,
        })
    }

    pub fn is_reverse_opening_leg(&self) -> bool {
        self.kind == CommandKind::Place
            && self.position_effect == Some(PositionEffect::Open)
            && self.depends_on.is_some()
    }

    pub fn is_reduce(&self) -> bool {
        self.position_effect == Some(PositionEffect::Reduce)
    }

    fn expired_at(&self, now: DateTime<Utc>) -> bool {
        self.expires_at.is_some_and(|e| now >= e)
    }

    pub fn delivery(&self, now: DateTime<Utc>, ctx: &DeliveryContext) -> Delivery {
        match self.state {
            CommandState::Pending => self.pending_delivery(now, ctx),
            // Already left the Core: retransmitted unchanged until acked, whatever its expiry.
            // The adapter answers `duplicate` or `command.expired`, which is how the Core learns
            // the outcome; dropping it would hide a possibly executed order.
            CommandState::Sent => {
                if now >= self.created_at + RETRANSMISSION_HORIZON {
                    Delivery::Undeliver
                } else {
                    match session_gate(self, ctx) {
                        Some(reason) => Delivery::Wait(reason),
                        None => Delivery::Send,
                    }
                }
            }
            _ => Delivery::Done,
        }
    }

    fn pending_delivery(&self, now: DateTime<Utc>, ctx: &DeliveryContext) -> Delivery {
        if self.expired_at(now) {
            if !self.is_reverse_opening_leg() {
                return Delivery::Expire {
                    flag: true,
                    reverse_close_only: false,
                };
            }
            return match ctx.prerequisite {
                Prerequisite::NotTerminal => Delivery::Wait(WaitReason::HeldForPrerequisite),
                Prerequisite::Succeeded { flat_proven: true } if !ctx.other_sync_reason_open => {
                    Delivery::Expire {
                        flag: false,
                        reverse_close_only: true,
                    }
                }
                _ => Delivery::Expire {
                    flag: true,
                    reverse_close_only: false,
                },
            };
        }
        if now >= self.created_at + RETRANSMISSION_HORIZON {
            return Delivery::Undeliver;
        }
        match session_gate(self, ctx) {
            Some(reason) => Delivery::Wait(reason),
            None => Delivery::Send,
        }
    }

    /// First send, committed before the socket write. Refuses an expired command, so a caller
    /// that raced the clock cannot send late.
    pub fn mark_sent(&mut self, now: DateTime<Utc>) -> Result<(), CommandError> {
        if self.state != CommandState::Pending || self.expired_at(now) {
            return Err(CommandError::IllegalTransition { from: self.state });
        }
        self.state = CommandState::Sent;
        self.sent_at = Some(now);
        Ok(())
    }

    /// The adapter durably recorded the command.
    pub fn mark_acked(&mut self) -> Result<(), CommandError> {
        match self.state {
            CommandState::Sent => {
                self.state = CommandState::Acked;
                Ok(())
            }
            CommandState::Acked => Ok(()),
            s if s.is_final() => Ok(()),
            from => Err(CommandError::IllegalTransition { from }),
        }
    }

    /// Applies an outcome. Final states are sticky; `Pending` commands never take reports
    /// (nothing was delivered).
    pub fn finish(&mut self, to: CommandState) -> Result<bool, CommandError> {
        if self.state.is_final() {
            return Ok(false);
        }
        if !to.is_final() {
            return Err(CommandError::IllegalTransition { from: self.state });
        }
        let allowed = match to {
            CommandState::Expired => true,
            CommandState::Undelivered => self.state.awaits_ack(),
            _ => self.state != CommandState::Pending,
        };
        if !allowed {
            return Err(CommandError::IllegalTransition { from: self.state });
        }
        self.state = to;
        Ok(true)
    }
}

fn session_gate(cmd: &Command, ctx: &DeliveryContext) -> Option<WaitReason> {
    let Some(version) = ctx.negotiated else {
        return Some(WaitReason::NoSession);
    };
    if !ctx.account_in_welcome {
        return Some(WaitReason::AccountNotInWelcome);
    }
    if !ctx.snapshot_evaluated {
        return Some(WaitReason::SnapshotPending);
    }
    if cmd.is_reduce() && !version.supports_reduce() {
        return Some(WaitReason::ProtocolBelowReduce);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use uuid::Uuid;

    fn t(ms: i64) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(1_800_000_000_000 + ms).unwrap()
    }

    fn id(n: u128) -> CommandId {
        CommandId::from_uuid(Uuid::from_u128(n))
    }

    fn ready() -> DeliveryContext {
        DeliveryContext {
            negotiated: Some(ProtocolVersion::V1_2),
            account_in_welcome: true,
            snapshot_evaluated: true,
            prerequisite: Prerequisite::None,
            other_sync_reason_open: false,
        }
    }

    fn entry() -> Command {
        Command::new_pending(
            id(1),
            CommandKind::Place,
            Some(PositionEffect::Open),
            None,
            Some(t(5_000)),
            t(0),
        )
        .unwrap()
    }

    fn reverse_open() -> Command {
        Command::new_pending(
            id(2),
            CommandKind::Place,
            Some(PositionEffect::Open),
            Some(id(1)),
            Some(t(5_000)),
            t(0),
        )
        .unwrap()
    }

    fn flatten() -> Command {
        Command::new_pending(id(3), CommandKind::Flatten, None, None, None, t(0)).unwrap()
    }

    #[test]
    fn construction_enforces_schema_rules() {
        assert_eq!(
            Command::new_pending(
                id(1),
                CommandKind::Place,
                Some(PositionEffect::Open),
                None,
                None,
                t(0)
            ),
            Err(CommandError::MissingExpiry)
        );
        assert!(
            Command::new_pending(
                id(1),
                CommandKind::Place,
                Some(PositionEffect::Reduce),
                None,
                None,
                t(0)
            )
            .is_ok()
        );
        assert_eq!(
            Command::new_pending(
                id(1),
                CommandKind::Flatten,
                Some(PositionEffect::Reduce),
                None,
                None,
                t(0)
            ),
            Err(CommandError::PositionEffectOnNonPlace)
        );
        assert_eq!(
            Command::new_pending(id(1), CommandKind::Place, None, None, Some(t(1)), t(0)),
            Err(CommandError::PositionEffectOnNonPlace)
        );
    }

    #[test]
    fn sends_only_when_every_session_gate_is_open() {
        let c = entry();
        assert_eq!(c.delivery(t(1), &ready()), Delivery::Send);
        let cases = [
            (
                DeliveryContext {
                    negotiated: None,
                    ..ready()
                },
                WaitReason::NoSession,
            ),
            (
                DeliveryContext {
                    account_in_welcome: false,
                    ..ready()
                },
                WaitReason::AccountNotInWelcome,
            ),
            (
                DeliveryContext {
                    snapshot_evaluated: false,
                    ..ready()
                },
                WaitReason::SnapshotPending,
            ),
        ];
        for (ctx, reason) in cases {
            assert_eq!(c.delivery(t(1), &ctx), Delivery::Wait(reason));
        }
    }

    #[test]
    fn never_sends_after_expiry_and_flags() {
        let c = entry();
        let expired = Delivery::Expire {
            flag: true,
            reverse_close_only: false,
        };
        assert_eq!(c.delivery(t(5_000), &ready()), expired);
        // Also while waiting for the session: a pending entry found expired on reconnect.
        let no_session = DeliveryContext {
            negotiated: None,
            ..ready()
        };
        assert_eq!(c.delivery(t(9_000), &no_session), expired);
        let mut c = entry();
        assert!(c.mark_sent(t(5_000)).is_err());
        assert_eq!(c.state, CommandState::Pending);
    }

    #[test]
    fn exits_do_not_expire_but_hit_the_horizon() {
        let f = flatten();
        let no_session = DeliveryContext {
            negotiated: None,
            ..ready()
        };
        assert_eq!(
            f.delivery(t(3_600_000), &no_session),
            Delivery::Wait(WaitReason::NoSession)
        );
        let horizon = RETRANSMISSION_HORIZON.num_milliseconds();
        assert_eq!(f.delivery(t(horizon - 1), &ready()), Delivery::Send);
        assert_eq!(f.delivery(t(horizon), &ready()), Delivery::Undeliver);
    }

    #[test]
    fn sent_commands_are_retransmitted_until_acked_or_horizon() {
        let mut c = entry();
        c.mark_sent(t(1)).unwrap();
        // Expired but already left the Core: retransmitted unchanged.
        assert_eq!(c.delivery(t(60_000), &ready()), Delivery::Send);
        let snapshot_pending = DeliveryContext {
            snapshot_evaluated: false,
            ..ready()
        };
        assert_eq!(
            c.delivery(t(60_000), &snapshot_pending),
            Delivery::Wait(WaitReason::SnapshotPending)
        );
        let horizon = RETRANSMISSION_HORIZON.num_milliseconds();
        assert_eq!(c.delivery(t(horizon), &ready()), Delivery::Undeliver);
        c.mark_acked().unwrap();
        assert_eq!(c.delivery(t(2), &ready()), Delivery::Done);
    }

    #[test]
    fn reverse_opening_leg_is_held_until_its_flatten_is_terminal() {
        let c = reverse_open();
        let ctx = DeliveryContext {
            prerequisite: Prerequisite::NotTerminal,
            ..ready()
        };
        assert_eq!(
            c.delivery(t(6_000), &ctx),
            Delivery::Wait(WaitReason::HeldForPrerequisite)
        );
    }

    #[test]
    fn reverse_opening_leg_not_flagged_only_when_flat_is_proven_and_nothing_else_open() {
        let c = reverse_open();
        let ok = DeliveryContext {
            prerequisite: Prerequisite::Succeeded { flat_proven: true },
            ..ready()
        };
        assert_eq!(
            c.delivery(t(6_000), &ok),
            Delivery::Expire {
                flag: false,
                reverse_close_only: true
            }
        );
        let flagged = Delivery::Expire {
            flag: true,
            reverse_close_only: false,
        };
        for ctx in [
            DeliveryContext {
                prerequisite: Prerequisite::Succeeded { flat_proven: false },
                ..ready()
            },
            DeliveryContext {
                other_sync_reason_open: true,
                ..ok
            },
            DeliveryContext {
                prerequisite: Prerequisite::Failed,
                ..ready()
            },
            DeliveryContext {
                prerequisite: Prerequisite::TimedOut,
                ..ready()
            },
        ] {
            assert_eq!(c.delivery(t(6_000), &ctx), flagged);
        }
    }

    #[test]
    fn reverse_opening_leg_already_sent_is_never_exempt() {
        let mut c = reverse_open();
        c.mark_sent(t(1)).unwrap();
        assert!(c.finish(CommandState::Expired).unwrap());
        assert_eq!(c.state, CommandState::Expired);
        // A sent command is never decided by the pending rules.
        let mut c = reverse_open();
        c.mark_sent(t(1)).unwrap();
        let ok = DeliveryContext {
            prerequisite: Prerequisite::Succeeded { flat_proven: true },
            ..ready()
        };
        assert_eq!(c.delivery(t(6_000), &ok), Delivery::Send);
    }

    #[test]
    fn reduce_never_goes_below_1_2() {
        let c = Command::new_pending(
            id(9),
            CommandKind::Place,
            Some(PositionEffect::Reduce),
            None,
            None,
            t(0),
        )
        .unwrap();
        let old = DeliveryContext {
            negotiated: Some(ProtocolVersion::V1_1),
            ..ready()
        };
        assert_eq!(
            c.delivery(t(1), &old),
            Delivery::Wait(WaitReason::ProtocolBelowReduce)
        );
    }

    #[test]
    fn final_states_are_sticky_and_pending_takes_no_outcome() {
        let mut c = entry();
        assert!(c.finish(CommandState::Completed).is_err());
        c.mark_sent(t(1)).unwrap();
        c.mark_acked().unwrap();
        assert!(c.finish(CommandState::Undelivered).is_err());
        assert!(c.finish(CommandState::Completed).unwrap());
        assert!(!c.finish(CommandState::Rejected).unwrap());
        assert_eq!(c.state, CommandState::Completed);
        assert!(c.mark_acked().is_ok());
    }

    #[test]
    fn horizon_is_below_the_adapter_staleness_and_retention() {
        assert!(RETRANSMISSION_HORIZON < Duration::days(6));
        assert!(Duration::days(6) < Duration::days(7));
    }

    #[test]
    fn state_codes_round_trip() {
        for s in [
            "pending",
            "sent",
            "acked",
            "completed",
            "rejected",
            "expired",
            "failed",
            "unknown",
            "undelivered",
        ] {
            assert_eq!(CommandState::parse(s).unwrap().as_str(), s);
        }
    }
}
