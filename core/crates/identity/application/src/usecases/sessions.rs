//! Session start and device recognition shared by sign-in and sign-up (auth policy rev. 2 §5).

use chrono::{DateTime, Utc};
use identity_enterprise::channel::Channel;
use identity_enterprise::device::{KnownDevice, label_from_user_agent};
use identity_enterprise::ids::{DeviceId, SessionId};
use identity_enterprise::secret::SecretString;
use identity_enterprise::session::{ClientInfo, NewSession, Session};
use identity_enterprise::tokens::{SecretBytes, TokenHash};
use shared_kernel::UserId;

use crate::changes::Write;
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::views::{DeviceCookie, SessionCookie};

/// Which device a sign-in runs on, and the writes that record it.
pub(crate) struct DeviceResolution {
    pub device_id: DeviceId,
    /// No valid device cookie of this user came with the request.
    pub new_device: bool,
    pub label: String,
    pub writes: Vec<Write>,
    /// Value to set (renewed on use, policy §5.2).
    pub cookie: DeviceCookie,
}

/// Recognizes the browser for `user` (policy §5.2, data model §4.1):
/// - a non-revoked row of this user → known (touched, cookie renewed);
/// - a value valid only for another user (shared browser) → kept, and a row with the same hash
///   is inserted for this user;
/// - absent, unknown to everyone, or revoked for this user → a new random value and a new row.
pub(crate) async fn resolve_device(
    deps: &IdentityDeps,
    user: UserId,
    presented: Option<&str>,
    meta: &RequestMeta,
    now: DateTime<Utc>,
) -> Result<DeviceResolution, IdentityError> {
    let presented = presented.and_then(SecretBytes::decode);
    if let Some(secret) = &presented {
        let hash = deps.hasher.sha256(secret.as_bytes());
        match deps.store.device_by_key(user, &hash).await? {
            Some(d) if !d.is_revoked() => {
                return Ok(DeviceResolution {
                    device_id: d.id,
                    new_device: false,
                    label: d.display_label().to_owned(),
                    writes: vec![Write::TouchDevice {
                        user_id: user,
                        device_id: d.id,
                        ip: Some(meta.ip),
                        at: now,
                    }],
                    cookie: DeviceCookie {
                        value: secret.encode(),
                    },
                });
            }
            Some(_) => {}
            None => {
                if deps.store.device_key_in_use(&hash).await? {
                    return Ok(new_row(user, hash, secret.encode(), meta, now));
                }
            }
        }
    }
    let secret = SecretBytes::from_random(deps.random_bytes());
    let hash = deps.hasher.sha256(secret.as_bytes());
    Ok(new_row(user, hash, secret.encode(), meta, now))
}

fn new_row(
    user: UserId,
    hash: TokenHash,
    value: SecretString,
    meta: &RequestMeta,
    now: DateTime<Utc>,
) -> DeviceResolution {
    let device = KnownDevice::new(
        DeviceId::new_v7(),
        user,
        hash,
        meta.user_agent.as_deref(),
        Some(meta.ip),
        now,
    );
    DeviceResolution {
        device_id: device.id,
        new_device: true,
        label: device.display_label().to_owned(),
        writes: vec![Write::InsertDevice(device)],
        cookie: DeviceCookie { value },
    }
}

/// Whether the presented device cookie maps to a non-revoked device of `user`; for an unknown
/// identifier the same query runs with an id that matches nothing (policy §3).
pub(crate) async fn device_class_known(
    deps: &IdentityDeps,
    user: Option<UserId>,
    presented: Option<&str>,
) -> Result<bool, IdentityError> {
    let Some(secret) = presented.and_then(SecretBytes::decode) else {
        return Ok(false);
    };
    let hash = deps.hasher.sha256(secret.as_bytes());
    let subject = user.unwrap_or_else(UserId::new_v7);
    let row = deps.store.device_by_key(subject, &hash).await?;
    Ok(user.is_some() && row.is_some_and(|d| !d.is_revoked()))
}

pub(crate) fn new_session(
    deps: &IdentityDeps,
    user_id: UserId,
    device_id: DeviceId,
    login_channel: Channel,
    persistent: bool,
    meta: &RequestMeta,
    now: DateTime<Utc>,
) -> (Session, SessionCookie) {
    let token = deps.issue_token();
    let session = Session::start(
        NewSession {
            id: SessionId::new_v7(),
            user_id,
            device_id,
            token_hash: token.hash,
            persistent,
            login_channel,
            client: ClientInfo {
                ip: Some(meta.ip),
                user_agent: meta.user_agent.clone(),
            },
        },
        now,
    );
    let cookie = SessionCookie {
        value: token.value(),
        persistent,
        expires_at: session.absolute_expires_at,
    };
    (session, cookie)
}

/// Rotates `session` (channel change): new id and cookie, same device and login time.
pub(crate) fn rotate_session(
    deps: &IdentityDeps,
    session: &Session,
    now: DateTime<Utc>,
) -> (Session, SessionCookie) {
    let token = deps.issue_token();
    let rotated = session.rotate(SessionId::new_v7(), token.hash, now);
    let cookie = SessionCookie {
        value: token.value(),
        persistent: rotated.persistent,
        expires_at: rotated.absolute_expires_at,
    };
    (rotated, cookie)
}

/// Coarse device label of the requester (login code message, audit).
pub(crate) fn request_label(meta: &RequestMeta) -> Option<String> {
    meta.user_agent.as_deref().map(label_from_user_agent)
}
