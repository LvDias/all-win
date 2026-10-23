//! Routes (source → account) and their effective status (s7-routing.md §2; S7 AC-1, AC-6…AC-9).
//!
//! Everything here is a pure function of facts the application layer reads through query
//! ports; the UI renders the result and never re-implements a gate.

use std::collections::BTreeSet;

use uuid::Uuid;

use crate::instruction::id;

id!(
    /// A route (`routing.subscription.id`, UUIDv7).
    RouteId
);
id!(
    /// Account owned by `accounts`; routing keeps the plain id.
    AccountId
);

/// Signal types a TradingView route can select (ADR-0003). `master_account` belongs to
/// follower routes (S19) and is not selectable here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RouteSignalType {
    Trigger,
    TradePlan,
    NqTrigger,
}

impl RouteSignalType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trigger => "trigger",
            Self::TradePlan => "trade_plan",
            Self::NqTrigger => "nq_trigger",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "trigger" => Some(Self::Trigger),
            "trade_plan" => Some(Self::TradePlan),
            "nq_trigger" => Some(Self::NqTrigger),
            _ => None,
        }
    }
}

/// At least one signal type, no duplicates (S7 AC-2, Q-S7-1: nothing preselected).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalTypes(BTreeSet<RouteSignalType>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum InvalidSignalTypes {
    #[error("at least one signal type is required")]
    Empty,
    #[error("duplicate signal type")]
    Duplicate,
}

impl SignalTypes {
    pub fn new(
        types: impl IntoIterator<Item = RouteSignalType>,
    ) -> Result<Self, InvalidSignalTypes> {
        let mut set = BTreeSet::new();
        for t in types {
            if !set.insert(t) {
                return Err(InvalidSignalTypes::Duplicate);
            }
        }
        if set.is_empty() {
            return Err(InvalidSignalTypes::Empty);
        }
        Ok(Self(set))
    }

    pub fn contains(&self, t: RouteSignalType) -> bool {
        self.0.contains(&t)
    }

    /// In a stable order (as stored and returned).
    pub fn iter(&self) -> impl Iterator<Item = RouteSignalType> + '_ {
        self.0.iter().copied()
    }
}

/// What a reason blocks (S7 AC-8, P1: out-of-sync and risk never block exits).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blocks {
    AllActions,
    EntriesOnly,
}

impl Blocks {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllActions => "all_actions",
            Self::EntriesOnly => "entries_only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Paused,
    Disabled,
}

impl LifecycleState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Paused => "paused",
            Self::Disabled => "disabled",
        }
    }
}

/// Why an opt-in is not accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotAccepted {
    Declined,
    PendingReview,
}

impl NotAccepted {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Declined => "declined",
            Self::PendingReview => "pending_review",
        }
    }
}

/// One reason a route does not (fully) receive now (`RouteReason`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteReason {
    AccountInactive(LifecycleState),
    SourceRevoked,
    PlatformUnbound,
    AutomationNotAccepted(NotAccepted),
    CopyNotAccepted(NotAccepted),
    ConnectionDisconnected,
    OutOfSync,
    RiskBlocked,
    /// The enabled instruments without a stop/target profile for this account (never empty).
    NoBracketProfile(Vec<Uuid>),
    LimitsPending,
    RouteDisabled,
}

