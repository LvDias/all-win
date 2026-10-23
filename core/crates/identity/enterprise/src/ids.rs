//! Typed identifiers of the identity module. `UserId` lives in the shared kernel.

use uuid::Uuid;

macro_rules! uuid_id {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(Uuid);

        impl $name {
            pub fn new_v7() -> Self {
                Self(Uuid::now_v7())
            }

            pub fn from_uuid(id: Uuid) -> Self {
                Self(id)
            }

            pub fn as_uuid(&self) -> Uuid {
                self.0
            }
        }
    };
}

uuid_id!(SessionId);
uuid_id!(InvitationId);
uuid_id!(SourceId);
uuid_id!(ConnectionId);
uuid_id!(PairingCodeId);
uuid_id!(
    /// A browser that signed in (`known_device`).
    DeviceId
);
uuid_id!(
    /// A one-time code challenge (`one_time_code.id` or `signup_code.id`); the uuid inside the
    /// sealed `challenge_id` token.
    CodeId
);
uuid_id!(ChannelChangeId);
