//! `InitialProfilePlanner` (S5 create, same transaction): one copy per enabled instrument from the
//! user's default of the account's copy source (S6 AC-2, AC-7, Q60, Q73 b).

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use accounts_enterprise::profile::{self, AccountBracketProfile, CopySourceError, ProfileSources};
use accounts_enterprise::{AccountKind, SizeAmount};
use shared_kernel::UserId;

use super::ports::{ProfileCatalog, ProfileStore};
use crate::ports::{InitialProfileError, InitialProfilePlanner};

pub struct ProfilePlanner {
    store: Arc<dyn ProfileStore>,
    catalog: Arc<dyn ProfileCatalog>,
}

impl ProfilePlanner {
    pub fn new(store: Arc<dyn ProfileStore>, catalog: Arc<dyn ProfileCatalog>) -> Self {
        Self { store, catalog }
    }
}

#[async_trait]
impl InitialProfilePlanner for ProfilePlanner {
    /// `kind` no longer matters (Q73 b: a prop size outside the standard list also takes the
    /// nearest standard size); kept in the port's signature.
    async fn plan(
        &self,
        user_id: UserId,
        _kind: AccountKind,
        size: &SizeAmount,
        profile_size: Option<&SizeAmount>,
        now: DateTime<Utc>,
    ) -> Result<Vec<AccountBracketProfile>, InitialProfileError> {
        let unavailable = |m: String| InitialProfileError::Unavailable(m);
        let standard = self
            .catalog
            .standard_sizes()
            .await
            .map_err(|e| unavailable(e.0))?;
        // The tie is validated even when the user has no default (or no instrument) yet.
        profile::copy_source(size, profile_size, &standard).map_err(source_error)?;
        let instruments = self
            .catalog
            .enabled_instruments()
            .await
            .map_err(|e| unavailable(e.0))?;
        let defaults = self
            .store
            .defaults(user_id, None)
            .await
            .map_err(|e| unavailable(e.to_string()))?;
        let sources = ProfileSources {
            standard: &standard,
            defaults: &defaults,
        };
        let mut copies = Vec::new();
        for instrument in instruments {
            if let Some(copy) =
                profile::initial_copy(instrument.id, size, profile_size, sources, now)
                    .map_err(source_error)?
            {
                copies.push(copy);
            }
        }
        Ok(copies)
    }
}

fn source_error(e: CopySourceError) -> InitialProfileError {
    match e {
        CopySourceError::ProfileSizeRequired => InitialProfileError::ProfileSizeRequired,
        CopySourceError::ProfileSizeInvalid => InitialProfileError::ProfileSizeInvalid,
    }
}
