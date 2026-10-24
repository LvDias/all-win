//! Stop/target profiles (V1-S6): the PostgreSQL store, the outbox writes and the HTTP handlers.

pub mod http;
mod outbox;
pub mod schemas;
mod store;

pub use store::PgProfileStore;
pub(crate) use store::insert_initial_copies;
