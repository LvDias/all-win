//! `startChannelChange`, `verifyChannelChangeCode`, `cancelChannelChange` and the worker that
//! applies due holds (auth policy rev. 2 §7, §9; M2 approved; data model §4.3). The admin
//! recovery (AC-12) shares the hold, the cancel and the worker; it starts in `admin_recovery`.

use chrono::{DateTime, Utc};
use identity_enterprise::channel::{Channel, Destination, PhoneNumber};
use identity_enterprise::channel_change::{ChangeStatus, ChannelChange, HOLD};
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::{ChannelChangeId, CodeId, DeviceId};
use identity_enterprise::otp::{
    CodeRole, CodeState, DeviceClass, InvalidReason, OneTimeCode, Purpose,
};
use identity_enterprise::session::EndReason;
use identity_enterprise::user::User;
use shared_kernel::UserId;

use crate::auth::AuthContext;
use crate::changes::{
    Actor, AuditAction, ChangeAudit, ChangeSet, CodeTable, EndedSessions, IdentityEvent, Notice,
    Write,
};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::ports::{CodeDelivery, CodeMessage, SecurityAlert, StoreConflict, StoreError};
use crate::usecases::codes::{self, Check};
use crate::usecases::common::user_audit;
use crate::usecases::delivery::DeliveryJob;
use crate::usecases::sessions::rotate_session;
use crate::views::{ChangeCodeView, ChannelChangeVerified, ChannelChangeView};

/// Whether `value` belongs to an account other than `user` (channel clash, never revealed).
pub(crate) async fn value_taken(
    deps: &IdentityDeps,
    user: UserId,
    value: &Destination,
) -> Result<bool, IdentityError> {
    match value {
        Destination::Email(e) => Ok(deps
            .store
            .login_target(e.normalized())
            .await?
            .is_some_and(|t| t.user_id != user)),
        // WhatsApp clashes are checked by the unique key at apply time (S3b adds a lookup).
        Destination::Whatsapp(_) => Ok(false),
    }
}

/// The view of a change with one sealed challenge per code (re-sealed from the row, so the
/// tokens are the ones the codes were issued under). An admin recovery shows no code: only the
/// admin confirms it (HTTP 5.4.0).
pub(crate) fn view(
    deps: &IdentityDeps,
    change: &ChannelChange,
    codes: &[OneTimeCode],
    now: DateTime<Utc>,
) -> ChannelChangeView {
    let mut code_views: Vec<ChangeCodeView> = codes
        .iter()
        .filter(|_| change.recovery.is_none())
        .filter_map(|c| {
            let role = c.role?;
            let ident = codes::identifier(deps, c.destination.normalized());
            Some(ChangeCodeView {
                role,
                challenge_id: codes::seal(
                    deps,
                    c.id,
                    Purpose::ChannelChange,
                    &ident,
                    DeviceClass::Known,
                    c.state.created_at,
                ),
                confirmed: c.state.consumed_at.is_some(),
            })
        })
        .collect();
    code_views.sort_by_key(|c| match c.role {
        CodeRole::CurrentValue => 0,
        CodeRole::NewValue => 1,
        CodeRole::OtherChannel => 2,
    });
    ChannelChangeView {
        id: change.id,
        kind: change.kind(),
        channel: change.channel(),
        status: change.status_at(now),
        masked_new_value: change.new_value.masked(),
        codes: code_views,
        expires_at: change.expires_at,
        hold_until: change.hold_until,
    }
}

fn parse_value(channel: Channel, raw: &str) -> Result<Destination, IdentityError> {
    match channel {
        Channel::Email => EmailAddress::parse(raw)
            .map(Destination::Email)
            .map_err(|_| IdentityError::InvalidChannelValue),
        Channel::Whatsapp => PhoneNumber::parse(raw)
            .map(Destination::Whatsapp)
            .ok_or(IdentityError::InvalidChannelValue),
    }
}

