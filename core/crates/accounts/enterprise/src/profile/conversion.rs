//! Distance → whole ticks, the one conversion used by the preview and by execution (ADR-0004 §5,
//! S6 AC-4, Q-N1). A distance that is not a whole number of ticks is rounded toward the entry
//! (down to the tighter distance) and flagged. All arithmetic is exact (`rust_decimal`).

use rust_decimal::Decimal;

use super::distance::{DistanceProfile, DistanceUnit, ProfileSide};

/// Structural cap of command distances (contract `DistanceEquivalent.ticks`).
pub const MAX_TICKS: u32 = 10_000;

/// Most decimals a money string carries on the wire (`MoneyString`). A tick value with more cannot
/// be shown exactly, so such an instrument is refused, never rounded (contract delta first).
pub const MAX_MONEY_DECIMALS: u32 = 4;

/// An instrument's exact tick data, as the catalog gives it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickSpec {
    tick_size: Decimal,
    tick_value: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("tick size and point value must be positive, tick value at most 4 decimals")]
pub struct InvalidTickSpec;

impl TickSpec {
    pub fn new(tick_size: Decimal, point_value: Decimal) -> Result<Self, InvalidTickSpec> {
        if tick_size <= Decimal::ZERO || point_value <= Decimal::ZERO {
            return Err(InvalidTickSpec);
        }
        let tick_value = tick_size.checked_mul(point_value).ok_or(InvalidTickSpec)?;
        if tick_value.normalize().scale() > MAX_MONEY_DECIMALS {
            return Err(InvalidTickSpec);
        }
        Ok(Self {
            tick_size: tick_size.normalize(),
            tick_value: tick_value.normalize(),
        })
    }

    pub fn tick_size(&self) -> Decimal {
        self.tick_size
    }

    /// Money per tick (tick size × point value).
    pub fn tick_value(&self) -> Decimal {
        self.tick_value
    }

    /// Decimals of the tick size without trailing zeros (0.25 → 2): points are shown with these.
    pub fn price_decimals(&self) -> u32 {
        self.tick_size.scale()
    }

    /// Dollars are shown with at least 2 decimals, more only when the tick value has more (never cut).
    pub fn usd_decimals(&self) -> u32 {
        self.tick_value.scale().max(2)
    }
}

/// What execution uses for one side, all computed here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Equivalent {
    pub ticks: u32,
    /// `ticks × tick_size`, with `price_decimals` decimals.
    pub points: Decimal,
    /// `ticks × tick_value`, with `usd_decimals` decimals.
    pub usd: Decimal,
    /// The requested distance was not a whole number of ticks and was rounded toward the entry.
    pub rounded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceRule {
    /// Zero, or less than one tick after conversion (`profiles.distance_below_one_tick`).
    BelowOneTick,
    /// More than [`MAX_TICKS`] (`profiles.distance_too_large`).
    TooLarge,
}

impl DistanceRule {
    pub fn code(self) -> &'static str {
        match self {
            Self::BelowOneTick => "profiles.distance_below_one_tick",
            Self::TooLarge => "profiles.distance_too_large",
        }
    }
}

/// A distance refused by a rule (422), with the side for the error pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("{}: {}", .side.field(), .rule.code())]
pub struct DistanceRejected {
    pub side: ProfileSide,
    pub rule: DistanceRule,
}

/// Both sides of a profile converted for one instrument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileEquivalents {
    pub stop: Equivalent,
    pub target: Equivalent,
}

impl ProfileEquivalents {
    /// Stop first, then target: the first refused side is reported.
    pub fn convert(profile: &DistanceProfile, spec: &TickSpec) -> Result<Self, DistanceRejected> {
        let side = |side| {
            to_ticks(profile.unit(), profile.side(side), spec)
                .map_err(|rule| DistanceRejected { side, rule })
        };
        Ok(Self {
            stop: side(ProfileSide::Stop)?,
            target: side(ProfileSide::Target)?,
        })
    }

    pub fn side(&self, side: ProfileSide) -> &Equivalent {
        match side {
            ProfileSide::Stop => &self.stop,
            ProfileSide::Target => &self.target,
        }
    }
}

