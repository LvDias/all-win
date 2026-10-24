//! Per-size defaults and the account's own copy (S6 AC-1, AC-2; contract `AccountProfile`,
//! data model §5.4). Edits and restores apply to the next entries only (Q-S6-5).

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::values::SizeAmount;

use super::distance::DistanceProfile;
use super::nearest_size::{CopySourceError, copy_source};

/// Where an account profile's values came from when last copied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileOrigin {
    /// The default of the account's own (standard) size.
    SizeDefault,
    /// The default of the nearest standard size (real account of another size; on a tie the size the
    /// user picked, Q60).
    NearestSizeDefault,
    /// Typed by the user on an account that had no profile (Q43).
    SetByUser,
}

impl ProfileOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SizeDefault => "size_default",
            Self::NearestSizeDefault => "nearest_size_default",
            Self::SetByUser => "set_by_user",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "size_default" => Some(Self::SizeDefault),
            "nearest_size_default" => Some(Self::NearestSizeDefault),
            "set_by_user" => Some(Self::SetByUser),
            _ => None,
        }
    }
}

/// The user's default profile of one standard size for one instrument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultProfile {
    pub instrument_id: Uuid,
    pub size_amount: SizeAmount,
    pub profile: DistanceProfile,
    pub updated_at: DateTime<Utc>,
}

/// Result of `putDefaultProfile` on the current default (if any).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultChange {
    /// Same unit and values: nothing written, nothing audited.
    Unchanged,
    /// New or replaced; `previous` is `None` when the size had no default.
    Changed { previous: Option<DistanceProfile> },
}

pub fn change_default(
    current: Option<&DistanceProfile>,
    requested: &DistanceProfile,
) -> DefaultChange {
    match current {
        Some(current) if current.same_as(requested) => DefaultChange::Unchanged,
        previous => DefaultChange::Changed {
            previous: previous.copied(),
        },
    }
}

/// Provenance of a copy. Absent for `set_by_user`, so the data model's `CHECK`s hold by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CopiedFrom {
    pub size_amount: SizeAmount,
    pub at: DateTime<Utc>,
}

/// The account's own profile for one instrument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccountBracketProfile {
    instrument_id: Uuid,
    profile: DistanceProfile,
    origin: ProfileOrigin,
    copied_from: Option<CopiedFrom>,
    customized: bool,
    row_version: i32,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("stored account profile breaks its provenance rules")]
pub struct InconsistentProfile;

impl AccountBracketProfile {
    /// Rebuilds a stored row, checking the provenance invariants (data model §5.4 `CHECK`s).
    #[allow(clippy::too_many_arguments)]
    pub fn restore_from_storage(
        instrument_id: Uuid,
        profile: DistanceProfile,
        origin: ProfileOrigin,
        copied_from: Option<CopiedFrom>,
        customized: bool,
        row_version: i32,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, InconsistentProfile> {
        let is_copy = origin != ProfileOrigin::SetByUser;
        if is_copy != copied_from.is_some() || (!is_copy && customized) || row_version < 1 {
            return Err(InconsistentProfile);
        }
        Ok(Self {
            instrument_id,
            profile,
            origin,
            copied_from,
            customized,
            row_version,
            updated_at,
        })
    }

    fn copy(
        instrument_id: Uuid,
        default: &DefaultProfile,
        origin: ProfileOrigin,
        row_version: i32,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            instrument_id,
            profile: default.profile,
            origin,
            copied_from: Some(CopiedFrom {
                size_amount: default.size_amount,
                at: now,
            }),
            customized: false,
            row_version,
            updated_at: now,
        }
    }

    pub fn instrument_id(&self) -> Uuid {
        self.instrument_id
    }

    pub fn profile(&self) -> &DistanceProfile {
        &self.profile
    }

    pub fn origin(&self) -> ProfileOrigin {
        self.origin
    }

    pub fn copied_from(&self) -> Option<&CopiedFrom> {
        self.copied_from.as_ref()
    }

    pub fn customized(&self) -> bool {
        self.customized
    }