/// Notices to every current channel value of the user.
pub(crate) fn notice_to_all(
    user: &User,
    at: DateTime<Utc>,
    make: impl Fn(Destination) -> Notice,
) -> Vec<IdentityEvent> {
    [Channel::Email, Channel::Whatsapp]
        .into_iter()
        .filter_map(|c| user.destination(c))
        .map(|to| IdentityEvent::NoticeRequested {
            user_id: user.id,
            locale: user.locale,
            notice: make(to),
            at,
        })
        .collect()
}

pub struct StartChannelChangeCommand {
    pub channel: Channel,
    pub new_value: String,
    pub meta: RequestMeta,
}

/// Codes to the current value and to the new value (and to the other channel when the user has
/// two). The change applies only when every code is confirmed within 10 minutes.
pub struct StartChannelChange {
    deps: IdentityDeps,
}

impl StartChannelChange {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        cmd: StartChannelChangeCommand,
    ) -> Result<ChannelChangeView, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let user = &ctx.user;
        if cmd.channel == Channel::Whatsapp && !deps.settings.whatsapp_enabled {
            return Err(IdentityError::ChannelUnavailable);
        }
        let new_value = parse_value(cmd.channel, &cmd.new_value)?;
        if user.has_destination(&new_value) {
            return Err(IdentityError::ChannelUnchanged);
        }
        if user.channel_change_allowed_at() > now {
            return Err(IdentityError::ChannelChangeTooSoon);
        }
        if let Some(open) = deps.store.open_channel_change(user.id).await?
            && open.status_at(now).is_open()
        {
            return Err(IdentityError::ChannelChangePending);
        }
        // The first WhatsApp: "current" is the e-mail.
        let current = user
            .destination(cmd.channel)
            .unwrap_or_else(|| Destination::Email(user.email.clone()));
        let other = [Channel::Email, Channel::Whatsapp]
            .into_iter()
            .filter(|c| *c != cmd.channel && *c != current.channel())
            .find_map(|c| user.destination(c));
        let mut targets = vec![
            (CodeRole::CurrentValue, current),
            (CodeRole::NewValue, new_value.clone()),
        ];
        if let Some(other) = other {
            targets.push((CodeRole::OtherChannel, other));
        }
        let sends = i32::try_from(targets.len()).unwrap_or(i32::MAX);
        let limits = async {
            codes::admit_user_sends(deps, user.id, sends, now).await?;
            for (_, d) in &targets {
                let ident = codes::identifier(deps, d.normalized());
                codes::admit_send(deps, cmd.meta.ip, &ident, DeviceClass::Known, now).await?;
            }
            Ok::<(), IdentityError>(())
        };
        if let Err(e) = limits.await {
            if matches!(e, IdentityError::RateLimited { .. }) {
                codes::audit_rate_limited(deps, user.id, Purpose::ChannelChange, cmd.meta.ip, now)
                    .await?;
            }
            return Err(e);
        }
        if deps
            .store
            .latest_channel_change(user.id)
            .await?
            .is_some_and(|c| c.status == ChangeStatus::Cancelled)
        {
            deps.alerts
                .raise(SecurityAlert::ChannelChangeAfterCancel { user_id: user.id });
        }
        let clash = value_taken(deps, user.id, &new_value).await?;
        let change = ChannelChange::start(
            ChannelChangeId::new_v7(),
            user.id,
            new_value.clone(),
            ctx.session.id,
            now,
        );
        let mut changes = ChangeSet::new(user.id)
            .write(Write::ExpireStaleChannelChange {
                user_id: user.id,
                at: now,
            })
            .write(Write::InsertChannelChange(change.clone()));
        let mut rows = Vec::new();
        let mut jobs = Vec::new();
        for (role, destination) in targets {
            let code_id = CodeId::new_v7();
            let fresh = codes::fresh_code(deps, code_id)?;
            let row = OneTimeCode {
                id: code_id,
                user_id: user.id,
                purpose: Purpose::ChannelChange,
                destination: destination.clone(),
                action: None,
                target_id: None,
                session_id: Some(ctx.session.id),
                channel_change_id: Some(change.id),
                role: Some(role),
                request_ip: Some(cmd.meta.ip),
                state: CodeState::issue(fresh.hmac, now),
            };
            changes = changes.write(Write::InsertOneTimeCode(row.clone()));
            rows.push(row);
            jobs.push(DeliveryJob {
                table: CodeTable::OneTime,
                code_id,
                user: Some(user.id),
                purpose: Purpose::ChannelChange,
                delivery: CodeDelivery {
                    destination,
                    locale: user.locale,
                    code: fresh.plain,
                    message: CodeMessage::ChannelChange {
                        channel: cmd.channel,
                    },
                },
                suppressed: clash && role == CodeRole::NewValue,
                ip: Some(cmd.meta.ip),
            });
        }
        changes = changes.audit(user_audit(
            user.id,
            Actor::User(user.id),
            AuditAction::ChannelChangeRequested {
                change_id: change.id,
                kind: ChangeAudit::UserChange,
                channel: cmd.channel,
                masked_new_value: new_value.masked(),
            },
            now,
            Some(cmd.meta.ip),
        ));
        match deps.store.commit(changes).await {
            Ok(()) => {}
            Err(StoreError::Conflict(StoreConflict::ChannelChangeOpen)) => {
                return Err(IdentityError::ChannelChangePending);
            }
            Err(e) => return Err(e.into()),
        }
        for job in jobs {
            deps.delivery.dispatch(job);
        }
        Ok(view(deps, &change, &rows, now))
    }
}