impl RouteReason {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::AccountInactive(_) => "routing.account_inactive",
            Self::SourceRevoked => "routing.source_revoked",
            Self::PlatformUnbound => "routing.platform_unbound",
            Self::AutomationNotAccepted(_) => "routing.automation_not_accepted",
            Self::CopyNotAccepted(_) => "routing.copy_not_accepted",
            Self::ConnectionDisconnected => "routing.connection_disconnected",
            Self::OutOfSync => "routing.out_of_sync",
            Self::RiskBlocked => "routing.risk_blocked",
            Self::NoBracketProfile(_) => "routing.no_bracket_profile",
            Self::LimitsPending => "routing.limits_pending",
            Self::RouteDisabled => "routing.route_disabled",
        }
    }

    /// Fixed per code (s7-routing.md §2.3).
    pub const fn blocks(&self) -> Blocks {
        match self {
            Self::OutOfSync
            | Self::RiskBlocked
            | Self::NoBracketProfile(_)
            | Self::LimitsPending => Blocks::EntriesOnly,
            _ => Blocks::AllActions,
        }
    }

    /// Only these block creating a route (s7-routing.md §2.4; Q37), and the same ones refuse
    /// switching a route on (Q67 b).
    pub const fn blocks_creation(&self) -> bool {
        matches!(self, Self::AutomationNotAccepted(_) | Self::PlatformUnbound)
    }

    /// Position in the "most severe first" display order (Architect, 2026-10-23; s7-routing
    /// §7). Display only: the code of a skipped instruction follows the gate order
    /// (`decision::decide`).
    const fn rank(&self) -> u8 {
        match self {
            Self::SourceRevoked => 0,
            Self::RouteDisabled => 1,
            Self::PlatformUnbound => 2,
            Self::OutOfSync => 3,
            Self::ConnectionDisconnected => 4,
            Self::RiskBlocked => 5,
            Self::LimitsPending => 6,
            Self::NoBracketProfile(_) => 7,
            Self::AutomationNotAccepted(_) => 8,
            Self::CopyNotAccepted(_) => 9,
            Self::AccountInactive(_) => 10,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Receiving,
    ExitsOnly,
    NotReceiving,
}

impl Status {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Receiving => "receiving",
            Self::ExitsOnly => "exits_only",
            Self::NotReceiving => "not_receiving",
        }
    }
}

/// `RouteStatus` without the timestamp: the status and every reason, most severe first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveStatus {
    pub status: Status,
    pub reasons: Vec<RouteReason>,
}

impl EffectiveStatus {
    pub fn from_reasons(mut reasons: Vec<RouteReason>) -> Self {
        reasons.sort_by_key(RouteReason::rank);
        let status = if reasons.is_empty() {
            Status::Receiving
        } else if reasons.iter().any(|r| r.blocks() == Blocks::AllActions) {
            Status::NotReceiving
        } else {
            Status::ExitsOnly
        };
        Self { status, reasons }
    }
}

/// State of the account's opt-in for one kind of source (S5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptIn {
    /// Accepted; `with_step_up` = the acceptance itself was confirmed with a step-up.
    Accepted {
        with_step_up: bool,
    },
    /// The firm does not require a choice (e.g. a real account): accepted, without a step-up.
    NotRequired,
    NotAccepted(NotAccepted),
}

impl OptIn {
    const fn not_accepted(self) -> Option<NotAccepted> {
        match self {
            Self::NotAccepted(n) => Some(n),
            _ => None,
        }
    }

    /// Q35 (setting on): switching on an automated route needs a fresh `route_enable`
    /// step-up unless the automation acceptance was already confirmed with one.
    const fn accepted_with_step_up(self) -> bool {
        matches!(self, Self::Accepted { with_step_up: true })
    }
}

/// Facts about one (source, account) pair at one moment, read through query ports.
/// `None` for a fact whose owning slice is not wired yet means "no reason from it".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteFacts {
    pub source_revoked: bool,
    pub source_automated: bool,
    pub account_lifecycle: Option<LifecycleState>,
    pub platform_bound: bool,
    pub automation: OptIn,
    pub connection_connected: bool,
    pub out_of_sync: Option<bool>,
    pub risk_blocked: Option<bool>,
    /// Enabled instruments without a profile; `None` until S6 is wired.
    pub instruments_without_bracket: Option<Vec<Uuid>>,
    /// `None` until the ADR-0006 gate ships with the risk consumer (S13).
    pub limits_pending: Option<bool>,
}

