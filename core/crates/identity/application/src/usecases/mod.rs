//! One struct per contract operation: `new(deps)` builds it, `handle(..)` runs it.

mod common;

pub mod admin;
pub mod connections;
pub mod invitations;
pub mod me;
pub mod pairing;
pub mod signin;
pub mod sources;
