//! Ports of the per-instruction decision (`routing_enterprise::decision`). Provisional
//! (Maestro, 2026-10-23): the pure engine ships with S7; the use case and its wiring wait for
//! the owner of the expected position (gateway, M1), and no command is emitted until the
//! gateway consumes routing decisions.

use async_trait::async_trait;
use routing_enterprise::AccountId;
use shared_kernel::UserId;

use crate::routes::PortError;

/// The Core's expected position of one account in one instrument (gateway).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedPosition {
    /// Signed quantity: positive long, negative short, 0 flat.
    pub quantity: i64,
    /// The All Win identity of the open position (U13; ADR-0004 §5), when one is held.
    pub position_ref: Option<String>,
}

/// Query port owned by the gateway, read once per (instruction, account) at the decision.
#[async_trait]
pub trait PositionFor: Send + Sync {
    async fn position(
        &self,
        user: UserId,
        account: AccountId,
        instrument_symbol: &str,
    ) -> Result<ExpectedPosition, PortError>;
}
