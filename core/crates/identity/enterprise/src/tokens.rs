//! High-entropy tokens (ALIGN-3, data model §4.2): 256-bit secrets, stored only as SHA-256.
//!
//! - Session cookie, invitation and reset tokens: 43 base64url characters (32 bytes).
//! - Source (webhook) token `awk_<key_id>_<secret>` and device token `awd_<key_id>_<secret>`:
//!   `key_id` is 12 `[A-Za-z0-9]` characters, public, and may appear in audit/logs.
//!
//! The stored hash is SHA-256 of the 32 decoded secret bytes (computed by the hashing port).

use std::fmt;

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

use crate::secret::SecretString;

pub const SECRET_BYTES: usize = 32;
pub const SECRET_CHARS: usize = 43;
pub const KEY_ID_CHARS: usize = 12;
const KEY_ID_ALPHABET: &[u8; 62] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// SHA-256 of a secret; safe to store and compare.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenHash([u8; 32]);

impl TokenHash {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for TokenHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TokenHash(..)")
    }
}

/// HMAC-SHA-256(pepper, normalized code) for low-entropy human-typed codes (ALIGN-4).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeHmac {
    pub bytes: [u8; 32],
    pub pepper_version: i16,
}

impl fmt::Debug for CodeHmac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeHmac(v{})", self.pepper_version)
    }
}

/// 32 random bytes; the only representation from which a hash is computed.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretBytes([u8; SECRET_BYTES]);

impl SecretBytes {
    pub fn from_random(bytes: [u8; SECRET_BYTES]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; SECRET_BYTES] {
        &self.0
    }

    pub fn encode(&self) -> SecretString {
        SecretString::new(URL_SAFE_NO_PAD.encode(self.0))
    }

    /// Parses the 43-character base64url form; anything else is `None`.
    pub fn decode(encoded: &str) -> Option<Self> {
        if encoded.len() != SECRET_CHARS {
            return None;
        }
        let bytes = URL_SAFE_NO_PAD.decode(encoded).ok()?;
        let array: [u8; SECRET_BYTES] = bytes.try_into().ok()?;
        // Reject non-canonical encodings (trailing bits set), so one secret has one spelling.
        (URL_SAFE_NO_PAD.encode(array) == encoded).then_some(Self(array))
    }
}

impl fmt::Debug for SecretBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretBytes(REDACTED)")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyId(String);

impl KeyId {
    /// Uniform over the 62-letter alphabet (rejection sampling). `fill` supplies random bytes.
    pub fn generate(mut fill: impl FnMut(&mut [u8])) -> Self {
        let mut out = String::with_capacity(KEY_ID_CHARS);
        let mut buf = [0u8; 16];
        while out.len() < KEY_ID_CHARS {
            fill(&mut buf);
            for b in buf {
                // 248 = 4 × 62: values above it would bias the distribution.
                if b < 248 && out.len() < KEY_ID_CHARS {
                    out.push(char::from(KEY_ID_ALPHABET[usize::from(b % 62)]));
                }
            }
        }
        Self(out)
    }

