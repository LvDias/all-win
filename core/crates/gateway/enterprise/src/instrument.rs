//! Instrument in wire form (`{symbol, contract_month}`); positions are kept per contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "RawInstrument")]
pub struct Instrument {
    symbol: String,
    contract_month: String,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid instrument")]
pub struct InvalidInstrument;

#[derive(Deserialize)]
struct RawInstrument {
    symbol: String,
    contract_month: String,
}

impl TryFrom<RawInstrument> for Instrument {
    type Error = InvalidInstrument;

    fn try_from(raw: RawInstrument) -> Result<Self, Self::Error> {
        Self::new(raw.symbol, raw.contract_month)
    }
}

impl Instrument {
    /// `symbol` matches `^[A-Z0-9]{1,10}$`, `contract_month` matches `YYYY-MM` (schema `instrument`).
    pub fn new(
        symbol: impl Into<String>,
        contract_month: impl Into<String>,
    ) -> Result<Self, InvalidInstrument> {
        let symbol = symbol.into();
        let contract_month = contract_month.into();
        let symbol_ok = (1..=10).contains(&symbol.len())
            && symbol
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit());
        if symbol_ok && valid_month(&contract_month) {
            Ok(Self {
                symbol,
                contract_month,
            })
        } else {
            Err(InvalidInstrument)
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn contract_month(&self) -> &str {
        &self.contract_month
    }
}

fn valid_month(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 7 || bytes[4] != b'-' {
        return false;
    }
    if !bytes[..4].iter().chain(&bytes[5..]).all(u8::is_ascii_digit) {
        return false;
    }
    matches!(
        &value[5..],
        "01" | "02" | "03" | "04" | "05" | "06" | "07" | "08" | "09" | "10" | "11" | "12"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_wire_form() {
        let i = Instrument::new("MNQ", "2026-12").unwrap();
        assert_eq!((i.symbol(), i.contract_month()), ("MNQ", "2026-12"));
    }

    #[test]
    fn refuses_bad_symbol_or_month() {
        for (s, m) in [
            ("mnq", "2026-12"),
            ("", "2026-12"),
            ("ABCDEFGHIJK", "2026-12"),
            ("MNQ", "2026-13"),
            ("MNQ", "2026-00"),
            ("MNQ", "26-12"),
            ("MNQ", "2026/12"),
        ] {
            assert!(Instrument::new(s, m).is_err(), "{s} {m}");
        }
    }

    #[test]
    fn deserialization_validates() {
        let ok: Instrument =
            serde_json::from_str(r#"{"symbol":"MNQ","contract_month":"2026-12"}"#).unwrap();
        assert_eq!(ok.symbol(), "MNQ");
        assert!(
            serde_json::from_str::<Instrument>(r#"{"symbol":"MNQ","contract_month":"2026-1"}"#)
                .is_err()
        );
    }
}
