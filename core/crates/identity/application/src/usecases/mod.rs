//! One struct per contract operation: `new(deps)` builds it, `handle(..)` runs it.

mod common;

pub mod access_port;
pub mod admin;
pub mod admin_recovery;
pub mod channel_change;
pub(crate) mod codes;
pub mod connections;
pub mod delivery;
pub mod devices;
pub mod invitations;
pub mod me;
pub mod pairing;
pub(crate) mod sessions;
pub mod signin;
pub mod signup;
pub mod sources;
pub mod step_up;
