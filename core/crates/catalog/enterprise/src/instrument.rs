//! An enabled instrument with its exact tick data (contract `Instrument`, data model §5.1).

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instrument {
    pub id: Uuid,
    pub symbol: Symbol,
    pub name: String,
    pub exchange: String,
    pub currency: String,
    pub tick_size: Decimal,
    pub point_value: Decimal,
}

impl Instrument {
    /// Money per tick: tick size × point value, computed so the two can never disagree.
    /// `None` only on overflow, which the column types rule out.
    pub fn tick_value(&self) -> Option<Decimal> {
        self.tick_size
            .checked_mul(self.point_value)
            .map(|v| v.normalize())
    }

    /// Decimals of the tick size without trailing zeros (0.25 → 2): prices and points are shown with
    /// these (Compasso, S6 §5; no column).
    pub fn price_decimals(&self) -> u32 {
        self.tick_size.normalize().scale()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn instrument(tick: &str, point: &str) -> Instrument {
        Instrument {
            id: Uuid::nil(),
            symbol: Symbol::parse("MNQ").unwrap(),
            name: "Micro E-mini Nasdaq-100".into(),
            exchange: "CME".into(),
            currency: "USD".into(),
            tick_size: Decimal::from_str(tick).unwrap(),
            point_value: Decimal::from_str(point).unwrap(),
        }
    }

    #[test]
    fn mnq_as_stored() {
        let mnq = instrument("0.2500000000", "2.0000");
        assert_eq!(mnq.price_decimals(), 2);
        assert_eq!(mnq.tick_value().map(|v| v.to_string()), Some("0.5".into()));
    }

    #[test]
    fn price_decimals_ignore_storage_scale() {
        for (tick, dec) in [
            ("1.0000000000", 0),
            ("0.0050000000", 3),
            ("0.0312500000", 5),
        ] {
            assert_eq!(instrument(tick, "1").price_decimals(), dec, "{tick}");
        }
    }
}
