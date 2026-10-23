//! `gateway` module — infrastructure: PostgreSQL store (`gateway.*`, migration #9).
//!
//! Queries are bound at runtime (`sqlx::query` + `.bind`), never formatted: every value is a
//! parameter. They are not compile-time checked macros so this module adds nothing to the shared
//! `.sqlx` cache; the PostgreSQL integration tests cover every statement instead.

pub mod schema;
pub mod store;
pub mod ws;

pub use schema::ProtocolSchema;
pub use store::PgGatewayStore;
pub use ws::{GatewayRuntime, router};
