//! `risk` module — application layer (V1-S13 / S13b): the ADR-0006 consumer that derives each
//! account's limits from its accounts inputs, the trading-day reset step, and (next) the fills and
//! the risk-state queries. Ports are consumer-owned and wired in `crates/app`.

pub mod limits;
pub mod ports;

pub use limits::{DeriveLimits, LimitsDeps, Outcome, ResetTradingDay, RiskError};

#[cfg(test)]
mod tests;