    pub fn parse(raw: &str) -> Option<Self> {
        (raw.len() == KEY_ID_CHARS && raw.bytes().all(|b| b.is_ascii_alphanumeric()))
            .then(|| Self(raw.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrefixedTokenKind {
    /// `awk_`: instruction source (webhook) token.
    Source,
    /// `awd_`: adapter device token.
    Device,
}

impl PrefixedTokenKind {
    fn prefix(self) -> &'static str {
        match self {
            Self::Source => "awk_",
            Self::Device => "awd_",
        }
    }
}

/// `aw?_<key_id>_<secret>`; `Debug` prints only the public `key_id`.
#[derive(Clone, PartialEq, Eq)]
pub struct PrefixedToken {
    kind: PrefixedTokenKind,
    key_id: KeyId,
    secret: SecretBytes,
}

impl PrefixedToken {
    pub fn new(kind: PrefixedTokenKind, key_id: KeyId, secret: SecretBytes) -> Self {
        Self {
            kind,
            key_id,
            secret,
        }
    }

    pub fn parse(kind: PrefixedTokenKind, raw: &str) -> Option<Self> {
        let rest = raw.strip_prefix(kind.prefix())?;
        let (key_id, secret) = rest.split_at_checked(KEY_ID_CHARS)?;
        let secret = secret.strip_prefix('_')?;
        Some(Self {
            kind,
            key_id: KeyId::parse(key_id)?,
            secret: SecretBytes::decode(secret)?,
        })
    }

    pub fn kind(&self) -> PrefixedTokenKind {
        self.kind
    }

    pub fn key_id(&self) -> &KeyId {
        &self.key_id
    }

    pub fn secret(&self) -> &SecretBytes {
        &self.secret
    }

    /// Full token; only ever returned once to the user/plugin.
    pub fn expose(&self) -> SecretString {
        SecretString::new(format!(
            "{}{}_{}",
            self.kind.prefix(),
            self.key_id.as_str(),
            self.secret.encode().expose()
        ))
    }
}

impl fmt::Debug for PrefixedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}_REDACTED", self.kind.prefix(), self.key_id.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn counter_fill() -> impl FnMut(&mut [u8]) {
        let mut n: u8 = 0;
        move |buf: &mut [u8]| {
            for b in buf {
                *b = n;
                n = n.wrapping_add(37);
            }
        }
    }

    #[test]
    fn secret_round_trips_in_43_chars() {
        let s = SecretBytes::from_random([0xAB; 32]);
        let enc = s.encode();
        assert_eq!(enc.expose().len(), SECRET_CHARS);
        assert_eq!(SecretBytes::decode(enc.expose()), Some(s));
        assert_eq!(SecretBytes::decode("too-short"), None);
        assert_eq!(SecretBytes::decode(&"!".repeat(43)), None);
    }

    #[test]
    fn non_canonical_secret_encoding_is_rejected() {
        let mut enc = SecretBytes::from_random([0; 32])
            .encode()
            .expose()
            .to_owned();
        enc.pop();
        enc.push('B'); // sets trailing bits that a canonical encoder never produces
        assert_eq!(SecretBytes::decode(&enc), None);
    }

    #[test]
    fn key_id_is_12_alphanumerics() {
        let k = KeyId::generate(counter_fill());
        assert_eq!(k.as_str().len(), 12);
        assert!(KeyId::parse(k.as_str()).is_some());
        assert!(KeyId::parse("abc").is_none());
        assert!(KeyId::parse("abcdefghijk_").is_none());
    }

    #[test]
    fn key_id_skips_biased_bytes() {
        let mut calls = 0;
        let k = KeyId::generate(|buf: &mut [u8]| {
            calls += 1;
            let v = if calls == 1 { 255 } else { 1 };
            buf.fill(v);
        });
        assert_eq!(k.as_str(), "BBBBBBBBBBBB");
        assert_eq!(calls, 2);
    }

    #[test]
    fn prefixed_tokens_format_parse_and_redact() {
        let key = KeyId::parse("Abc123Def456").unwrap();
        let t = PrefixedToken::new(
            PrefixedTokenKind::Source,
            key,
            SecretBytes::from_random([9; 32]),
        );
        let full = t.expose();
        assert!(full.expose().starts_with("awk_Abc123Def456_"));
        assert_eq!(full.expose().len(), 4 + 12 + 1 + 43);
        assert_eq!(
            PrefixedToken::parse(PrefixedTokenKind::Source, full.expose()),
            Some(t.clone())
        );
        assert_eq!(
            PrefixedToken::parse(PrefixedTokenKind::Device, full.expose()),
            None
        );
        assert_eq!(format!("{t:?}"), "awk_Abc123Def456_REDACTED");
        assert!(PrefixedToken::parse(PrefixedTokenKind::Source, "awk_Abc123Def456").is_none());
    }
}
