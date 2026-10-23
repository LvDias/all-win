//! Value objects of an account. Constructors refuse what the database would silently round or
//! truncate (wave1-data-model §2): sizes have at most 2 decimals, money at most 4.

use std::fmt;

use rust_decimal::Decimal;
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ValueError {
    #[error("blank")]
    Blank,
    #[error("too long")]
    TooLong,
    #[error("out of range")]
    OutOfRange,
    #[error("too many decimals")]
    Scale,
    #[error("invalid format")]
    Format,
}

/// NFC-normalized, trimmed, not blank, at most `max` characters.
fn clean_text(raw: &str, max: usize) -> Result<String, ValueError> {
    let s: String = raw.nfc().collect::<String>().trim().to_owned();
    if s.is_empty() {
        return Err(ValueError::Blank);
    }
    if s.chars().count() > max {
        return Err(ValueError::TooLong);
    }
    Ok(s)
}

/// Integer digits of a decimal (`0.5` has 0).
fn integer_digits(d: Decimal) -> usize {
    let t = d.abs().trunc();
    if t.is_zero() { 0 } else { t.to_string().len() }
}

/// Account display name: unique per user among live accounts, case-insensitive (database index).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountName(String);

impl AccountName {
    pub const MAX_CHARS: usize = 80;

