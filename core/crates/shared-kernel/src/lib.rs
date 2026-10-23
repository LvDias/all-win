//! Minimal shared kernel (ADR-0002 §2): identifiers, exact-decimal newtypes and time.
//!
//! Business modules depend on this crate only; it must stay small and free of
//! module-specific rules.

pub mod clock;
pub mod decimal;
pub mod id;

pub use clock::{Clock, SystemClock};
pub use decimal::{Money, Price};
pub use id::UserId;
