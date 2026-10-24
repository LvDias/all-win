//! Outbox dispatcher (ALIGN-2; wave1-data-model §6.4 rule 2, #14): finds unpublished events as
//! `allwin_worker` and hands each one to its registered consumers (on the `allwin_app` pool with
//! the event's user context). Delivery is at least once; consumers dedupe on the event id.
//!
//! Delivery is tracked per consumer (`app.outbox_delivery`), with each consumer's start cursor per
//! event type in `app.outbox_consumer`. `app.outbox.published_at` is the summary "done for every
//! registered consumer": a row is done for consumer C when C consumed it, or when C starts
//! `from_now` and the row's `seq` is below C's `start_seq`. Rules:
//! - (Architect decision (A)) consumers are registered before the dispatcher starts, and startup
//!   fails when an emitted event type has no consumer ([`Consumers::require`]), unless the type is
//!   allowlisted ([`crate::unconsumed`]). Events without a consumer are never fetched.
//! - Registration ([`OutboxDispatcher::register`]) runs at startup, before any reconciliation (the
//!   order rule of `from_now` consumers). It records new (consumer, type) pairs, refuses a changed
//!   start policy and a pair the code no longer registers (retirement is an operator step), resets
//!   `published_at` of the retained rows of a type that gets a new `backfill` consumer, and
//!   publishes the rows that start cursors or retirements made done for everyone.
//! - One loop per consumer, guarded by a transaction-scoped advisory lock on its name, fetches by
//!   `seq`; a failed delivery is retried with exponential backoff and dead-lettered for that
//!   consumer only after [`MAX_ATTEMPTS`], never blocking the other consumers of the row.
//! - The loops wake on `NOTIFY allwin_outbox` (sent at commit by a trigger on `app.outbox`), on the
//!   in-process wake-up, and on a short poll as the fallback.
//!
//! Consumer contract (Crivo, 061400 review):
//! - delivery is at least once: a crash or a failed commit re-delivers the batch, so every
//!   consumer is idempotent on the event id; a `backfill` consumer also receives the whole
//!   retained history when it is first registered (cutover included);
//! - a consumer with an external side effect (an e-mail) declares [`EventConsumer::batch`] = 1, so
//!   a failed commit re-sends at most one message and no transaction stays open across many I/Os;
//! - order is by `seq` but not strict: a row in backoff is passed by newer ones. A future consumer
//!   that needs strict order per key (e.g. the gateway's commands) needs its own loop.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use audit_application::{PublishedEvent, RecordAuditEntry};
use chrono::{DateTime, Utc};
use serde_json::Value;
use shared_kernel::UserId;
use sqlx::PgPool;
use sqlx::postgres::PgListener;
use tokio::sync::Notify;
use uuid::Uuid;

const BATCH: i64 = 100;
const POLL: Duration = Duration::from_millis(500);
/// Attempts before an event is dead-lettered for a consumer (Code Review 5 C2).
pub const MAX_ATTEMPTS: i32 = 20;
/// Retry delay: 2^attempts seconds, capped.
const MAX_BACKOFF_SECS: i64 = 600;
/// `app.outbox_delivery.last_error` length limit.
const MAX_ERROR_LEN: usize = 500;
/// Channel of the `app.outbox` insert trigger (#14).
pub const NOTIFY_CHANNEL: &str = "allwin_outbox";
/// Delay before the listener reconnects after losing its connection.
const LISTEN_RETRY: Duration = Duration::from_secs(5);

/// An outbox row as delivered to consumers.
#[derive(Debug, Clone, PartialEq)]
pub struct OutboxEvent {
    pub id: Uuid,
    pub user_id: UserId,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    /// `app.outbox.seq`: the producer's causal order (audit `event_seq`).
    pub seq: i64,
    pub payload: Value,
    /// Failed deliveries of this event to this consumer so far (0 on the first try). With
    /// `occurred_at` it tells a consumer how long the event has been waiting.
    pub attempts: i32,
}

