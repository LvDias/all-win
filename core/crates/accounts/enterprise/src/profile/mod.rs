//! Stop/target profiles (V1-S6; owner Torno). Placeholder until the S6 files land: the account
//! create carries the initial copies (`NewAccount::initial_profiles`) in its transaction.

/// An account's stop/target profile for one instrument (S6 defines it).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountBracketProfile;
