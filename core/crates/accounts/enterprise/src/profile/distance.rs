//! Stop/target distances as the user typed them (S6 AC-1), with the per-unit shape rules of the
//! contract (`DistanceProfile`, `DistanceProfileShape`).

use std::str::FromStr;

use rust_decimal::Decimal;

/// Unit of a profile's two distances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DistanceUnit {
    Points,
    Ticks,
    Usd,
}

impl DistanceUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Points => "points",
            Self::Ticks => "ticks",
            Self::Usd => "usd",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "points" => Some(Self::Points),
            "ticks" => Some(Self::Ticks),
            "usd" => Some(Self::Usd),
            _ => None,
        }
    }

    /// Most decimals a distance in this unit may carry (`ticks` whole, `usd` 2 per Q-S6-6).
    pub fn max_decimals(self) -> u32 {
        match self {
            Self::Ticks => 0,
            Self::Usd => 2,
            Self::Points => 10,
        }
    }

    /// Most integer digits (`ticks` `^[0-9]{1,5}$`, otherwise `DistanceValue`'s 8).
    fn max_integer_digits(self) -> usize {
        match self {
            Self::Ticks => 5,
            Self::Usd | Self::Points => 8,
        }
    }
}

/// Which of the two distances a rule refers to (error pointer `/stop_distance` or `/target_distance`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileSide {
    Stop,
    Target,
}

impl ProfileSide {
    pub fn field(self) -> &'static str {
        match self {
            Self::Stop => "stop_distance",
            Self::Target => "target_distance",
        }
    }
}

/// A distance whose text does not fit its unit: a schema error (400), never a business rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("{} does not fit unit {}", .side.field(), .unit.as_str())]
pub struct DistanceShapeError {
    pub side: ProfileSide,
    pub unit: DistanceUnit,
}

/// One profile: a unit and two non-negative distances in it. The minimum of one tick is checked by
/// the conversion, because it depends on the instrument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DistanceProfile {
    unit: DistanceUnit,
    stop: Decimal,
    target: Decimal,
}

impl DistanceProfile {
    /// From wire text (`DistanceValue` plus the unit's shape).
    pub fn parse(unit: DistanceUnit, stop: &str, target: &str) -> Result<Self, DistanceShapeError> {
        let side_value =
            |side, text| parse_distance(unit, text).ok_or(DistanceShapeError { side, unit });
        Ok(Self {
            unit,
            stop: side_value(ProfileSide::Stop, stop)?,
            target: side_value(ProfileSide::Target, target)?,
        })
    }

    /// From stored values (`NUMERIC(22,10)`), checked against the same shape rules.
    pub fn from_decimals(
        unit: DistanceUnit,
        stop: Decimal,
        target: Decimal,
    ) -> Result<Self, DistanceShapeError> {
        let check = |side, value: Decimal| {
            if fits_unit(unit, value) {
                Ok(value.normalize())
            } else {
                Err(DistanceShapeError { side, unit })
            }
        };
        Ok(Self {
            unit,
            stop: check(ProfileSide::Stop, stop)?,
            target: check(ProfileSide::Target, target)?,
        })
    }

    pub fn unit(&self) -> DistanceUnit {
        self.unit
    }

    pub fn stop(&self) -> Decimal {
        self.stop
    }

    pub fn target(&self) -> Decimal {
        self.target
    }

    pub fn side(&self, side: ProfileSide) -> Decimal {
        match side {
            ProfileSide::Stop => self.stop,
            ProfileSide::Target => self.target,
        }
    }

    /// Same unit and numerically equal distances (`"100.1"` equals `"100.10"`). A different unit is
    /// a change even when both convert to the same ticks (Compasso, S6 §5 (c)).
    pub fn same_as(&self, other: &Self) -> bool {
        self.unit == other.unit && self.stop == other.stop && self.target == other.target
    }

