//! What execution needs from a profile: the converted distances and the protective prices from a
//! fill (ADR-0004 §5; S6 AC-3, AC-4). The routing module reads it through its own port (ADR-0006 §3.7).

use rust_decimal::Decimal;
use shared_kernel::Price;

use super::conversion::{DistanceRejected, ProfileEquivalents, TickSpec};
use super::distance::DistanceProfile;

/// A profile resolved for execution: what the user typed and the whole ticks that will be sent, so
/// the decision audit can show the adjustment without recomputing (S6 AC-4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bracket {
    pub requested: DistanceProfile,
    pub equivalents: ProfileEquivalents,
}

impl Bracket {
    pub fn resolve(requested: DistanceProfile, spec: &TickSpec) -> Result<Self, DistanceRejected> {
        Ok(Self {
            requested,
            equivalents: ProfileEquivalents::convert(&requested, spec)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionSide {
    Long,
    Short,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectivePrices {
    pub stop: Price,
    pub target: Price,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ProtectiveError {
    #[error("tick size must be positive")]
    InvalidTickSize,
    #[error("entry price must be positive")]
    InvalidEntry,
    /// The stop would be at or below zero (a long whose stop distance exceeds the price).
    #[error("protective price out of range")]
    OutOfRange,
}

/// Stop and target from the entry price (the entry's VWAP in individual mode). Each price is on the
/// tick grid; an entry off the grid is rounded toward the entry, i.e. the tighter distance
/// (ADR-0004 §5): long stop up / target down, short stop down / target up.
pub fn protective_prices(
    side: PositionSide,
    entry: Price,
    tick_size: Decimal,
    bracket: &Bracket,
) -> Result<ProtectivePrices, ProtectiveError> {
    if tick_size <= Decimal::ZERO {
        return Err(ProtectiveError::InvalidTickSize);
    }
    let entry = entry.value();
    if entry <= Decimal::ZERO {
        return Err(ProtectiveError::InvalidEntry);
    }
    let distance = |ticks: u32| {
        Decimal::from(ticks)
            .checked_mul(tick_size)
            .ok_or(ProtectiveError::OutOfRange)
    };
    let stop_d = distance(bracket.equivalents.stop.ticks)?;
    let target_d = distance(bracket.equivalents.target.ticks)?;
    let (stop, target) = match side {
        PositionSide::Long => (
            ceil_to_tick(sub(entry, stop_d)?, tick_size)?,
            floor_to_tick(add(entry, target_d)?, tick_size)?,
        ),
        PositionSide::Short => (
            floor_to_tick(add(entry, stop_d)?, tick_size)?,
            ceil_to_tick(sub(entry, target_d)?, tick_size)?,
        ),
    };
    if stop <= Decimal::ZERO || target <= Decimal::ZERO {
        return Err(ProtectiveError::OutOfRange);
    }
    Ok(ProtectivePrices {
        stop: Price::new(stop.normalize()),
        target: Price::new(target.normalize()),
    })
}

fn add(a: Decimal, b: Decimal) -> Result<Decimal, ProtectiveError> {
    a.checked_add(b).ok_or(ProtectiveError::OutOfRange)
}

fn sub(a: Decimal, b: Decimal) -> Result<Decimal, ProtectiveError> {
    a.checked_sub(b).ok_or(ProtectiveError::OutOfRange)
}

/// Largest multiple of `tick` ≤ `value` (exact remainder; `value` may be ≤ 0, rejected later).
fn floor_to_tick(value: Decimal, tick: Decimal) -> Result<Decimal, ProtectiveError> {
    let r = value.checked_rem(tick).ok_or(ProtectiveError::OutOfRange)?;
    if r.is_zero() {
        Ok(value)
    } else if r.is_sign_positive() {
        sub(value, r)
    } else {
        sub(sub(value, r)?, tick)
    }
}

/// Smallest multiple of `tick` ≥ `value`.
fn ceil_to_tick(value: Decimal, tick: Decimal) -> Result<Decimal, ProtectiveError> {
    let floor = floor_to_tick(value, tick)?;
    if floor == value {
        Ok(value)
    } else {
        add(floor, tick)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::profile::distance::DistanceUnit;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    fn bracket(unit: DistanceUnit, stop: &str, target: &str) -> Bracket {
        let spec = TickSpec::new(d("0.25"), d("2")).unwrap();
        Bracket::resolve(DistanceProfile::parse(unit, stop, target).unwrap(), &spec).unwrap()
    }

    fn prices(side: PositionSide, entry: &str, b: &Bracket) -> (String, String) {
        let p = protective_prices(side, Price::new(d(entry)), d("0.25"), b).unwrap();
        (p.stop.to_string(), p.target.to_string())
    }

    #[test]
    fn bracket_keeps_requested_and_rounded_ticks() {
        let b = bracket(DistanceUnit::Usd, "100.10", "200.00");
        assert_eq!(
            b.requested
                .text(crate::profile::distance::ProfileSide::Stop),
            "100.10"
        );
        assert_eq!(
            (b.equivalents.stop.ticks, b.equivalents.stop.rounded),
            (200, true)
        );
        assert_eq!(
            (b.equivalents.target.ticks, b.equivalents.target.rounded),
            (400, false)
        );
    }

    #[test]
    fn on_tick_entries_are_exact() {
        let b = bracket(DistanceUnit::Ticks, "40", "80");
        assert_eq!(
            prices(PositionSide::Long, "21450.25", &b),
            ("21440.25".into(), "21470.25".into())
        );
        assert_eq!(
            prices(PositionSide::Short, "21450.25", &b),
            ("21460.25".into(), "21430.25".into())
        );
    }

    #[test]
    fn off_tick_entry_rounds_toward_the_entry() {
        // VWAP 21450.10: long stop 21440.10 → 21440.25 (up), target 21470.10 → 21470.00 (down).
        let b = bracket(DistanceUnit::Ticks, "40", "80");
        assert_eq!(
            prices(PositionSide::Long, "21450.10", &b),
            ("21440.25".into(), "21470".into())
        );
        // Short: stop 21460.10 → 21460.00 (down), target 21430.10 → 21430.25 (up).
        assert_eq!(
            prices(PositionSide::Short, "21450.10", &b),
            ("21460".into(), "21430.25".into())
        );
        // An entry a hair off the grid.
        assert_eq!(
            prices(
                PositionSide::Long,
                "100.0000000001",
                &bracket(DistanceUnit::Ticks, "1", "1")
            ),
            ("100".into(), "100.25".into())
        );
    }

    #[test]
    fn out_of_range_prices_are_refused() {
        let b = bracket(DistanceUnit::Ticks, "40", "80");
        let p = |side, entry: &str, tick: &str| {
            protective_prices(side, Price::new(d(entry)), d(tick), &b)
        };
        assert_eq!(
            p(PositionSide::Long, "10", "0.25"),
            Err(ProtectiveError::OutOfRange)
        );
        assert_eq!(
            p(PositionSide::Short, "10", "0.25"),
            Err(ProtectiveError::OutOfRange)
        );
        assert_eq!(
            p(PositionSide::Long, "0", "0.25"),
            Err(ProtectiveError::InvalidEntry)
        );
        assert_eq!(
            p(PositionSide::Long, "100", "0"),
            Err(ProtectiveError::InvalidTickSize)
        );
    }

    #[test]
    fn grid_helpers() {
        let t = d("0.25");
        assert_eq!(floor_to_tick(d("1.30"), t), Ok(d("1.25")));
        assert_eq!(ceil_to_tick(d("1.30"), t), Ok(d("1.50")));
        assert_eq!(floor_to_tick(d("-0.10"), t), Ok(d("-0.25")));
        assert_eq!(ceil_to_tick(d("-0.10"), t), Ok(d("0.00")));
        assert_eq!(ceil_to_tick(d("1.25"), t), Ok(d("1.25")));
    }
}
