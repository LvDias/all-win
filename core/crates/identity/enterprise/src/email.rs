//! E-mail address as entered plus its normalized form (trimmed, lower-cased; data model §4.1).

use thiserror::Error;

pub const MAX_EMAIL_LEN: usize = 254;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmailAddress {
    as_entered: String,
    normalized: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("invalid e-mail address")]
pub struct InvalidEmail;

impl EmailAddress {
    /// Light structural check only; the HTTP schema (`format: email`) is the first barrier.
    pub fn parse(raw: &str) -> Result<Self, InvalidEmail> {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.chars().count() > MAX_EMAIL_LEN {
            return Err(InvalidEmail);
        }
        let (local, domain) = trimmed.rsplit_once('@').ok_or(InvalidEmail)?;
        if local.is_empty() || domain.is_empty() || trimmed.chars().any(char::is_whitespace) {
            return Err(InvalidEmail);
        }
        Ok(Self {
            as_entered: trimmed.to_owned(),
            normalized: trimmed.to_lowercase(),
        })
    }

    /// Rebuilds a stored address without re-validating it.
    pub fn from_stored(as_entered: String, normalized: String) -> Self {
        Self {
            as_entered,
            normalized,
        }
    }

    pub fn as_entered(&self) -> &str {
        &self.as_entered
    }

    pub fn normalized(&self) -> &str {
        &self.normalized
    }

    /// Lower-cased local part (before the last `@`).
    pub fn local_part(&self) -> &str {
        self.normalized
            .rsplit_once('@')
            .map_or(self.normalized.as_str(), |(local, _)| local)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_by_trimming_and_lower_casing() {
        let e = EmailAddress::parse("  Ana.Trader@Example.COM ").unwrap();
        assert_eq!(e.as_entered(), "Ana.Trader@Example.COM");
        assert_eq!(e.normalized(), "ana.trader@example.com");
        assert_eq!(e.local_part(), "ana.trader");
    }

    #[test]
    fn rejects_malformed_addresses() {
        for raw in [
            "",
            "   ",
            "no-at-sign",
            "@example.com",
            "ana@",
            "a b@example.com",
        ] {
            assert_eq!(EmailAddress::parse(raw), Err(InvalidEmail), "{raw:?}");
        }
        let long = format!("{}@example.com", "a".repeat(250));
        assert_eq!(EmailAddress::parse(&long), Err(InvalidEmail));
    }
}