/// One distance in `unit` → whole ticks, floored (toward the entry). The range is checked after
/// the floor. Uses the exact remainder, never a rounded quotient.
pub fn to_ticks(
    unit: DistanceUnit,
    value: Decimal,
    spec: &TickSpec,
) -> Result<Equivalent, DistanceRule> {
    if value.is_sign_negative() {
        return Err(DistanceRule::BelowOneTick);
    }
    let per_tick = match unit {
        DistanceUnit::Ticks => Decimal::ONE,
        DistanceUnit::Points => spec.tick_size,
        DistanceUnit::Usd => spec.tick_value,
    };
    let remainder = value.checked_rem(per_tick).ok_or(DistanceRule::TooLarge)?;
    let whole = (value - remainder)
        .checked_div(per_tick)
        .ok_or(DistanceRule::TooLarge)?
        .normalize();
    if whole < Decimal::ONE {
        return Err(DistanceRule::BelowOneTick);
    }
    if whole > Decimal::from(MAX_TICKS) {
        return Err(DistanceRule::TooLarge);
    }
    let ticks = u32::try_from(whole).map_err(|_| DistanceRule::TooLarge)?;
    Ok(equivalent(ticks, !remainder.is_zero(), spec))
}

/// Equivalents of a whole number of ticks (1 ..= [`MAX_TICKS`]).
pub fn equivalent(ticks: u32, rounded: bool, spec: &TickSpec) -> Equivalent {
    let ticks_dec = Decimal::from(ticks);
    // Exact: both factors are bounded (≤ 10 000 × a NUMERIC(22,10) / NUMERIC(18,4) product), and the
    // rescale only pads, since each product's scale is at most the target scale.
    let mut points = ticks_dec * spec.tick_size;
    points.rescale(spec.price_decimals());
    let mut usd = ticks_dec * spec.tick_value;
    usd.rescale(spec.usd_decimals());
    Equivalent {
        ticks,
        points,
        usd,
        rounded,
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    fn mnq() -> TickSpec {
        // Stored as NUMERIC(22,10) / NUMERIC(18,4).
        TickSpec::new(d("0.2500000000"), d("2.0000")).unwrap()
    }

    fn conv(unit: DistanceUnit, value: &str) -> Result<Equivalent, DistanceRule> {
        to_ticks(unit, d(value), &mnq())
    }

    #[test]
    fn mnq_tick_data() {
        let s = mnq();
        assert_eq!(s.tick_size().to_string(), "0.25");
        assert_eq!(s.tick_value().to_string(), "0.5");
        assert_eq!(s.price_decimals(), 2);
        assert_eq!(s.usd_decimals(), 2);
    }

    #[test]
    fn price_decimals_from_normalized_scale() {
        for (tick, dec) in [
            ("1", 0),
            ("1.0000000000", 0),
            ("0.005", 3),
            ("0.03125", 5),
            ("10", 0),
        ] {
            assert_eq!(
                TickSpec::new(d(tick), d("32")).unwrap().price_decimals(),
                dec,
                "{tick}"
            );
        }
    }

    #[test]
    fn usd_decimals_never_cut() {
        // tick 0.0025 × 5 = 0.0125 → 4 decimals, shown in full.
        let s = TickSpec::new(d("0.0025"), d("5")).unwrap();
        assert_eq!(s.usd_decimals(), 4);
        assert_eq!(equivalent(3, false, &s).usd.to_string(), "0.0375");
    }

    #[test]
    fn tick_value_beyond_money_decimals_is_refused() {
        // tick 0.03125 × 1 = 0.03125: 5 decimals do not fit `MoneyString`.
        assert_eq!(TickSpec::new(d("0.03125"), d("1")), Err(InvalidTickSpec));
    }

    #[test]
    fn invalid_tick_spec() {
        assert!(TickSpec::new(d("0"), d("2")).is_err());
        assert!(TickSpec::new(d("0.25"), d("-2")).is_err());
    }

    #[test]
    fn whole_ticks_are_exact() {
        let e = conv(DistanceUnit::Ticks, "40").unwrap();
        assert_eq!(
            (e.ticks, e.points.to_string(), e.usd.to_string(), e.rounded),
            (40, "10.00".into(), "20.00".into(), false)
        );
        let e = conv(DistanceUnit::Points, "10").unwrap();
        assert_eq!((e.ticks, e.rounded), (40, false));
        let e = conv(DistanceUnit::Usd, "20.00").unwrap();
        assert_eq!((e.ticks, e.rounded), (40, false));
    }

    #[test]
    fn off_tick_rounds_toward_the_entry() {
        // 10.1 points = 40.4 ticks → 40.
        let e = conv(DistanceUnit::Points, "10.1").unwrap();
        assert_eq!(
            (e.ticks, e.points.to_string(), e.usd.to_string(), e.rounded),
            (40, "10.00".into(), "20.00".into(), true)
        );
        // 0.24 points: one remainder short of a tick.
        assert_eq!(
            conv(DistanceUnit::Points, "0.2499999999"),
            Err(DistanceRule::BelowOneTick)
        );
        // US$ 100.10 = 200.2 ticks → 200 (US$ 100.00).
        let e = conv(DistanceUnit::Usd, "100.10").unwrap();
        assert_eq!(
            (e.ticks, e.usd.to_string(), e.points.to_string(), e.rounded),
            (200, "100.00".into(), "50.00".into(), true)
        );
        // US$ 0.99 = 1.98 ticks → 1.
        let e = conv(DistanceUnit::Usd, "0.99").unwrap();
        assert_eq!((e.ticks, e.rounded), (1, true));
    }

    #[test]
    fn range_is_checked_after_the_floor() {
        assert_eq!(
            conv(DistanceUnit::Ticks, "0"),
            Err(DistanceRule::BelowOneTick)
        );
        assert_eq!(
            conv(DistanceUnit::Points, "0"),
            Err(DistanceRule::BelowOneTick)
        );
        assert_eq!(
            conv(DistanceUnit::Usd, "0.49"),
            Err(DistanceRule::BelowOneTick)
        );
        assert_eq!(conv(DistanceUnit::Usd, "0.50").map(|e| e.ticks), Ok(1));
        assert_eq!(
            conv(DistanceUnit::Ticks, "10000").map(|e| e.ticks),
            Ok(MAX_TICKS)
        );
        assert_eq!(
            conv(DistanceUnit::Ticks, "10001"),
            Err(DistanceRule::TooLarge)
        );
        // 2500.24 points = 10000.96 ticks → 10000: accepted, rounded.
        assert_eq!(
            conv(DistanceUnit::Points, "2500.24").map(|e| (e.ticks, e.rounded)),
            Ok((MAX_TICKS, true))
        );
        assert_eq!(
            conv(DistanceUnit::Points, "2500.25"),
            Err(DistanceRule::TooLarge)
        );
        assert_eq!(
            conv(DistanceUnit::Usd, "99999999.99"),
            Err(DistanceRule::TooLarge)
        );
        assert_eq!(
            conv(DistanceUnit::Points, "99999999.9999999999"),
            Err(DistanceRule::TooLarge)
        );
    }

    #[test]
    fn smallest_fractions_are_exact() {
        // A remainder of 1e-10 must not vanish into a rounded quotient.
        let e = conv(DistanceUnit::Points, "10.0000000001").unwrap();
        assert_eq!((e.ticks, e.rounded), (40, true));
        let e = conv(DistanceUnit::Points, "9.9999999999").unwrap();
        assert_eq!((e.ticks, e.rounded), (39, true));
    }

    #[test]
    fn profile_reports_the_first_refused_side() {
        let p = DistanceProfile::parse(DistanceUnit::Ticks, "0", "0").unwrap();
        let err = ProfileEquivalents::convert(&p, &mnq()).unwrap_err();
        assert_eq!(
            (err.side, err.rule),
            (ProfileSide::Stop, DistanceRule::BelowOneTick)
        );
        let p = DistanceProfile::parse(DistanceUnit::Ticks, "4", "20000").unwrap();
        let err = ProfileEquivalents::convert(&p, &mnq()).unwrap_err();
        assert_eq!(
            (err.side, err.rule),
            (ProfileSide::Target, DistanceRule::TooLarge)
        );
    }

    #[test]
    fn exhaustive_usd_grid_matches_integer_arithmetic() {
        // Every cent from US$ 0.00 to US$ 60.00 on MNQ (US$ 0.50 per tick): ticks = cents / 50.
        for cents in 0u32..=6000 {
            let value = Decimal::new(i64::from(cents), 2);
            let got = to_ticks(DistanceUnit::Usd, value, &mnq());
            let ticks = cents / 50;
            if ticks == 0 {
                assert_eq!(got, Err(DistanceRule::BelowOneTick), "{value}");
            } else {
                let e = got.unwrap();
                assert_eq!((e.ticks, e.rounded), (ticks, cents % 50 != 0), "{value}");
                assert_eq!(e.usd, Decimal::new(i64::from(ticks) * 50, 2));
            }
        }
    }
}
