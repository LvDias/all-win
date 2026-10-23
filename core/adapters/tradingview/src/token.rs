//! Source token `awk_<key_id>_<secret>` (ADR-0003 §7, ALIGN-3).
//!
//! The secret never leaves this type: `Debug`/`Display` print
//! `awk_<key_id>_REDACTED`, and only the SHA-256 of the decoded bytes is compared.

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

const PREFIX: &str = "awk_";
const KEY_ID_LEN: usize = 12;
const SECRET_LEN: usize = 43;
const TOKEN_LEN: usize = PREFIX.len() + KEY_ID_LEN + 1 + SECRET_LEN;

pub struct IngressToken {
    key_id: String,
    secret: [u8; 32],
}

impl IngressToken {
    /// Accepts only the canonical form `^awk_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}$` whose
    /// secret decodes to exactly 32 bytes with zero trailing bits. Percent-encoded or
    /// otherwise altered tokens are rejected, so a near-miss never authenticates.
    pub fn parse(raw: &str) -> Option<Self> {
        if raw.len() != TOKEN_LEN {
            return None;
        }
        let rest = raw.strip_prefix(PREFIX)?;
        let (key_id, rest) = rest.split_at_checked(KEY_ID_LEN)?;
        let secret = rest.strip_prefix('_')?;
        if !key_id.bytes().all(|b| b.is_ascii_alphanumeric())
            || !secret
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return None;
        }
        // The default engine rejects non-zero trailing bits (canonical encoding only).
        let bytes = URL_SAFE_NO_PAD.decode(secret).ok()?;
        let secret: [u8; 32] = bytes.try_into().ok()?;
        Some(Self {
            key_id: key_id.to_owned(),
            secret,
        })
    }

    /// Public part; the only identifying value allowed in logs and audit.
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Constant-time comparison of SHA-256(secret) with the stored hash.
    pub fn matches(&self, stored_hash: &[u8]) -> bool {
        let digest = Sha256::digest(self.secret);
        stored_hash.len() == digest.len() && bool::from(digest.as_slice().ct_eq(stored_hash))
    }
}

impl std::fmt::Debug for IngressToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{PREFIX}{}_REDACTED", self.key_id)
    }
}

impl std::fmt::Display for IngressToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Drop for IngressToken {
    fn drop(&mut self) {
        self.secret = [0; 32];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET_BYTES: [u8; 32] = [7; 32];

    fn token() -> String {
        format!("awk_AbCdEf123456_{}", URL_SAFE_NO_PAD.encode(SECRET_BYTES))
    }

    #[test]
    fn canonical_token_parses_and_matches_its_hash() {
        let t = IngressToken::parse(&token()).unwrap();
        assert_eq!(t.key_id(), "AbCdEf123456");
        assert!(t.matches(Sha256::digest(SECRET_BYTES).as_slice()));
        assert!(!t.matches(Sha256::digest([8u8; 32]).as_slice()));
        assert!(!t.matches(&[0u8; 31]));
    }

    #[test]
    fn debug_and_display_redact_the_secret() {
        let raw = token();
        let t = IngressToken::parse(&raw).unwrap();
        let secret = &raw[17..];
        for shown in [format!("{t:?}"), format!("{t}")] {
            assert_eq!(shown, "awk_AbCdEf123456_REDACTED");
            assert!(!shown.contains(secret));
        }
    }

    #[test]
    fn near_misses_are_rejected() {
        let raw = token();
        let secret = &raw[17..];
        let cases = [
            format!("{raw}X"),
            raw[..raw.len() - 1].to_owned(),
            format!("awk_AbCdEf123456%5F{secret}"),
            format!("awk_AbCdEf123456_{}%2B{}", &secret[..20], &secret[20..]),
            format!("AWK_AbCdEf123456_{secret}"),
            format!("awk_AbCdEf12345!_{secret}"),
            secret.to_owned(),
            format!("{raw}/"),
            String::new(),
        ];
        for c in cases {
            assert!(IngressToken::parse(&c).is_none(), "{c}");
        }
    }

    #[test]
    fn non_canonical_trailing_bits_are_rejected() {
        let raw = token();
        // Last char of a 32-byte base64url body carries 2 padding bits; flip one.
        let last = raw.as_bytes()[raw.len() - 1];
        let alt = if last == b'w' { 'x' } else { 'B' };
        let altered = format!("{}{alt}", &raw[..raw.len() - 1]);
        assert!(IngressToken::parse(&altered).is_none());
    }
}
