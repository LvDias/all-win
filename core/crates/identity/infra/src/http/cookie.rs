//! `__Host-aw_session` (auth policy §5.1): `HttpOnly; Secure; SameSite=Lax; Path=/`, no
//! `Domain`; `Max-Age` only for "keep me signed in". `__Host-aw_device` (§5.2): same attributes,
//! `Max-Age` 400 days (the browser cap), renewed on use.

use axum::http::{HeaderMap, HeaderValue, header};
use chrono::{DateTime, Utc};
use identity_application::views::{DeviceCookie, SessionCookie};

pub const NAME: &str = "__Host-aw_session";
pub const DEVICE: &str = "__Host-aw_device";
const DEVICE_MAX_AGE: i64 = 34_560_000;

pub fn set(cookie: &SessionCookie, now: DateTime<Utc>) -> Option<HeaderValue> {
    let mut v = format!(
        "{NAME}={}; HttpOnly; Secure; SameSite=Lax; Path=/",
        cookie.value.expose()
    );
    if cookie.persistent {
        let secs = (cookie.expires_at - now).num_seconds().max(0);
        v.push_str(&format!("; Max-Age={secs}"));
    }
    HeaderValue::from_str(&v).ok()
}

pub fn set_device(cookie: &DeviceCookie) -> Option<HeaderValue> {
    HeaderValue::from_str(&format!(
        "{DEVICE}={}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age={DEVICE_MAX_AGE}",
        cookie.value.expose()
    ))
    .ok()
}

pub fn clear() -> HeaderValue {
    HeaderValue::from_static(
        "__Host-aw_session=; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=0",
    )
}

pub fn clear_device() -> HeaderValue {
    HeaderValue::from_static("__Host-aw_device=; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=0")
}

/// The session cookie value, if the request carries exactly one.
pub fn read(headers: &HeaderMap) -> Option<String> {
    read_named(headers, NAME)
}

/// The device cookie value, if the request carries exactly one.
pub fn read_device(headers: &HeaderMap) -> Option<String> {
    read_named(headers, DEVICE)
}

fn read_named(headers: &HeaderMap, name: &str) -> Option<String> {
    let mut found = None;
    for h in headers.get_all(header::COOKIE) {
        let Ok(s) = h.to_str() else { continue };
        for pair in s.split(';') {
            if let Some((k, v)) = pair.trim().split_once('=')
                && k == name
            {
                if found.is_some() {
                    return None;
                }
                found = Some(v.to_owned());
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use identity_enterprise::secret::SecretString;

    #[test]
    fn attributes_and_max_age() {
        let now = Utc::now();
        let c = SessionCookie {
            value: SecretString::new("abc"),
            persistent: false,
            expires_at: now,
        };
        assert_eq!(
            set(&c, now).unwrap(),
            "__Host-aw_session=abc; HttpOnly; Secure; SameSite=Lax; Path=/"
        );
        let p = SessionCookie {
            persistent: true,
            expires_at: now + chrono::Duration::days(30),
            ..c
        };
        assert!(
            set(&p, now)
                .unwrap()
                .to_str()
                .unwrap()
                .ends_with("; Max-Age=2592000")
        );
    }

    #[test]
    fn reads_one_cookie_and_rejects_duplicates() {
        let mut h = HeaderMap::new();
        h.insert(
            header::COOKIE,
            "a=1; __Host-aw_session=xyz; b=2".parse().unwrap(),
        );
        assert_eq!(read(&h).as_deref(), Some("xyz"));
        h.insert(
            header::COOKIE,
            "__Host-aw_session=x; __Host-aw_session=y".parse().unwrap(),
        );
        assert_eq!(read(&h), None);
        h.insert(
            header::COOKIE,
            "__Host-aw_device=dev; __Host-aw_session=s".parse().unwrap(),
        );
        assert_eq!(read_device(&h).as_deref(), Some("dev"));
        let d = set_device(&DeviceCookie {
            value: SecretString::new("v"),
        })
        .unwrap();
        assert!(d.to_str().unwrap().ends_with("Path=/; Max-Age=34560000"));
    }
}