pub struct VerifyChannelChangeCodeCommand {
    pub change_id: ChannelChangeId,
    pub challenge_id: String,
    pub code: String,
    pub meta: RequestMeta,
}

/// Confirms one code. With the last one the change completes (other sessions end, this one
/// rotates) or, from a device first seen less than 7 days ago, goes on hold for 72 h (M2).
pub struct VerifyChannelChangeCode {
    deps: IdentityDeps,
}

impl VerifyChannelChangeCode {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        cmd: VerifyChannelChangeCodeCommand,
    ) -> Result<ChannelChangeVerified, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let user = &ctx.user;
        let Some(change) = deps.store.channel_change(user.id, cmd.change_id).await? else {
            return Err(IdentityError::NotFound);
        };
        if change.status_at(now) != ChangeStatus::AwaitingCodes {
            return Err(IdentityError::ChannelChangeClosed);
        }
        // Only the admin confirms a recovery, with the operator command (AC-12): no session
        // may confirm its code, whoever holds it.
        if change.recovery.is_some() {
            return Err(IdentityError::CodeInvalid);
        }
        codes::admit_verify(deps, cmd.meta.ip, now).await?;
        let codes_now = deps.store.change_codes(user.id, change.id).await?;
        let claims =
            codes::open(deps, &cmd.challenge_id).filter(|c| c.purpose == Purpose::ChannelChange);
        let row = claims.and_then(|c| codes_now.iter().find(|r| r.id == c.code_id).cloned());
        let bound = row.filter(|r| {
            r.session_id == Some(ctx.session.id) && destination_still_valid(user, &change, r)
        });
        let Some(row) = bound else {
            deps.code_throttle.record_wrong(cmd.meta.ip, now).await;
            return Err(IdentityError::CodeInvalid);
        };
        let hmac = match codes::check(deps, row.id, &row.state, &cmd.code, now) {
            Check::Match(h) => h,
            Check::Closed => {
                deps.code_throttle.record_wrong(cmd.meta.ip, now).await;
                return Err(IdentityError::CodeInvalid);
            }
            Check::Wrong => {
                codes::record_wrong(
                    deps,
                    user.id,
                    CodeTable::OneTime,
                    row.id,
                    None,
                    cmd.meta.ip,
                    now,
                )
                .await?;
                return Err(IdentityError::CodeInvalid);
            }
        };
        let mut changes = ChangeSet::new(user.id).write(Write::ConsumeCode {
            table: CodeTable::OneTime,
            code_id: row.id,
            code_hmac: hmac,
            at: now,
        });
        let last = codes_now
            .iter()
            .all(|c| c.id == row.id || c.state.consumed_at.is_some());
        let mut rotated_cookie = None;
        let mut confirmed: Vec<OneTimeCode> = codes_now.clone();
        if let Some(c) = confirmed.iter_mut().find(|c| c.id == row.id) {
            c.state.consumed_at = Some(now);
        }
        let mut result = change.clone();
        if !last {
            changes = changes.write(Write::AssertChannelChangeAwaiting {
                change_id: change.id,
                at: now,
            });
        } else if self.device_is_new(ctx, now).await? {
            let hold_until = now + HOLD;
            changes = changes
                .write(Write::HoldChannelChange {
                    change_id: change.id,
                    hold_until,
                    at: now,
                })
                .audit(user_audit(
                    user.id,
                    Actor::User(user.id),
                    AuditAction::ChannelChangeOnHold {
                        change_id: change.id,
                        kind: ChangeAudit::UserChange,
                        channel: change.channel(),
                        hold_until,
                    },
                    now,
                    Some(cmd.meta.ip),
                ));
            for e in notice_to_all(user, now, |to| Notice::ChannelChangeOnHold {
                to,
                channel: change.channel(),
                masked_new_value: change.new_value.masked(),
                hold_until,
            }) {
                changes = changes.event(e);
            }
            result.status = ChangeStatus::OnHold;
            result.hold_until = Some(hold_until);
        } else {
            let (set, cookie) = complete_now(deps, ctx, &change, &cmd.meta, now);
            changes = changes.extend(set);
            rotated_cookie = Some(cookie);
            result.status = ChangeStatus::Completed;
            result.completed_at = Some(now);
        }
        deps.store
            .commit(changes)
            .await
            .map_err(codes::consume_conflict)?;
        Ok(ChannelChangeVerified {
            change: view(deps, &result, &confirmed, now),
            session: rotated_cookie,
        })
    }

    async fn device_is_new(
        &self,
        ctx: &AuthContext,
        now: DateTime<Utc>,
    ) -> Result<bool, IdentityError> {
        Ok(self
            .deps
            .store
            .device(ctx.user.id, ctx.session.device_id)
            .await?
            .is_none_or(|d| d.is_new(now)))
    }
}

