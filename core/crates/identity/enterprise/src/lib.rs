//! Identity domain (V1-S3): entities, value objects and invariants.
//!
//! Policies: docs/architecture/security/auth-and-session-policy.md, ADR-0004 §8,
//! data model docs/architecture/data/wave1-data-model.md §4. No I/O, no randomness, no clock:
//! callers pass `now` and random bytes.

pub mod codes;
pub mod connection;
pub mod email;
pub mod ids;
pub mod invitation;
pub mod password;
pub mod password_reset;
pub mod profile;
pub mod secret;
pub mod session;
pub mod source;
pub mod tokens;
pub mod totp;
pub mod user;
