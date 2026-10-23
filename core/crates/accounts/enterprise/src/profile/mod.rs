//! Stop/target profiles (V1-S6): per-size defaults, the account's own copy and the exact
//! conversion to whole ticks. Platform-neutral; instruments are catalog ids, never symbols (X-1).

pub mod account_profile;
pub mod conversion;
pub mod distance;
pub mod nearest_size;
pub mod protective;

pub use account_profile::{
    AccountBracketProfile, CopiedFrom, DefaultChange, DefaultProfile, InconsistentProfile,
    ProfileOrigin, ProfileSources, ProfileWrite, RestoreError, VersionConflict, change_default,
    default_now, edit, initial_copy, restore,
};
pub use conversion::{
    DistanceRejected, DistanceRule, Equivalent, InvalidTickSpec, MAX_TICKS, ProfileEquivalents,
    TickSpec,
};
pub use distance::{DistanceProfile, DistanceShapeError, DistanceUnit, ProfileSide};
pub use nearest_size::{CopySourceError, copy_source, nearest_profile_sizes};
pub use protective::{Bracket, PositionSide, ProtectiveError, ProtectivePrices, protective_prices};

#[cfg(test)]
pub(crate) mod test_support {
    use std::str::FromStr;

    use rust_decimal::Decimal;

    use crate::values::SizeAmount;

    pub fn size(amount: &str) -> SizeAmount {
        SizeAmount::new(Decimal::from_str(amount).unwrap()).unwrap()
    }

    /// `catalog.standard_account_size` (vision §8).
    pub fn standard() -> Vec<SizeAmount> {
        ["25000", "50000", "100000", "150000", "200000", "300000"]
            .map(size)
            .to_vec()
    }
}
