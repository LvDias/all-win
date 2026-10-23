//! `requestLoginCode`, `verifyLoginCode`, `resendCode`, `getChallengeDelivery`, `logout`,
//! `signOutEverywhere` (auth policy rev. 2 §2–§5, data model §4.3).

use identity_enterprise::channel::{Channel, Destination};
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::CodeId;
use identity_enterprise::otp::{CodeState, DeliveryState, DeviceClass, OneTimeCode, Purpose};
use identity_enterprise::session::EndReason;
use identity_enterprise::user::UserStatus;

use crate::auth::{AuthContext, AuthenticateSession};
use crate::changes::{
    Actor, AuditAction, ChangeSet, CodeTable, EndedSessions, IdentityEvent, Notice, Write,
};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::ports::{CodeDelivery, CodeMessage};
use crate::usecases::codes::{self, Check};
use crate::usecases::common::user_audit;
use crate::usecases::delivery::DeliveryJob;
use crate::usecases::sessions::{device_class_known, new_session, request_label, resolve_device};
use crate::views::{ChallengeIssued, DeliveryView, SignedIn};

pub struct RequestLoginCodeCommand {
    pub email: String,
    pub channel: Channel,
    /// Raw `__Host-aw_device` value, if the browser sent one.
    pub device_cookie: Option<String>,
    pub meta: RequestMeta,
}

/// Always a fresh challenge, whether or not the e-mail is registered, active or has the channel;
/// nothing is sent in those cases. The statements run are the same up to the commit.
pub struct RequestLoginCode {
    deps: IdentityDeps,
}

impl RequestLoginCode {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        cmd: RequestLoginCodeCommand,
    ) -> Result<ChallengeIssued, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let normalized = EmailAddress::parse(&cmd.email)
            .map(|e| e.normalized().to_owned())
            .unwrap_or_else(|_| cmd.email.trim().to_lowercase());
        let target = deps.store.login_target(&normalized).await?;
        let user = target.as_ref().map(|t| t.user_id);
        let known = device_class_known(deps, user, cmd.device_cookie.as_deref()).await?;
        let class = if known {
            DeviceClass::Known
        } else {
            DeviceClass::Unknown
        };
        let ident = codes::identifier(deps, &normalized);
        if let Err(e) = codes::admit_send(deps, cmd.meta.ip, &ident, class, now).await {
            if let (IdentityError::RateLimited { .. }, Some(u)) = (&e, user) {
                codes::audit_rate_limited(deps, u, Purpose::Login, cmd.meta.ip, now).await?;
            }
            return Err(e);
        }
        let code_id = CodeId::new_v7();
        let challenge = codes::seal(deps, code_id, Purpose::Login, &ident, class, now);
        let usable = target.as_ref().filter(|t| {
            t.status == UserStatus::Active
                && (cmd.channel == Channel::Email || deps.settings.whatsapp_enabled)
        });
        let destination = usable.and_then(|t| t.destination(cmd.channel).cloned());
        match (usable, destination) {
            (Some(t), Some(destination)) => {
                let fresh = codes::fresh_code(deps, code_id)?;
                let row = OneTimeCode {
                    id: code_id,
                    user_id: t.user_id,
                    purpose: Purpose::Login,
                    destination: destination.clone(),
                    action: None,
                    target_id: None,
                    session_id: None,
                    channel_change_id: None,
                    role: None,
                    request_ip: Some(cmd.meta.ip),
                    state: CodeState::issue(fresh.hmac, now),
                };
                deps.store
                    .commit(ChangeSet::new(t.user_id).write(Write::InsertOneTimeCode(row)))
                    .await?;
                deps.delivery.dispatch(DeliveryJob {
                    table: CodeTable::OneTime,
                    code_id,
                    user: Some(t.user_id),
                    purpose: Purpose::Login,
                    delivery: CodeDelivery {
                        destination,
                        locale: t.locale,
                        code: fresh.plain,
                        message: CodeMessage::Login {
                            device_label: request_label(&cmd.meta),
                            requested_at: now,
                        },
                    },
                    suppressed: false,
                    ip: Some(cmd.meta.ip),
                });
            }
            _ => deps.store.login_code_placeholder().await?,
        }
        Ok(codes::issued(challenge, now))
    }
}

