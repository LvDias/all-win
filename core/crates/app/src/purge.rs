//! Outbox retention (wave1-data-model §6.5, rev. 16.9): a periodic job, as `allwin_maint`, deletes
//! rows older than 7 days that are done for every consumer (`published_at`) or whose type is on the
//! no-consumer allowlist. Dead-lettered rows are never purged (legacy `app.outbox.dead_lettered_at`
//! here; a per-consumer dead letter keeps `published_at` NULL). Delivery rows go with their event
//! through the FK cascade. The DELETE re-checks the predicate on the locked row, so a row that a
//! new backfill consumer just took back (`published_at` reset) is never purged.

use std::time::Duration;

use sqlx::PgPool;

/// Rows per DELETE.
const BATCH: i64 = 1000;
/// How often the job runs.
pub const PURGE_EVERY: Duration = Duration::from_secs(3600);

/// Deletes every purgeable row, batch by batch. Returns the number deleted.
pub async fn purge_outbox(maint: &PgPool, listed: &[String]) -> Result<u64, sqlx::Error> {
    let mut total = 0;
    loop {
        let deleted = sqlx::query!(
            r#"DELETE FROM app.outbox
               WHERE id IN (SELECT id FROM app.outbox
                             WHERE dead_lettered_at IS NULL
                               AND occurred_at < now() - interval '7 days'
                               AND (published_at IS NOT NULL OR event_type = ANY($1))
                             ORDER BY seq LIMIT $2
                             FOR UPDATE SKIP LOCKED)
                 AND dead_lettered_at IS NULL
                 AND (published_at IS NOT NULL OR event_type = ANY($1))"#,
            listed,
            BATCH
        )
        .execute(maint)
        .await?
        .rows_affected();
        total += deleted;
        if deleted < u64::try_from(BATCH).unwrap_or(u64::MAX) {
            return Ok(total);
        }
    }
}

/// Runs [`purge_outbox`] at startup and then every [`PURGE_EVERY`].
pub async fn run(maint: PgPool, listed: Vec<String>) {
    let mut tick = tokio::time::interval(PURGE_EVERY);
    loop {
        tick.tick().await;
        match purge_outbox(&maint, &listed).await {
            Ok(0) => {}
            Ok(n) => tracing::info!(rows = n, "outbox purged"),
            Err(e) => tracing::error!(error = %e, "outbox purge failed"),
        }
    }
}
