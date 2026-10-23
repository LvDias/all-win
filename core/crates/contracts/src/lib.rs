//! Rust DTOs of the canonical wire contracts in `contracts/` (ADR-0002 §4).
//!
//! The JSON Schemas are embedded from the repository root and are the source of
//! truth; DTOs are hand-written and tested against the shared fixtures.

pub mod instruction;
