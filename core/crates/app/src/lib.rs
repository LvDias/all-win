//! Composition root: configuration, telemetry, HTTP server and wiring of module ports.

pub mod config;
pub mod http;
pub mod telemetry;

use anyhow::Context;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Shared state handed to HTTP handlers. Module ports are added here as slices wire them.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub async fn connect(config: &config::Config) -> anyhow::Result<Self> {
        let db = PgPoolOptions::new()
            .max_connections(config.db_max_connections)
            .connect(config.database_url.expose())
            .await
            // The sqlx error may echo connection details; keep it out of the message.
            .map_err(|_| anyhow::anyhow!("database connection failed"))
            .context("connect DATABASE_URL")?;
        Ok(Self { db })
    }
}