pub struct VerifyLoginCodeCommand {
    pub challenge_id: String,
    pub code: String,
    pub remember_me: bool,
    pub device_cookie: Option<String>,
    pub meta: RequestMeta,
}

/// Every failure is the single `auth.code_invalid`.
pub struct VerifyLoginCode {
    deps: IdentityDeps,
}

impl VerifyLoginCode {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, cmd: VerifyLoginCodeCommand) -> Result<SignedIn, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        codes::admit_verify(deps, cmd.meta.ip, now).await?;
        let wrong = || async {
            deps.code_throttle.record_wrong(cmd.meta.ip, now).await;
            Err(IdentityError::CodeInvalid)
        };
        let Some(claims) = codes::open(deps, &cmd.challenge_id) else {
            return wrong().await;
        };
        if claims.purpose != Purpose::Login {
            return wrong().await;
        }
        let Some((owner, Purpose::Login)) = deps.store.one_time_code_owner(claims.code_id).await?
        else {
            return wrong().await;
        };
        let (Some(row), Some(user)) = (
            deps.store.one_time_code(owner, claims.code_id).await?,
            deps.store.user(owner).await?,
        ) else {
            return wrong().await;
        };
        if !user.is_active() || !user.has_destination(&row.destination) {
            return wrong().await;
        }
        let channel = row.destination.channel();
        let hmac = match codes::check(deps, row.id, &row.state, &cmd.code, now) {
            Check::Match(h) => h,
            Check::Closed => return wrong().await,
            Check::Wrong => {
                let audit = user_audit(
                    owner,
                    Actor::User(owner),
                    AuditAction::SignInFailed { channel },
                    now,
                    Some(cmd.meta.ip),
                );
                codes::record_wrong(
                    deps,
                    owner,
                    CodeTable::OneTime,
                    row.id,
                    Some(audit),
                    cmd.meta.ip,
                    now,
                )
                .await?;
                return Err(IdentityError::CodeInvalid);
            }
        };
        let device =
            resolve_device(deps, owner, cmd.device_cookie.as_deref(), &cmd.meta, now).await?;
        let (session, cookie) = new_session(
            deps,
            owner,
            device.device_id,
            channel,
            cmd.remember_me,
            &cmd.meta,
            now,
        );
        let mut changes = ChangeSet::new(owner).write(Write::ConsumeCode {
            table: CodeTable::OneTime,
            code_id: row.id,
            code_hmac: hmac,
            at: now,
        });
        for w in device.writes {
            changes = changes.write(w);
        }
        changes = changes
            .write(Write::InsertSession(session))
            .audit(user_audit(
                owner,
                Actor::User(owner),
                AuditAction::SignedIn {
                    channel,
                    new_device: device.new_device,
                    device_label: device.label.clone(),
                },
                now,
                Some(cmd.meta.ip),
            ));
        if device.new_device {
            changes = changes
                .audit(user_audit(
                    owner,
                    Actor::System,
                    AuditAction::NewDeviceNoticeSent {
                        device_label: device.label.clone(),
                    },
                    now,
                    Some(cmd.meta.ip),
                ))
                .event(IdentityEvent::NoticeRequested {
                    user_id: owner,
                    locale: user.locale,
                    notice: Notice::NewDevice {
                        device_label: device.label,
                        ip: Some(cmd.meta.ip),
                        channel,
                    },
                    at: now,
                });
        }
        deps.store
            .commit(changes)
            .await
            .map_err(codes::consume_conflict)?;
        Ok(SignedIn {
            user,
            new_device: device.new_device,
            session: cookie,
            device: device.cookie,
        })
    }
}

pub struct ResendCodeCommand {
    pub challenge_id: String,
    pub meta: RequestMeta,
}

/// The response and the limits depend only on the token and the counters, never on the code row
/// (policy §3, M1): every resend of a valid token counts and gets the same body.
pub struct ResendCode {
    deps: IdentityDeps,
}

