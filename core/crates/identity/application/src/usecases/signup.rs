//! `startSignup`, `completeSignup` (S3 AC-1/AC-2, auth policy rev. 2 §10, data model §4.3).
//! The account exists only once the e-mail code is confirmed; an abandoned sign-up leaves no row.

use identity_enterprise::channel::{Channel, Destination};
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::CodeId;
use identity_enterprise::invitation::InvitationStatus;
use identity_enterprise::otp::{CodeState, DeviceClass, Purpose, SignupCode};
use identity_enterprise::profile::{Locale, TimeZoneName};
use identity_enterprise::user::{User, UserStatus};
use shared_kernel::UserId;

use crate::changes::{Actor, AuditAction, ChangeSet, CodeTable, Write};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::ports::{CodeDelivery, CodeMessage};
use crate::usecases::codes::{self, Check};
use crate::usecases::common::{check_public_throttle, user_audit};
use crate::usecases::delivery::DeliveryJob;
use crate::usecases::invitations::pending_invitation;
use crate::usecases::sessions::{new_session, resolve_device};
use crate::views::{ChallengeIssued, SignedIn};

/// Sends the sign-up code to the invited e-mail.
pub struct StartSignup {
    deps: IdentityDeps,
}

impl StartSignup {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        token: &str,
        meta: &RequestMeta,
    ) -> Result<ChallengeIssued, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        check_public_throttle(deps, meta.ip, now).await?;
        let invitation = pending_invitation(deps, token).await?;
        let ident = codes::identifier(deps, &invitation.email);
        codes::admit_send(deps, meta.ip, &ident, DeviceClass::Unknown, now).await?;
        let code_id = CodeId::new_v7();
        let fresh = codes::fresh_code(deps, code_id)?;
        let row = SignupCode {
            id: code_id,
            invitation_id: invitation.id,
            request_ip: Some(meta.ip),
            state: CodeState::issue(fresh.hmac, now),
        };
        deps.store
            .commit(ChangeSet::new(UserId::new_v7()).write(Write::InsertSignupCode(row)))
            .await?;
        let email = EmailAddress::from_stored(invitation.email.clone(), invitation.email.clone());
        deps.delivery.dispatch(DeliveryJob {
            table: CodeTable::Signup,
            code_id,
            user: None,
            purpose: Purpose::Signup,
            delivery: CodeDelivery {
                destination: Destination::Email(email),
                locale: invitation.locale,
                code: fresh.plain,
                message: CodeMessage::Signup,
            },
            suppressed: false,
            ip: Some(meta.ip),
        });
        let challenge = codes::seal(
            deps,
            code_id,
            Purpose::Signup,
            &ident,
            DeviceClass::Unknown,
            now,
        );
        Ok(codes::issued(challenge, now))
    }
}

pub struct CompleteSignupCommand {
    pub challenge_id: String,
    pub code: String,
    pub locale: Locale,
    pub time_zone: String,
    pub remember_me: bool,
    pub device_cookie: Option<String>,
    pub meta: RequestMeta,
}

/// Creates the user (e-mail confirmed), consumes the invitation and starts the first session with
/// its device. The sign-up session sends no new-device notice (AC-9).
pub struct CompleteSignup {
    deps: IdentityDeps,
}

impl CompleteSignup {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, cmd: CompleteSignupCommand) -> Result<SignedIn, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let time_zone =
            TimeZoneName::parse(&cmd.time_zone).map_err(|_| IdentityError::InvalidTimeZone)?;
        codes::admit_verify(deps, cmd.meta.ip, now).await?;
        let invalid = || async {
            deps.code_throttle.record_wrong(cmd.meta.ip, now).await;
            Err(IdentityError::CodeInvalid)
        };
        let Some(claims) = codes::open(deps, &cmd.challenge_id) else {
            return invalid().await;
        };
        if claims.purpose != Purpose::Signup {
            return invalid().await;
        }
        let Some(row) = deps.store.signup_code(claims.code_id).await? else {
            return invalid().await;
        };
        let Some(invitation) = deps.store.invitation(row.invitation_id).await? else {
            return invalid().await;
        };
        if invitation.status(now) != InvitationStatus::Pending {
            return invalid().await;
        }
        let hmac = match codes::check(deps, row.id, &row.state, &cmd.code, now) {
            Check::Match(h) => h,
            Check::Closed => return invalid().await,
            Check::Wrong => {
                codes::record_wrong(
                    deps,
                    UserId::new_v7(),
                    CodeTable::Signup,
                    row.id,
                    None,
                    cmd.meta.ip,
                    now,
                )
                .await?;
                return Err(IdentityError::CodeInvalid);
            }
        };
        let email = EmailAddress::from_stored(invitation.email.clone(), invitation.email.clone());
        let user = User {
            id: UserId::new_v7(),
            email,
            email_verified_at: now,
            whatsapp: None,
            locale: cmd.locale,
            time_zone,
            is_platform_admin: false,
            status: UserStatus::Active,
            created_at: now,
            channel_changed_at: None,
        };
        let device =
            resolve_device(deps, user.id, cmd.device_cookie.as_deref(), &cmd.meta, now).await?;
        let (session, cookie) = new_session(
            deps,
            user.id,
            device.device_id,
            Channel::Email,
            cmd.remember_me,
            &cmd.meta,
            now,
        );
        // The user row first: the invitation's accepted_user_id references it.
        let mut changes = ChangeSet::new(user.id)
            .write(Write::InsertUser(user.clone()))
            .write(Write::ConsumeCode {
                table: CodeTable::Signup,
                code_id: row.id,
                code_hmac: hmac,
                at: now,
            })
            .write(Write::AcceptInvitation {
                invitation_id: invitation.id,
                user_id: user.id,
                at: now,
            });
        for w in device.writes {
            changes = changes.write(w);
        }
        changes = changes
            .write(Write::InsertSession(session))
            .audit(user_audit(
                user.id,
                Actor::TokenHolder,
                AuditAction::AccountCreated {
                    invitation_id: invitation.id,
                },
                now,
                Some(cmd.meta.ip),
            ))
            .audit(user_audit(
                user.id,
                Actor::User(user.id),
                AuditAction::SignedIn {
                    channel: Channel::Email,
                    new_device: false,
                    device_label: device.label,
                },
                now,
                Some(cmd.meta.ip),
            ));
        deps.store
            .commit(changes)
            .await
            .map_err(codes::consume_conflict)?;
        Ok(SignedIn {
            user,
            new_device: false,
            session: cookie,
            device: device.cookie,
        })
    }
}