impl RouteFacts {
    /// Every reason except `route_disabled`, which depends on the route itself.
    fn account_reasons(&self) -> Vec<RouteReason> {
        let mut out = Vec::new();
        if let Some(state) = self.account_lifecycle {
            out.push(RouteReason::AccountInactive(state));
        }
        if self.source_revoked {
            out.push(RouteReason::SourceRevoked);
        }
        if !self.platform_bound {
            out.push(RouteReason::PlatformUnbound);
        }
        if self.source_automated
            && let Some(n) = self.automation.not_accepted()
        {
            out.push(RouteReason::AutomationNotAccepted(n));
        }
        if !self.connection_connected {
            out.push(RouteReason::ConnectionDisconnected);
        }
        if self.out_of_sync == Some(true) {
            out.push(RouteReason::OutOfSync);
        }
        if self.risk_blocked == Some(true) {
            out.push(RouteReason::RiskBlocked);
        }
        if let Some(missing) = &self.instruments_without_bracket
            && !missing.is_empty()
        {
            out.push(RouteReason::NoBracketProfile(missing.clone()));
        }
        if self.limits_pending == Some(true) {
            out.push(RouteReason::LimitsPending);
        }
        out
    }

    /// Effective status of an existing route.
    pub fn status_of(&self, is_enabled: bool) -> EffectiveStatus {
        let mut reasons = self.account_reasons();
        if !is_enabled {
            reasons.push(RouteReason::RouteDisabled);
        }
        EffectiveStatus::from_reasons(reasons)
    }

    /// The status a new enabled route would have now (`status_if_routed`).
    pub fn status_if_routed(&self) -> EffectiveStatus {
        self.status_of(true)
    }

    /// Reasons that make the account unselectable in the picker, most severe first.
    pub fn blocking_reasons(&self) -> Vec<RouteReason> {
        let blocking = self
            .account_reasons()
            .into_iter()
            .filter(RouteReason::blocks_creation)
            .collect();
        EffectiveStatus::from_reasons(blocking).reasons
    }

    /// Q67 b: the most severe reason that refuses creating or switching on a route, if any.
    /// Switching off is never refused.
    pub fn enable_refusal(&self) -> Option<RouteReason> {
        self.blocking_reasons().into_iter().next()
    }

    /// Q35 (setting on): whether switching this route on needs a `route_enable` step-up.
    pub fn enable_needs_step_up(&self) -> bool {
        self.source_automated && !self.automation.accepted_with_step_up()
    }

    /// Whether the account's routes from this source must be switched off now
    /// (Q67 b: the Core switches routes off when their account becomes ineligible).
    pub fn must_switch_off(&self) -> bool {
        self.enable_refusal().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eligible() -> RouteFacts {
        RouteFacts {
            source_revoked: false,
            source_automated: true,
            account_lifecycle: None,
            platform_bound: true,
            automation: OptIn::Accepted { with_step_up: true },
            connection_connected: true,
            out_of_sync: Some(false),
            risk_blocked: Some(false),
            instruments_without_bracket: Some(vec![]),
            limits_pending: Some(false),
        }
    }

    fn codes(s: &EffectiveStatus) -> Vec<&'static str> {
        s.reasons.iter().map(RouteReason::code).collect()
    }

    #[test]
    fn signal_types_need_one_and_reject_duplicates() {
        use RouteSignalType::*;
        assert_eq!(SignalTypes::new([]), Err(InvalidSignalTypes::Empty));
        assert_eq!(
            SignalTypes::new([Trigger, Trigger]),
            Err(InvalidSignalTypes::Duplicate)
        );
        let s = SignalTypes::new([NqTrigger, Trigger]).unwrap();
        assert_eq!(s.iter().collect::<Vec<_>>(), [Trigger, NqTrigger]);
        assert_eq!(RouteSignalType::parse("master_account"), None);
        for t in [Trigger, TradePlan, NqTrigger] {
            assert_eq!(RouteSignalType::parse(t.as_str()), Some(t));
        }
    }

    #[test]
    fn eligible_enabled_route_receives() {
        let s = eligible().status_of(true);
        assert_eq!(s.status, Status::Receiving);
        assert!(s.reasons.is_empty());
    }

