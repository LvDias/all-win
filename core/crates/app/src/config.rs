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
//! | `APP_PUBLIC_ORIGIN` | unset | exact origin of the web app (`https://host[:port]`) for the `/api/v1` CSRF `Origin` check; required once `/api/v1` routes are wired |
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
            public_origin: match non_empty("APP_PUBLIC_ORIGIN") {
                Some(origin) if is_origin(&origin) => Some(origin),
                Some(_) => return Err(ConfigError::Invalid("APP_PUBLIC_ORIGIN")),
                None => None,
            },
        })
    }
}

/// `http(s)://host[:port]`: no path, query, fragment, credentials or trailing slash.
fn is_origin(value: &str) -> bool {
    let Some(authority) = value
        .strip_prefix("https://")
        .or_else(|| value.strip_prefix("http://"))
    else {
        return false;
    };
    // `host[:port]`, where host is a DNS name, an IPv4 address or a bracketed IPv6 literal.
    let (host, port) = match authority.strip_prefix('[') {
        Some(v6) => match v6.split_once(']') {
            Some((addr, rest)) if addr.parse::<std::net::Ipv6Addr>().is_ok() => match rest {
                "" => (addr, None),
                rest => match rest.strip_prefix(':') {
                    Some(port) => (addr, Some(port)),
                    None => return false,
                },
            },
            _ => return false,
        },
        None => match authority.split_once(':') {
            Some((host, port)) => (host, Some(port)),
            None => (authority, None),
        },
    };
    let host_ok = authority.starts_with('[')
        || (!host.is_empty()
            && host.split('.').all(|label| {
                !label.is_empty()
                    && label
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b == b'-')
            }));
    let port_ok = port.is_none_or(|p| {
        (1..=5).contains(&p.len())
            && p.bytes().all(|b| b.is_ascii_digit())
            && p.parse::<u16>().is_ok_and(|n| n > 0)
    });
    host_ok && port_ok
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
        for good in [
            "https://localhost:8443",
            "https://app.allwin.example",
            "http://127.0.0.1:3000",
            "https://[::1]:8443",
            "https://[2001:db8::1]",
        ] {
            assert_eq!(with(good).unwrap().public_origin.as_deref(), Some(good));
        }
        for bad in [
            "https://x/",
            "https://x/path",
            "ftp://x",
            "https://u@x",
            "https://",
            "https://:",
            "https://x:",
            "https://:8443",
            "https://x:0",
            "https://x:65536",
            "https://x:84a3",
            "https://x:1:2",
            "https://x..y",
            "https://.x",
            "https://[::1",
            "https://[nope]:1",
            "https://[::1]x",
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
}
