//! `routing` module — enterprise layer: instruction intake rules (ADR-0003 §5, §7) and
//! routes with their effective status (S7).

pub mod decision;
pub mod ingress;
pub mod instruction;
pub mod route;

pub use ingress::{IngressEvent, IngressEventKind, RejectionCode};
pub use instruction::{InstructionId, IntakeTimes, MessageId, SourceId};
pub use route::{
    AccountId, Blocks, EffectiveStatus, InvalidSignalTypes, LifecycleState, NotAccepted, OptIn,
    RouteFacts, RouteId, RouteReason, RouteSignalType, SignalTypes, Status,
};
/// Re-exported so dependent routing crates share the same id type.
pub use uuid::Uuid;