    /// Wire text of one distance: whole ticks, dollars with 2 decimals, points without trailing zeros.
    pub fn text(&self, side: ProfileSide) -> String {
        let mut value = self.side(side).normalize();
        if self.unit == DistanceUnit::Usd {
            value.rescale(2);
        }
        value.to_string()
    }
}

/// `^[0-9]{1,N}(\.[0-9]{1,D})?$` for the unit, parsed exactly.
fn parse_distance(unit: DistanceUnit, text: &str) -> Option<Decimal> {
    let (integer, fraction) = match text.split_once('.') {
        Some((integer, fraction)) => (integer, Some(fraction)),
        None => (text, None),
    };
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    if !digits(integer) || integer.len() > unit.max_integer_digits() {
        return None;
    }
    if let Some(fraction) = fraction {
        let max = usize::try_from(unit.max_decimals()).ok()?;
        if !digits(fraction) || fraction.len() > max {
            return None;
        }
    }
    Decimal::from_str(text).ok().map(|d| d.normalize())
}

fn fits_unit(unit: DistanceUnit, value: Decimal) -> bool {
    let value = value.normalize();
    let integer_digits = value.trunc().to_string().trim_start_matches('0').len();
    !value.is_sign_negative()
        && value.scale() <= unit.max_decimals()
        && integer_digits <= unit.max_integer_digits()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    #[test]
    fn shapes_per_unit() {
        assert!(DistanceProfile::parse(DistanceUnit::Ticks, "40", "80").is_ok());
        assert!(DistanceProfile::parse(DistanceUnit::Ticks, "100000", "1").is_err());
        let err = DistanceProfile::parse(DistanceUnit::Ticks, "4", "8.5").unwrap_err();
        assert_eq!(err.side, ProfileSide::Target);
        assert!(DistanceProfile::parse(DistanceUnit::Usd, "100.10", "200").is_ok());
        assert!(DistanceProfile::parse(DistanceUnit::Usd, "100.105", "200").is_err());
        assert!(DistanceProfile::parse(DistanceUnit::Points, "12.3456789012", "1").is_ok());
        assert!(DistanceProfile::parse(DistanceUnit::Points, "1.12345678901", "1").is_err());
        assert!(DistanceProfile::parse(DistanceUnit::Points, "123456789", "1").is_err());
        for bad in ["", ".5", "5.", "-1", "+1", "1e3", " 1", "1,5", "0x10"] {
            assert!(
                DistanceProfile::parse(DistanceUnit::Points, bad, "1").is_err(),
                "{bad}"
            );
        }
    }

    #[test]
    fn zero_is_a_valid_shape_and_left_to_the_tick_rule() {
        assert!(DistanceProfile::parse(DistanceUnit::Ticks, "0", "1").is_ok());
    }

    #[test]
    fn numeric_equality_and_unit_change() {
        let a = DistanceProfile::parse(DistanceUnit::Usd, "100.1", "200").unwrap();
        let b = DistanceProfile::parse(DistanceUnit::Usd, "100.10", "200.00").unwrap();
        assert!(a.same_as(&b));
        let c = DistanceProfile::parse(DistanceUnit::Points, "100.1", "200").unwrap();
        assert!(!a.same_as(&c));
    }

    #[test]
    fn stored_values_round_trip() {
        let p = DistanceProfile::from_decimals(
            DistanceUnit::Usd,
            d("100.1000000000"),
            d("5.0000000000"),
        )
        .unwrap();
        assert_eq!(p.text(ProfileSide::Stop), "100.10");
        assert_eq!(p.text(ProfileSide::Target), "5.00");
        let t = DistanceProfile::from_decimals(DistanceUnit::Ticks, d("40.0000000000"), d("80"))
            .unwrap();
        assert_eq!(t.text(ProfileSide::Stop), "40");
        assert!(DistanceProfile::from_decimals(DistanceUnit::Ticks, d("4.5"), d("8")).is_err());
        assert!(DistanceProfile::from_decimals(DistanceUnit::Usd, d("1.0001"), d("8")).is_err());
    }
}
