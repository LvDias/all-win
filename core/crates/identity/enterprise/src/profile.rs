//! Profile preferences: locale and IANA time zone (Q23).

use std::str::FromStr;

use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locale {
    PtBr,
    En,
}

impl Locale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PtBr => "pt-BR",
            Self::En => "en",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "pt-BR" => Some(Self::PtBr),
            "en" => Some(Self::En),
            _ => None,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("unknown IANA time zone")]
pub struct InvalidTimeZone;

/// IANA time zone name validated against the bundled tz database.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimeZoneName(String);

impl TimeZoneName {
    pub fn parse(raw: &str) -> Result<Self, InvalidTimeZone> {
        chrono_tz::Tz::from_str(raw)
            .map(|tz| Self(tz.name().to_owned()))
            .map_err(|_| InvalidTimeZone)
    }

    pub fn from_stored(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locales_round_trip() {
        for l in [Locale::PtBr, Locale::En] {
            assert_eq!(Locale::parse(l.as_str()), Some(l));
        }
        assert_eq!(Locale::parse("pt"), None);
    }

    #[test]
    fn time_zone_must_exist() {
        assert_eq!(
            TimeZoneName::parse("America/Sao_Paulo").unwrap().as_str(),
            "America/Sao_Paulo"
        );
        assert_eq!(TimeZoneName::parse("UTC").unwrap().as_str(), "UTC");
        assert_eq!(
            TimeZoneName::parse("Mars/Olympus_Mons"),
            Err(InvalidTimeZone)
        );
    }
}
