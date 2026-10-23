//! Profile use cases against in-memory fakes of the store and the catalog.

use std::str::FromStr;
use std::sync::{Arc, Mutex};

use access::Caller;
use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use accounts_enterprise::profile::{
    AccountBracketProfile, DefaultProfile, DistanceProfile, DistanceRule, DistanceUnit,
    ProfileOrigin, ProfileSide, TickSpec,
};
use accounts_enterprise::{AccountId, AccountKind, AccountName, SizeAmount};
use shared_kernel::UserId;
use shared_kernel::clock::FixedClock;

use super::*;
use crate::ports::{InitialProfileError, InitialProfilePlanner, PortError};
use ports::{
    AccountFacts, AccountProfileWrite, DefaultWrite, EnabledInstrument, LiveAccountProfiles,
    ProfileCatalog, ProfileStore, ProfileStoreError,
};

const MNQ: Uuid = Uuid::from_u128(0x01a1_66e7_a8d0_700f_bade_6d84_9313_2056);

fn d(s: &str) -> Decimal {
    Decimal::from_str(s).unwrap()
}

fn size(s: &str) -> SizeAmount {
    SizeAmount::new(d(s)).unwrap()
}

fn now() -> DateTime<Utc> {
    Utc.timestamp_opt(1_800_000_000, 0).unwrap()
}

fn caller(user: UserId) -> Caller {
    Caller {
        user_id: user,
        session_id: Uuid::nil(),
        device_id: Uuid::nil(),
        is_platform_admin: false,
    }
}

fn ticks(stop: &str, target: &str) -> DistanceProfile {
    DistanceProfile::parse(DistanceUnit::Ticks, stop, target).unwrap()
}

fn usd(stop: &str, target: &str) -> DistanceProfile {
    DistanceProfile::parse(DistanceUnit::Usd, stop, target).unwrap()
}

struct Catalog;

#[async_trait]
impl ProfileCatalog for Catalog {
    async fn enabled_instruments(&self) -> Result<Vec<EnabledInstrument>, PortError> {
        Ok(vec![EnabledInstrument {
            id: MNQ,
            ticks: TickSpec::new(d("0.25"), d("2")).unwrap(),
        }])
    }

    async fn standard_sizes(&self) -> Result<Vec<SizeAmount>, PortError> {
        // Deliberately unsorted.
        Ok(["300000", "25000", "50000", "100000", "150000", "200000"]
            .map(size)
            .to_vec())
    }
}

#[derive(Default)]
struct State {
    defaults: Vec<(UserId, DefaultProfile)>,
    accounts: Vec<(UserId, AccountId, AccountFacts)>,
    profiles: Vec<(UserId, AccountId, AccountBracketProfile)>,
    events: Vec<ProfileEvent>,
}

#[derive(Default)]
struct Store(Mutex<State>);

impl Store {
    fn events(&self) -> Vec<ProfileEvent> {
        self.0.lock().unwrap().events.clone()
    }

    fn add_account(&self, user: UserId, kind: AccountKind, amount: &str) -> AccountId {
        let id = AccountId::new_v7();
        let facts = AccountFacts {
            kind,
            size_amount: size(amount),
            display_name: AccountName::parse("Conta").unwrap(),
        };
        self.0.lock().unwrap().accounts.push((user, id, facts));
        id
    }

    fn add_default(&self, user: UserId, amount: &str, profile: DistanceProfile) {
        self.0.lock().unwrap().defaults.push((
            user,
            DefaultProfile {
                instrument_id: MNQ,
                size_amount: size(amount),
                profile,
                updated_at: now(),
            },
        ));
    }
}

