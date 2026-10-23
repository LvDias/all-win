//! Simple implementations of identity ports that have no external system yet.
//!
//! - [`DisabledMailer`]: no transactional e-mail provider is chosen; every send reports
//!   failure, so `createInvitation` answers `email_sent: false` and the admin delivers the link
//!   (Q20 fallback). Links are never logged.
//! - [`BundledBreachedPasswords`]: the common/breached list (auth policy §2) loaded from a file
//!   shipped with the release, one password per line.
//! - [`AdapterVersionPolicy`]: minimum plugin version per adapter type; development builds only
//!   where allowed (outside production).

use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use identity_application::ports::{AdapterCompatibility, BreachedPasswords, MailError, Mailer};
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

    async fn send_password_reset(
        &self,
        _to: &str,
        _locale: Locale,
        _url: &SecretString,
    ) -> Result<(), MailError> {
        Err(MailError)
    }
}

pub struct BundledBreachedPasswords {
    list: HashSet<String>,
}

impl BundledBreachedPasswords {
    /// One password per line; blank lines ignored. Entries are compared after the same NFKC
    /// normalization the policy applies to candidates, case-insensitively.
    pub fn from_lines(text: &str) -> Self {
        let list = text
            .lines()
            .map(str::trim_end)
            .filter(|l| !l.is_empty())
            .map(|l| {
                identity_enterprise::password::normalize(&SecretString::new(l))
                    .expose()
                    .to_lowercase()
            })
            .collect();
        Self { list }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

impl BreachedPasswords for BundledBreachedPasswords {
    fn contains(&self, normalized_password: &str) -> bool {
        self.list.contains(&normalized_password.to_lowercase())
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
    fn breached_list_is_normalized_and_case_insensitive() {
        let l =
            BundledBreachedPasswords::from_lines("Password123456\n\nｑｗｅｒｔｙ１２３４５６\n");
        assert_eq!(l.len(), 2);
        assert!(l.contains("password123456"));
        assert!(l.contains("QWERTY123456"));
        assert!(!l.contains("correct horse battery"));
    }

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