    #[test]
    fn entries_only_reasons_give_exits_only() {
        let mut f = eligible();
        f.out_of_sync = Some(true);
        f.limits_pending = Some(true);
        f.instruments_without_bracket = Some(vec![Uuid::nil()]);
        let s = f.status_of(true);
        assert_eq!(s.status, Status::ExitsOnly);
        assert!(s.reasons.iter().all(|r| r.blocks() == Blocks::EntriesOnly));
        assert_eq!(
            codes(&s),
            [
                "routing.out_of_sync",
                "routing.limits_pending",
                "routing.no_bracket_profile"
            ]
        );
    }

    #[test]
    fn any_all_actions_reason_gives_not_receiving() {
        let mut f = eligible();
        f.risk_blocked = Some(true);
        f.account_lifecycle = Some(LifecycleState::Paused);
        let s = f.status_of(true);
        assert_eq!(s.status, Status::NotReceiving);
        assert_eq!(
            codes(&s),
            ["routing.risk_blocked", "routing.account_inactive"]
        );
        assert_eq!(f.status_of(false).reasons[0], RouteReason::RouteDisabled);
    }

    #[test]
    fn blocks_is_fixed_per_code() {
        let entries_only = [
            RouteReason::OutOfSync,
            RouteReason::RiskBlocked,
            RouteReason::NoBracketProfile(vec![Uuid::nil()]),
            RouteReason::LimitsPending,
        ];
        for r in entries_only {
            assert_eq!(r.blocks(), Blocks::EntriesOnly, "{}", r.code());
        }
        let all = [
            RouteReason::AccountInactive(LifecycleState::Disabled),
            RouteReason::SourceRevoked,
            RouteReason::PlatformUnbound,
            RouteReason::AutomationNotAccepted(NotAccepted::Declined),
            RouteReason::CopyNotAccepted(NotAccepted::PendingReview),
            RouteReason::ConnectionDisconnected,
            RouteReason::RouteDisabled,
        ];
        for r in all {
            assert_eq!(r.blocks(), Blocks::AllActions, "{}", r.code());
        }
    }

    #[test]
    fn unwired_facts_give_no_reason() {
        let mut f = eligible();
        f.out_of_sync = None;
        f.risk_blocked = None;
        f.instruments_without_bracket = None;
        f.limits_pending = None;
        assert_eq!(f.status_of(true).status, Status::Receiving);
    }

    #[test]
    fn only_automation_and_binding_block_creation_and_enabling() {
        let mut f = eligible();
        f.account_lifecycle = Some(LifecycleState::Paused);
        f.connection_connected = false;
        f.out_of_sync = Some(true);
        assert!(f.blocking_reasons().is_empty());
        assert_eq!(f.enable_refusal(), None);
        assert_eq!(f.status_if_routed().status, Status::NotReceiving);

        f.automation = OptIn::NotAccepted(NotAccepted::PendingReview);
        f.platform_bound = false;
        assert_eq!(
            f.blocking_reasons(),
            [
                RouteReason::PlatformUnbound,
                RouteReason::AutomationNotAccepted(NotAccepted::PendingReview)
            ]
        );
        assert_eq!(f.enable_refusal(), Some(RouteReason::PlatformUnbound));
        assert!(f.must_switch_off());
    }

    #[test]
    fn automation_opt_in_matters_only_for_automated_sources() {
        let mut f = eligible();
        f.source_automated = false;
        f.automation = OptIn::NotAccepted(NotAccepted::Declined);
        assert!(f.blocking_reasons().is_empty());
        assert!(!f.enable_needs_step_up());
    }

    #[test]
    fn step_up_is_needed_unless_automation_was_accepted_with_one() {
        let mut f = eligible();
        assert!(!f.enable_needs_step_up());
        f.automation = OptIn::Accepted {
            with_step_up: false,
        };
        assert!(f.enable_needs_step_up());
        f.automation = OptIn::NotRequired;
        assert!(f.enable_needs_step_up());
    }
}