#[async_trait]
impl ProfileStore for Store {
    async fn defaults(
        &self,
        user_id: UserId,
        instrument_id: Option<Uuid>,
    ) -> Result<Vec<DefaultProfile>, ProfileStoreError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .defaults
            .iter()
            .filter(|(u, d)| *u == user_id && instrument_id.is_none_or(|i| i == d.instrument_id))
            .map(|(_, d)| *d)
            .collect())
    }

    async fn put_default(&self, w: DefaultWrite) -> Result<(), ProfileStoreError> {
        let mut s = self.0.lock().unwrap();
        let pos = s.defaults.iter().position(|(u, d)| {
            *u == w.user_id
                && d.instrument_id == w.default.instrument_id
                && d.size_amount == w.default.size_amount
        });
        match (pos, w.expected) {
            (None, None) => s.defaults.push((w.user_id, w.default)),
            (Some(i), Some(e)) if s.defaults[i].1.profile.same_as(&e) => {
                s.defaults[i].1 = w.default
            }
            _ => return Err(ProfileStoreError::Raced),
        }
        s.events.push(w.event);
        Ok(())
    }

    async fn account_profiles(
        &self,
        user_id: UserId,
        account_id: AccountId,
    ) -> Result<Option<(AccountFacts, Vec<AccountBracketProfile>)>, ProfileStoreError> {
        let s = self.0.lock().unwrap();
        let Some((_, _, facts)) = s
            .accounts
            .iter()
            .find(|(u, a, _)| *u == user_id && *a == account_id)
        else {
            return Ok(None);
        };
        let profiles = s
            .profiles
            .iter()
            .filter(|(u, a, _)| *u == user_id && *a == account_id)
            .map(|(_, _, p)| *p)
            .collect();
        Ok(Some((facts.clone(), profiles)))
    }

    async fn write_account_profile(&self, w: AccountProfileWrite) -> Result<(), ProfileStoreError> {
        let mut s = self.0.lock().unwrap();
        if !s
            .accounts
            .iter()
            .any(|(u, a, _)| *u == w.user_id && *a == w.account_id)
        {
            return Err(ProfileStoreError::AccountNotFound);
        }
        let pos = s.profiles.iter().position(|(_, a, p)| {
            *a == w.account_id && p.instrument_id() == w.profile.instrument_id()
        });
        match (pos, w.expected_row_version) {
            (None, None) => s.profiles.push((w.user_id, w.account_id, w.profile)),
            (Some(i), Some(v)) if s.profiles[i].2.row_version() == v => s.profiles[i].2 = w.profile,
            _ => return Err(ProfileStoreError::Raced),
        }
        s.events.push(w.event);
        Ok(())
    }

    async fn live_profiles(
        &self,
        user_id: UserId,
        account_ids: &[AccountId],
        instrument_ids: &[Uuid],
    ) -> Result<Vec<LiveAccountProfiles>, ProfileStoreError> {
        let s = self.0.lock().unwrap();
        Ok(s.accounts
            .iter()
            .filter(|(u, a, _)| *u == user_id && account_ids.contains(a))
            .map(|(_, a, _)| LiveAccountProfiles {
                account_id: *a,
                profiles: s
                    .profiles
                    .iter()
                    .filter(|(_, pa, p)| pa == a && instrument_ids.contains(&p.instrument_id()))
                    .map(|(_, _, p)| *p)
                    .collect(),
            })
            .collect())
    }
}

struct Env {
    store: Arc<Store>,
    uc: ProfileUseCases,
    me: UserId,
    other: UserId,
}

fn env() -> Env {
    let store = Arc::new(Store::default());
    let deps = ProfileDeps {
        store: store.clone(),
        catalog: Arc::new(Catalog),
        clock: Arc::new(FixedClock(now())),
    };
    Env {
        store,
        uc: ProfileUseCases::new(deps),
        me: UserId::new_v7(),
        other: UserId::new_v7(),
    }
}

impl Env {
    fn planner(&self) -> ProfilePlanner {
        ProfilePlanner::new(self.store.clone(), Arc::new(Catalog))
    }

    fn brackets(&self) -> BracketQuery {
        BracketQuery::new(self.store.clone(), Arc::new(Catalog))
    }

