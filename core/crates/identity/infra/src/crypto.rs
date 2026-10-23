//! Crypto adapters: SHA-256, HMAC-SHA-256 with HKDF-derived keys of a versioned pepper (codes,
//! identifiers, sealed challenges), OS-seeded CSPRNG. Keys come from the environment (wired in
//! `app`); none of these types prints key material in `Debug`.

use std::collections::BTreeMap;
use std::fmt;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use hmac::{Hmac, KeyInit, Mac};
use identity_application::ports::{RandomSource, SecretHasher};
use identity_enterprise::codes::NormalizedCode;
use identity_enterprise::ids::CodeId;
use identity_enterprise::otp::{CLAIMS_LEN, ChallengeClaims};
use identity_enterprise::secret::SecretString;
use identity_enterprise::tokens::{CodeHmac, TokenHash};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

/// Secret key material indexed by version; `current` is used for new values.
pub struct VersionedKeys {
    current: i16,
    keys: BTreeMap<i16, Vec<u8>>,
}

impl VersionedKeys {
    /// Every key must be at least 32 bytes; `current` must be present.
    pub fn new(current: i16, keys: BTreeMap<i16, Vec<u8>>) -> Result<Self, String> {
        if !keys.contains_key(&current) {
            return Err("current key version missing".into());
        }
        if keys.values().any(|k| k.len() < 32) {
            return Err("keys must be at least 32 bytes".into());
        }
        Ok(Self { current, keys })
    }
}

impl fmt::Debug for VersionedKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VersionedKeys(current = {}, versions = {:?})",
            self.current,
            self.keys.keys().collect::<Vec<_>>()
        )
    }
}

/// HKDF `info` labels (auth policy rev. 2 §2.3): one derived key per use, so a leak of one never
/// helps with another.
const INFO_CODE: &[u8] = b"allwin/otp-code/v1";
const INFO_IDENT: &[u8] = b"allwin/identifier/v1";
const INFO_CHALLENGE: &[u8] = b"allwin/challenge/v1";
const INFO_PAIRING: &[u8] = b"allwin/pairing-code/v1";

fn hmac(key: &[u8], data: &[u8]) -> [u8; 32] {
    // HMAC accepts keys of any length.
    let mut mac = <Hmac<Sha256> as KeyInit>::new_from_slice(key).unwrap_or_else(|_| unreachable!());
    mac.update(data);
    mac.finalize().into_bytes().into()
}

/// HKDF-SHA-256 (RFC 5869) with an empty salt and one 32-byte output block.
fn hkdf(ikm: &[u8], info: &[u8]) -> [u8; 32] {
    let prk = hmac(&[0u8; 32], ikm);
    let mut t = info.to_vec();
    t.push(1);
    hmac(&prk, &t)
}

/// Keys derived from one pepper version.
struct DerivedKeys {
    code: [u8; 32],
    ident: [u8; 32],
    challenge: [u8; 32],
    pairing: [u8; 32],
}

impl DerivedKeys {
    fn from_pepper(pepper: &[u8]) -> Self {
        Self {
            code: hkdf(pepper, INFO_CODE),
            ident: hkdf(pepper, INFO_IDENT),
            challenge: hkdf(pepper, INFO_CHALLENGE),
            pairing: hkdf(pepper, INFO_PAIRING),
        }
    }
}

/// SHA-256 for 256-bit tokens; HMAC-SHA-256 with keys derived from the versioned server pepper
/// for human-typed codes, identifiers and challenge tokens.
pub struct PepperedHasher {
    current: i16,
    keys: BTreeMap<i16, DerivedKeys>,
}

impl PepperedHasher {
    pub fn new(peppers: VersionedKeys) -> Self {
        Self {
            current: peppers.current,
            keys: peppers
                .keys
                .iter()
                .map(|(v, k)| (*v, DerivedKeys::from_pepper(k)))
                .collect(),
        }
    }

    fn current_keys(&self) -> Option<&DerivedKeys> {
        self.keys.get(&self.current)
    }

    fn with_current(&self, pick: impl Fn(&DerivedKeys) -> &[u8; 32], data: &[u8]) -> CodeHmac {
        let key = self.current_keys().map(&pick);
        CodeHmac {
            bytes: key.map_or([0u8; 32], |k| hmac(k, data)),
            pepper_version: self.current,
        }
    }
}

fn otp_input(code_id: CodeId, code: &NormalizedCode) -> Vec<u8> {
    let mut data = code_id.as_uuid().as_bytes().to_vec();
    data.extend_from_slice(code.as_bytes());
    data
}

impl SecretHasher for PepperedHasher {
    fn sha256(&self, bytes: &[u8]) -> TokenHash {
        TokenHash::from_bytes(Sha256::digest(bytes).into())
    }

    fn code_hmac(&self, code: &NormalizedCode) -> CodeHmac {
        self.with_current(|k| &k.pairing, code.as_bytes())
    }

    fn code_hmac_candidates(&self, code: &NormalizedCode) -> Vec<CodeHmac> {
        self.keys
            .iter()
            .map(|(version, k)| CodeHmac {
                bytes: hmac(&k.pairing, code.as_bytes()),
                pepper_version: *version,
            })
            .collect()
    }

    fn otp_hmac(&self, code_id: CodeId, code: &NormalizedCode) -> CodeHmac {
        self.with_current(|k| &k.code, &otp_input(code_id, code))
    }