    pub fn parse(raw: &str) -> Result<Self, ValueError> {
        clean_text(raw, Self::MAX_CHARS).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AccountName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Free text of a prop account without a preset (firm, plan, phase).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManualText(String);

impl ManualText {
    pub const MAX_CHARS: usize = 80;

    pub fn parse(raw: &str) -> Result<Self, ValueError> {
        clean_text(raw, Self::MAX_CHARS).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Code of the firm-side platform (`^[a-z][a-z0-9_]{0,31}$`); never the execution platform.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirmPlatform(String);

impl FirmPlatform {
    pub fn parse(raw: &str) -> Result<Self, ValueError> {
        let mut chars = raw.chars();
        let ok = raw.len() <= 32
            && chars.next().is_some_and(|c| c.is_ascii_lowercase())
            && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
        if ok {
            Ok(Self(raw.to_owned()))
        } else {
            Err(ValueError::Format)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Account size in the account currency: > 0, at most 2 decimals and 12 integer digits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SizeAmount(Decimal);

impl SizeAmount {
    pub fn new(value: Decimal) -> Result<Self, ValueError> {
        if value <= Decimal::ZERO || integer_digits(value) > 12 {
            return Err(ValueError::OutOfRange);
        }
        if value.normalize().scale() > 2 {
            return Err(ValueError::Scale);
        }
        Ok(Self(value))
    }

    pub const fn value(&self) -> Decimal {
        self.0
    }

    /// Wire form with exactly 2 decimals (`"50000.00"`).
    pub fn to_wire(&self) -> String {
        let mut v = self.0;
        v.rescale(2);
        v.to_string()
    }
}

/// Non-negative money as published (`MoneyAmount`): at most 4 decimals and 14 integer digits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MoneyAmount(Decimal);

impl MoneyAmount {
    pub fn new(value: Decimal) -> Result<Self, ValueError> {
        if (value.is_sign_negative() && !value.is_zero()) || integer_digits(value) > 14 {
            return Err(ValueError::OutOfRange);
        }
        if value.normalize().scale() > 4 {
            return Err(ValueError::Scale);
        }
        Ok(Self(value))
    }

    pub const fn value(&self) -> Decimal {
        self.0
    }

    /// Wire form without trailing zeros beyond the cents (`"0.37"`, `"3000.00"`, `"0.3125"`).
    pub fn to_wire(&self) -> String {
        let mut v = self.0.normalize();
        if v.scale() < 2 {
            v.rescale(2);
        }
        v.to_string()
    }
}

/// Maximum position in contracts (S5 AC-3): 1–10000.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaxPositionQty(u32);

impl MaxPositionQty {
    pub fn new(value: i64) -> Result<Self, ValueError> {
        match u32::try_from(value) {
            Ok(v) if (1..=10_000).contains(&v) => Ok(Self(v)),
            _ => Err(ValueError::OutOfRange),
        }
    }

    pub const fn value(&self) -> u32 {
        self.0
    }
}

/// Whole-number quantity multiplier (S5 AC-5): 1–1000.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Multiplier(u32);

impl Multiplier {
    pub const ONE: Self = Self(1);

    pub fn new(value: i64) -> Result<Self, ValueError> {
        match u32::try_from(value) {
            Ok(v) if (1..=1_000).contains(&v) => Ok(Self(v)),
            _ => Err(ValueError::OutOfRange),
        }
    }

    pub const fn value(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    #[test]
    fn names_are_nfc_trimmed_and_bounded() {
        // "e" + combining acute → NFC "é" (one char).
        let n = AccountName::parse("  Conta\u{0065}\u{0301}  ").unwrap();
        assert_eq!(n.as_str(), "Conta\u{00e9}");
        assert_eq!(AccountName::parse(" \t "), Err(ValueError::Blank));
        assert!(AccountName::parse(&"a".repeat(80)).is_ok());
        assert_eq!(
            AccountName::parse(&"a".repeat(81)),
            Err(ValueError::TooLong)
        );
    }

    #[test]
    fn size_refuses_zero_negative_and_extra_decimals() {
        assert!(SizeAmount::new(d("50000.00")).is_ok());
        assert!(SizeAmount::new(d("0.01")).is_ok());
        assert_eq!(SizeAmount::new(d("0")), Err(ValueError::OutOfRange));
        assert_eq!(SizeAmount::new(d("-1")), Err(ValueError::OutOfRange));
        assert_eq!(SizeAmount::new(d("1.001")), Err(ValueError::Scale));
        assert!(
            SizeAmount::new(d("1.100")).is_ok(),
            "trailing zeros are not extra precision"
        );
        assert_eq!(
            SizeAmount::new(d("1000000000000")),
            Err(ValueError::OutOfRange)
        );
        assert_eq!(SizeAmount::new(d("25000")).unwrap().to_wire(), "25000.00");
    }

    #[test]
    fn money_allows_zero_and_four_decimals() {
        assert!(MoneyAmount::new(d("0")).is_ok());
        assert_eq!(MoneyAmount::new(d("0.3125")).unwrap().to_wire(), "0.3125");
        assert_eq!(MoneyAmount::new(d("0.37")).unwrap().to_wire(), "0.37");
        assert_eq!(MoneyAmount::new(d("3000")).unwrap().to_wire(), "3000.00");
        assert_eq!(MoneyAmount::new(d("0.00001")), Err(ValueError::Scale));
        assert_eq!(MoneyAmount::new(d("-0.01")), Err(ValueError::OutOfRange));
    }

    #[test]
    fn quantities_are_bounded_whole_numbers() {
        assert!(MaxPositionQty::new(1).is_ok());
        assert!(MaxPositionQty::new(10_000).is_ok());
        assert!(MaxPositionQty::new(0).is_err());
        assert!(MaxPositionQty::new(10_001).is_err());
        assert!(Multiplier::new(1000).is_ok());
        assert!(Multiplier::new(0).is_err());
        assert!(Multiplier::new(-3).is_err());
    }

    #[test]
    fn firm_platform_pattern() {
        assert!(FirmPlatform::parse("rithmic").is_ok());
        assert!(FirmPlatform::parse("t_2").is_ok());
        assert!(FirmPlatform::parse("Rithmic").is_err());
        assert!(FirmPlatform::parse("2x").is_err());
        assert!(FirmPlatform::parse("").is_err());
        assert!(FirmPlatform::parse(&"a".repeat(33)).is_err());
    }
}
