//! Process configuration from the environment (see `infra/compose/.env.example`).
//!
//! | Variable | Default | Meaning |
//! |---|---|---|
//! | `DATABASE_URL` | required | PostgreSQL connection string (runtime role) |
//! | `CORE_BIND` | `127.0.0.1:8080` | listen address |
//! | `DB_MAX_CONNECTIONS` | `10` | pool size |
//! | `RUST_LOG` | `info` | tracing filter |

use std::net::SocketAddr;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("{0} is required")]
    Missing(&'static str),
    #[error("{0} is invalid")]
    Invalid(&'static str),
}

/// A value that must never reach logs; `Debug` prints `REDACTED`.
#[derive(Clone)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("REDACTED")
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: SecretString,
    pub bind: SocketAddr,
    pub db_max_connections: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|k| std::env::var(k).ok())
    }

    pub fn from_lookup(get: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let database_url = get("DATABASE_URL")
            .filter(|v| !v.is_empty())
            .ok_or(ConfigError::Missing("DATABASE_URL"))?;
        let bind = get("CORE_BIND")
            .unwrap_or_else(|| "127.0.0.1:8080".to_owned())
            .parse()
            .map_err(|_| ConfigError::Invalid("CORE_BIND"))?;
        let db_max_connections = match get("DB_MAX_CONNECTIONS") {
            Some(v) => v
                .parse()
                .map_err(|_| ConfigError::Invalid("DB_MAX_CONNECTIONS"))?,
            None => 10,
        };
        Ok(Self {
            database_url: SecretString::new(database_url),
            bind,
            db_max_connections,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_and_redaction() {
        let c = Config::from_lookup(|k| {
            (k == "DATABASE_URL").then(|| "postgres://u:pw-secret@h/db".to_owned())
        })
        .unwrap();
        assert_eq!(c.bind.to_string(), "127.0.0.1:8080");
        assert_eq!(c.db_max_connections, 10);
        assert!(!format!("{c:?}").contains("pw-secret"));
    }

    #[test]
    fn database_url_is_required() {
        assert!(matches!(
            Config::from_lookup(|_| None),
            Err(ConfigError::Missing("DATABASE_URL"))
        ));
    }
}