    /// An account created through the planner, as the S5 create does.
    async fn account_with_copy(
        &self,
        kind: AccountKind,
        amount: &str,
        pick: Option<&str>,
    ) -> AccountId {
        let id = self.store.add_account(self.me, kind, amount);
        let pick = pick.map(size);
        let copies = self
            .planner()
            .plan(self.me, kind, &size(amount), pick.as_ref(), now())
            .await
            .unwrap();
        let mut s = self.store.0.lock().unwrap();
        for p in copies {
            s.profiles.push((self.me, id, p));
        }
        id
    }
}

// ---- defaults -----------------------------------------------------------------------------

#[tokio::test]
async fn defaults_list_every_standard_size_in_order() {
    let e = env();
    e.store.add_default(e.me, "50000", ticks("40", "80"));
    e.store.add_default(e.other, "100000", ticks("1", "2"));
    let list = e.uc.list_defaults(&caller(e.me), MNQ).await.unwrap();
    let sizes: Vec<String> = list.iter().map(|x| x.size_amount.to_wire()).collect();
    assert_eq!(
        sizes,
        [
            "25000.00",
            "50000.00",
            "100000.00",
            "150000.00",
            "200000.00",
            "300000.00"
        ]
    );
    let defined: Vec<bool> = list.iter().map(|x| x.profile.is_some()).collect();
    assert_eq!(
        defined,
        [false, true, false, false, false, false],
        "other user's default leaks"
    );
    assert_eq!(
        e.uc.list_defaults(&caller(e.me), Uuid::nil()).await,
        Err(ProfilesError::NotFound)
    );
}

#[tokio::test]
async fn put_default_validates_then_upserts_and_audits_changes_only() {
    let e = env();
    let me = caller(e.me);
    assert_eq!(
        e.uc.put_default(&me, MNQ, size("75000"), ticks("4", "8"))
            .await,
        Err(ProfilesError::NotFound),
        "not a standard size"
    );
    assert_eq!(
        e.uc.put_default(&me, Uuid::nil(), size("50000"), ticks("4", "8"))
            .await,
        Err(ProfilesError::NotFound)
    );
    let err =
        e.uc.put_default(&me, MNQ, size("50000"), usd("0.49", "10"))
            .await
            .unwrap_err();
    assert_eq!(
        (err.code(), err.field()),
        ("profiles.distance_below_one_tick", Some("stop_distance"))
    );
    assert!(e.store.events().is_empty());

    let saved =
        e.uc.put_default(&me, MNQ, size("50000.00"), usd("100.1", "200"))
            .await
            .unwrap();
    assert_eq!(saved.profile, Some((usd("100.10", "200.00"), now())));
    // The same values again: no write, no audit.
    e.uc.put_default(&me, MNQ, size("50000"), usd("100.10", "200.00"))
        .await
        .unwrap();
    assert_eq!(e.store.events().len(), 1);
    // A different unit is a change even with the same ticks.
    e.uc.put_default(&me, MNQ, size("50000"), ticks("200", "400"))
        .await
        .unwrap();
    let events = e.store.events();
    assert_eq!(events.len(), 2);
    let ProfileEvent::DefaultChanged { previous, new, .. } = &events[1] else {
        panic!()
    };
    assert_eq!(
        (previous, new),
        (&Some(usd("100.1", "200")), &ticks("200", "400"))
    );
}

// ---- initial copy (planner) ---------------------------------------------------------------

