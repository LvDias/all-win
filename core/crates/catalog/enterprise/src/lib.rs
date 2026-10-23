//! `catalog` module — enterprise layer: platform-neutral instrument reference data.

pub mod instrument;
pub mod release;

/// Neutral root symbol, e.g. `MNQ` (`^[A-Z0-9]{1,10}$`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid instrument symbol")]
pub struct InvalidSymbol;

impl Symbol {
    pub fn parse(value: &str) -> Result<Self, InvalidSymbol> {
        let valid = (1..=10).contains(&value.len())
            && value
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit());
        if valid {
            Ok(Self(value.to_owned()))
        } else {
            Err(InvalidSymbol)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_pattern() {
        assert!(Symbol::parse("MNQ").is_ok());
        assert!(Symbol::parse("mnq").is_err());
        assert!(Symbol::parse("").is_err());
        assert!(Symbol::parse("ABCDEFGHIJK").is_err());
    }
}
