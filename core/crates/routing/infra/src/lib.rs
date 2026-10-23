//! `routing` module — infra layer: PostgreSQL adapters for the `routing` schema
//! (plus the producer side of `app.outbox`, written in the same transaction).

mod ingest_store;
mod route_store;

pub use ingest_store::PgIngestStore;
pub use route_store::PgRouteStore;
