//! Profile use cases (contract `modules/profiles.yaml`): defaults per standard size, the account's
//! own copy, restore, the nearest size (Q60) and the stateless preview. Every user read and write
//! is scoped by the caller's `user_id`.

use std::sync::Arc;

use access::Caller;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use accounts_enterprise::profile::{
    self, AccountBracketProfile, DefaultChange, DefaultProfile, DistanceProfile,
    ProfileEquivalents, ProfileSources, ProfileWrite,
};
use accounts_enterprise::{AccountId, SizeAmount};
use shared_kernel::{Clock, UserId};

use super::error::ProfilesError;
use super::events::{ProfileAction, ProfileEvent};
use super::ports::{
    AccountFacts, AccountProfileWrite, DefaultWrite, EnabledInstrument, ProfileCatalog,
    ProfileStore, ProfileStoreError,
};

/// Everything the profile use cases need, injected by `crates/app`.
#[derive(Clone)]
pub struct ProfileDeps {
    pub store: Arc<dyn ProfileStore>,
    pub catalog: Arc<dyn ProfileCatalog>,
    pub clock: Arc<dyn Clock>,
}

/// An account profile as the contract shows it (`AccountProfile`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountProfileView {
    pub account_id: AccountId,
    pub profile: AccountBracketProfile,
    pub equivalents: ProfileEquivalents,
    /// The current default of `copied_from_size`, if any.
    pub default_now: Option<DistanceProfile>,
}

/// One `AccountProfileList` entry: `None` when the account has no profile for it (Q43).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountProfileEntry {
    pub instrument_id: Uuid,
    pub profile: Option<AccountProfileView>,
}

/// One `DefaultProfileList` entry: `None` when the user has not defined the size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultEntry {
    pub instrument_id: Uuid,
    pub size_amount: SizeAmount,
    pub profile: Option<(DistanceProfile, DateTime<Utc>)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NearestCandidate {
    pub size_amount: SizeAmount,
    pub has_default: bool,
}

/// How many times an upsert is retried when another request changed the same default.
const DEFAULT_WRITE_ATTEMPTS: usize = 3;

#[derive(Clone)]
pub struct ProfileUseCases {
    deps: ProfileDeps,
}

impl ProfileUseCases {
    pub fn new(deps: ProfileDeps) -> Self {
        Self { deps }
    }

    // ---- reads ------------------------------------------------------------------------------

    /// `listDefaultProfiles`: one entry per standard size, ascending.
    pub async fn list_defaults(
        &self,
        caller: &Caller,
        instrument_id: Uuid,
    ) -> Result<Vec<DefaultEntry>, ProfilesError> {
        self.instrument(instrument_id).await?;
        let mut sizes = self.deps.catalog.standard_sizes().await?;
        sizes.sort_by_key(SizeAmount::value);
        let defaults = self.defaults(caller.user_id, Some(instrument_id)).await?;
        Ok(sizes
            .into_iter()
            .map(|size| DefaultEntry {
                instrument_id,
                size_amount: size,
                profile: defaults
                    .iter()
                    .find(|d| d.size_amount.value() == size.value())
                    .map(|d| (d.profile, d.updated_at)),
            })
            .collect())
    }

    /// `listAccountProfiles`: one entry per enabled instrument, in catalog order.
    pub async fn list_account_profiles(
        &self,
        caller: &Caller,
        account_id: AccountId,
    ) -> Result<Vec<AccountProfileEntry>, ProfilesError> {
        let (_, profiles) = self.account(caller.user_id, account_id).await?;
        let instruments = self.deps.catalog.enabled_instruments().await?;
        let defaults = self.defaults(caller.user_id, None).await?;
        instruments
            .iter()
            .map(|i| {
                let profile = profiles
                    .iter()
                    .find(|p| p.instrument_id() == i.id)
                    .map(|p| view(account_id, *p, i, &defaults))
                    .transpose()?;
                Ok(AccountProfileEntry {
                    instrument_id: i.id,
                    profile,
                })
            })
            .collect()
    }

