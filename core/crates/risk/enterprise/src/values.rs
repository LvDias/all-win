//! Exact value types of the risk rules (wire: decimal strings, never floats).

use std::str::FromStr;

use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ValueError {
    #[error("not a decimal")]
    Malformed,
    #[error("out of range")]
    OutOfRange,
    #[error("too many decimals")]
    Scale,
}

/// The wire form only: `-?digits(.digits)?` (no exponent, sign or spaces elsewhere).
fn parse(s: &str) -> Result<Decimal, ValueError> {
    let body = s.strip_prefix('-').unwrap_or(s);
    let (int, frac) = body.split_once('.').unwrap_or((body, "0"));
    let digits = |p: &str| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit());
    if !digits(int) || !digits(frac) {
        return Err(ValueError::Malformed);
    }
    Decimal::from_str(s).map_err(|_| ValueError::Malformed)
}

fn integer_digits(v: Decimal) -> u32 {
    let int = v.abs().trunc();
    if int.is_zero() {
        0
    } else {
        int.to_string().len() as u32
    }
}

/// A limit or target amount: money > 0, at most 14 integer digits and 4 decimals
/// (`NUMERIC(18,4)`, `CHECK (… > 0)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LimitAmount(Decimal);

impl LimitAmount {
    pub fn new(value: Decimal) -> Result<Self, ValueError> {
        if value <= Decimal::ZERO || integer_digits(value) > 14 {
            return Err(ValueError::OutOfRange);
        }
        if value.normalize().scale() > 4 {
            return Err(ValueError::Scale);
        }
        Ok(Self(value))
    }

    pub fn parse(s: &str) -> Result<Self, ValueError> {
        Self::new(parse(s)?)
    }

    pub const fn value(&self) -> Decimal {
        self.0
    }
}

/// A commission rate per contract per side: money ≥ 0 (U20 a).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rate(Decimal);

impl Rate {
    pub fn new(value: Decimal) -> Result<Self, ValueError> {
        if value.is_sign_negative() && !value.is_zero() || integer_digits(value) > 14 {
            return Err(ValueError::OutOfRange);
        }
        if value.normalize().scale() > 4 {
            return Err(ValueError::Scale);
        }
        Ok(Self(value))
    }

    pub fn parse(s: &str) -> Result<Self, ValueError> {
        Self::new(parse(s)?)
    }

    pub const fn value(&self) -> Decimal {
        self.0
    }
}

/// The consistency threshold: (0, 100] with at most one decimal (Q-S13-7, `NUMERIC(4,1)`). 0 is
/// refused (`risk.amount_invalid`): no day's share could be compared with it, so the meter would
/// never warn. The contract pattern still admits "0" (change requested from the Architect).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThresholdPct(Decimal);

impl ThresholdPct {
    pub fn new(value: Decimal) -> Result<Self, ValueError> {
        if value <= Decimal::ZERO || value > Decimal::ONE_HUNDRED {
            return Err(ValueError::OutOfRange);
        }
        if value.normalize().scale() > 1 {
            return Err(ValueError::Scale);
        }
        Ok(Self(value))
    }

    pub fn parse(s: &str) -> Result<Self, ValueError> {
        Self::new(parse(s)?)
    }

    pub const fn value(&self) -> Decimal {
        self.0
    }
}

/// `used / limit × 100`, truncated (F§6.2) so 80 is never shown while the state is `normal`;
/// never negative.
pub fn truncated_percent(used: Decimal, limit: Decimal) -> u32 {
    if limit <= Decimal::ZERO || used <= Decimal::ZERO {
        return 0;
    }
    let pct = (used * Decimal::ONE_HUNDRED / limit).trunc();
    u32::try_from(pct.mantissa() / 10i128.pow(pct.scale())).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    #[test]
    fn limit_amounts_are_positive_with_four_decimals() {
        assert_eq!(LimitAmount::parse("2500.00").unwrap().value(), d("2500"));
        assert_eq!(LimitAmount::parse("0"), Err(ValueError::OutOfRange));
        assert_eq!(LimitAmount::parse("-1"), Err(ValueError::OutOfRange));
        assert_eq!(LimitAmount::parse("1.00001"), Err(ValueError::Scale));
        assert_eq!(LimitAmount::parse("1e3"), Err(ValueError::Malformed));
        assert!(LimitAmount::parse("99999999999999.9999").is_ok());
        assert_eq!(
            LimitAmount::parse("100000000000000"),
            Err(ValueError::OutOfRange)
        );
    }

    #[test]
    fn rates_allow_zero() {
        assert!(Rate::parse("0").is_ok());
        assert!(Rate::parse("0.3125").is_ok());
        assert_eq!(Rate::parse("-0.01"), Err(ValueError::OutOfRange));
    }

    #[test]
    fn thresholds_are_bounded_with_one_decimal() {
        assert!(ThresholdPct::parse("100.0").is_ok());
        assert!(ThresholdPct::parse("0.1").is_ok());
        assert_eq!(ThresholdPct::parse("0"), Err(ValueError::OutOfRange));
        assert_eq!(ThresholdPct::parse("0.0"), Err(ValueError::OutOfRange));
        assert_eq!(ThresholdPct::parse("100.1"), Err(ValueError::OutOfRange));
        assert_eq!(ThresholdPct::parse("30.25"), Err(ValueError::Scale));
    }

    #[test]
    fn percent_truncates_and_never_goes_negative() {
        assert_eq!(truncated_percent(d("399.99"), d("500")), 79);
        assert_eq!(truncated_percent(d("400"), d("500")), 80);
        assert_eq!(truncated_percent(d("750"), d("500")), 150);
        assert_eq!(truncated_percent(d("-10"), d("500")), 0);
        assert_eq!(truncated_percent(d("10"), d("0")), 0);
        assert_eq!(truncated_percent(d("1"), d("3")), 33);
    }
}
