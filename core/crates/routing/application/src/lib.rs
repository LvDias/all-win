//! `routing` module — application layer: idempotent instruction intake and
//! ingress outcome recording (s1-webhook-ingest §2 steps 8–9, §3), and route
//! configuration (S7).

pub mod commands;
pub mod decisions;
#[cfg(feature = "dev-stubs")]
pub mod dev_stubs;
pub mod ingest;
pub mod ports;
pub mod routes;

pub use ingest::{IngestInstruction, RecordIngressRejection};
pub use ports::{IngestOutcome, IngestStore, NewInstruction, StoreError};
pub use routing_enterprise as enterprise;