    /// `getNearestProfileSize` (Q60, Q73 b): 1 candidate, or 2 on an exact tie.
    pub async fn nearest(
        &self,
        caller: &Caller,
        size: &SizeAmount,
        instrument_id: Uuid,
    ) -> Result<Vec<NearestCandidate>, ProfilesError> {
        self.instrument(instrument_id).await?;
        let standard = self.deps.catalog.standard_sizes().await?;
        let defaults = self.defaults(caller.user_id, Some(instrument_id)).await?;
        Ok(profile::nearest_profile_sizes(size, &standard)
            .into_iter()
            .map(|s| NearestCandidate {
                size_amount: s,
                has_default: defaults.iter().any(|d| d.size_amount.value() == s.value()),
            })
            .collect())
    }

    /// `previewProfile`: stateless, no storage and no audit (S6 §2.1).
    pub async fn preview(
        &self,
        instrument_id: Uuid,
        requested: &DistanceProfile,
    ) -> Result<ProfileEquivalents, ProfilesError> {
        let instrument = self.instrument(instrument_id).await?;
        Ok(ProfileEquivalents::convert(requested, &instrument.ticks)?)
    }

    // ---- writes -----------------------------------------------------------------------------

    /// `putDefaultProfile`: idempotent upsert; an unchanged default is neither written nor
    /// audited. Affects accounts created afterwards only (Q-S6-3).
    pub async fn put_default(
        &self,
        caller: &Caller,
        instrument_id: Uuid,
        size: SizeAmount,
        requested: DistanceProfile,
    ) -> Result<DefaultEntry, ProfilesError> {
        let instrument = self.instrument(instrument_id).await?;
        let standard = self.deps.catalog.standard_sizes().await?;
        let Some(size) = standard.into_iter().find(|s| s.value() == size.value()) else {
            return Err(ProfilesError::NotFound);
        };
        ProfileEquivalents::convert(&requested, &instrument.ticks)?;
        for _ in 0..DEFAULT_WRITE_ATTEMPTS {
            let current = self
                .defaults(caller.user_id, Some(instrument_id))
                .await?
                .into_iter()
                .find(|d| d.size_amount.value() == size.value());
            let previous =
                match profile::change_default(current.as_ref().map(|d| &d.profile), &requested) {
                    DefaultChange::Unchanged => {
                        return Ok(DefaultEntry {
                            instrument_id,
                            size_amount: size,
                            profile: current.map(|d| (d.profile, d.updated_at)),
                        });
                    }
                    DefaultChange::Changed { previous } => previous,
                };
            let now = self.deps.clock.now();
            let write = DefaultWrite {
                user_id: caller.user_id,
                expected: previous,
                default: DefaultProfile {
                    instrument_id,
                    size_amount: size,
                    profile: requested,
                    updated_at: now,
                },
                event: ProfileEvent::DefaultChanged {
                    at: now,
                    instrument_id,
                    size_amount: size,
                    previous,
                    new: requested,
                },
            };
            match self.deps.store.put_default(write).await {
                Ok(()) => {
                    return Ok(DefaultEntry {
                        instrument_id,
                        size_amount: size,
                        profile: Some((requested, now)),
                    });
                }
                Err(ProfileStoreError::Raced) => continue,
                Err(e) => return Err(e.into()),
            }
        }
        Err(ProfilesError::Internal(
            "default kept changing concurrently".into(),
        ))
    }

    /// `putAccountProfile`: next entries only (Q-S6-5). 404 → 409 → 422.
    pub async fn put_account_profile(
        &self,
        caller: &Caller,
        account_id: AccountId,
        instrument_id: Uuid,
        requested: DistanceProfile,
        row_version: Option<i32>,
    ) -> Result<AccountProfileView, ProfilesError> {
        let (facts, profiles) = self.account(caller.user_id, account_id).await?;
        let instrument = self.instrument(instrument_id).await?;
        let existing = find(&profiles, instrument_id);
        let now = self.deps.clock.now();
        let write = profile::edit(instrument_id, existing, requested, row_version, now)?;
        self.apply(
            caller.user_id,
            account_id,
            &facts,
            &instrument,
            write,
            ProfileAction::Edited,
        )
        .await
    }

