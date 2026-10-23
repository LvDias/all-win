//! Password policy D10 (auth policy §2): 12–128 characters after NFKC, no composition rules,
//! common/breached list, no e-mail local part, no "allwin".

use thiserror::Error;
use unicode_normalization::UnicodeNormalization;

use crate::email::EmailAddress;
use crate::secret::SecretString;

pub const MIN_PASSWORD_CHARS: usize = 12;
pub const MAX_PASSWORD_CHARS: usize = 128;
const PRODUCT_NAME: &str = "allwin";

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum PasswordRejection {
    #[error("password too short")]
    TooShort,
    #[error("password too long")]
    TooLong,
    #[error("password is common, breached or derived from the account")]
    Breached,
}

impl PasswordRejection {
    pub fn code(self) -> &'static str {
        match self {
            Self::TooShort => "auth.password_too_short",
            Self::TooLong => "auth.password_too_long",
            Self::Breached => "auth.password_breached",
        }
    }
}

/// A password that passed the policy, already NFKC-normalized (the form that is hashed).
#[derive(Clone, Debug)]
pub struct NewPassword(SecretString);

impl NewPassword {
    /// `is_breached` answers whether the normalized candidate is in the bundled common/breached list.
    pub fn validate(
        raw: &SecretString,
        email: &EmailAddress,
        is_breached: impl FnOnce(&str) -> bool,
    ) -> Result<Self, PasswordRejection> {
        let normalized = normalize(raw);
        let chars = normalized.expose().chars().count();
        if chars < MIN_PASSWORD_CHARS {
            return Err(PasswordRejection::TooShort);
        }
        if chars > MAX_PASSWORD_CHARS {
            return Err(PasswordRejection::TooLong);
        }
        let lowered = normalized.expose().to_lowercase();
        let local = email.local_part();
        if lowered.contains(PRODUCT_NAME)
            || (!local.is_empty() && lowered.contains(local))
            || is_breached(normalized.expose())
        {
            return Err(PasswordRejection::Breached);
        }
        Ok(Self(normalized))
    }

    pub fn as_secret(&self) -> &SecretString {
        &self.0
    }
}

/// NFKC normalization applied to every password before hashing or verifying, so the same
/// passphrase typed on different keyboards/IMEs verifies.
pub fn normalize(raw: &SecretString) -> SecretString {
    SecretString::new(raw.expose().nfkc().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn email() -> EmailAddress {
        EmailAddress::parse("ana.trader@example.com").unwrap()
    }

    fn check(pw: &str) -> Result<NewPassword, PasswordRejection> {
        NewPassword::validate(&SecretString::new(pw), &email(), |c| c == "password1234")
    }

    #[test]
    fn length_is_counted_in_characters_after_nfkc() {
        assert_eq!(
            check("short pass").unwrap_err(),
            PasswordRejection::TooShort
        );
        assert!(check("twelve chars").is_ok());
        // 12 multi-byte characters pass although they exceed 12 bytes.
        assert!(check("ççççççççççç ").is_ok());
        assert!(check(&"x".repeat(128)).is_ok());
        assert_eq!(
            check(&"x".repeat(129)).unwrap_err(),
            PasswordRejection::TooLong
        );
        // NFKC folds the "ﬁ" ligature into two characters: 11 ligatures become 22 chars.
        assert!(check(&"ﬁ".repeat(11)).is_ok());
    }

    #[test]
    fn no_composition_rules_and_spaces_allowed() {
        assert!(check("correct horse battery staple").is_ok());
    }

    #[test]
    fn rejects_breached_product_name_and_email_local_part() {
        assert_eq!(
            check("password1234").unwrap_err(),
            PasswordRejection::Breached
        );
        assert_eq!(
            check("my AllWin secret!").unwrap_err(),
            PasswordRejection::Breached
        );
        assert_eq!(
            check("xx-ANA.TRADER-xx").unwrap_err(),
            PasswordRejection::Breached
        );
    }

    #[test]
    fn hashes_the_normalized_form() {
        let p = check("ｆｕｌｌｗｉｄｔｈ ｐａｓｓ").unwrap();
        assert_eq!(p.as_secret().expose(), "fullwidth pass");
    }
}