impl ResendCode {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        session: Option<&AuthContext>,
        cmd: ResendCodeCommand,
    ) -> Result<ChallengeIssued, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let Some(claims) = codes::open(deps, &cmd.challenge_id) else {
            // A forged token counts only in the IP limits.
            if let crate::throttle::ThrottleDecision::Limited { retry_after } =
                deps.code_throttle.admit_send(cmd.meta.ip, now).await
            {
                return Err(IdentityError::RateLimited { retry_after });
            }
            return Ok(codes::issued(
                identity_enterprise::secret::SecretString::new(cmd.challenge_id),
                now,
            ));
        };
        let ident = identity_enterprise::tokens::CodeHmac {
            bytes: claims.identifier_hmac,
            pepper_version: claims.pepper_version,
        };
        codes::admit_send(deps, cmd.meta.ip, &ident, claims.device_class, now).await?;
        let body = codes::issued(
            identity_enterprise::secret::SecretString::new(cmd.challenge_id.clone()),
            now,
        );
        match claims.purpose {
            Purpose::Signup => self.resend_signup(claims.code_id, &cmd.meta, now).await?,
            purpose => {
                self.resend_code(purpose, claims.code_id, session, &cmd.meta, now)
                    .await?;
            }
        }
        Ok(body)
    }

    async fn resend_signup(
        &self,
        code_id: CodeId,
        meta: &RequestMeta,
        now: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), IdentityError> {
        let deps = &self.deps;
        let Some(row) = deps.store.signup_code(code_id).await? else {
            return Ok(());
        };
        let Some(invitation) = deps.store.invitation(row.invitation_id).await? else {
            return Ok(());
        };
        if !row.state.is_open(now)
            || invitation.status(now) != identity_enterprise::invitation::InvitationStatus::Pending
        {
            return Ok(());
        }
        let fresh = codes::fresh_code(deps, code_id)?;
        let state = row.state.resent(fresh.hmac, now);
        let scope = shared_kernel::UserId::new_v7();
        let written = deps
            .store
            .commit(ChangeSet::new(scope).write(Write::ResendCode {
                table: CodeTable::Signup,
                code_id,
                code_hmac: fresh.hmac,
                sent_at: state.sent_at,
                expires_at: state.expires_at,
            }))
            .await;
        if written.is_err() {
            return Ok(());
        }
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
        Ok(())
    }

    async fn resend_code(
        &self,
        purpose: Purpose,
        code_id: CodeId,
        session: Option<&AuthContext>,
        meta: &RequestMeta,
        now: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), IdentityError> {
        let deps = &self.deps;
        let Some((owner, stored_purpose)) = deps.store.one_time_code_owner(code_id).await? else {
            return Ok(());
        };
        if stored_purpose != purpose {
            return Ok(());
        }
        if matches!(purpose, Purpose::StepUp | Purpose::ChannelChange)
            && session.is_none_or(|c| c.user.id != owner)
        {
            return Ok(());
        }
        let (Some(row), Some(user)) = (
            deps.store.one_time_code(owner, code_id).await?,
            deps.store.user(owner).await?,
        ) else {
            return Ok(());
        };
        let bound_elsewhere = row
            .session_id
            .is_some_and(|s| session.is_none_or(|c| c.session.id != s));
        if bound_elsewhere || !row.state.is_open(now) || !user.is_active() {
            return Ok(());
        }
        let fresh = codes::fresh_code(deps, code_id)?;
        let state = row.state.resent(fresh.hmac, now);
        let written = deps
            .store
            .commit(ChangeSet::new(owner).write(Write::ResendCode {
                table: CodeTable::OneTime,
                code_id,
                code_hmac: fresh.hmac,
                sent_at: state.sent_at,
                expires_at: state.expires_at,
            }))
            .await;
        if written.is_err() {
            return Ok(());
        }
        let suppressed = row.role == Some(identity_enterprise::otp::CodeRole::NewValue)
            && crate::usecases::channel_change::value_taken(deps, owner, &row.destination).await?;
        let message = match purpose {
            Purpose::Login => CodeMessage::Login {
                device_label: request_label(meta),
                requested_at: now,
            },
            Purpose::StepUp => match row.action {
                Some(a) => CodeMessage::StepUp(a),
                None => return Ok(()),
            },
            Purpose::ChannelChange | Purpose::Signup => CodeMessage::ChannelChange {
                channel: row.destination.channel(),
            },
        };
        deps.delivery.dispatch(DeliveryJob {
            table: CodeTable::OneTime,
            code_id,
            user: Some(owner),
            purpose,
            delivery: CodeDelivery {
                destination: row.destination,
                locale: user.locale,
                code: fresh.plain,
                message,
            },
            suppressed,
            ip: Some(meta.ip),
        });
        Ok(())
    }
}