    pub fn row_version(&self) -> i32 {
        self.row_version
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

/// What a copy reads: the standard sizes and the user's defaults for the instrument.
#[derive(Debug, Clone, Copy)]
pub struct ProfileSources<'a> {
    pub standard: &'a [SizeAmount],
    pub defaults: &'a [DefaultProfile],
}

/// The first copy at account creation (S6 AC-2), in the creating transaction. `defaults` are the
/// user's defaults for this instrument. `Ok(None)`: no source or no default for it, so the account
/// starts without a profile (Q43).
pub fn initial_copy(
    instrument_id: Uuid,
    size: &SizeAmount,
    profile_size: Option<&SizeAmount>,
    sources: ProfileSources<'_>,
    now: DateTime<Utc>,
) -> Result<Option<AccountBracketProfile>, CopySourceError> {
    let Some((source, origin)) = copy_source(size, profile_size, sources.standard)? else {
        return Ok(None);
    };
    Ok(find_default(sources.defaults, instrument_id, &source)
        .map(|default| AccountBracketProfile::copy(instrument_id, default, origin, 1, now)))
}

fn find_default<'a>(
    defaults: &'a [DefaultProfile],
    instrument_id: Uuid,
    size: &SizeAmount,
) -> Option<&'a DefaultProfile> {
    defaults
        .iter()
        .find(|d| d.instrument_id == instrument_id && d.size_amount.value() == size.value())
}

/// Optimistic concurrency failed (`common.version_conflict`, 409).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("version conflict")]
pub struct VersionConflict;

/// Outcome of a write that may be a no-op (same values: 200, not audited, `row_version` unchanged).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileWrite {
    Unchanged(AccountBracketProfile),
    Changed {
        previous: Option<AccountBracketProfile>,
        current: AccountBracketProfile,
    },
}

