//! Crypto adapters: SHA-256 / HMAC-SHA-256 with a versioned pepper, OS-seeded CSPRNG. Keys come
//! from the environment (wired in `app`); none of these types prints key material in `Debug`.

use std::collections::BTreeMap;
use std::fmt;

use hmac::{Hmac, KeyInit, Mac};
use identity_application::ports::{RandomSource, SecretHasher};
use identity_enterprise::codes::NormalizedCode;
use identity_enterprise::tokens::{CodeHmac, TokenHash};
use sha2::{Digest, Sha256};

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

/// SHA-256 for 256-bit tokens; HMAC-SHA-256 with a server pepper for human-typed codes.
pub struct PepperedHasher {
    peppers: VersionedKeys,
}

impl PepperedHasher {
    pub fn new(peppers: VersionedKeys) -> Self {
        Self { peppers }
    }

    fn hmac(key: &[u8], data: &[u8]) -> [u8; 32] {
        // HMAC accepts keys of any length.
        let mut mac =
            <Hmac<Sha256> as KeyInit>::new_from_slice(key).unwrap_or_else(|_| unreachable!());
        mac.update(data);
        mac.finalize().into_bytes().into()
    }
}

impl SecretHasher for PepperedHasher {
    fn sha256(&self, bytes: &[u8]) -> TokenHash {
        TokenHash::from_bytes(Sha256::digest(bytes).into())
    }

    fn code_hmac(&self, code: &NormalizedCode) -> CodeHmac {
        let version = self.peppers.current;
        let key = self
            .peppers
            .keys
            .get(&version)
            .map(Vec::as_slice)
            .unwrap_or_default();
        CodeHmac {
            bytes: Self::hmac(key, code.as_bytes()),
            pepper_version: version,
        }
    }

    fn code_hmac_candidates(&self, code: &NormalizedCode) -> Vec<CodeHmac> {
        self.peppers
            .keys
            .iter()
            .map(|(version, key)| CodeHmac {
                bytes: Self::hmac(key, code.as_bytes()),
                pepper_version: *version,
            })
            .collect()
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
}
