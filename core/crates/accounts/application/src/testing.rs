//! In-memory fakes of the ports owned by other modules (feature `testing`; used by this crate's
//! tests and by `accounts-infra`'s integration tests).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::type_complexity)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use access::{
    AccessError, Activity, Caller, RequestOrigin, SessionAuthenticator, StepUpAction, StepUpGate,
    StepUpGrant, StepUpProof,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use rust_decimal::Decimal;
use shared_kernel::UserId;
use uuid::Uuid;

use accounts_enterprise::profile::AccountBracketProfile;
use accounts_enterprise::*;

use crate::ports::*;

pub struct GenericRules;

#[async_trait]
impl RuleVersions for GenericRules {
    async fn current(
        &self,
        selection: &RuleSelection,
        kind: OptInKind,
    ) -> Result<Option<CurrentRule>, PortError> {
        Ok(match selection {
            RuleSelection::Generic => Some(CurrentRule {
                rule: RuleVersionRef {
                    id: match kind {
                        OptInKind::Automation => GENERIC_AUTOMATION_RULE_VERSION_ID,
                        OptInKind::Copy => GENERIC_COPY_RULE_VERSION_ID,
                    },
                    kind,
                    stance: FirmStance::Unknown,
                },
                sources: vec![],
            }),
            RuleSelection::Preset { .. } => None,
        })
    }
}

pub struct NoPresets;

#[async_trait]
impl PresetCatalog for NoPresets {
    async fn version(&self, _: PresetVersionId) -> Result<Option<PresetFacts>, PortError> {
        Ok(None)
    }
}

#[derive(Default, Clone)]
pub struct Venues {
    /// (user, connection, platform account) → facts.
    pub known: Arc<Mutex<HashMap<(UserId, ConnectionId, String), VenueAccountFacts>>>,
    pub open: Arc<Mutex<Vec<VenueAccountId>>>,
}

#[async_trait]
impl VenueAccounts for Venues {
    async fn resolve(
        &self,
        user_id: UserId,
        connection_id: ConnectionId,
        platform_account: &str,
    ) -> Result<Option<VenueAccountFacts>, PortError> {
        Ok(self
            .known
            .lock()
            .unwrap()
            .get(&(user_id, connection_id, platform_account.to_owned()))
            .copied())
    }
    async fn describe(
        &self,
        user_id: UserId,
        ids: &[VenueAccountId],
    ) -> Result<Vec<BindingView>, PortError> {
        Ok(self
            .known
            .lock()
            .unwrap()
            .iter()
            .filter(|((u, _, _), f)| *u == user_id && ids.contains(&f.venue_account_id))
            .map(|((_, c, p), f)| BindingView {
                venue_account_id: f.venue_account_id,
                connection_id: *c,
                connection_name: "PC".into(),
                connection_status: "connected".into(),
                platform_account: p.clone(),
                kind: "simulation".into(),
                oco_location: "local".into(),
            })
            .collect())
    }
    async fn has_open_position(&self, _: UserId, v: VenueAccountId) -> Result<bool, PortError> {
        Ok(self.open.lock().unwrap().contains(&v))
    }
}

pub struct NoRoutes;

#[async_trait]
impl AccountRoutes for NoRoutes {
    async fn routes_of(&self, _: UserId, _: AccountId) -> Result<Vec<RouteRef>, PortError> {
        Ok(vec![])
    }
    async fn has_enabled_automated_route(
        &self,
        _: UserId,
        _: AccountId,
    ) -> Result<bool, PortError> {
        Ok(false)
    }
}

pub struct SaoPaulo;

#[async_trait]
impl UserTimeZone for SaoPaulo {
    async fn time_zone(&self, _: UserId) -> Result<Tz, PortError> {
        Ok(chrono_tz::America::Sao_Paulo)
    }
}

pub struct NoProfiles;

#[async_trait]
impl InitialProfilePlanner for NoProfiles {
    async fn plan(
        &self,
        _: UserId,
        _: AccountKind,
        size: &SizeAmount,
        profile_size: Option<&SizeAmount>,
        _: DateTime<Utc>,
    ) -> Result<Vec<AccountBracketProfile>, InitialProfileError> {
        // 37 500 is a tie between 25k and 50k in these tests.
        if size.value() == Decimal::from(37_500) {
            return match profile_size.map(|s| s.value()) {
                None => Err(InitialProfileError::ProfileSizeRequired),
                Some(v) if v == Decimal::from(25_000) || v == Decimal::from(50_000) => Ok(vec![]),
                Some(_) => Err(InitialProfileError::ProfileSizeInvalid),
            };
        }
        Ok(vec![])
    }
}

/// Grants when a proof is sent (code `000000` is wrong); records every call.
#[derive(Default, Clone)]
pub struct Gate(pub Arc<Mutex<Vec<(StepUpAction, Option<Uuid>)>>>);

#[async_trait]
impl StepUpGate for Gate {
    async fn authorize(
        &self,
        caller: &Caller,
        action: StepUpAction,
        target_id: Option<Uuid>,
        proof: Option<StepUpProof>,
        _: &RequestOrigin,
    ) -> Result<StepUpGrant, AccessError> {
        self.0.lock().unwrap().push((action, target_id));
        match proof {
            None => Err(AccessError::StepUpRequired),
            Some(p) if p.code != "000000" => {
                Ok(StepUpGrant::issue(caller, action, target_id, true))
            }
            Some(_) => Err(AccessError::CodeInvalid),
        }
    }
}

/// Session cookie value = the caller's user id (a test session per user).
pub struct CookieIsUser;

#[async_trait]
impl SessionAuthenticator for CookieIsUser {
    async fn authenticate(&self, cookie: Option<&str>, _: Activity) -> Result<Caller, AccessError> {
        let id = cookie
            .and_then(|c| Uuid::parse_str(c).ok())
            .ok_or(AccessError::Unauthenticated)?;
        Ok(Caller {
            user_id: UserId::from_uuid(id),
            session_id: id,
            device_id: id,
            is_platform_admin: false,
        })
    }
}
