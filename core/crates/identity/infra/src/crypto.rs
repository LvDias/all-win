//! Crypto adapters: argon2id passwords, SHA-256 / HMAC-SHA-256 with a versioned pepper,
//! AES-256-GCM for TOTP seeds, OS-seeded CSPRNG. Keys come from the environment (wired in
//! `app`); none of these types prints key material in `Debug`.

use std::collections::BTreeMap;
use std::fmt;

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::password_hash::phc::PasswordHash as Phc;
use argon2::password_hash::{PasswordHasher as _, PasswordVerifier as _};
use argon2::{Algorithm, Argon2, Params, Version};
use async_trait::async_trait;
use hmac::{Hmac, KeyInit, Mac};
use identity_application::ports::{
    PasswordCheck, PasswordHasher, RandomSource, SecretCipher, SecretHasher,
};
use identity_enterprise::codes::NormalizedCode;
use identity_enterprise::secret::SecretString;
use identity_enterprise::tokens::{CodeHmac, TokenHash};
use identity_enterprise::user::PasswordHash;
use sha2::{Digest, Sha256};

/// argon2id cost (auth policy §2: at least m = 19 MiB, t = 2, p = 1; tuned per VM).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Argon2Cost {
    pub m_kib: u32,
    pub t: u32,
    pub p: u32,
}

impl Argon2Cost {
    pub const MINIMUM: Self = Self {
        m_kib: 19 * 1024,
        t: 2,
        p: 1,
    };
}

pub struct Argon2Passwords {
    cost: Argon2Cost,
    /// Hash of a random value, verified when the e-mail is unknown (same cost as a real check).
    dummy: String,
}

impl Argon2Passwords {
    pub fn new(cost: Argon2Cost) -> Result<Self, String> {
        if cost.m_kib < Argon2Cost::MINIMUM.m_kib || cost.t < Argon2Cost::MINIMUM.t || cost.p < 1 {
            return Err("argon2 cost below the policy minimum".into());
        }
        let mut seed = [0u8; 32];
        rand::fill(&mut seed);
        let dummy = hash_with(cost, &seed)?;
        Ok(Self { cost, dummy })
    }
}

fn argon(cost: Argon2Cost) -> Result<Argon2<'static>, String> {
    let params = Params::new(cost.m_kib, cost.t, cost.p, None).map_err(|e| e.to_string())?;
    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}

fn hash_with(cost: Argon2Cost, password: &[u8]) -> Result<String, String> {
    argon(cost)?
        .hash_password(password)
        .map(|h| h.to_string())
        .map_err(|e| e.to_string())
}

fn verify_with(cost: Argon2Cost, phc: &str, password: &[u8]) -> Result<PasswordCheck, String> {
    let parsed = Phc::new(phc).map_err(|e| e.to_string())?;
    let stored_cost = Params::try_from(&parsed).map_err(|e| e.to_string())?;
    let ok = argon(cost)?.verify_password(password, &parsed).is_ok();
    if !ok {
        return Ok(PasswordCheck::Invalid);
    }
    let weaker = stored_cost.m_cost() < cost.m_kib
        || stored_cost.t_cost() < cost.t
        || stored_cost.p_cost() < cost.p;
    let not_id = parsed.algorithm.as_str() != "argon2id";
    Ok(PasswordCheck::Valid {
        needs_rehash: weaker || not_id,
    })
}

#[async_trait]
impl PasswordHasher for Argon2Passwords {
    async fn hash(&self, password: &SecretString) -> Result<PasswordHash, String> {
        let (cost, pw) = (self.cost, password.expose().as_bytes().to_vec());
        let phc = tokio::task::spawn_blocking(move || hash_with(cost, &pw))
            .await
            .map_err(|e| e.to_string())??;
        Ok(PasswordHash::new(phc))
    }

    async fn verify(
        &self,
        hash: &PasswordHash,
        password: &SecretString,
    ) -> Result<PasswordCheck, String> {
        let (cost, phc, pw) = (
            self.cost,
            hash.as_str().to_owned(),
            password.expose().as_bytes().to_vec(),
        );
        tokio::task::spawn_blocking(move || verify_with(cost, &phc, &pw))
            .await
            .map_err(|e| e.to_string())?
    }