    /// `restoreAccountProfile` (Q-S6-4): the current default of the copied size, or of the
    /// account's copy source when the profile has none.
    pub async fn restore_account_profile(
        &self,
        caller: &Caller,
        account_id: AccountId,
        instrument_id: Uuid,
        profile_size: Option<SizeAmount>,
        row_version: Option<i32>,
    ) -> Result<AccountProfileView, ProfilesError> {
        let (facts, profiles) = self.account(caller.user_id, account_id).await?;
        let instrument = self.instrument(instrument_id).await?;
        let standard = self.deps.catalog.standard_sizes().await?;
        let defaults = self.defaults(caller.user_id, Some(instrument_id)).await?;
        let write = profile::restore(
            instrument_id,
            &facts.size_amount,
            find(&profiles, instrument_id),
            profile_size.as_ref(),
            row_version,
            ProfileSources {
                standard: &standard,
                defaults: &defaults,
            },
            self.deps.clock.now(),
        )?;
        self.apply(
            caller.user_id,
            account_id,
            &facts,
            &instrument,
            write,
            ProfileAction::Restored,
        )
        .await
    }

    // ---- helpers ----------------------------------------------------------------------------

    async fn apply(
        &self,
        user_id: UserId,
        account_id: AccountId,
        facts: &AccountFacts,
        instrument: &EnabledInstrument,
        write: ProfileWrite,
        action: ProfileAction,
    ) -> Result<AccountProfileView, ProfilesError> {
        let defaults = self.defaults(user_id, Some(instrument.id)).await?;
        let (previous, current) = match write {
            ProfileWrite::Unchanged(p) => return view(account_id, p, instrument, &defaults),
            ProfileWrite::Changed { previous, current } => (previous, current),
        };
        // The values must convert for this instrument before anything is stored (422).
        ProfileEquivalents::convert(current.profile(), &instrument.ticks)?;
        let event = ProfileEvent::AccountChanged {
            at: current.updated_at(),
            account_id,
            account_name: facts.display_name.clone(),
            instrument_id: instrument.id,
            action,
            copied_from_size: current.copied_from().map(|c| c.size_amount),
            previous: previous.map(|p| *p.profile()),
            new: *current.profile(),
        };
        self.deps
            .store
            .write_account_profile(AccountProfileWrite {
                user_id,
                account_id,
                expected_row_version: previous.map(|p| p.row_version()),
                profile: current,
                event,
            })
            .await?;
        view(account_id, current, instrument, &defaults)
    }

    async fn instrument(&self, id: Uuid) -> Result<EnabledInstrument, ProfilesError> {
        self.deps
            .catalog
            .enabled_instruments()
            .await?
            .into_iter()
            .find(|i| i.id == id)
            .ok_or(ProfilesError::NotFound)
    }

    async fn account(
        &self,
        user_id: UserId,
        account_id: AccountId,
    ) -> Result<(AccountFacts, Vec<AccountBracketProfile>), ProfilesError> {
        self.deps
            .store
            .account_profiles(user_id, account_id)
            .await?
            .ok_or(ProfilesError::NotFound)
    }

    async fn defaults(
        &self,
        user_id: UserId,
        instrument_id: Option<Uuid>,
    ) -> Result<Vec<DefaultProfile>, ProfilesError> {
        Ok(self.deps.store.defaults(user_id, instrument_id).await?)
    }
}

fn find(profiles: &[AccountBracketProfile], instrument_id: Uuid) -> Option<AccountBracketProfile> {
    profiles
        .iter()
        .find(|p| p.instrument_id() == instrument_id)
        .copied()
}

/// A stored profile that no longer converts (the catalog's tick data changed) cannot be shown
/// with equivalents; execution treats it as missing (`routing.no_bracket_profile`).
fn view(
    account_id: AccountId,
    profile: AccountBracketProfile,
    instrument: &EnabledInstrument,
    defaults: &[DefaultProfile],
) -> Result<AccountProfileView, ProfilesError> {
    let equivalents = ProfileEquivalents::convert(profile.profile(), &instrument.ticks)
        .map_err(|e| ProfilesError::Internal(format!("stored profile no longer converts: {e}")))?;
    Ok(AccountProfileView {
        account_id,
        profile,
        equivalents,
        default_now: profile::default_now(&profile, defaults).copied(),
    })
}
