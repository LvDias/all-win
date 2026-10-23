//! Human-typed codes in Crockford Base32 (ADR-0004 §8, auth policy §2):
//! pairing code (8 characters, 40 bits, shown `XXXX-XXXX`) and recovery code
//! (10 characters, 50 bits, shown `XXXXX-XXXXX`). Input ignores case, spaces and hyphens and maps
//! O→0 and I/L→1. Codes are stored as HMAC of the normalized form, never in clear.

use std::fmt;

use crate::secret::SecretString;

const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeKind {
    Pairing,
    Recovery,
}

impl CodeKind {
    pub fn chars(self) -> usize {
        match self {
            Self::Pairing => 8,
            Self::Recovery => 10,
        }
    }

    /// Random bytes needed to generate one code (40 or 50 bits, rounded up).
    pub fn random_bytes(self) -> usize {
        match self {
            Self::Pairing => 5,
            Self::Recovery => 7,
        }
    }

    fn group(self) -> usize {
        self.chars() / 2
    }
}

/// Canonical form (upper-case Crockford, no separators): the HMAC input.
#[derive(Clone, PartialEq, Eq)]
pub struct NormalizedCode {
    kind: CodeKind,
    value: String,
}

impl NormalizedCode {
    /// Generates a code from at least `kind.random_bytes()` random bytes.
    pub fn generate(kind: CodeKind, random: &[u8]) -> Option<Self> {
        if random.len() < kind.random_bytes() {
            return None;
        }
        let mut acc: u64 = 0;
        for b in &random[..kind.random_bytes()] {
            acc = (acc << 8) | u64::from(*b);
        }
        let bits = kind.chars() * 5;
        acc >>= kind.random_bytes() * 8 - bits;
        let value = (0..kind.chars())
            .rev()
            .map(|i| char::from(ALPHABET[((acc >> (i * 5)) & 0x1F) as usize]))
            .collect();
        Some(Self { kind, value })
    }

    /// Normalizes user input; `None` when it cannot be a code of this kind.
    pub fn parse(kind: CodeKind, raw: &str) -> Option<Self> {
        let mut value = String::with_capacity(kind.chars());
        for c in raw.chars() {
            let mapped = match c.to_ascii_uppercase() {
                ' ' | '-' => continue,
                'O' => '0',
                'I' | 'L' => '1',
                other => other,
            };
            if !mapped.is_ascii() || !ALPHABET.contains(&(mapped as u8)) {
                return None;
            }
            value.push(mapped);
            if value.len() > kind.chars() {
                return None;
            }
        }
        (value.len() == kind.chars()).then_some(Self { kind, value })
    }

    pub fn kind(&self) -> CodeKind {
        self.kind
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }

    /// Display form, returned exactly once to the user.
    pub fn display(&self) -> SecretString {
        let (a, b) = self.value.split_at(self.kind.group());
        SecretString::new(format!("{a}-{b}"))
    }
}

impl fmt::Debug for NormalizedCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NormalizedCode({:?}, REDACTED)", self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_codes_match_the_contract_patterns() {
        let p =
            NormalizedCode::generate(CodeKind::Pairing, &[0xFF, 0x00, 0xAA, 0x55, 0x12]).unwrap();
        let shown = p.display();
        assert_eq!(shown.expose().len(), 9);
        assert!(is_contract_form(shown.expose(), 4));
        let r = NormalizedCode::generate(CodeKind::Recovery, &[1, 2, 3, 4, 5, 6, 7]).unwrap();
        assert!(is_contract_form(r.display().expose(), 5));
        assert!(NormalizedCode::generate(CodeKind::Recovery, &[1, 2, 3]).is_none());
    }

    #[test]
    fn generation_uses_all_bits_and_is_deterministic() {
        let zero = NormalizedCode::generate(CodeKind::Pairing, &[0; 5]).unwrap();
        assert_eq!(zero.display().expose(), "0000-0000");
        let ones = NormalizedCode::generate(CodeKind::Pairing, &[0xFF; 5]).unwrap();
        assert_eq!(ones.display().expose(), "ZZZZ-ZZZZ");
        let ones = NormalizedCode::generate(CodeKind::Recovery, &[0xFF; 7]).unwrap();
        assert_eq!(ones.display().expose(), "ZZZZZ-ZZZZZ");
    }

    #[test]
    fn input_is_forgiving_about_case_separators_and_confusables() {
        let p = NormalizedCode::parse(CodeKind::Pairing, " k7q2-oil1 ").unwrap();
        assert_eq!(p.display().expose(), "K7Q2-0111");
        assert_eq!(
            NormalizedCode::parse(CodeKind::Pairing, "K7Q20111"),
            Some(p)
        );
    }

    #[test]
    fn invalid_input_is_rejected() {
        for raw in [
            "",
            "K7Q2-011",
            "K7Q2-01111",
            "K7Q2-011U",
            "K7Q2-011!",
            "K7Q2-011é",
        ] {
            assert!(
                NormalizedCode::parse(CodeKind::Pairing, raw).is_none(),
                "{raw:?}"
            );
        }
        assert!(NormalizedCode::parse(CodeKind::Recovery, "abcde-fghjk").is_some());
    }

    #[test]
    fn debug_hides_the_code() {
        let p = NormalizedCode::parse(CodeKind::Pairing, "K7Q2-0111").unwrap();
        assert!(!format!("{p:?}").contains("K7Q2"));
    }

    fn is_contract_form(s: &str, group: usize) -> bool {
        let (a, rest) = s.split_at(group);
        let Some(b) = rest.strip_prefix('-') else {
            return false;
        };
        b.len() == group && a.bytes().chain(b.bytes()).all(|c| ALPHABET.contains(&c))
    }
}