/// Delivery state for sign-up, step-up and channel-change challenges; `not_reported` for login,
/// closed, unknown, forged and other sessions' challenges.
pub struct GetChallengeDelivery {
    deps: IdentityDeps,
}

impl GetChallengeDelivery {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        session: Option<&AuthContext>,
        challenge_id: &str,
    ) -> Result<DeliveryView, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let Some(claims) = codes::open(deps, challenge_id) else {
            return Ok(DeliveryView::NotReported);
        };
        let state = match claims.purpose {
            Purpose::Login => None,
            Purpose::Signup => deps
                .store
                .signup_code(claims.code_id)
                .await?
                .map(|r| r.state),
            Purpose::StepUp | Purpose::ChannelChange => {
                let Some(ctx) = session else {
                    return Ok(DeliveryView::NotReported);
                };
                deps.store
                    .one_time_code(ctx.user.id, claims.code_id)
                    .await?
                    .filter(|r| r.session_id == Some(ctx.session.id) && r.purpose == claims.purpose)
                    .map(|r| r.state)
            }
        };
        Ok(
            match state.filter(|s| s.is_open(now)).map(|s| s.delivery_state) {
                Some(DeliveryState::Pending) => DeliveryView::Pending,
                Some(DeliveryState::Sent) => DeliveryView::Sent,
                Some(DeliveryState::Failed) => DeliveryView::Failed,
                None => DeliveryView::NotReported,
            },
        )
    }
}

/// Idempotent: accepts a session or none. The device cookie is kept.
pub struct Logout {
    deps: IdentityDeps,
}

impl Logout {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, cookie: Option<&str>) -> Result<(), IdentityError> {
        let Some(session) = AuthenticateSession::new(self.deps.clone())
            .find(cookie)
            .await?
        else {
            return Ok(());
        };
        let now = self.deps.clock.now();
        let changes = ChangeSet::new(session.user_id)
            .write(Write::EndSession {
                session_id: session.id,
                reason: EndReason::Logout,
                at: now,
            })
            .event(IdentityEvent::SessionsEnded {
                user_id: session.user_id,
                sessions: EndedSessions::One(session.id),
                reason: EndReason::Logout,
                at: now,
            });
        self.deps.store.commit(changes).await?;
        Ok(())
    }
}

/// "Sair de todos os dispositivos" (Q34 a): every session ends and every device is revoked. No
/// step-up: it only reduces access.
pub struct SignOutEverywhere {
    deps: IdentityDeps,
}

impl SignOutEverywhere {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, ctx: &AuthContext, meta: &RequestMeta) -> Result<(), IdentityError> {
        let now = self.deps.clock.now();
        let user = ctx.user.id;
        let changes = ChangeSet::new(user)
            .write(Write::EndUserSessions {
                user_id: user,
                except: None,
                reason: EndReason::SignedOutEverywhere,
                at: now,
            })
            .write(Write::RevokeAllDevices {
                user_id: user,
                at: now,
            })
            .audit(user_audit(
                user,
                Actor::User(user),
                AuditAction::SignedOutEverywhere,
                now,
                Some(meta.ip),
            ))
            .event(IdentityEvent::SessionsEnded {
                user_id: user,
                sessions: EndedSessions::AllExcept(Vec::new()),
                reason: EndReason::SignedOutEverywhere,
                at: now,
            });
        self.deps.store.commit(changes).await?;
        Ok(())
    }
}
