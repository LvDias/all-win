//! `gateway` module — application layer: adapter sessions, command delivery, execution-report
//! intake, snapshots and the out-of-sync flag (ADR-0004; Esquadro B1–B6).
//!
//! Ports are defined here and implemented in `gateway-infra` (own tables) or in `app`
//! (other modules' query ports). Cross-module traffic is query ports and outbox events only.

pub mod audit;
pub mod consume;
pub mod deliver;
pub mod disconnect;
pub mod events;
pub mod frames;
pub mod hello;
pub mod intake;
pub mod ports;
pub mod session;
pub mod snapshot;
pub mod wire;

#[cfg(test)]
mod testing;
#[cfg(test)]
mod tests;

pub use gateway_enterprise as enterprise;
