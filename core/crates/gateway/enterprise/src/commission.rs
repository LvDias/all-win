//! Effective commission of a fill (ADR-0004 §7, U20 a; data model rev. 15).
//!
//! Computed once when the fill is recorded, with the rate in force read from risk; never
//! recomputed. An absent reported commission (platform said nothing) is not zero.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared_kernel::Money;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommissionSource {
    /// The platform-reported value is ≥ floor × quantity.
    Platform,
    /// The floor won: reported lower, zero, or absent with a floor.
    Configured,
    /// Absent and no floor: net equals gross (the plugin warns).
    None,
}

impl CommissionSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Platform => "platform",
            Self::Configured => "configured",
            Self::None => "none",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectiveCommission {
    pub effective: Money,
    pub source: CommissionSource,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CommissionError {
    #[error("commission amounts must not be negative")]
    Negative,
    #[error("commission overflow")]
    Overflow,
}

/// `rate` is the account's `commission_per_contract_side` in force, `None` when no floor is
/// configured. A configured rate of zero is a floor of zero: it is still "configured", so a
/// missing platform value is recorded as `configured` with 0, not as `none`.
pub fn effective_commission(
    reported: Option<Money>,
    rate: Option<Money>,
    quantity: u32,
) -> Result<EffectiveCommission, CommissionError> {
    if reported.is_some_and(|r| r.value().is_sign_negative() && !r.value().is_zero())
        || rate.is_some_and(|r| r.value().is_sign_negative() && !r.value().is_zero())
    {
        return Err(CommissionError::Negative);
    }
    let Some(rate) = rate else {
        return Ok(match reported {
            Some(r) => EffectiveCommission {
                effective: r,
                source: CommissionSource::Platform,
            },
            None => EffectiveCommission {
                effective: Money::new(Decimal::ZERO),
                source: CommissionSource::None,
            },
        });
    };
    let floor = rate
        .value()
        .checked_mul(Decimal::from(quantity))
        .ok_or(CommissionError::Overflow)?;
    Ok(match reported {
        Some(r) if r.value() >= floor => EffectiveCommission {
            effective: r,
            source: CommissionSource::Platform,
        },
        _ => EffectiveCommission {
            effective: Money::new(floor),
            source: CommissionSource::Configured,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn m(s: &str) -> Money {
        Money::new(Decimal::from_str(s).unwrap())
    }

    #[test]
    fn platform_wins_when_at_or_above_floor() {
        let c = effective_commission(Some(m("2.50")), Some(m("1.25")), 2).unwrap();
        assert_eq!(
            (c.effective, c.source),
            (m("2.50"), CommissionSource::Platform)
        );
        let c = effective_commission(Some(m("3.10")), Some(m("1.25")), 2).unwrap();
        assert_eq!(
            (c.effective, c.source),
            (m("3.10"), CommissionSource::Platform)
        );
    }

    #[test]
    fn floor_wins_when_reported_lower_zero_or_absent() {
        for reported in [Some(m("1.00")), Some(m("0")), None] {
            let c = effective_commission(reported, Some(m("1.25")), 3).unwrap();
            assert_eq!(
                (c.effective, c.source),
                (m("3.75"), CommissionSource::Configured)
            );
        }
    }

    #[test]
    fn absent_is_not_zero_without_floor() {
        let absent = effective_commission(None, None, 1).unwrap();
        assert_eq!(absent.source, CommissionSource::None);
        assert!(absent.effective.value().is_zero());
        let zero = effective_commission(Some(m("0")), None, 1).unwrap();
        assert_eq!(zero.source, CommissionSource::Platform);
    }

    #[test]
    fn zero_rate_is_a_configured_floor() {
        let c = effective_commission(None, Some(m("0")), 1).unwrap();
        assert_eq!(c.source, CommissionSource::Configured);
    }

    #[test]
    fn exact_four_decimal_arithmetic() {
        let c = effective_commission(None, Some(m("0.6150")), 7).unwrap();
        assert_eq!(c.effective, m("4.3050"));
    }

    #[test]
    fn negative_inputs_are_refused() {
        assert!(effective_commission(Some(m("-0.01")), None, 1).is_err());
        assert!(effective_commission(None, Some(m("-1")), 1).is_err());
    }
}