    async fn verify_dummy(&self, password: &SecretString) {
        let (cost, phc, pw) = (
            self.cost,
            self.dummy.clone(),
            password.expose().as_bytes().to_vec(),
        );
        let _ = tokio::task::spawn_blocking(move || verify_with(cost, &phc, &pw)).await;
    }
}

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

/// AES-256-GCM; stored form is `nonce (12) || ciphertext || tag (16)`.
pub struct AesGcmCipher {
    keys: VersionedKeys,
}

const NONCE_LEN: usize = 12;

impl AesGcmCipher {
    /// Keys must be exactly 32 bytes.
    pub fn new(keys: VersionedKeys) -> Result<Self, String> {
        if keys.keys.values().any(|k| k.len() != 32) {
            return Err("AES-256 keys must be 32 bytes".into());
        }
        Ok(Self { keys })
    }

    fn cipher(&self, version: i16) -> Result<Aes256Gcm, String> {
        let key = self.keys.keys.get(&version).ok_or("unknown key version")?;
        Aes256Gcm::new_from_slice(key).map_err(|_| "invalid key".to_owned())
    }
}

impl SecretCipher for AesGcmCipher {
    fn encrypt(&self, plaintext: &[u8]) -> Result<(Vec<u8>, i16), String> {
        let version = self.keys.current;
        let mut nonce_bytes = [0u8; NONCE_LEN];
        rand::fill(&mut nonce_bytes);
        let nonce = Nonce::try_from(&nonce_bytes[..]).map_err(|_| "nonce".to_owned())?;
        let ciphertext = self
            .cipher(version)?
            .encrypt(&nonce, plaintext)
            .map_err(|_| "encryption failed".to_owned())?;
        let mut out = nonce_bytes.to_vec();
        out.extend_from_slice(&ciphertext);
        Ok((out, version))
    }

    fn decrypt(&self, stored: &[u8], key_version: i16) -> Result<Vec<u8>, String> {
        let (nonce_bytes, ciphertext) = stored
            .split_at_checked(NONCE_LEN)
            .ok_or("ciphertext too short")?;
        let nonce = Nonce::try_from(nonce_bytes).map_err(|_| "nonce".to_owned())?;
        self.cipher(key_version)?
            .decrypt(&nonce, ciphertext)
            .map_err(|_| "decryption failed".to_owned())
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

    #[tokio::test]
    async fn argon2id_hash_verify_and_rehash_flag() {
        let p = Argon2Passwords::new(Argon2Cost::MINIMUM).unwrap();
        let pw = SecretString::new("correct horse battery");
        let h = p.hash(&pw).await.unwrap();
        assert!(h.as_str().starts_with("$argon2id$v=19$m=19456,t=2,p=1$"));
        assert_eq!(
            p.verify(&h, &pw).await.unwrap(),
            PasswordCheck::Valid {
                needs_rehash: false
            }
        );
        assert_eq!(
            p.verify(&h, &SecretString::new("wrong")).await.unwrap(),
            PasswordCheck::Invalid
        );
        let stronger = Argon2Passwords::new(Argon2Cost {
            t: 3,
            ..Argon2Cost::MINIMUM
        })
        .unwrap();
        assert_eq!(
            stronger.verify(&h, &pw).await.unwrap(),
            PasswordCheck::Valid { needs_rehash: true }
        );
        p.verify_dummy(&pw).await;
        assert!(
            Argon2Passwords::new(Argon2Cost {
                m_kib: 1024,
                ..Argon2Cost::MINIMUM
            })
            .is_err()
        );
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
    fn aes_gcm_round_trip_and_tamper_detection() {
        let c = AesGcmCipher::new(keys(1, &[1])).unwrap();
        let (ct, v) = c.encrypt(b"totp seed 20 bytes!!").unwrap();
        assert_eq!(ct.len(), 12 + 20 + 16);
        assert_eq!(c.decrypt(&ct, v).unwrap(), b"totp seed 20 bytes!!");
        let mut bad = ct.clone();
        bad[20] ^= 1;
        assert!(c.decrypt(&bad, v).is_err());
        assert!(c.decrypt(&ct, 9).is_err());
        let (ct2, _) = c.encrypt(b"totp seed 20 bytes!!").unwrap();
        assert_ne!(ct, ct2, "fresh nonce per encryption");
    }
}
