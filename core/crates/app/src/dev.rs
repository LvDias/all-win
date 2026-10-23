//! Development and test commands (simulation only): seed a user with a TradingView
//! source key, and revoke a key. Enabled only with `ALLWIN_ENABLE_DEV_COMMANDS=true`
//! and run as `allwin_owner` (`MIGRATION_DATABASE_URL`).
//!
//! The key is created the way S3 creates it: only `key_id` and SHA-256 of the 32 decoded
//! secret bytes are stored; the token is returned once and never logged or written to a file.

use anyhow::{Context, bail};
use base64::Engine;
use base64::engine::general_purpose::{STANDARD_NO_PAD, URL_SAFE_NO_PAD};
use rand::{Rng, RngExt};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

const SOURCE_NAME: &str = "tradingview-seed";
const KEY_ID_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// Creates (or reuses) the user `seed-<label>@example.invalid` and its TradingView source,
/// rotates the source key and returns the new token `awk_<key_id>_<secret>`.
pub async fn seed(owner: &PgPool, label: &str) -> anyhow::Result<String> {
    if label.is_empty()
        || label.len() > 32
        || !label
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        bail!("SEED_LABEL must match ^[a-z0-9-]{{1,32}}$");
    }
    let email = format!("seed-{label}@example.invalid");
    let mut tx = owner.begin().await.context("begin")?;

    let existing = sqlx::query_scalar!(
        "SELECT id FROM identity.app_user WHERE email_normalized = $1",
        email
    )
    .fetch_optional(&mut *tx)
    .await
    .context("find user")?;
    let user_id = existing.unwrap_or_else(Uuid::now_v7);
    sqlx::query!(
        "SELECT set_config('app.user_id', $1, true)",
        user_id.to_string()
    )
    .fetch_one(&mut *tx)
    .await
    .context("user context")?;
    if existing.is_none() {
        sqlx::query!(
            r#"INSERT INTO identity.app_user (id, email, email_normalized, password_hash, locale, time_zone)
               VALUES ($1, $2, $2, $3, 'en', 'UTC')"#,
            user_id,
            email,
            unusable_password_hash(),
        )
        .execute(&mut *tx)
        .await
        .context("insert user")?;
    }

    let source_id = match sqlx::query_scalar!(
        r#"SELECT id FROM identity.source
           WHERE user_id = $1 AND source_type = 'tradingview' AND archived_at IS NULL AND lower(name) = $2"#,
        user_id,
        SOURCE_NAME
    )
    .fetch_optional(&mut *tx)
    .await
    .context("find source")?
    {
        Some(id) => id,
        None => {
            let id = Uuid::now_v7();
            sqlx::query!(
                "INSERT INTO identity.source (id, user_id, source_type, name) VALUES ($1, $2, 'tradingview', $3)",
                id,
                user_id,
                SOURCE_NAME
            )
            .execute(&mut *tx)
            .await
            .context("insert source")?;
            id
        }
    };

    // Rotation = revoke + insert in one transaction.
    sqlx::query!(
        "UPDATE identity.source_key SET revoked_at = now() WHERE source_id = $1 AND revoked_at IS NULL",
        source_id
    )
    .execute(&mut *tx)
    .await
    .context("revoke previous key")?;

    let mut rng = rand::rng();
    let key_id: String = (0..12)
        .map(|_| char::from(KEY_ID_ALPHABET[rng.random_range(0..KEY_ID_ALPHABET.len())]))
        .collect();
    let mut secret = [0u8; 32];
    rng.fill_bytes(&mut secret);
    let secret_hash = Sha256::digest(secret);
    sqlx::query!(
        r#"INSERT INTO identity.source_key (id, user_id, source_id, key_id, secret_hash)
           VALUES ($1, $2, $3, $4, $5)"#,
        Uuid::now_v7(),
        user_id,
        source_id,
        key_id,
        secret_hash.as_slice(),
    )
    .execute(&mut *tx)
    .await
    .context("insert key")?;
    tx.commit().await.context("commit")?;

    let token = format!("awk_{key_id}_{}", URL_SAFE_NO_PAD.encode(secret));
    secret = [0; 32];
    let _ = secret;
    Ok(token)
}

/// Revokes the key; `false` when no active key has this `key_id`.
pub async fn revoke_key(owner: &PgPool, key_id: &str) -> anyhow::Result<bool> {
    let mut tx = owner.begin().await.context("begin")?;
    let Some(user_id) = sqlx::query_scalar!(
        "SELECT user_id FROM identity.source_key WHERE key_id = $1",
        key_id
    )
    .fetch_optional(&mut *tx)
    .await
    .context("find key")?
    else {
        return Ok(false);
    };
    sqlx::query!(
        "SELECT set_config('app.user_id', $1, true)",
        user_id.to_string()
    )
    .fetch_one(&mut *tx)
    .await
    .context("user context")?;
    let revoked = sqlx::query!(
        "UPDATE identity.source_key SET revoked_at = now() WHERE key_id = $1 AND revoked_at IS NULL",
        key_id
    )
    .execute(&mut *tx)
    .await
    .context("revoke")?
    .rows_affected();
    tx.commit().await.context("commit")?;
    Ok(revoked == 1)
}

/// A well-formed argon2id PHC string over random bytes: no password verifies against it.
fn unusable_password_hash() -> String {
    let mut rng = rand::rng();
    let mut salt = [0u8; 16];
    let mut hash = [0u8; 32];
    rng.fill_bytes(&mut salt);
    rng.fill_bytes(&mut hash);
    format!(
        "$argon2id$v=19$m=19456,t=2,p=1${}${}",
        STANDARD_NO_PAD.encode(salt),
        STANDARD_NO_PAD.encode(hash)
    )
}
