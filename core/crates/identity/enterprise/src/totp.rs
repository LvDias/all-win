//! TOTP (auth policy §2): RFC 6238, HMAC-SHA-1, 6 digits, 30 s step, ±1 step, replay protection.

use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, KeyInit, Mac};
use sha1::Sha1;

pub const SECRET_BYTES: usize = 20;
pub const STEP_SECONDS: i64 = 30;
pub const DIGITS: u32 = 6;
pub const TOLERANCE_STEPS: i64 = 1;
/// An unconfirmed enrolment expires after this.
pub const SETUP_LIFETIME: Duration = Duration::minutes(10);
pub const ISSUER: &str = "All Win";

/// Stored TOTP credential (the secret stays encrypted outside the verify call).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TotpCredential {
    pub secret_encrypted: Vec<u8>,
    pub key_version: i16,
    pub created_at: DateTime<Utc>,
    pub enabled_at: Option<DateTime<Utc>>,
    pub last_used_step: Option<i64>,
}

impl TotpCredential {
    pub fn is_enabled(&self) -> bool {
        self.enabled_at.is_some()
    }

    pub fn setup_expires_at(&self) -> DateTime<Utc> {
        self.created_at + SETUP_LIFETIME
    }

    /// A pending enrolment that can still be confirmed.
    pub fn setup_open(&self, now: DateTime<Utc>) -> bool {
        !self.is_enabled() && now < self.setup_expires_at()
    }
}

pub fn step_at(now: DateTime<Utc>) -> i64 {
    now.timestamp().div_euclid(STEP_SECONDS)
}

/// Returns the matched time step when `code` is valid at `now` (±1 step) and newer than
/// `last_used_step`. The caller must persist the step atomically before accepting (replay rule).
pub fn verify(
    secret: &[u8],
    code: &str,
    now: DateTime<Utc>,
    last_used_step: Option<i64>,
) -> Option<i64> {
    if code.len() != DIGITS as usize || !code.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let wanted: u32 = code.parse().ok()?;
    let current = step_at(now);
    let mut matched = None;
    // Evaluate every candidate step so timing does not depend on which one matches.
    for step in (current - TOLERANCE_STEPS).max(0)..=(current + TOLERANCE_STEPS) {
        let fresh = last_used_step.is_none_or(|used| step > used);
        if hotp(secret, step)? == wanted && fresh && matched.is_none() {
            matched = Some(step);
        }
    }
    matched
}

/// Code an authenticator shows at `now` (tests and tooling; verification uses [`verify`]).
pub fn code_at(secret: &[u8], now: DateTime<Utc>) -> Option<String> {
    hotp(secret, step_at(now)).map(|code| format!("{code:06}"))
}

fn hotp(secret: &[u8], step: i64) -> Option<u32> {
    let mut mac = Hmac::<Sha1>::new_from_slice(secret).ok()?;
    mac.update(&u64::try_from(step).ok()?.to_be_bytes());
    let digest = mac.finalize().into_bytes();
    let offset = usize::from(digest[19] & 0x0F);
    let binary = (u32::from(digest[offset]) & 0x7F) << 24
        | u32::from(digest[offset + 1]) << 16
        | u32::from(digest[offset + 2]) << 8
        | u32::from(digest[offset + 3]);
    Some(binary % 10u32.pow(DIGITS))
}

/// RFC 4648 Base32 without padding, as authenticator apps expect.
pub fn base32_secret(secret: &[u8]) -> String {
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut out = String::with_capacity(secret.len().div_ceil(5) * 8);
    let (mut buffer, mut bits) = (0u32, 0u32);
    for byte in secret {
        buffer = (buffer << 8) | u32::from(*byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out.push(char::from(ALPHABET[((buffer >> bits) & 0x1F) as usize]));
        }
    }
    if bits > 0 {
        out.push(char::from(
            ALPHABET[((buffer << (5 - bits)) & 0x1F) as usize],
        ));
    }
    out
}

/// `otpauth://totp/All%20Win:<email>?secret=…&issuer=All%20Win` (contract `startTotpSetup`).
pub fn otpauth_uri(account_email: &str, secret_base32: &str) -> String {
    let issuer = percent_encode(ISSUER);
    format!(
        "otpauth://totp/{issuer}:{}?secret={secret_base32}&issuer={issuer}&algorithm=SHA1&digits={DIGITS}&period={STEP_SECONDS}",
        percent_encode(account_email)
    )
}

fn percent_encode(raw: &str) -> String {
    raw.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                char::from(b).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    const RFC_SECRET: &[u8] = b"12345678901234567890";

    fn at(secs: i64) -> DateTime<Utc> {
        Utc.timestamp_opt(secs, 0).unwrap()
    }

    #[test]
    fn rfc6238_sha1_vectors_truncated_to_6_digits() {
        assert_eq!(verify(RFC_SECRET, "287082", at(59), None), Some(1));
        assert_eq!(
            verify(RFC_SECRET, "081804", at(1_111_111_109), None),
            Some(1_111_111_109 / 30)
        );
        assert_eq!(
            verify(RFC_SECRET, "005924", at(1_234_567_890), None),
            Some(1_234_567_890 / 30)
        );
    }

    #[test]
    fn code_at_matches_verify() {
        assert_eq!(code_at(RFC_SECRET, at(59)).as_deref(), Some("287082"));
    }

    #[test]
    fn accepts_one_step_of_drift_each_way_only() {
        assert!(verify(RFC_SECRET, "287082", at(59 + 30), None).is_some());
        assert!(verify(RFC_SECRET, "287082", at(59 - 30), None).is_some());
        assert!(verify(RFC_SECRET, "287082", at(59 + 60), None).is_none());
    }

    #[test]
    fn replayed_or_older_step_is_rejected() {
        assert_eq!(verify(RFC_SECRET, "287082", at(59), Some(1)), None);
        assert_eq!(verify(RFC_SECRET, "287082", at(59), Some(0)), Some(1));
    }

    #[test]
    fn malformed_codes_are_rejected() {
        for code in ["", "28708", "2870820", "28708a", " 87082"] {
            assert_eq!(verify(RFC_SECRET, code, at(59), None), None, "{code:?}");
        }
    }

    #[test]
    fn base32_and_uri() {
        assert_eq!(
            base32_secret(b"12345678901234567890"),
            "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ"
        );
        assert_eq!(base32_secret(b"f"), "MY");
        let uri = otpauth_uri("ana+x@example.com", "ABC");
        assert!(uri.starts_with(
            "otpauth://totp/All%20Win:ana%2Bx%40example.com?secret=ABC&issuer=All%20Win"
        ));
    }

    #[test]
    fn setup_window() {
        let c = TotpCredential {
            secret_encrypted: vec![],
            key_version: 1,
            created_at: at(0),
            enabled_at: None,
            last_used_step: None,
        };
        assert!(c.setup_open(at(599)));
        assert!(!c.setup_open(at(600)));
    }
}
