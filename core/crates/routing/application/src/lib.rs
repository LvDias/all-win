//! `routing` module — application layer: idempotent instruction intake and
//! ingress outcome recording (s1-webhook-ingest §2 steps 8–9, §3).

pub mod duplicate;
pub mod ingest;
pub mod ports;

pub use ingest::{IngestInstruction, RecordIngressRejection};
pub use ports::{IngestOutcome, IngestStore, NewInstruction, StoreError};
pub use routing_enterprise as enterprise;