/// The destination of a change code is still the one its role requires (policy §2.2).
fn destination_still_valid(user: &User, change: &ChannelChange, code: &OneTimeCode) -> bool {
    match code.role {
        Some(CodeRole::CurrentValue | CodeRole::OtherChannel) => {
            user.has_destination(&code.destination)
        }
        Some(CodeRole::NewValue) => code.destination.normalized() == change.new_value.normalized(),
        None => false,
    }
}

/// Completion without a hold, in the verifying session's transaction.
fn complete_now(
    deps: &IdentityDeps,
    ctx: &AuthContext,
    change: &ChannelChange,
    meta: &RequestMeta,
    now: DateTime<Utc>,
) -> (ChangeSet, crate::views::SessionCookie) {
    let user = &ctx.user;
    let old = user.destination(change.channel());
    let (rotated, cookie) = rotate_session(deps, &ctx.session, now);
    let rotated_id = rotated.id;
    let mut set = ChangeSet::new(user.id)
        .write(Write::CompleteChannelChange {
            change_id: change.id,
            at: now,
        })
        .write(Write::SetChannelValue {
            user_id: user.id,
            value: change.new_value.clone(),
            at: now,
        })
        .write(Write::InvalidateUserCodes {
            user_id: user.id,
            reason: InvalidReason::ChannelChanged,
            at: now,
        })
        .write(Write::EndUserSessions {
            user_id: user.id,
            except: Some(ctx.session.id),
            reason: EndReason::ChannelChanged,
            at: now,
        })
        .write(Write::EndSession {
            session_id: ctx.session.id,
            reason: EndReason::Rotated,
            at: now,
        })
        .write(Write::InsertSession(rotated))
        .audit(user_audit(
            user.id,
            Actor::User(user.id),
            AuditAction::ChannelChangeCompleted {
                change_id: change.id,
                kind: ChangeAudit::UserChange,
                channel: change.channel(),
                masked_old_value: old.as_ref().map(Destination::masked),
                masked_new_value: change.new_value.masked(),
            },
            now,
            Some(meta.ip),
        ))
        .event(IdentityEvent::SessionsEnded {
            user_id: user.id,
            // Both ids of the caller's tab: a stream still open on the old one, and one the tab
            // reopens with the new cookie before this event is dispatched (QA M1).
            sessions: EndedSessions::AllExcept(vec![ctx.session.id, rotated_id]),
            reason: EndReason::ChannelChanged,
            at: now,
        });
    for e in notice_to_all(user, now, |to| Notice::ChannelChanged {
        to,
        channel: change.channel(),
        masked_new_value: change.new_value.masked(),
    }) {
        set = set.event(e);
    }
    (set, cookie)
}

