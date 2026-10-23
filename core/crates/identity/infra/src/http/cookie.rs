//! `__Host-aw_session` (auth policy §1): `HttpOnly; Secure; SameSite=Lax; Path=/`, no
//! `Domain`; `Max-Age` only for "keep me signed in".

use axum::http::{HeaderMap, HeaderValue, header};
use chrono::{DateTime, Utc};
use identity_application::views::SessionCookie;

pub const NAME: &str = "__Host-aw_session";

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

pub fn clear() -> HeaderValue {
    HeaderValue::from_static(
        "__Host-aw_session=; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=0",
    )
}

/// The session cookie value, if the request carries exactly one.
pub fn read(headers: &HeaderMap) -> Option<String> {
    let mut found = None;
    for h in headers.get_all(header::COOKIE) {
        let Ok(s) = h.to_str() else { continue };
        for pair in s.split(';') {
            if let Some((k, v)) = pair.trim().split_once('=')
                && k == NAME
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
    }
}