/// How a consumer's failed deliveries are retried.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    /// Dead-letter after this many failed attempts; `None` never dead-letters by count (the
    /// consumer ends the wait itself, e.g. an exit's 72 h horizon; Esquadro L4).
    pub max_attempts: Option<i32>,
    /// Cap of the exponential backoff (2, 4, 8 … s).
    pub max_backoff_secs: i64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: Some(MAX_ATTEMPTS),
            max_backoff_secs: MAX_BACKOFF_SECS,
        }
    }
}

/// Where a consumer starts reading an event type when it is first registered (§6.4 rule 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartPolicy {
    /// Every retained row, from the start (anything that becomes an audit entry, X-5).
    Backfill,
    /// Only rows written after registration; for consumers that act on the current state and
    /// reconcile it at startup (SSE, notices, gateway, risk).
    FromNow,
}

impl StartPolicy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Backfill => "backfill",
            Self::FromNow => "from_now",
        }
    }
}

#[async_trait]
pub trait EventConsumer: Send + Sync {
    /// Durable key of the consumer in `app.outbox_consumer` and `app.outbox_delivery`
    /// (`^[a-z][a-z0-9_.]{0,63}$`); never rename a registered consumer.
    fn name(&self) -> &'static str;
    /// Never lose an event by default.
    fn start(&self) -> StartPolicy {
        StartPolicy::Backfill
    }
    /// Events per transaction; 1 for consumers with external side effects (see the module docs).
    fn batch(&self) -> i64 {
        BATCH
    }
    fn retry(&self) -> RetryPolicy {
        RetryPolicy::default()
    }
    async fn handle(&self, event: &OutboxEvent) -> Result<(), String>;
}

#[derive(Debug, thiserror::Error)]
#[error("no consumer registered for outbox event types {0:?}")]
pub struct MissingConsumer(pub Vec<String>);

/// Consumers by event type, fixed before the dispatcher starts.
#[derive(Default, Clone)]
pub struct Consumers {
    by_type: BTreeMap<String, Vec<Arc<dyn EventConsumer>>>,
}

impl Consumers {
    pub fn register(&mut self, event_type: &str, consumer: Arc<dyn EventConsumer>) {
        self.by_type
            .entry(event_type.to_owned())
            .or_default()
            .push(consumer);
    }

    /// Fails unless every type in `emitted` has at least one consumer or is in `allowed`.
    pub fn require(
        &self,
        emitted: &[&str],
        allowed: &[crate::unconsumed::Unconsumed],
    ) -> Result<(), MissingConsumer> {
        let missing: Vec<String> = emitted
            .iter()
            .filter(|t| !self.by_type.contains_key(**t))
            .filter(|t| !allowed.iter().any(|u| u.event_type == **t))
            .map(|t| (*t).to_owned())
            .collect();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(MissingConsumer(missing))
        }
    }

    /// Event types with at least one consumer.
    pub fn types(&self) -> Vec<String> {
        self.by_type.keys().cloned().collect()
    }

    /// Each consumer once, with the event types it is registered for.
    fn by_consumer(&self) -> Vec<(Arc<dyn EventConsumer>, Vec<String>)> {
        let mut out: BTreeMap<&'static str, (Arc<dyn EventConsumer>, Vec<String>)> =
            BTreeMap::new();
        for (event_type, consumers) in &self.by_type {
            for c in consumers {
                out.entry(c.name())
                    .or_insert_with(|| (c.clone(), Vec::new()))
                    .1
                    .push(event_type.clone());
            }
        }
        out.into_values().collect()
    }
}

/// Audit consumer: records the event in the affected user's audit log.
pub struct AuditConsumer(pub RecordAuditEntry);

