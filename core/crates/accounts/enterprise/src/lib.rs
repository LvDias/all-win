//! `accounts` module — enterprise layer (V1-S5): the trading account, its lifecycle, its prop
//! selection, its per-kind opt-in state and its binding to a platform account. No platform is named
//! here (X-1): a binding is gateway's opaque `venue_account_id`.

pub mod account;
pub mod ids;
pub mod lifecycle;
pub mod opt_in;
pub mod preset_values;
pub mod profile;
pub mod rule_value;
pub mod values;

pub use account::{Account, AccountKind, Binding, PropSelection};
pub use ids::{AccountId, ConnectionId, FirmId, PresetVersionId, RuleVersionId, VenueAccountId};
pub use lifecycle::{LifecycleError, LifecycleState};
pub use opt_in::{
    Choice, ChoiceRequest, FirmStance, GENERIC_AUTOMATION_RULE_VERSION_ID,
    GENERIC_COPY_RULE_VERSION_ID, OptInError, OptInKind, OptInState, PendingReason, PreviousChoice,
    RuleVersionRef,
};
pub use rule_value::{RuleField, RuleOrigin};
pub use values::{
    AccountName, FirmPlatform, ManualText, MaxPositionQty, MoneyAmount, Multiplier, SizeAmount,
    ValueError,
};
