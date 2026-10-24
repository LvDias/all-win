//! `routing` module — infra layer: PostgreSQL adapters for the `routing` schema
//! (plus the producer side of `app.outbox`, written in the same transaction), and the HTTP
//! handlers of the route operations (S7).

mod decision_store;
pub mod http;
mod ingest_store;
mod route_store;

pub use decision_store::PgDecisionStore;
pub use ingest_store::PgIngestStore;
pub use route_store::PgRouteStore;