#[tokio::test]
async fn planner_copies_the_nearest_default_for_real_and_prop() {
    let e = env();
    e.store.add_default(e.me, "50000", ticks("40", "80"));
    e.store.add_default(e.me, "100000", ticks("60", "120"));
    let planner = e.planner();
    for kind in [AccountKind::Real, AccountKind::Prop] {
        // Q73 b: a prop size outside the standard list takes the nearest standard size too.
        let copies = planner
            .plan(e.me, kind, &size("60000"), None, now())
            .await
            .unwrap();
        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0].origin(), ProfileOrigin::NearestSizeDefault);
        assert_eq!(
            copies[0].copied_from().map(|c| c.size_amount),
            Some(size("50000"))
        );
        let exact = planner
            .plan(e.me, kind, &size("100000"), None, now())
            .await
            .unwrap();
        assert_eq!(exact[0].origin(), ProfileOrigin::SizeDefault);
    }
    // No default for the source size: no profile (Q43).
    assert!(
        planner
            .plan(e.me, AccountKind::Real, &size("300000"), None, now())
            .await
            .unwrap()
            .is_empty()
    );
    // Another user's defaults are never copied.
    assert!(
        planner
            .plan(e.other, AccountKind::Real, &size("50000"), None, now())
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn planner_tie_needs_the_users_choice_even_without_defaults() {
    let e = env();
    let planner = e.planner();
    for kind in [AccountKind::Real, AccountKind::Prop] {
        assert_eq!(
            planner.plan(e.me, kind, &size("75000"), None, now()).await,
            Err(InitialProfileError::ProfileSizeRequired)
        );
        assert_eq!(
            planner
                .plan(e.me, kind, &size("75000"), Some(&size("25000")), now())
                .await,
            Err(InitialProfileError::ProfileSizeInvalid)
        );
        assert_eq!(
            planner
                .plan(e.me, kind, &size("75000"), Some(&size("100000")), now())
                .await,
            Ok(vec![])
        );
    }
}

// ---- account profiles ---------------------------------------------------------------------

#[tokio::test]
async fn account_profiles_are_isolated_per_user() {
    let e = env();
    e.store.add_default(e.me, "50000", ticks("40", "80"));
    let id = e.account_with_copy(AccountKind::Real, "50000", None).await;
    let them = caller(e.other);
    assert_eq!(
        e.uc.list_account_profiles(&them, id).await,
        Err(ProfilesError::NotFound)
    );
    assert_eq!(
        e.uc.put_account_profile(&them, id, MNQ, ticks("4", "8"), Some(1))
            .await,
        Err(ProfilesError::NotFound)
    );
    assert_eq!(
        e.uc.restore_account_profile(&them, id, MNQ, None, None)
            .await,
        Err(ProfilesError::NotFound)
    );
    assert!(e.store.events().is_empty());
}

#[tokio::test]
async fn list_account_profiles_shows_equivalents_and_default_now() {
    let e = env();
    e.store.add_default(e.me, "50000", usd("100.10", "200"));
    let id = e.account_with_copy(AccountKind::Real, "60000", None).await;
    let list = e.uc.list_account_profiles(&caller(e.me), id).await.unwrap();
    assert_eq!(list.len(), 1);
    let v = list[0].profile.as_ref().unwrap();
    assert_eq!(
        (v.equivalents.stop.ticks, v.equivalents.stop.rounded),
        (200, true)
    );
    assert_eq!(v.equivalents.stop.usd.to_string(), "100.00");
    assert_eq!(v.default_now, Some(usd("100.1", "200")));

    let bare = e.store.add_account(e.me, AccountKind::Real, "300000");
    let list =
        e.uc.list_account_profiles(&caller(e.me), bare)
            .await
            .unwrap();
    assert_eq!(
        (list[0].instrument_id, list[0].profile.is_none()),
        (MNQ, true)
    );
}

#[tokio::test]
async fn put_account_profile_precedence_and_audit() {
    let e = env();
    e.store.add_default(e.me, "50000", ticks("40", "80"));
    let id = e.account_with_copy(AccountKind::Real, "50000", None).await;
    let me = caller(e.me);
    assert_eq!(
        e.uc.put_account_profile(&me, id, Uuid::nil(), ticks("4", "8"), Some(1))
            .await,
        Err(ProfilesError::NotFound)
    );
    // 409 before 422.
    assert_eq!(
        e.uc.put_account_profile(&me, id, MNQ, ticks("0", "8"), Some(9))
            .await,
        Err(ProfilesError::VersionConflict)
    );
    assert_eq!(
        e.uc.put_account_profile(&me, id, MNQ, ticks("4", "8"), None)
            .await,
        Err(ProfilesError::VersionConflict)
    );
    let err =
        e.uc.put_account_profile(&me, id, MNQ, ticks("4", "10001"), Some(1))
            .await
            .unwrap_err();
    assert_eq!(
        (err.code(), err.field()),
        ("profiles.distance_too_large", Some("target_distance"))
    );
    // Same values: 200, not audited, row_version unchanged.
    let same =
        e.uc.put_account_profile(&me, id, MNQ, ticks("40", "80"), Some(1))
            .await
            .unwrap();
    assert_eq!(same.profile.row_version(), 1);
    assert!(e.store.events().is_empty());

    let v =
        e.uc.put_account_profile(&me, id, MNQ, ticks("30", "90"), Some(1))
            .await
            .unwrap();
    assert!(v.profile.customized());
    assert_eq!(v.profile.row_version(), 2);
    assert_eq!(v.default_now, Some(ticks("40", "80")));
    let events = e.store.events();
    let ProfileEvent::AccountChanged {
        action,
        copied_from_size,
        previous,
        new,
        ..
    } = &events[0]
    else {
        panic!()
    };
    assert_eq!(*action, ProfileAction::Edited);
    assert_eq!(*copied_from_size, Some(size("50000")));
    assert_eq!(
        (previous, new),
        (&Some(ticks("40", "80")), &ticks("30", "90"))
    );
    let p = events[0].payload();
    assert_eq!(p["details"]["new"]["stop_distance"], "30");
    assert_eq!(p["refs"]["account_name"], "Conta");
}

#[tokio::test]
async fn account_without_profile_gets_set_by_user() {
    let e = env();
    let id = e.store.add_account(e.me, AccountKind::Prop, "60000");
    let me = caller(e.me);
    assert_eq!(
        e.uc.put_account_profile(&me, id, MNQ, ticks("4", "8"), Some(1))
            .await,
        Err(ProfilesError::VersionConflict)
    );
    let v =
        e.uc.put_account_profile(&me, id, MNQ, usd("10", "20"), None)
            .await
            .unwrap();
    assert_eq!(
        (v.profile.origin(), v.profile.row_version(), v.default_now),
        (ProfileOrigin::SetByUser, 1, None)
    );
    assert_eq!(v.profile.profile().text(ProfileSide::Stop), "10.00");
}

#[tokio::test]
async fn restore_copies_the_current_default() {
    let e = env();
    e.store.add_default(e.me, "100000", ticks("60", "120"));
    let id = e
        .account_with_copy(AccountKind::Real, "75000", Some("100000"))
        .await;
    let me = caller(e.me);
    e.uc.put_account_profile(&me, id, MNQ, ticks("1", "2"), Some(1))
        .await
        .unwrap();
    e.uc.put_default(&me, MNQ, size("100000"), ticks("70", "140"))
        .await
        .unwrap();
    assert_eq!(
        e.uc.restore_account_profile(&me, id, MNQ, None, Some(1))
            .await,
        Err(ProfilesError::VersionConflict)
    );
    let v =
        e.uc.restore_account_profile(&me, id, MNQ, None, Some(2))
            .await
            .unwrap();
    assert_eq!(v.profile.profile(), &ticks("70", "140"));
    assert_eq!(
        (v.profile.customized(), v.profile.row_version()),
        (false, 3)
    );
    assert_eq!(
        v.profile.copied_from().map(|c| c.size_amount),
        Some(size("100000"))
    );
    let n = e.store.events().len();
    // Already equal and not customized: no-op.
    e.uc.restore_account_profile(&me, id, MNQ, None, None)
        .await
        .unwrap();
    assert_eq!(e.store.events().len(), n);
    let ProfileEvent::AccountChanged { action, .. } = &e.store.events()[n - 1] else {
        panic!()
    };
    assert_eq!(*action, ProfileAction::Restored);
}

#[tokio::test]
async fn restore_without_source_or_default() {
    let e = env();
    let me = caller(e.me);
    let tie = e.store.add_account(e.me, AccountKind::Prop, "75000");
    assert_eq!(
        e.uc.restore_account_profile(&me, tie, MNQ, None, None)
            .await,
        Err(ProfilesError::ProfileSizeRequired)
    );
    let err =
        e.uc.restore_account_profile(&me, tie, MNQ, Some(size("25000")), None)
            .await
            .unwrap_err();
    assert_eq!(
        (err.code(), err.field()),
        ("profiles.profile_size_invalid", Some("profile_size"))
    );
    assert_eq!(
        e.uc.restore_account_profile(&me, tie, MNQ, Some(size("50000")), None)
            .await,
        Err(ProfilesError::DefaultMissing)
    );
    e.store.add_default(e.me, "50000", ticks("40", "80"));
    let v =
        e.uc.restore_account_profile(&me, tie, MNQ, Some(size("50000")), None)
            .await
            .unwrap();
    assert_eq!(
        (v.profile.origin(), v.profile.row_version()),
        (ProfileOrigin::NearestSizeDefault, 1)
    );
}

// ---- nearest and preview ------------------------------------------------------------------

#[tokio::test]
async fn nearest_reports_candidates_with_has_default() {
    let e = env();
    e.store.add_default(e.me, "100000", ticks("60", "120"));
    let c =
        e.uc.nearest(&caller(e.me), &size("75000"), MNQ)
            .await
            .unwrap();
    let got: Vec<(String, bool)> = c
        .iter()
        .map(|c| (c.size_amount.to_wire(), c.has_default))
        .collect();
    assert_eq!(
        got,
        [("50000.00".into(), false), ("100000.00".into(), true)]
    );
    let other =
        e.uc.nearest(&caller(e.other), &size("75000"), MNQ)
            .await
            .unwrap();
    assert!(other.iter().all(|c| !c.has_default));
    assert_eq!(
        e.uc.nearest(&caller(e.me), &size("75000"), Uuid::nil())
            .await,
        Err(ProfilesError::NotFound)
    );
}

#[tokio::test]
async fn preview_converts_without_storing() {
    let e = env();
    let eq = e.uc.preview(MNQ, &usd("100.10", "200")).await.unwrap();
    assert_eq!(
        (
            eq.stop.ticks,
            eq.stop.rounded,
            eq.target.ticks,
            eq.target.rounded
        ),
        (200, true, 400, false)
    );
    let err = e.uc.preview(MNQ, &ticks("0", "1")).await.unwrap_err();
    assert_eq!(
        err,
        ProfilesError::Distance(accounts_enterprise::profile::DistanceRejected {
            side: ProfileSide::Stop,
            rule: DistanceRule::BelowOneTick,
        })
    );
    assert_eq!(
        e.uc.preview(Uuid::nil(), &ticks("1", "1")).await,
        Err(ProfilesError::NotFound)
    );
    assert!(e.store.events().is_empty());
}

// ---- execution query ----------------------------------------------------------------------

#[tokio::test]
async fn brackets_for_and_missing_brackets() {
    let e = env();
    e.store.add_default(e.me, "50000", usd("100.10", "200"));
    let with = e.account_with_copy(AccountKind::Real, "50000", None).await;
    let without = e.store.add_account(e.me, AccountKind::Real, "300000");
    let theirs = e.store.add_account(e.other, AccountKind::Real, "50000");
    let ids = [with, without, theirs];
    let q = e.brackets();

    let got = q.brackets_for(e.me, MNQ, &ids).await.unwrap();
    assert_eq!(got.keys().collect::<Vec<_>>(), [&with]);
    let b = got[&with];
    assert_eq!(
        (b.equivalents.stop.ticks, b.equivalents.stop.rounded),
        (200, true)
    );
    assert_eq!(b.requested, usd("100.1", "200"));

    let missing = q
        .missing_brackets(e.me, &ids, &[MNQ, Uuid::nil()])
        .await
        .unwrap();
    assert_eq!(missing.len(), 1, "other users' accounts never appear");
    assert_eq!(missing[&without], [MNQ]);
    assert!(
        q.brackets_for(e.me, Uuid::nil(), &ids)
            .await
            .unwrap()
            .is_empty()
    );
}