#[async_trait]
impl EventConsumer for AuditConsumer {
    fn name(&self) -> &'static str {
        "audit"
    }

    async fn handle(&self, e: &OutboxEvent) -> Result<(), String> {
        let event = PublishedEvent {
            id: e.id,
            user_id: e.user_id,
            event_type: e.event_type.clone(),
            occurred_at: e.occurred_at,
            event_seq: e.seq,
            payload: e.payload.clone(),
        };
        match self.0.handle(&event).await {
            Ok(true) => Ok(()),
            // Registered for a type audit cannot record: a wiring error, never "done".
            Ok(false) => Err(format!("event type {} is not auditable", e.event_type)),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistrationError {
    #[error(
        "outbox consumer {consumer} for {event_type} is registered as {stored}, the code says {code}"
    )]
    PolicyChanged {
        consumer: String,
        event_type: String,
        stored: String,
        code: &'static str,
    },
    #[error(
        "outbox consumers registered in the database but not in the code (retire them first): {0:?}"
    )]
    Unregistered(Vec<String>),
    #[error("outbox registration: {0}")]
    Db(#[from] sqlx::Error),
}

pub struct OutboxDispatcher {
    worker: PgPool,
    consumers: Consumers,
    wake: Arc<Notify>,
}

impl OutboxDispatcher {
    pub fn new(worker: PgPool, consumers: Consumers, wake: Arc<Notify>) -> Self {
        Self {
            worker,
            consumers,
            wake,
        }
    }

