//! Process configuration from the environment (see `infra/compose/.env.example`).
//!
//! | Variable | Default | Meaning |
//! |---|---|---|
//! | `DATABASE_URL` | required | PostgreSQL, role `allwin_app` (RLS enforced) |
//! | `WORKER_DATABASE_URL` | `DATABASE_URL` | role `allwin_worker` (BYPASSRLS) for the outbox dispatcher |
//! | `MIGRATION_DATABASE_URL` | unset | role `allwin_owner`; when set, migrations run at startup and the dev seed uses it |
//! | `CORE_BIND` | `127.0.0.1:8080` | listen address |
//! | `DB_MAX_CONNECTIONS` | `10` | pool size |
//! | `INGRESS_TRUST_FORWARDED_FOR` | `false` | client IP from the last `X-Forwarded-For` hop; only when Caddy sets it and the Core is reachable only through Caddy |
//! | `INGRESS_IP_FLOOD_PER_MIN` | `3000` | step 1: every request, per IPv4 address / IPv6 /64 |
//! | `INGRESS_TRADINGVIEW_FLOOD_PER_MIN` | `30000` | step 1 for addresses in the TradingView ranges |
//! | `INGRESS_TRADINGVIEW_SOURCE_RANGES` | empty | comma-separated CIDRs of TradingView's webhook IPs (IPv4 /16 or narrower, IPv6 /32 or narrower); non-empty requires `INGRESS_TRUST_FORWARDED_FOR=true` |
//! | `INGRESS_IP_TOKEN_FAILURES_PER_MIN` | `30` | step 2: token failures per IP before the identical 429 |
//! | `INGRESS_RATE_PER_KEY_PER_MIN` | `60` | step 3: per authenticated key |
//! | `INGRESS_UNKNOWN_KEY_CACHE_TTL_S` | `60` | step 2: negative cache of unknown `key_id`s (0 disables) |
//! | `APP_PUBLIC_ORIGIN` | unset | exact origin of the web app (`https://host[:port]`) for the `/api/v1` CSRF `Origin` check; required with `IDENTITY_PEPPERS` |
//! | `APP_ENV` | `production` | `development`, `test` or `production` |
//! | `IDENTITY_PEPPERS` | unset | versioned server peppers, `VERSION:BASE64URL[,…]` (each at least 32 bytes); unset = the identity routes are not mounted |
//! | `IDENTITY_PEPPER_CURRENT` | highest version | pepper version for new codes and tokens |
//! | `STEP_UP_Q35_PERSISTENCE_ACTIONS` | `true` | `false` waives the conditional step-ups (Q35); missing or invalid = `true` (fail closed); logged at startup |
//! | `CODE_SENDER` | `disabled` | `disabled` (no provider: every code delivery fails) or `dev_smtp` (only with `APP_ENV` development or test) |
//! | `DEV_SMTP_ADDR` | `127.0.0.1:1025` | SMTP of the development mail catcher (Mailpit), for `CODE_SENDER=dev_smtp` |
//! | `DEV_SMTP_FROM` | `core@allwin.local` | sender address of development mail |
//! | `IDENTITY_ADAPTER_MIN_VERSIONS` | `ninjatrader8=1.0.0` | minimum plugin version per adapter type, `type=semver[,…]`; development builds pair only outside production |
//! | `RUST_LOG` | `info` | tracing filter |
//!
//! Limits burst up to their per-minute value (s1-webhook-ingest §2, HTTP 1.1.2).

