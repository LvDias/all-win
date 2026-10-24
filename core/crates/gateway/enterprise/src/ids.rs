//! Identifiers used by the gateway. Cross-module ids (account, connection, instruction) are
//! kept as plain typed UUIDs: the gateway never reads another module's tables.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            pub const fn from_uuid(id: Uuid) -> Self {
                Self(id)
            }

            pub const fn as_uuid(&self) -> Uuid {
                self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}

id!(
    /// Deterministic `command_id` = UUIDv5(instruction_id, account_id, step) (ADR-0003 §5),
    /// computed by routing.
    CommandId
);
id!(
    /// All Win trading account (`accounts` module).
    AccountId
);
id!(
    /// Paired adapter connection (`identity` module).
    ConnectionId
);
id!(
    /// Accepted instruction (`routing` module).
    InstructionId
);
id!(
    /// Adapter-generated id of one platform event; `UNIQUE` in the Core (ADR-0004 §4).
    EventId
);
id!(
    /// One WebSocket session of a connection (`welcome.session_id`).
    SessionId
);
id!(
    /// Platform account fact owned by the gateway (ADR-0008).
    VenueAccountId
);