/// `putAccountProfile`: `row_version` is required when a profile exists and must be absent when it
/// does not. Editing a copy marks it customized and keeps its provenance; on an account without a
/// profile the values become `set_by_user`.
pub fn edit(
    instrument_id: Uuid,
    existing: Option<AccountBracketProfile>,
    requested: DistanceProfile,
    row_version: Option<i32>,
    now: DateTime<Utc>,
) -> Result<ProfileWrite, VersionConflict> {
    match existing {
        None => {
            if row_version.is_some() {
                return Err(VersionConflict);
            }
            Ok(ProfileWrite::Changed {
                previous: None,
                current: AccountBracketProfile {
                    instrument_id,
                    profile: requested,
                    origin: ProfileOrigin::SetByUser,
                    copied_from: None,
                    customized: false,
                    row_version: 1,
                    updated_at: now,
                },
            })
        }
        Some(existing) => {
            if row_version != Some(existing.row_version) {
                return Err(VersionConflict);
            }
            if existing.profile.same_as(&requested) {
                return Ok(ProfileWrite::Unchanged(existing));
            }
            Ok(ProfileWrite::Changed {
                previous: Some(existing),
                current: AccountBracketProfile {
                    profile: requested,
                    customized: existing.origin != ProfileOrigin::SetByUser,
                    row_version: existing.row_version.saturating_add(1),
                    updated_at: now,
                    ..existing
                },
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum RestoreError {
    #[error(transparent)]
    VersionConflict(#[from] VersionConflict),
    /// The source size has no default (`profiles.default_missing`), or the account has no source.
    #[error("default missing")]
    DefaultMissing,
    #[error(transparent)]
    Source(#[from] CopySourceError),
}

/// `restoreAccountProfile` (Q-S6-4): copies the current default of `copied_from_size` (keeping a
/// Q60 tie choice); a profile without one (`set_by_user`, or none) takes the account's copy source.
/// `row_version` is optional; when sent it must match (absent profile: any value conflicts).
pub fn restore(
    instrument_id: Uuid,
    size: &SizeAmount,
    existing: Option<AccountBracketProfile>,
    profile_size: Option<&SizeAmount>,
    row_version: Option<i32>,
    sources: ProfileSources<'_>,
    now: DateTime<Utc>,
) -> Result<ProfileWrite, RestoreError> {
    if let Some(sent) = row_version
        && existing.map(|e| e.row_version) != Some(sent)
    {
        return Err(VersionConflict.into());
    }
    let (source, origin) =
        match existing.and_then(|e| e.copied_from.map(|c| (c.size_amount, e.origin))) {
            Some(kept) => kept,
            None => copy_source(size, profile_size, sources.standard)?
                .ok_or(RestoreError::DefaultMissing)?,
        };
    let default = find_default(sources.defaults, instrument_id, &source)
        .ok_or(RestoreError::DefaultMissing)?;
    match existing {
        Some(e) if e.origin == origin && !e.customized && e.profile.same_as(&default.profile) => {
            Ok(ProfileWrite::Unchanged(e))
        }
        previous => {
            let next_version = previous.map_or(1, |p| p.row_version.saturating_add(1));
            Ok(ProfileWrite::Changed {
                previous,
                current: AccountBracketProfile::copy(
                    instrument_id,
                    default,
                    origin,
                    next_version,
                    now,
                ),
            })
        }
    }
}

/// `default_now` of an account profile: the current default of `copied_from_size`, if any.
pub fn default_now<'a>(
    profile: &AccountBracketProfile,
    defaults: &'a [DefaultProfile],
) -> Option<&'a DistanceProfile> {
    let from = profile.copied_from.as_ref()?;
    find_default(defaults, profile.instrument_id, &from.size_amount).map(|d| &d.profile)
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;
    use crate::profile::distance::DistanceUnit;
    use crate::profile::test_support::{size, standard};

    fn instrument() -> Uuid {
        Uuid::from_u128(0x01a1_66e7_a8d0_700f_bade_6d84_9313_2056)
    }

    fn t(sec: i64) -> DateTime<Utc> {
        Utc.timestamp_opt(1_800_000_000 + sec, 0).unwrap()
    }

    fn ticks(stop: &str, target: &str) -> DistanceProfile {
        DistanceProfile::parse(DistanceUnit::Ticks, stop, target).unwrap()
    }

    fn default(amount: &str, profile: DistanceProfile) -> DefaultProfile {
        DefaultProfile {
            instrument_id: instrument(),
            size_amount: size(amount),
            profile,
            updated_at: t(0),
        }
    }

    fn defaults() -> Vec<DefaultProfile> {
        vec![
            default("50000", ticks("40", "80")),
            default("100000", ticks("60", "120")),
        ]
    }

    fn src<'a>(defaults: &'a [DefaultProfile]) -> ProfileSources<'a> {
        static STANDARD: std::sync::LazyLock<Vec<SizeAmount>> = std::sync::LazyLock::new(standard);
        ProfileSources {
            standard: &STANDARD,
            defaults,
        }
    }

    fn copied(amount: &str, pick: Option<&str>) -> Option<AccountBracketProfile> {
        let pick = pick.map(size);
        initial_copy(
            instrument(),
            &size(amount),
            pick.as_ref(),
            src(&defaults()),
            t(1),
        )
        .unwrap()
    }

    #[test]
    fn default_change_is_numeric_and_unit_sensitive() {
        let usd = DistanceProfile::parse(DistanceUnit::Usd, "100.1", "200").unwrap();
        let same = DistanceProfile::parse(DistanceUnit::Usd, "100.10", "200.00").unwrap();
        assert_eq!(change_default(Some(&usd), &same), DefaultChange::Unchanged);
        assert_eq!(
            change_default(None, &usd),
            DefaultChange::Changed { previous: None }
        );
        let pts = DistanceProfile::parse(DistanceUnit::Points, "100.1", "200").unwrap();
        assert_eq!(
            change_default(Some(&usd), &pts),
            DefaultChange::Changed {
                previous: Some(usd)
            }
        );
    }

    #[test]
    fn initial_copy_of_a_standard_size() {
        let p = copied("50000", None).unwrap();
        assert_eq!(p.origin(), ProfileOrigin::SizeDefault);
        assert_eq!(
            p.copied_from(),
            Some(&CopiedFrom {
                size_amount: size("50000"),
                at: t(1)
            })
        );
        assert!(!p.customized());
        assert_eq!(p.row_version(), 1);
        assert!(p.profile().same_as(&ticks("40", "80")));
    }

    #[test]
    fn initial_copy_of_a_real_tie_keeps_the_choice() {
        let p = copied("75000", Some("100000")).unwrap();
        assert_eq!(p.origin(), ProfileOrigin::NearestSizeDefault);
        assert_eq!(p.copied_from().map(|c| c.size_amount), Some(size("100000")));
        let err = initial_copy(instrument(), &size("75000"), None, src(&defaults()), t(1));
        assert_eq!(err, Err(CopySourceError::ProfileSizeRequired));
    }

    #[test]
    fn no_default_or_no_source_means_no_profile() {
        assert_eq!(copied("25000", None), None);
        // Nearest is 300000, which has no default.
        assert_eq!(copied("280000", None), None);
        // Q73 b: any non-standard size (real or prop) takes the nearest standard size.
        let p = copied("60000", None).unwrap();
        assert_eq!(p.origin(), ProfileOrigin::NearestSizeDefault);
        assert_eq!(p.copied_from().map(|c| c.size_amount), Some(size("50000")));
        // The tie is still validated when the chosen size has no default.
        let err = initial_copy(instrument(), &size("37500"), None, src(&defaults()), t(1));
        assert_eq!(err, Err(CopySourceError::ProfileSizeRequired));
    }

    #[test]
    fn edit_a_copy_marks_it_customized() {
        let p = copied("50000", None).unwrap();
        assert_eq!(
            edit(instrument(), Some(p), ticks("30", "90"), None, t(2)),
            Err(VersionConflict)
        );
        assert_eq!(
            edit(instrument(), Some(p), ticks("30", "90"), Some(2), t(2)),
            Err(VersionConflict)
        );
        let ProfileWrite::Changed { previous, current } =
            edit(instrument(), Some(p), ticks("30", "90"), Some(1), t(2)).unwrap()
        else {
            panic!("expected a change")
        };
        assert_eq!(previous, Some(p));
        assert!(current.customized());
        assert_eq!(current.origin(), ProfileOrigin::SizeDefault);
        assert_eq!(current.copied_from(), p.copied_from());
        assert_eq!((current.row_version(), current.updated_at()), (2, t(2)));
    }

    #[test]
    fn edit_with_same_values_is_a_no_op() {
        let p = copied("50000", None).unwrap();
        assert_eq!(
            edit(instrument(), Some(p), ticks("40", "80"), Some(1), t(2)),
            Ok(ProfileWrite::Unchanged(p))
        );
        // A stale version still conflicts first.
        assert_eq!(
            edit(instrument(), Some(p), ticks("40", "80"), Some(7), t(2)),
            Err(VersionConflict)
        );
    }

    #[test]
    fn edit_without_a_profile_is_set_by_user() {
        assert_eq!(
            edit(instrument(), None, ticks("4", "8"), Some(1), t(2)),
            Err(VersionConflict)
        );
        let ProfileWrite::Changed {
            previous: None,
            current,
        } = edit(instrument(), None, ticks("4", "8"), None, t(2)).unwrap()
        else {
            panic!("expected a first profile")
        };
        assert_eq!(current.origin(), ProfileOrigin::SetByUser);
        assert_eq!(
            (
                current.copied_from(),
                current.customized(),
                current.row_version()
            ),
            (None, false, 1)
        );
        // Editing it again keeps it set_by_user and never customized.
        let ProfileWrite::Changed { current: again, .. } =
            edit(instrument(), Some(current), ticks("5", "8"), Some(1), t(3)).unwrap()
        else {
            panic!("expected a change")
        };
        assert_eq!(
            (again.origin(), again.customized(), again.row_version()),
            (ProfileOrigin::SetByUser, false, 2)
        );
    }

    #[test]
    fn restore_copies_the_current_default_of_the_copied_size() {
        let p = copied("75000", Some("100000")).unwrap();
        let ProfileWrite::Changed {
            current: edited, ..
        } = edit(instrument(), Some(p), ticks("1", "2"), Some(1), t(2)).unwrap()
        else {
            panic!()
        };
        let mut now_defaults = defaults();
        now_defaults[1].profile = ticks("70", "140");
        let acc = size("75000");
        // The kept tie choice needs no profile_size.
        let ProfileWrite::Changed { previous, current } = restore(
            instrument(),
            &acc,
            Some(edited),
            None,
            Some(2),
            src(&now_defaults),
            t(3),
        )
        .unwrap() else {
            panic!()
        };
        assert_eq!(previous, Some(edited));
        assert!(current.profile().same_as(&ticks("70", "140")));
        assert_eq!(current.origin(), ProfileOrigin::NearestSizeDefault);
        assert_eq!(
            current.copied_from(),
            Some(&CopiedFrom {
                size_amount: size("100000"),
                at: t(3)
            })
        );
        assert_eq!((current.customized(), current.row_version()), (false, 3));
        // Restoring again is a no-op.
        assert_eq!(
            restore(
                instrument(),
                &acc,
                Some(current),
                None,
                None,
                src(&now_defaults),
                t(4)
            ),
            Ok(ProfileWrite::Unchanged(current))
        );
    }

    #[test]
    fn restore_conflicts_and_missing_defaults() {
        let acc = size("50000");
        let p = copied("50000", None).unwrap();
        assert_eq!(
            restore(
                instrument(),
                &acc,
                Some(p),
                None,
                Some(9),
                src(&defaults()),
                t(2)
            ),
            Err(RestoreError::VersionConflict(VersionConflict))
        );
        assert_eq!(
            restore(
                instrument(),
                &acc,
                None,
                None,
                Some(1),
                src(&defaults()),
                t(2)
            ),
            Err(RestoreError::VersionConflict(VersionConflict))
        );
        let none: Vec<DefaultProfile> = Vec::new();
        assert_eq!(
            restore(instrument(), &acc, Some(p), None, None, src(&none), t(2)),
            Err(RestoreError::DefaultMissing)
        );
        let no_default = size("280000");
        assert_eq!(
            restore(
                instrument(),
                &no_default,
                None,
                None,
                None,
                src(&defaults()),
                t(2)
            ),
            Err(RestoreError::DefaultMissing)
        );
    }

    #[test]
    fn restore_of_set_by_user_or_absent_takes_the_account_source() {
        let acc = size("75000");
        let ProfileWrite::Changed { current: set, .. } =
            edit(instrument(), None, ticks("4", "8"), None, t(1)).unwrap()
        else {
            panic!()
        };
        assert_eq!(
            restore(
                instrument(),
                &acc,
                Some(set),
                None,
                None,
                src(&defaults()),
                t(2)
            ),
            Err(RestoreError::Source(CopySourceError::ProfileSizeRequired))
        );
        assert_eq!(
            restore(
                instrument(),
                &acc,
                Some(set),
                Some(&size("25000")),
                None,
                src(&defaults()),
                t(2)
            ),
            Err(RestoreError::Source(CopySourceError::ProfileSizeInvalid))
        );
        let ProfileWrite::Changed { current, .. } = restore(
            instrument(),
            &acc,
            Some(set),
            Some(&size("50000")),
            Some(1),
            src(&defaults()),
            t(2),
        )
        .unwrap() else {
            panic!()
        };
        assert_eq!(
            (current.origin(), current.row_version()),
            (ProfileOrigin::NearestSizeDefault, 2)
        );
        // An account without a profile gets its first copy.
        let ProfileWrite::Changed {
            previous: None,
            current,
        } = restore(
            instrument(),
            &size("100000"),
            None,
            None,
            None,
            src(&defaults()),
            t(2),
        )
        .unwrap()
        else {
            panic!()
        };
        assert_eq!(
            (current.origin(), current.row_version()),
            (ProfileOrigin::SizeDefault, 1)
        );
    }

    #[test]
    fn storage_invariants() {
        let p = ticks("4", "8");
        let from = Some(CopiedFrom {
            size_amount: size("50000"),
            at: t(0),
        });
        let ok = |origin, from, customized, v| {
            AccountBracketProfile::restore_from_storage(
                instrument(),
                p,
                origin,
                from,
                customized,
                v,
                t(0),
            )
            .is_ok()
        };
        assert!(ok(ProfileOrigin::SizeDefault, from, true, 3));
        assert!(ok(ProfileOrigin::SetByUser, None, false, 1));
        assert!(!ok(ProfileOrigin::SetByUser, from, false, 1));
        assert!(!ok(ProfileOrigin::SetByUser, None, true, 1));
        assert!(!ok(ProfileOrigin::NearestSizeDefault, None, false, 1));
        assert!(!ok(ProfileOrigin::SizeDefault, from, false, 0));
    }

    #[test]
    fn default_now_follows_the_copied_size() {
        let p = copied("75000", Some("50000")).unwrap();
        assert_eq!(default_now(&p, &defaults()), Some(&ticks("40", "80")));
        assert_eq!(default_now(&p, &[]), None);
    }
}
