//! `adapter-sim`: a fake venue that speaks adapter-protocol v1 to the Core (ADR-0001 §8,
//! ADR-0004 §10). Simulation only; it never touches a real platform. Used by Core integration
//! tests, QA E2E and the M1 demo without NinjaTrader.

pub mod client;
pub mod venue;

pub use client::{SimClient, SimConfig, SimEnd};
pub use venue::{Behavior, Script, Venue};
