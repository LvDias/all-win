//! Simple implementations of identity ports that have no external system yet.
//!
//! - [`DisabledMailer`]: no transactional e-mail provider is chosen; every send reports
//!   failure, so `createInvitation` answers `email_sent: false` and the admin delivers the link
//!   (Q20 fallback). Links are never logged.
//! - [`AdapterVersionPolicy`]: minimum plugin version per adapter type; development builds only
//!   where allowed (outside production).

use std::collections::HashMap;

use async_trait::async_trait;
use identity_application::ports::{AdapterCompatibility, MailError, Mailer};
use identity_enterprise::connection::{AdapterBuild, AdapterInfo};
use identity_enterprise::profile::Locale;
use identity_enterprise::secret::SecretString;

pub struct DisabledMailer;

#[async_trait]
impl Mailer for DisabledMailer {
    async fn send_invitation(
        &self,
        _to: &str,
        _locale: Locale,
        _url: &SecretString,
    ) -> Result<(), MailError> {
        Err(MailError)
    }
}

/// `major.minor.patch` (pre-release/build suffixes ignored for the comparison).
fn version(raw: &str) -> Option<(u64, u64, u64)> {
    let core = raw.split(['-', '+']).next()?;
    let mut parts = core.split('.').map(|p| p.parse::<u64>().ok());
    Some((
        parts.next()??,
        parts.next().unwrap_or(Some(0))?,
        parts.next().unwrap_or(Some(0))?,
    ))
}

pub struct AdapterVersionPolicy {
    minimum: HashMap<String, (u64, u64, u64)>,
    allow_development_builds: bool,
}

impl AdapterVersionPolicy {
    /// `minimum`: adapter type → lowest accepted version; unknown types are refused.
    pub fn new(minimum: &[(&str, &str)], allow_development_builds: bool) -> Result<Self, String> {
        let minimum = minimum
            .iter()
            .map(|(t, v)| {
                version(v)
                    .map(|v| ((*t).to_owned(), v))
                    .ok_or_else(|| format!("invalid version for {t}"))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            minimum,
            allow_development_builds,
        })
    }
}

impl AdapterCompatibility for AdapterVersionPolicy {
    fn is_supported(&self, adapter: &AdapterInfo) -> bool {
        if adapter.build == AdapterBuild::Development && !self.allow_development_builds {
            return false;
        }
        match (
            self.minimum.get(&adapter.adapter_type),
            version(&adapter.version),
        ) {
            (Some(min), Some(v)) => v >= *min,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_policy() {
        let p = AdapterVersionPolicy::new(&[("ninjatrader8", "1.2.0")], false).unwrap();
        let a = |t: &str, v: &str, b| AdapterInfo {
            adapter_type: t.into(),
            version: v.into(),
            build: b,
        };
        assert!(p.is_supported(&a("ninjatrader8", "1.2.0", AdapterBuild::Release)));
        assert!(p.is_supported(&a("ninjatrader8", "1.10.0-rc1", AdapterBuild::Release)));
        assert!(!p.is_supported(&a("ninjatrader8", "1.1.9", AdapterBuild::Release)));
        assert!(!p.is_supported(&a("ninjatrader8", "1.2.0", AdapterBuild::Development)));
        assert!(!p.is_supported(&a("other", "9.0.0", AdapterBuild::Release)));
        assert!(!p.is_supported(&a("ninjatrader8", "garbage", AdapterBuild::Release)));
        assert!(AdapterVersionPolicy::new(&[("x", "v1")], true).is_err());
    }
}
