//! Account lifecycle (S5 AC-4). A paused or disabled account receives no instruction at all,
//! exits included (S7 AC-8); open positions and orders are not touched. Archiving is separate.

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Active,
    Paused,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("accounts.lifecycle_transition_invalid")]
pub struct LifecycleError;

impl LifecycleState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Disabled => "disabled",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "active" => Some(Self::Active),
            "paused" => Some(Self::Paused),
            "disabled" => Some(Self::Disabled),
            _ => None,
        }
    }

    /// `Ok(None)`: already in `target` (a no-op, not audited). Allowed: active → paused,
    /// paused → active, active|paused → disabled, disabled → active.
    pub fn transition(self, target: Self) -> Result<Option<Self>, LifecycleError> {
        use LifecycleState::*;
        match (self, target) {
            (a, b) if a == b => Ok(None),
            (Active, Paused)
            | (Paused, Active)
            | (Active | Paused, Disabled)
            | (Disabled, Active) => Ok(Some(target)),
            _ => Err(LifecycleError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LifecycleState::*;
    use super::*;

    #[test]
    fn allowed_transitions_and_no_ops() {
        assert_eq!(Active.transition(Paused), Ok(Some(Paused)));
        assert_eq!(Paused.transition(Active), Ok(Some(Active)));
        assert_eq!(Active.transition(Disabled), Ok(Some(Disabled)));
        assert_eq!(Paused.transition(Disabled), Ok(Some(Disabled)));
        assert_eq!(Disabled.transition(Active), Ok(Some(Active)));
        for s in [Active, Paused, Disabled] {
            assert_eq!(s.transition(s), Ok(None));
            assert_eq!(LifecycleState::parse(s.as_str()), Some(s));
        }
    }

    #[test]
    fn disabled_cannot_be_paused() {
        assert_eq!(Disabled.transition(Paused), Err(LifecycleError));
    }
}