/// Which pending change a session-ending action cancels (user decision Q88 (a)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CancelScope {
    /// "Sair de todos os dispositivos": the pending change, whoever requested it.
    Any,
    /// "Esquecer dispositivo": only a change requested from that device.
    RequestedFrom(DeviceId),
}

/// Q88 (a): adds the cancellation of the user's pending channel change to `changes` (the same
/// transaction as the session end), with its audit and the notices to every channel. No 24-h
/// freeze follows. Nothing is added when there is no open change or the scope does not match.
pub(crate) async fn cancel_pending_with(
    deps: &IdentityDeps,
    ctx: &AuthContext,
    scope: CancelScope,
    mut changes: ChangeSet,
    meta: &RequestMeta,
) -> Result<ChangeSet, IdentityError> {
    let now = deps.clock.now();
    let user = &ctx.user;
    let Some(change) = deps.store.open_channel_change(user.id).await? else {
        return Ok(changes);
    };
    // Q88 (a): only a change the user requested; an admin recovery ends only by "Cancelar
    // troca" or the admin's cancel (policy 3.3 §9.1).
    if change.recovery.is_some()
        || !matches!(
            change.status_at(now),
            ChangeStatus::AwaitingCodes | ChangeStatus::OnHold
        )
    {
        return Ok(changes);
    }
    if let CancelScope::RequestedFrom(device) = scope {
        let from_device = match change.requested_session_id {
            Some(session) => deps
                .store
                .session(user.id, session)
                .await?
                .is_some_and(|s| s.device_id == device),
            None => false,
        };
        if !from_device {
            return Ok(changes);
        }
    }
    changes = changes
        .write(Write::CancelChannelChange {
            change_id: change.id,
            by_session: Some(ctx.session.id),
            at: now,
        })
        .audit(user_audit(
            user.id,
            Actor::User(user.id),
            AuditAction::ChannelChangeCancelled {
                change_id: change.id,
                kind: ChangeAudit::UserChange,
                channel: change.channel(),
                by_user: true,
            },
            now,
            Some(meta.ip),
        ));
    for e in notice_to_all(user, now, |to| Notice::ChannelChangeCancelled {
        to,
        channel: change.channel(),
    }) {
        changes = changes.event(e);
    }
    Ok(changes)
}

/// Commits a session-ending change set that may also cancel the pending channel change; if the
/// change closed meanwhile (applied or cancelled by another request), the session end still
/// happens without it.
pub(crate) async fn commit_with_cancel(
    deps: &IdentityDeps,
    with_cancel: ChangeSet,
    without: ChangeSet,
) -> Result<(), IdentityError> {
    match deps.store.commit(with_cancel).await {
        Err(StoreError::Conflict(StoreConflict::ChannelChangeGone)) => {
            deps.store.commit(without).await.map_err(Into::into)
        }
        other => other.map_err(Into::into),
    }
}