    /// Startup registration (§6.4 rule 2), before the loops and before any reconciliation.
    pub async fn register(&self) -> Result<(), RegistrationError> {
        let mut tx = self.worker.begin().await?;
        let mut known: Vec<String> = Vec::new();
        for (consumer, types) in self.consumers.by_consumer() {
            let policy = consumer.start();
            for event_type in &types {
                known.push(format!("{}/{event_type}", consumer.name()));
                // start_seq = 1 for backfill (CHECK); for from_now, the first seq after the rows
                // already written (an index-only read through outbox_seq_key).
                let inserted = sqlx::query_scalar!(
                    r#"INSERT INTO app.outbox_consumer (consumer, event_type, start_policy, start_seq)
                       SELECT $1, $2, $3,
                              CASE WHEN $3 = 'from_now'
                                   THEN coalesce((SELECT max(seq) FROM app.outbox), 0) + 1
                                   ELSE 1 END
                       ON CONFLICT (consumer, event_type) DO NOTHING
                       RETURNING start_seq"#,
                    consumer.name(),
                    event_type,
                    policy.as_str()
                )
                .fetch_optional(&mut *tx)
                .await?;
                let stored = sqlx::query_scalar!(
                    "SELECT start_policy FROM app.outbox_consumer WHERE consumer = $1 AND event_type = $2",
                    consumer.name(),
                    event_type
                )
                .fetch_one(&mut *tx)
                .await?;
                if stored != policy.as_str() {
                    return Err(RegistrationError::PolicyChanged {
                        consumer: consumer.name().to_owned(),
                        event_type: event_type.clone(),
                        stored,
                        code: policy.as_str(),
                    });
                }
                if inserted.is_some() && policy == StartPolicy::Backfill {
                    // Rows other consumers completed are not done for the new one (Esmeril M1).
                    sqlx::query!(
                        r#"UPDATE app.outbox SET published_at = NULL
                           WHERE event_type = $1 AND published_at IS NOT NULL
                             AND dead_lettered_at IS NULL"#,
                        event_type
                    )
                    .execute(&mut *tx)
                    .await?;
                }
                if let Some(start_seq) = inserted {
                    tracing::info!(
                        consumer = consumer.name(),
                        event_type,
                        policy = policy.as_str(),
                        start_seq,
                        "outbox consumer registered"
                    );
                }
            }
        }
        let unregistered: Vec<String> = sqlx::query_scalar!(
            r#"SELECT consumer || '/' || event_type AS "pair!" FROM app.outbox_consumer
               WHERE retired_at IS NULL ORDER BY 1"#
        )
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .filter(|p| !known.contains(p))
        .collect();
        if !unregistered.is_empty() {
            return Err(RegistrationError::Unregistered(unregistered));
        }
        // Rows that start cursors or operator retirements made done for every consumer.
        let published = sqlx::query!(
            r#"UPDATE app.outbox o SET published_at = now()
               WHERE o.published_at IS NULL AND o.dead_lettered_at IS NULL
                 AND EXISTS (SELECT 1 FROM app.outbox_consumer c
                             WHERE c.event_type = o.event_type AND c.retired_at IS NULL)
                 AND NOT EXISTS (
                   SELECT 1 FROM app.outbox_consumer c
                   WHERE c.event_type = o.event_type AND c.retired_at IS NULL
                     AND NOT (c.start_policy = 'from_now' AND o.seq < c.start_seq)
                     AND NOT EXISTS (SELECT 1 FROM app.outbox_delivery d
                                     WHERE d.consumer = c.consumer AND d.event_id = o.id
                                       AND d.delivered_at IS NOT NULL))"#
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();
        tx.commit().await?;
        if published > 0 {
            tracing::info!(
                rows = published,
                "outbox rows done for every consumer at registration"
            );
        }
        Ok(())
    }

    /// One loop per consumer, plus the `NOTIFY` listener. Run [`Self::register`] first.
    pub async fn run(self) {
        let this = Arc::new(self);
        tokio::spawn(listen(this.worker.clone(), this.wake.clone()));
        let mut loops = Vec::new();
        for (consumer, _) in this.consumers.by_consumer() {
            let this = this.clone();
            loops.push(tokio::spawn(async move { this.consume(consumer).await }));
        }
        for l in loops {
            let _ = l.await;
        }
    }

    /// One batch for every consumer (tests and drains). Returns the summed `(seen, delivered)`.
    pub async fn dispatch_all(&self) -> Result<(usize, usize), sqlx::Error> {
        let mut total = (0, 0);
        for (consumer, _) in self.consumers.by_consumer() {
            let (seen, delivered) = self.dispatch_batch(consumer.as_ref()).await?;
            total = (total.0 + seen, total.1 + delivered);
        }
        Ok(total)
    }

    async fn consume(&self, consumer: Arc<dyn EventConsumer>) {
        loop {
            let _ = tokio::time::timeout(POLL, self.wake.notified()).await;
            loop {
                match self.dispatch_batch(consumer.as_ref()).await {
                    // A full, fully delivered batch: more may be waiting.
                    Ok((seen, delivered))
                        if seen == usize::try_from(consumer.batch()).unwrap_or(0)
                            && delivered == seen =>
                    {
                        continue;
                    }
                    Ok(_) => break,
                    Err(e) => {
                        tracing::error!(consumer = consumer.name(), error = %e, "outbox dispatch failed");
                        break;
                    }
                }
            }
        }
    }

    /// One batch for one consumer. Returns `(events seen, events delivered)`; `(0, 0)` when
    /// another dispatcher holds this consumer's lock.
    pub async fn dispatch_batch(
        &self,
        consumer: &dyn EventConsumer,
    ) -> Result<(usize, usize), sqlx::Error> {
        let name = consumer.name();
        let mut tx = self.worker.begin().await?;
        let locked = sqlx::query_scalar!(
            r#"SELECT pg_try_advisory_xact_lock(hashtextextended('allwin_outbox:' || $1, 0)) AS "locked!""#,
            name
        )
        .fetch_one(&mut *tx)
        .await?;
        if !locked {
            return Ok((0, 0));
        }
        let rows = sqlx::query!(
            r#"SELECT o.id, o.user_id, o.event_type, o.occurred_at, o.seq, o.payload,
                      coalesce(d.attempts, 0) AS "attempts!"
               FROM app.outbox o
               JOIN app.outbox_consumer c
                 ON c.consumer = $1 AND c.event_type = o.event_type AND c.retired_at IS NULL
               LEFT JOIN app.outbox_delivery d ON d.consumer = $1 AND d.event_id = o.id
               WHERE o.published_at IS NULL AND o.dead_lettered_at IS NULL
                 AND o.seq >= c.start_seq
                 AND (d.event_id IS NULL
                      OR (d.delivered_at IS NULL AND d.dead_lettered_at IS NULL
                          AND d.next_attempt_at <= now()))
               ORDER BY o.seq LIMIT $2"#,
            name,
            consumer.batch()
        )
        .fetch_all(&mut *tx)
        .await?;
        let seen = rows.len();
        let mut delivered: Vec<Uuid> = Vec::with_capacity(seen);
        for row in rows {
            let event = OutboxEvent {
                id: row.id,
                user_id: UserId::from_uuid(row.user_id),
                event_type: row.event_type,
                occurred_at: row.occurred_at,
                seq: row.seq,
                payload: row.payload,
                attempts: row.attempts,
            };
            let attempts = row.attempts.saturating_add(1);
            let policy = consumer.retry();
            match consumer.handle(&event).await {
                Ok(()) => {
                    sqlx::query!(
                        r#"INSERT INTO app.outbox_delivery
                             (consumer, event_id, attempts, next_attempt_at, delivered_at)
                           VALUES ($1, $2, $3, now(), now())
                           ON CONFLICT (consumer, event_id) DO UPDATE
                             SET attempts = EXCLUDED.attempts, delivered_at = EXCLUDED.delivered_at,
                                 last_error = NULL"#,
                        name,
                        event.id,
                        attempts
                    )
                    .execute(&mut *tx)
                    .await?;
                    delivered.push(event.id);
                }
                Err(e) => {
                    tracing::error!(event_id = %event.id, consumer = name, error = %e, "outbox consumer failed");
                    let dead = policy.max_attempts.is_some_and(|max| attempts >= max);
                    if dead {
                        // The consumer's error says what is stuck (e.g. a close waiting for sync).
                        tracing::error!(event_id = %event.id, event_type = %event.event_type, consumer = name, attempts, error = %e, "ALERT outbox event dead-lettered");
                    }
                    sqlx::query!(
                        r#"INSERT INTO app.outbox_delivery
                             (consumer, event_id, attempts, next_attempt_at, last_error, dead_lettered_at)
                           VALUES ($1, $2, $3, now() + $4::integer * interval '1 second', $5,
                                   CASE WHEN $6 THEN now() END)
                           ON CONFLICT (consumer, event_id) DO UPDATE
                             SET attempts = EXCLUDED.attempts,
                                 next_attempt_at = EXCLUDED.next_attempt_at,
                                 last_error = EXCLUDED.last_error,
                                 dead_lettered_at = EXCLUDED.dead_lettered_at"#,
                        name,
                        event.id,
                        attempts,
                        i32::try_from(backoff_secs(attempts).min(policy.max_backoff_secs))
                            .unwrap_or(i32::MAX),
                        truncate(redact_tokens(&e), MAX_ERROR_LEN),
                        dead,
                    )
                    .execute(&mut *tx)
                    .await?;
                }
            }
        }
        if !delivered.is_empty() {
            // Done for every active consumer of its type: the summary the purge and alerts read.
            sqlx::query!(
                r#"UPDATE app.outbox o SET published_at = now()
                   WHERE o.id = ANY($1) AND o.published_at IS NULL
                     AND NOT EXISTS (
                       SELECT 1 FROM app.outbox_consumer c
                       WHERE c.event_type = o.event_type AND c.retired_at IS NULL
                         AND NOT (c.start_policy = 'from_now' AND o.seq < c.start_seq)
                         AND NOT EXISTS (SELECT 1 FROM app.outbox_delivery d
                                         WHERE d.consumer = c.consumer AND d.event_id = o.id
                                           AND d.delivered_at IS NOT NULL))"#,
                &delivered
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok((seen, delivered.len()))
    }
}

/// Wakes the loops on `NOTIFY allwin_outbox`. The listener connection only waits, outside any
/// transaction (a listener holding a transaction lets the notification queue fill, and then the
/// producers' commits fail); the poll covers the gaps while it reconnects.
async fn listen(worker: PgPool, wake: Arc<Notify>) {
    loop {
        match PgListener::connect_with(&worker).await {
            Ok(mut listener) => match listener.listen(NOTIFY_CHANNEL).await {
                Ok(()) => loop {
                    match listener.recv().await {
                        Ok(_) => wake.notify_waiters(),
                        Err(e) => {
                            tracing::warn!(error = %e, "outbox listener lost its connection");
                            break;
                        }
                    }
                },
                Err(e) => tracing::warn!(error = %e, "outbox LISTEN failed"),
            },
            Err(e) => tracing::warn!(error = %e, "outbox listener could not connect"),
        }
        tokio::time::sleep(LISTEN_RETRY).await;
    }
}

/// Replaces every token-shaped part (`aw[kd]_<12 alnum>_<43 base64url>`) before an error is
/// stored, so the `outbox_delivery_last_error_no_token` CHECK never refuses the failure record
/// (which would roll back the batch and retry it without backoff).
fn redact_tokens(s: &str) -> String {
    const LEN: usize = 4 + 12 + 1 + 43;
    let b = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < b.len() {
        let token = b.get(i..i + LEN).is_some_and(|w| {
            (w.starts_with(b"awk_") || w.starts_with(b"awd_"))
                && w[4..16].iter().all(u8::is_ascii_alphanumeric)
                && w[16] == b'_'
                && w[17..]
                    .iter()
                    .all(|c| c.is_ascii_alphanumeric() || *c == b'_' || *c == b'-')
        });
        if token {
            out.push_str("[redacted]");
            i += LEN;
        } else {
            // Copy one whole UTF-8 character.
            let ch_len = s[i..].chars().next().map_or(1, char::len_utf8);
            out.push_str(&s[i..i + ch_len]);
            i += ch_len;
        }
    }
    out
}

/// 2, 4, 8 … seconds, capped at [`MAX_BACKOFF_SECS`].
fn backoff_secs(attempts: i32) -> i64 {
    let exp = u32::try_from(attempts.clamp(0, 30)).unwrap_or(30);
    2_i64.saturating_pow(exp).min(MAX_BACKOFF_SECS)
}

fn truncate(mut s: String, max: usize) -> String {
    if s.len() > max {
        let mut end = max;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        s.truncate(end);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_grows_and_caps() {
        assert_eq!(backoff_secs(1), 2);
        assert_eq!(backoff_secs(5), 32);
        assert_eq!(backoff_secs(MAX_ATTEMPTS), MAX_BACKOFF_SECS);
        assert_eq!(truncate("é".repeat(400), 501).len(), 500);
    }

    struct Noop(&'static str);

    #[async_trait]
    impl EventConsumer for Noop {
        fn name(&self) -> &'static str {
            self.0
        }

        async fn handle(&self, _: &OutboxEvent) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn stored_errors_never_carry_a_token() {
        let token = format!("awk_AbCdEf123456_{}", "x".repeat(43));
        assert_eq!(
            redact_tokens(&format!("bad url /ingress/{token} é")),
            "bad url /ingress/[redacted] é"
        );
        assert_eq!(redact_tokens("plain"), "plain");
    }

    #[test]
    fn startup_fails_when_an_emitted_type_has_no_consumer() {
        let mut consumers = Consumers::default();
        consumers.register("ingress.received", Arc::new(Noop("noop")));
        assert!(consumers.require(&["ingress.received"], &[]).is_ok());
        let err = consumers
            .require(&["ingress.received", "instruction.accepted"], &[])
            .unwrap_err();
        assert_eq!(err.0, ["instruction.accepted"]);
    }

    #[test]
    fn each_consumer_runs_once_over_all_its_types() {
        let mut consumers = Consumers::default();
        let audit: Arc<dyn EventConsumer> = Arc::new(Noop("audit"));
        consumers.register("ingress.received", audit.clone());
        consumers.register("ingress.rejected", audit);
        consumers.register("ingress.received", Arc::new(Noop("identity.sse")));
        let by: Vec<(&str, Vec<String>)> = consumers
            .by_consumer()
            .into_iter()
            .map(|(c, t)| (c.name(), t))
            .collect();
        assert_eq!(
            by,
            [
                (
                    "audit",
                    vec!["ingress.received".to_owned(), "ingress.rejected".to_owned()]
                ),
                ("identity.sse", vec!["ingress.received".to_owned()]),
            ]
        );
    }
}