    fn otp_hmac_with(
        &self,
        code_id: CodeId,
        code: &NormalizedCode,
        version: i16,
    ) -> Option<CodeHmac> {
        self.keys.get(&version).map(|k| CodeHmac {
            bytes: hmac(&k.code, &otp_input(code_id, code)),
            pepper_version: version,
        })
    }

    fn identifier_hmac(&self, normalized_identifier: &str) -> CodeHmac {
        self.with_current(|k| &k.ident, normalized_identifier.as_bytes())
    }

    fn seal_challenge(&self, claims: &ChallengeClaims) -> SecretString {
        let bytes = claims.to_bytes();
        let mac = self
            .current_keys()
            .map_or([0u8; 32], |k| hmac(&k.challenge, &bytes));
        let mut out = bytes.to_vec();
        out.extend_from_slice(&self.current.to_be_bytes());
        out.extend_from_slice(&mac);
        SecretString::new(URL_SAFE_NO_PAD.encode(out))
    }

    fn open_challenge(&self, token: &str) -> Option<ChallengeClaims> {
        let raw = URL_SAFE_NO_PAD.decode(token).ok()?;
        if raw.len() != CLAIMS_LEN + 2 + 32 {
            return None;
        }
        let (bytes, rest) = raw.split_at(CLAIMS_LEN);
        let (version, mac) = rest.split_at(2);
        let version = i16::from_be_bytes([version[0], version[1]]);
        let expected = hmac(&self.keys.get(&version)?.challenge, bytes);
        if !bool::from(expected.ct_eq(mac)) {
            return None;
        }
        ChallengeClaims::from_bytes(bytes)
    }

    fn mac_eq(&self, a: &CodeHmac, b: &CodeHmac) -> bool {
        a.pepper_version == b.pepper_version && bool::from(a.bytes.ct_eq(&b.bytes))
    }
}

/// Thread-local CSPRNG seeded from the OS.
pub struct OsRandom;

impl RandomSource for OsRandom {
    fn fill(&self, buf: &mut [u8]) {
        rand::fill(buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(current: i16, versions: &[i16]) -> VersionedKeys {
        VersionedKeys::new(
            current,
            versions
                .iter()
                .map(|v| (*v, vec![u8::try_from(*v).unwrap(); 32]))
                .collect(),
        )
        .unwrap()
    }

    #[test]
    fn sha256_known_vector() {
        let h = PepperedHasher::new(keys(1, &[1]));
        let d = h.sha256(b"abc");
        assert_eq!(d.as_bytes()[..4], [0xba, 0x78, 0x16, 0xbf]);
    }

    #[test]
    fn code_hmac_depends_on_pepper_and_covers_rotation() {
        let code =
            NormalizedCode::parse(identity_enterprise::codes::CodeKind::Pairing, "K7Q2-0111")
                .unwrap();
        let old = PepperedHasher::new(keys(1, &[1]));
        let rotated = PepperedHasher::new(keys(2, &[1, 2]));
        let stored = old.code_hmac(&code);
        assert_eq!(stored.pepper_version, 1);
        assert_ne!(rotated.code_hmac(&code).bytes, stored.bytes);
        assert!(rotated.code_hmac_candidates(&code).contains(&stored));
        assert!(!format!("{:?}", keys(1, &[1])).contains("1, 1"));
    }

    #[test]
    fn hkdf_matches_rfc5869_shape_and_separates_uses() {
        let h = PepperedHasher::new(keys(1, &[1]));
        let code =
            NormalizedCode::parse(identity_enterprise::codes::CodeKind::OneTime, "K7Q2-0111")
                .unwrap();
        let id = CodeId::new_v7();
        let otp = h.otp_hmac(id, &code);
        assert_ne!(
            otp.bytes,
            h.code_hmac(&code).bytes,
            "pairing and OTP keys differ"
        );
        assert_ne!(
            otp.bytes,
            h.otp_hmac(CodeId::new_v7(), &code).bytes,
            "bound to the challenge"
        );
        assert_eq!(h.otp_hmac_with(id, &code, 1), Some(otp));
        assert!(h.otp_hmac_with(id, &code, 9).is_none());
        assert!(h.mac_eq(&otp, &otp));
        assert_ne!(
            h.identifier_hmac("a@b.c").bytes,
            h.identifier_hmac("a@b.d").bytes
        );
    }

    #[test]
    fn sealed_challenges_open_only_unaltered_and_survive_rotation() {
        use chrono::TimeZone;
        use identity_enterprise::otp::{DeviceClass, Purpose};
        let old = PepperedHasher::new(keys(1, &[1]));
        let claims = ChallengeClaims {
            code_id: CodeId::new_v7(),
            purpose: Purpose::Login,
            identifier_hmac: [3; 32],
            pepper_version: 1,
            device_class: DeviceClass::Unknown,
            issued_at: chrono::Utc.with_ymd_and_hms(2026, 10, 23, 9, 0, 0).unwrap(),
        };
        let token = old.seal_challenge(&claims);
        let t = token.expose();
        assert!(
            t.len() <= 256
                && t.chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        );
        assert_eq!(old.open_challenge(t), Some(claims.clone()));
        let mut altered = t.to_owned();
        let last = if altered.ends_with('A') { "B" } else { "A" };
        altered.replace_range(altered.len() - 1.., last);
        assert!(old.open_challenge(&altered).is_none());
        assert!(old.open_challenge("not a token").is_none());
        let rotated = PepperedHasher::new(keys(2, &[1, 2]));
        assert_eq!(rotated.open_challenge(t), Some(claims));
        let other = PepperedHasher::new(keys(7, &[7]));
        assert!(other.open_challenge(t).is_none());
    }
}
