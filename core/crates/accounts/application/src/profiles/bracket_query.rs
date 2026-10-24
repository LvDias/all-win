//! Query for execution (routing's `BracketFor` port, adapter in `crates/app`): the profiles of a
//! user's accounts resolved to whole ticks. An absent entry means no usable profile — none, a
//! stored one that no longer converts, or an account that is unknown, archived or another user's —
//! and routing skips the entry with `routing.no_bracket_profile` (S6 AC-6; exits never depend on it).

use std::collections::HashMap;
use std::sync::Arc;

use uuid::Uuid;

use accounts_enterprise::AccountId;
use accounts_enterprise::profile::Bracket;
use shared_kernel::UserId;

use super::error::ProfilesError;
use super::ports::{EnabledInstrument, ProfileCatalog, ProfileStore};

#[derive(Clone)]
pub struct BracketQuery {
    store: Arc<dyn ProfileStore>,
    catalog: Arc<dyn ProfileCatalog>,
}

impl BracketQuery {
    pub fn new(store: Arc<dyn ProfileStore>, catalog: Arc<dyn ProfileCatalog>) -> Self {
        Self { store, catalog }
    }

    /// One instruction to N accounts of one user and instrument.
    pub async fn brackets_for(
        &self,
        user_id: UserId,
        instrument_id: Uuid,
        account_ids: &[AccountId],
    ) -> Result<HashMap<AccountId, Bracket>, ProfilesError> {
        let instruments = self.instruments(&[instrument_id]).await?;
        let Some(instrument) = instruments.first() else {
            return Ok(HashMap::new());
        };
        let live = self
            .store
            .live_profiles(user_id, account_ids, &[instrument_id])
            .await?;
        Ok(live
            .into_iter()
            .filter_map(|a| {
                let p = a
                    .profiles
                    .iter()
                    .find(|p| p.instrument_id() == instrument_id)?;
                let bracket = Bracket::resolve(*p.profile(), &instrument.ticks).ok()?;
                Some((a.account_id, bracket))
            })
            .collect())
    }

    /// For `listRoutes`: per live account, the enabled instruments among `instrument_ids` without a
    /// usable profile (accounts with none missing are left out). Unknown or other users' accounts
    /// are left out too.
    pub async fn missing_brackets(
        &self,
        user_id: UserId,
        account_ids: &[AccountId],
        instrument_ids: &[Uuid],
    ) -> Result<HashMap<AccountId, Vec<Uuid>>, ProfilesError> {
        let instruments = self.instruments(instrument_ids).await?;
        let enabled: Vec<Uuid> = instruments.iter().map(|i| i.id).collect();
        let live = self
            .store
            .live_profiles(user_id, account_ids, &enabled)
            .await?;
        Ok(live
            .into_iter()
            .filter_map(|a| {
                let missing: Vec<Uuid> = instruments
                    .iter()
                    .filter(|i| {
                        !a.profiles.iter().any(|p| {
                            p.instrument_id() == i.id
                                && Bracket::resolve(*p.profile(), &i.ticks).is_ok()
                        })
                    })
                    .map(|i| i.id)
                    .collect();
                (!missing.is_empty()).then_some((a.account_id, missing))
            })
            .collect())
    }

    /// The enabled instruments among `ids`.
    async fn instruments(&self, ids: &[Uuid]) -> Result<Vec<EnabledInstrument>, ProfilesError> {
        Ok(self
            .catalog
            .enabled_instruments()
            .await?
            .into_iter()
            .filter(|i| ids.contains(&i.id))
            .collect())
    }
}
