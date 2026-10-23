//! Exact decimal newtypes. Never floats; strings on the wire (CLAUDE.md §2).

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

macro_rules! decimal_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        #[cfg_attr(feature = "sqlx", derive(sqlx::Type), sqlx(transparent))]
        pub struct $name(#[serde(with = "rust_decimal::serde::str")] Decimal);

        impl $name {
            pub const fn new(value: Decimal) -> Self {
                Self(value)
            }

            pub const fn value(&self) -> Decimal {
                self.0
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}

decimal_newtype!(
    /// Instrument price (`NUMERIC` in PostgreSQL).
    Price
);
decimal_newtype!(
    /// Money amount, account currency (`NUMERIC` in PostgreSQL).
    Money
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn price_round_trips_as_string_without_losing_scale() {
        let p: Price = serde_json::from_str("\"21450.2500000000\"").unwrap();
        assert_eq!(p.value(), Decimal::from_str("21450.2500000000").unwrap());
        assert_eq!(serde_json::to_string(&p).unwrap(), "\"21450.2500000000\"");
    }

    #[test]
    fn price_rejects_json_number() {
        assert!(serde_json::from_str::<Price>("21450.25").is_err());
    }
}
