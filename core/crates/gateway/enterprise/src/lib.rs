//! Gateway domain (ADR-0001 §4, ADR-0004): venue-neutral command delivery, execution-report
//! intake rules, expected state per account and contract, and the out-of-sync flag (Q14).
//!
//! Pure: no I/O, no clock reads. Callers pass `now` explicitly.

pub mod command;
pub mod commission;
pub mod ids;
pub mod instrument;
pub mod position;
pub mod protocol;
pub mod report;
pub mod snapshot;
pub mod sync;
pub mod triggers;
