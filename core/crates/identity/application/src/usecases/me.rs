//! `getMe`, `updateMe` (Q23), `getMySecurity`.

use identity_enterprise::profile::{Locale, TimeZoneName};
use identity_enterprise::session;
use identity_enterprise::user::User;

use crate::auth::AuthContext;
use crate::changes::{ChangeSet, Write};
use crate::deps::IdentityDeps;
use crate::error::IdentityError;
use crate::views::{SecurityView, SessionInfo};

pub struct GetMe;

impl GetMe {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&self, ctx: &AuthContext) -> User {
        ctx.user.clone()
    }
}

impl Default for GetMe {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UpdateMe {
    deps: IdentityDeps,
}

impl UpdateMe {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        locale: Option<Locale>,
        time_zone: Option<&str>,
    ) -> Result<User, IdentityError> {
        let time_zone = time_zone
            .map(TimeZoneName::parse)
            .transpose()
            .map_err(|_| IdentityError::InvalidTimeZone)?;
        let mut user = ctx.user.clone();
        if let Some(locale) = locale {
            user.locale = locale;
        }
        if let Some(tz) = &time_zone {
            user.time_zone = tz.clone();
        }
        let changes = ChangeSet::new(user.id).write(Write::UpdateProfile {
            user_id: user.id,
            locale,
            time_zone,
        });
        self.deps.store.commit(changes).await?;
        Ok(user)
    }
}

pub struct GetMySecurity;

impl GetMySecurity {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&self, ctx: &AuthContext) -> SecurityView {
        let s = &ctx.session;
        SecurityView {
            session: SessionInfo {
                persistent: s.persistent,
                idle_timeout_s: session::idle_timeout(s.persistent).num_seconds(),
                absolute_timeout_s: session::absolute_timeout(s.persistent).num_seconds(),
                idle_expires_at: s.idle_expires_at,
                absolute_expires_at: s.absolute_expires_at,
                step_up_valid_until: s.step_up_valid_until(),
            },
        }
    }
}

impl Default for GetMySecurity {
    fn default() -> Self {
        Self::new()
    }
}
