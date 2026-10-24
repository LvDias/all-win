//! Identity adapters (V1-S3): crypto, in-memory throttles, PostgreSQL store, HTTP.

pub mod crypto;
pub mod delivery;
pub mod http;
pub mod outbox;
pub mod ports;
pub mod store;
pub mod throttle;
