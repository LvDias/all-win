//! `listUsers`.

use crate::auth::AuthContext;
use crate::deps::IdentityDeps;
use crate::error::IdentityError;
use crate::ports::UserSummary;

pub struct ListUsers {
    deps: IdentityDeps,
}

impl ListUsers {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, ctx: &AuthContext) -> Result<Vec<UserSummary>, IdentityError> {
        ctx.require_admin()?;
        Ok(self.deps.store.users(ctx.user.id).await?)
    }
}
