//! Delivery channels and their values (auth policy rev. 2 §1, §10): the account e-mail and, from
//! S3b, a WhatsApp number in E.164. Values are shown masked everywhere except to their owner's own
//! request that just entered them.

use std::fmt;

use crate::email::EmailAddress;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Channel {
    Email,
    Whatsapp,
}

impl Channel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Whatsapp => "whatsapp",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "email" => Some(Self::Email),
            "whatsapp" => Some(Self::Whatsapp),
            _ => None,
        }
    }
}

/// A WhatsApp number in E.164 (`+` and 8–15 digits, no leading zero).
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    /// Accepts `+` followed by digits, ignoring spaces, `-`, `(` and `)` in the input.
    pub fn parse(raw: &str) -> Option<Self> {
        let rest = raw.trim().strip_prefix('+')?;
        let digits: String = rest
            .chars()
            .filter(|c| !matches!(c, ' ' | '-' | '(' | ')'))
            .collect();
        let valid = (8..=15).contains(&digits.len())
            && digits.bytes().all(|b| b.is_ascii_digit())
            && !digits.starts_with('0');
        valid.then(|| Self(format!("+{digits}")))
    }

    pub fn from_stored(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// `+55******1234`: country prefix (2 digits) and the last 4 digits.
    pub fn masked(&self) -> String {
        let digits = &self.0[1..];
        let keep_head = digits.len().min(2);
        let keep_tail = 4.min(digits.len() - keep_head);
        let hidden = digits.len() - keep_head - keep_tail;
        format!(
            "+{}{}{}",
            &digits[..keep_head],
            "*".repeat(hidden),
            &digits[digits.len() - keep_tail..]
        )
    }
}

impl fmt::Debug for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhoneNumber({})", self.masked())
    }
}

/// `j***@d***.com`: first character of the local part and of the domain, plus the top-level domain.
pub fn mask_email(email: &EmailAddress) -> String {
    let normalized = email.normalized();
    let Some((local, domain)) = normalized.rsplit_once('@') else {
        return "***".into();
    };
    let first = |s: &str| s.chars().next().map(String::from).unwrap_or_default();
    let (host, tld) = domain.rsplit_once('.').unwrap_or((domain, ""));
    let tld = if tld.is_empty() {
        String::new()
    } else {
        format!(".{tld}")
    };
    format!("{}***@{}***{tld}", first(local), first(host))
}

/// The value a code is sent to: an e-mail address or a phone number, normalized.
#[derive(Clone, PartialEq, Eq)]
pub enum Destination {
    Email(EmailAddress),
    Whatsapp(PhoneNumber),
}

impl Destination {
    pub fn channel(&self) -> Channel {
        match self {
            Self::Email(_) => Channel::Email,
            Self::Whatsapp(_) => Channel::Whatsapp,
        }
    }

    /// Normalized form, as stored in `one_time_code.destination` / `channel_change.new_value`.
    pub fn normalized(&self) -> &str {
        match self {
            Self::Email(e) => e.normalized(),
            Self::Whatsapp(p) => p.as_str(),
        }
    }

    pub fn masked(&self) -> String {
        match self {
            Self::Email(e) => mask_email(e),
            Self::Whatsapp(p) => p.masked(),
        }
    }

    /// Rebuilds a stored destination of a known channel.
    pub fn from_stored(channel: Channel, normalized: String) -> Self {
        match channel {
            Channel::Email => {
                Self::Email(EmailAddress::from_stored(normalized.clone(), normalized))
            }
            Channel::Whatsapp => Self::Whatsapp(PhoneNumber::from_stored(normalized)),
        }
    }
}

impl fmt::Debug for Destination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Destination({})", self.masked())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phone_numbers_are_e164() {
        assert_eq!(
            PhoneNumber::parse(" +55 (11) 98765-4321 ")
                .unwrap()
                .as_str(),
            "+5511987654321"
        );
        for bad in [
            "5511987654321",
            "+0511987654",
            "+55",
            "+55abc12345678",
            "+1234567890123456",
        ] {
            assert!(PhoneNumber::parse(bad).is_none(), "{bad}");
        }
    }

    #[test]
    fn values_are_masked() {
        let p = PhoneNumber::parse("+5511987654321").unwrap();
        assert_eq!(p.masked(), "+55*******4321");
        assert!(!format!("{p:?}").contains("98765"));
        let e = EmailAddress::parse("Joao.Silva@Dominio.com").unwrap();
        assert_eq!(mask_email(&e), "j***@d***.com");
        let d = Destination::Email(e);
        assert!(!format!("{d:?}").contains("silva"));
    }
}