use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("{0} is required")]
    Missing(&'static str),
    #[error("{0} is invalid")]
    Invalid(&'static str),
    #[error("{0}")]
    Conflict(&'static str),
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
    pub worker_database_url: SecretString,
    pub migration_database_url: Option<SecretString>,
    pub bind: SocketAddr,
    pub db_max_connections: u32,
    pub trust_forwarded_for: bool,
    pub ip_flood_per_min: u32,
    pub tradingview_flood_per_min: u32,
    pub tradingview_source_ranges: Vec<adapter_tradingview::Cidr>,
    pub ip_token_failures_per_min: u32,
    pub rate_per_key_per_min: u32,
    pub unknown_key_cache_ttl_s: u64,
    /// Origin allowed on state-changing `/api/v1` requests (`scheme://host[:port]`).
    pub public_origin: Option<String>,
    pub app_env: AppEnv,
    /// Identity (S3); `None` when `IDENTITY_PEPPERS` is unset.
    pub identity: Option<IdentityConfig>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEnv {
    Development,
    Test,
    Production,
}

impl AppEnv {
    pub fn is_production(self) -> bool {
        self == Self::Production
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeSenderConfig {
    /// No provider (Q8/U17): every code delivery fails, notices are dropped with a warning.
    Disabled,
    /// Plain SMTP to a development mail catcher; refused outside development and test.
    DevSmtp {
        host: String,
        port: u16,
        from: String,
    },
}

#[derive(Debug, Clone)]
pub struct IdentityConfig {
    /// `(version, key)` pairs; the keys are secrets.
    pub peppers: Vec<(i16, SecretBytes)>,
    pub pepper_current: i16,
    pub step_up_q35_persistence_actions: bool,
    pub code_sender: CodeSenderConfig,
    pub adapter_min_versions: Vec<(String, String)>,
}

/// Key bytes that never reach logs.
#[derive(Clone)]
pub struct SecretBytes(Vec<u8>);

impl SecretBytes {
    pub fn expose(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Debug for SecretBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("REDACTED")
    }
}

fn identity_config(
    non_empty: &impl Fn(&str) -> Option<String>,
    app_env: AppEnv,
) -> Result<Option<IdentityConfig>, ConfigError> {
    use base64::Engine;
    let Some(raw) = non_empty("IDENTITY_PEPPERS") else {
        return Ok(None);
    };
    let peppers = raw
        .split(',')
        .map(|entry| {
            let (v, key) = entry.trim().split_once(':')?;
            let version: i16 = v.parse().ok()?;
            let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(key.trim_end_matches('='))
                .ok()?;
            (bytes.len() >= 32).then_some((version, SecretBytes(bytes)))
        })
        .collect::<Option<Vec<_>>>()
        .filter(|p| !p.is_empty())
        .ok_or(ConfigError::Invalid("IDENTITY_PEPPERS"))?;
    let highest = peppers.iter().map(|(v, _)| *v).max().unwrap_or_default();
    let pepper_current = match non_empty("IDENTITY_PEPPER_CURRENT") {
        Some(v) => v
            .parse()
            .ok()
            .filter(|c| peppers.iter().any(|(p, _)| p == c))
            .ok_or(ConfigError::Invalid("IDENTITY_PEPPER_CURRENT"))?,
        None => highest,
    };
    // Fail closed: anything but an explicit "false" keeps the step-ups on.
    let step_up_q35_persistence_actions =
        non_empty("STEP_UP_Q35_PERSISTENCE_ACTIONS").as_deref() != Some("false");
    let code_sender = match non_empty("CODE_SENDER").as_deref() {
        None | Some("disabled") => CodeSenderConfig::Disabled,
        Some("dev_smtp") => {
            if app_env.is_production() {
                return Err(ConfigError::Conflict(
                    "CODE_SENDER=dev_smtp is only allowed with APP_ENV development or test",
                ));
            }
            let addr = non_empty("DEV_SMTP_ADDR").unwrap_or_else(|| "127.0.0.1:1025".into());
            let (host, port) = addr
                .rsplit_once(':')
                .and_then(|(h, p)| Some((h.to_owned(), p.parse().ok()?)))
                .ok_or(ConfigError::Invalid("DEV_SMTP_ADDR"))?;
            CodeSenderConfig::DevSmtp {
                host,
                port,
                from: non_empty("DEV_SMTP_FROM").unwrap_or_else(|| "core@allwin.local".into()),
            }
        }
        Some(_) => return Err(ConfigError::Invalid("CODE_SENDER")),
    };
    let adapter_min_versions = non_empty("IDENTITY_ADAPTER_MIN_VERSIONS")
        .unwrap_or_else(|| "ninjatrader8=1.0.0".into())
        .split(',')
        .map(|pair| {
            pair.trim()
                .split_once('=')
                .map(|(t, v)| (t.to_owned(), v.to_owned()))
        })
        .collect::<Option<Vec<_>>>()
        .ok_or(ConfigError::Invalid("IDENTITY_ADAPTER_MIN_VERSIONS"))?;
    Ok(Some(IdentityConfig {
        peppers,
        pepper_current,
        step_up_q35_persistence_actions,
        code_sender,
        adapter_min_versions,
    }))
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|k| std::env::var(k).ok())
    }

    pub fn from_lookup(get: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let non_empty = |k: &str| get(k).filter(|v| !v.is_empty());
        let database_url = non_empty("DATABASE_URL").ok_or(ConfigError::Missing("DATABASE_URL"))?;
        let worker_database_url =
            non_empty("WORKER_DATABASE_URL").unwrap_or_else(|| database_url.clone());
        let tradingview_source_ranges = match non_empty("INGRESS_TRADINGVIEW_SOURCE_RANGES") {
            Some(list) => list
                .split(',')
                .filter(|r| !r.trim().is_empty())
                .map(|r| r.parse())
                .collect::<Result<Vec<adapter_tradingview::Cidr>, _>>()
                .ok()
                .filter(|ranges| {
                    ranges
                        .iter()
                        .all(adapter_tradingview::Cidr::is_narrow_enough)
                })
                .ok_or(ConfigError::Invalid("INGRESS_TRADINGVIEW_SOURCE_RANGES"))?,
            None => Vec::new(),
        };
        let trust_forwarded_for = parse_or(&get, "INGRESS_TRUST_FORWARDED_FOR", Some(false))?;
        // Without the forwarded client address every request would appear to come from
        // Caddy, and the whole ingress would get the TradingView cap (1.1.2 deployment rule).
        if !tradingview_source_ranges.is_empty() && !trust_forwarded_for {
            return Err(ConfigError::Conflict(
                "INGRESS_TRADINGVIEW_SOURCE_RANGES requires INGRESS_TRUST_FORWARDED_FOR=true",
            ));
        }
        let app_env = match non_empty("APP_ENV").as_deref() {
            None | Some("production") => AppEnv::Production,
            Some("development") => AppEnv::Development,
            Some("test") => AppEnv::Test,
            Some(_) => return Err(ConfigError::Invalid("APP_ENV")),
        };
        let identity = identity_config(&non_empty, app_env)?;
        let public_origin = match non_empty("APP_PUBLIC_ORIGIN") {
            Some(origin) if is_origin(&origin) => Some(origin),
            Some(_) => return Err(ConfigError::Invalid("APP_PUBLIC_ORIGIN")),
            None => None,
        };
        if identity.is_some() && public_origin.is_none() {
            return Err(ConfigError::Conflict(
                "IDENTITY_PEPPERS requires APP_PUBLIC_ORIGIN (the /api/v1 Origin check)",
            ));
        }
        Ok(Self {
            database_url: SecretString::new(database_url),
            worker_database_url: SecretString::new(worker_database_url),
            migration_database_url: non_empty("MIGRATION_DATABASE_URL").map(SecretString::new),
            bind: parse_or(&get, "CORE_BIND", "127.0.0.1:8080".parse().ok())?,
            db_max_connections: parse_or(&get, "DB_MAX_CONNECTIONS", Some(10))?,
            trust_forwarded_for,
            ip_flood_per_min: parse_or(&get, "INGRESS_IP_FLOOD_PER_MIN", Some(3000))?,
            tradingview_flood_per_min: parse_or(
                &get,
                "INGRESS_TRADINGVIEW_FLOOD_PER_MIN",
                Some(30000),
            )?,
            tradingview_source_ranges,
            ip_token_failures_per_min: parse_or(
                &get,
                "INGRESS_IP_TOKEN_FAILURES_PER_MIN",
                Some(30),
            )?,
            rate_per_key_per_min: parse_or(&get, "INGRESS_RATE_PER_KEY_PER_MIN", Some(60))?,
            unknown_key_cache_ttl_s: parse_or(&get, "INGRESS_UNKNOWN_KEY_CACHE_TTL_S", Some(60))?,
            public_origin,
            app_env,
            identity,
        })
    }
}

/// `http(s)://host[:port]`: no path, query, fragment, credentials or trailing slash.
fn is_origin(value: &str) -> bool {
    let Some(rest) = value
        .strip_prefix("https://")
        .or_else(|| value.strip_prefix("http://"))
    else {
        return false;
    };
    !rest.is_empty()
        && rest
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b':' | b'[' | b']'))
}

fn parse_or<T: FromStr>(
    get: &impl Fn(&str) -> Option<String>,
    key: &'static str,
    default: Option<T>,
) -> Result<T, ConfigError> {
    match get(key).filter(|v| !v.is_empty()) {
        Some(v) => v.parse().map_err(|_| ConfigError::Invalid(key)),
        None => default.ok_or(ConfigError::Missing(key)),
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
        assert_eq!(c.worker_database_url.expose(), c.database_url.expose());
        assert!(c.migration_database_url.is_none());
        assert!(!c.trust_forwarded_for);
        assert_eq!(
            (
                c.ip_flood_per_min,
                c.tradingview_flood_per_min,
                c.ip_token_failures_per_min
            ),
            (3000, 30000, 30)
        );
        assert_eq!(
            (c.rate_per_key_per_min, c.unknown_key_cache_ttl_s),
            (60, 60)
        );
        assert!(c.tradingview_source_ranges.is_empty());
        assert!(!format!("{c:?}").contains("pw-secret"));
        assert!(c.public_origin.is_none());
    }

    #[test]
    fn public_origin_must_be_a_bare_origin() {
        let with = |origin: &'static str| {
            Config::from_lookup(move |k| match k {
                "DATABASE_URL" => Some("postgres://x".into()),
                "APP_PUBLIC_ORIGIN" => Some(origin.into()),
                _ => None,
            })
        };
        assert_eq!(
            with("https://localhost:8443")
                .unwrap()
                .public_origin
                .as_deref(),
            Some("https://localhost:8443")
        );
        for bad in [
            "https://x/",
            "https://x/path",
            "ftp://x",
            "https://u@x",
            "https://",
        ] {
            assert!(with(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn database_url_is_required_and_values_are_validated() {
        assert!(matches!(
            Config::from_lookup(|_| None),
            Err(ConfigError::Missing("DATABASE_URL"))
        ));
        let bad = Config::from_lookup(|k| match k {
            "DATABASE_URL" => Some("postgres://x".into()),
            "INGRESS_TRUST_FORWARDED_FOR" => Some("yes".into()),
            _ => None,
        });
        assert!(matches!(
            bad,
            Err(ConfigError::Invalid("INGRESS_TRUST_FORWARDED_FOR"))
        ));
    }

    #[test]
    fn tradingview_ranges_require_trusted_forwarding_and_narrow_prefixes() {
        let with = |ranges: &'static str, trust: &'static str| {
            Config::from_lookup(move |k| match k {
                "DATABASE_URL" => Some("postgres://x".into()),
                "INGRESS_TRADINGVIEW_SOURCE_RANGES" => Some(ranges.into()),
                "INGRESS_TRUST_FORWARDED_FOR" => Some(trust.into()),
                _ => None,
            })
        };
        let tv = "52.89.214.238/32, 34.212.75.30";
        assert!(matches!(with(tv, "false"), Err(ConfigError::Conflict(_))));
        assert_eq!(with(tv, "true").unwrap().tradingview_source_ranges.len(), 2);
        for bad in ["not-a-cidr", "0.0.0.0/0", "10.0.0.0/8", "::/0"] {
            assert!(
                matches!(
                    with(bad, "true"),
                    Err(ConfigError::Invalid("INGRESS_TRADINGVIEW_SOURCE_RANGES"))
                ),
                "{bad}"
            );
        }
        // Empty list: every address gets the normal cap; no forwarding requirement.
        assert!(
            with("", "false")
                .unwrap()
                .tradingview_source_ranges
                .is_empty()
        );
    }

    #[test]
    fn identity_needs_peppers_and_origin_and_keeps_dev_smtp_out_of_production() {
        let key = "AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA"; // 32 bytes
        let with = |extra: &'static [(&'static str, &'static str)]| {
            let key = key.to_owned();
            Config::from_lookup(move |k| match k {
                "DATABASE_URL" => Some("postgres://x".into()),
                "IDENTITY_PEPPERS" => Some(format!("1:{key}")),
                _ => extra
                    .iter()
                    .find(|(n, _)| *n == k)
                    .map(|(_, v)| (*v).to_owned()),
            })
        };
        assert!(
            matches!(with(&[]), Err(ConfigError::Conflict(_))),
            "origin required"
        );
        let ok = with(&[("APP_PUBLIC_ORIGIN", "https://app.test")]).unwrap();
        let id = ok.identity.unwrap();
        assert_eq!(
            (id.pepper_current, id.code_sender.clone()),
            (1, CodeSenderConfig::Disabled)
        );
        assert!(id.step_up_q35_persistence_actions);
        assert!(!format!("{id:?}").contains(key));
        assert!(
            matches!(
                with(&[
                    ("APP_PUBLIC_ORIGIN", "https://app.test"),
                    ("CODE_SENDER", "dev_smtp")
                ]),
                Err(ConfigError::Conflict(_))
            ),
            "never in production"
        );
        let dev = with(&[
            ("APP_PUBLIC_ORIGIN", "https://app.test"),
            ("CODE_SENDER", "dev_smtp"),
            ("APP_ENV", "development"),
            ("STEP_UP_Q35_PERSISTENCE_ACTIONS", "nope"),
        ])
        .unwrap()
        .identity
        .unwrap();
        assert!(matches!(
            dev.code_sender,
            CodeSenderConfig::DevSmtp { port: 1025, .. }
        ));
        assert!(dev.step_up_q35_persistence_actions, "invalid means on");
        assert!(
            matches!(
                Config::from_lookup(|k| match k {
                    "DATABASE_URL" => Some("postgres://x".into()),
                    "APP_PUBLIC_ORIGIN" => Some("https://app.test".into()),
                    "IDENTITY_PEPPERS" => Some("1:c2hvcnQ".into()),
                    _ => None,
                }),
                Err(ConfigError::Invalid("IDENTITY_PEPPERS"))
            ),
            "short keys are refused"
        );
    }
}