/// "Cancelar troca": from any session of the user, while awaiting codes or on hold.
pub struct CancelChannelChange {
    deps: IdentityDeps,
}

impl CancelChannelChange {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        change_id: ChannelChangeId,
        meta: &RequestMeta,
    ) -> Result<(), IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let user = &ctx.user;
        let Some(change) = deps.store.channel_change(user.id, change_id).await? else {
            return Err(IdentityError::NotFound);
        };
        match change.status_at(now) {
            ChangeStatus::Cancelled => return Ok(()),
            ChangeStatus::Completed | ChangeStatus::Expired => {
                return Err(IdentityError::ChannelChangeClosed);
            }
            ChangeStatus::AwaitingCodes | ChangeStatus::OnHold => {}
        }
        let mut changes = ChangeSet::new(user.id)
            .write(Write::CancelChannelChange {
                change_id,
                by_session: Some(ctx.session.id),
                at: now,
            })
            .audit(user_audit(
                user.id,
                Actor::User(user.id),
                AuditAction::ChannelChangeCancelled {
                    change_id,
                    kind: ChangeAudit::of(&change),
                    channel: change.channel(),
                    by_user: true,
                },
                now,
                Some(meta.ip),
            ));
        for e in notice_to_all(user, now, |to| Notice::ChannelChangeCancelled {
            to,
            channel: change.channel(),
        }) {
            changes = changes.event(e);
        }
        match deps.store.commit(changes).await {
            Ok(()) => Ok(()),
            Err(StoreError::Conflict(StoreConflict::ChannelChangeGone)) => {
                match deps.store.channel_change(user.id, change_id).await? {
                    Some(c) if c.status == ChangeStatus::Cancelled => Ok(()),
                    _ => Err(IdentityError::ChannelChangeClosed),
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}

/// Worker: applies holds whose time has come. Each runs in its user's transaction; a hold that
/// can no longer apply (24-h rule, value taken meanwhile) is cancelled by the Core with the
/// generic `not_completed` reason (policy §3).
pub struct ApplyDueChannelHolds {
    deps: IdentityDeps,
}

impl ApplyDueChannelHolds {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// Returns how many holds were completed or cancelled.
    pub async fn handle(&self) -> Result<usize, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let mut done = 0;
        for (user_id, change_id) in deps.store.due_channel_holds(now).await? {
            let (Some(change), Some(user)) = (
                deps.store.channel_change(user_id, change_id).await?,
                deps.store.user(user_id).await?,
            ) else {
                continue;
            };
            if !change.hold_is_due(now) {
                continue;
            }
            let blocked = user
                .channel_changed_at
                .is_some_and(|at| at > change.created_at)
                || value_taken(deps, user_id, &change.new_value).await?;
            let changes = if blocked {
                self.not_completed(&user, &change, now)
            } else {
                let kept = match change.recovery {
                    Some(_) => Some(self.kept_access(user_id).await?),
                    None => None,
                };
                self.apply(&user, &change, kept, now)
            };
            match deps.store.commit(changes).await {
                Ok(()) => done += 1,
                Err(StoreError::Conflict(
                    StoreConflict::ChannelChangeGone
                    | StoreConflict::EmailTaken
                    | StoreConflict::WhatsappTaken,
                )) => {
                    // Cancelled meanwhile, or the value was taken between the check and the commit.
                    if deps
                        .store
                        .commit(self.not_completed(&user, &change, now))
                        .await
                        .is_ok()
                    {
                        done += 1;
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(done)
    }

    /// Source tokens and pairings that a recovery leaves in place (policy §9).
    async fn kept_access(&self, user: UserId) -> Result<(usize, usize), IdentityError> {
        let sources = self.deps.store.sources(user, false).await?.len();
        let pairings = self
            .deps
            .store
            .connections(user, false)
            .await?
            .iter()
            .filter(|c| c.connection.instance.is_some())
            .count();
        Ok((sources, pairings))
    }

    /// `kept`: the counts of a recovery's notice to the new e-mail (`None` for a user change).
    fn apply(
        &self,
        user: &User,
        change: &ChannelChange,
        kept: Option<(usize, usize)>,
        now: DateTime<Utc>,
    ) -> ChangeSet {
        let old = user.destination(change.channel());
        let mut set = ChangeSet::new(user.id)
            .write(Write::FinishHeldChannelChange {
                change_id: change.id,
                at: now,
            })
            .write(Write::SetChannelValue {
                user_id: user.id,
                value: change.new_value.clone(),
                at: now,
            })
            .write(Write::InvalidateUserCodes {
                user_id: user.id,
                reason: InvalidReason::ChannelChanged,
                at: now,
            })
            .write(Write::EndUserSessions {
                user_id: user.id,
                except: None,
                reason: EndReason::ChannelChanged,
                at: now,
            })
            // M2 (Esquadro): the requester may be the attacker, so every device is revoked too.
            .write(Write::RevokeAllDevices {
                user_id: user.id,
                at: now,
            })
            .audit(user_audit(
                user.id,
                Actor::System,
                AuditAction::ChannelChangeCompleted {
                    change_id: change.id,
                    kind: ChangeAudit::of(change),
                    channel: change.channel(),
                    masked_old_value: old.as_ref().map(Destination::masked),
                    masked_new_value: change.new_value.masked(),
                },
                now,
                None,
            ))
            .event(IdentityEvent::SessionsEnded {
                user_id: user.id,
                sessions: EndedSessions::AllExcept(Vec::new()),
                reason: EndReason::ChannelChanged,
                at: now,
            });
        let Some((sources, pairings)) = kept else {
            for e in notice_to_all(user, now, |to| Notice::ChannelChanged {
                to,
                channel: change.channel(),
                masked_new_value: change.new_value.masked(),
            }) {
                set = set.event(e);
            }
            return set;
        };
        // A recovery (Esquadro): the old channels learn only the fact; the new e-mail, now the
        // user's, gets the full notice with what to review.
        for e in notice_to_all(user, now, |to| Notice::RecoveredByAdmin {
            to,
            channel: change.channel(),
            masked_new_value: change.new_value.masked(),
            at: now,
        }) {
            set = set.event(e);
        }
        set.event(IdentityEvent::NoticeRequested {
            user_id: user.id,
            locale: user.locale,
            notice: Notice::AccountRecovered {
                to: change.new_value.clone(),
                sources,
                pairings,
            },
            at: now,
        })
    }

    fn not_completed(&self, user: &User, change: &ChannelChange, now: DateTime<Utc>) -> ChangeSet {
        not_completed(user, change, now)
    }
}

/// The Core closes an open change with the generic `not_completed` reason (policy §3): a hold
/// that can no longer apply, a recovery whose code was failed 5 times, or a user change that an
/// admin recovery replaces.
pub(crate) fn not_completed(user: &User, change: &ChannelChange, now: DateTime<Utc>) -> ChangeSet {
    let mut set = ChangeSet::new(user.id)
        .write(Write::CancelChannelChange {
            change_id: change.id,
            by_session: None,
            at: now,
        })
        .audit(user_audit(
            user.id,
            Actor::System,
            AuditAction::ChannelChangeNotCompleted {
                change_id: change.id,
                kind: ChangeAudit::of(change),
                channel: change.channel(),
            },
            now,
            None,
        ));
    for e in notice_to_all(user, now, |to| Notice::ChannelChangeCancelled {
        to,
        channel: change.channel(),
    }) {
        set = set.event(e);
    }
    set
}
