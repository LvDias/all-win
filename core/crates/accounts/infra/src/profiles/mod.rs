//! Stop/target profiles (V1-S6; owner Torno). Placeholder until the S6 files land.

use accounts_application::enterprise::profile::AccountBracketProfile;
use accounts_application::enterprise::{AccountId, AccountName};
use shared_kernel::UserId;
use sqlx::PgConnection;

/// Writes the initial profile copies of a new account in its create transaction (S6 fills it).
pub(crate) async fn insert_initial_copies(
    _conn: &mut PgConnection,
    _user_id: UserId,
    _account_id: AccountId,
    _account_name: &AccountName,
    profiles: &[AccountBracketProfile],
) -> Result<(), sqlx::Error> {
    if profiles.is_empty() {
        Ok(())
    } else {
        Err(sqlx::Error::Protocol("initial profiles need S6".into()))
    }
}
