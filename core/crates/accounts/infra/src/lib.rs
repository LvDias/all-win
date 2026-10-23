//! `accounts` module — infra layer (V1-S5): PostgreSQL store, outbox writes and HTTP handlers.

pub mod http;
mod outbox;
mod profiles;
pub mod store;

pub use store::PgAccountStore;
