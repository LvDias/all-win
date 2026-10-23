//! `routing` module — enterprise layer: instruction intake rules (ADR-0003 §5, §7).

pub mod ingress;
pub mod instruction;

pub use ingress::{IngressEvent, IngressEventKind, RejectionCode};
pub use instruction::{InstructionId, IntakeTimes, MessageId, SourceId};
