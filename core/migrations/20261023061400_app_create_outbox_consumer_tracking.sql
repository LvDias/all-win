-- Wave 1 #14 — per-consumer outbox delivery tracking (wave1-data-model §6.4 rule 2, rev. 16.5).
-- app.outbox.published_at becomes the summary "done for every registered consumer"; each
-- consumer's start cursor and retries live in the two system tables below. Expand-only: the #1b
-- columns attempts/last_error/next_attempt_at stay (no longer written); app.outbox.dead_lettered_at
-- keeps only legacy #1b dead letters. Runs connected as allwin_owner (see #1).
-- Cutover (Esmeril L4): only DEV databases exist. On live data, drain the #1b retries first: a row
-- still retrying under #1b restarts at attempts 0, and a from_now consumer counts it as before its
-- start.
SET lock_timeout = '5s';

CREATE TABLE app.outbox_consumer (                    -- one row per (consumer, event type); system table
  consumer      text NOT NULL CHECK (consumer ~ '^[a-z][a-z0-9_.]{0,63}$'),
  event_type    text NOT NULL CHECK (event_type ~ '^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$'),
  start_policy  text NOT NULL CHECK (start_policy IN ('from_now','backfill')),
  start_seq     bigint NOT NULL CHECK (start_seq >= 1), -- first receivable seq; never changes
  registered_at timestamptz NOT NULL DEFAULT now(),
  retired_at    timestamptz,                          -- set by an operator step only (never automatically)
  PRIMARY KEY (consumer, event_type),
  CHECK (start_policy = 'from_now' OR start_seq = 1)
);
ALTER TABLE app.outbox_consumer OWNER TO allwin_owner;

CREATE TABLE app.outbox_delivery (                    -- per-consumer delivery state; created on the first attempt
  consumer         text NOT NULL,
  event_id         uuid NOT NULL REFERENCES app.outbox (id) ON DELETE CASCADE,
  attempts         integer NOT NULL DEFAULT 0 CHECK (attempts >= 0),
  next_attempt_at  timestamptz NOT NULL,              -- backoff 2^n s, max 600 s
  last_error       text CHECK (char_length(last_error) <= 500),  -- error class, never payload
  delivered_at     timestamptz,
  dead_lettered_at timestamptz,                       -- after 20 attempts; kept, alerted
  PRIMARY KEY (consumer, event_id),
  CHECK (delivered_at IS NULL OR dead_lettered_at IS NULL),
  CONSTRAINT outbox_delivery_last_error_no_token CHECK (last_error !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}')
);
ALTER TABLE app.outbox_delivery OWNER TO allwin_owner;
CREATE INDEX outbox_delivery_event_idx ON app.outbox_delivery (event_id);  -- the FK cascade on purge (Esmeril M2)

CREATE INDEX outbox_pending_idx ON app.outbox (event_type, seq) WHERE published_at IS NULL;  -- per-consumer fetch
DROP INDEX app.outbox_due_idx;                        -- retries move to outbox_delivery (index only, no data)

-- No user_id and no RLS (system tables, like app.outbox for the worker). allwin_app: nothing.
-- Registration only inserts (start_seq never changes; retirement is an operator step), and a
-- delivery row never changes its key.
GRANT SELECT, INSERT ON app.outbox_consumer TO allwin_worker;
GRANT SELECT, INSERT ON app.outbox_delivery TO allwin_worker;
GRANT UPDATE (attempts, next_attempt_at, last_error, delivered_at, dead_lettered_at)
  ON app.outbox_delivery TO allwin_worker;
GRANT SELECT ON app.outbox_consumer, app.outbox_delivery TO allwin_maint;
GRANT UPDATE (retired_at) ON app.outbox_consumer TO allwin_maint;                       -- retirement
GRANT UPDATE (attempts, next_attempt_at, last_error, dead_lettered_at) ON app.outbox_delivery TO allwin_maint;  -- replay
GRANT UPDATE (dead_lettered_at) ON app.outbox TO allwin_maint;                          -- legacy #1b replay

-- Dispatcher wake-up (Esquadro B5; rev. 16.9): one notification per producing transaction,
-- delivered at commit, empty payload (never an id or a type). The dispatcher LISTENs on one
-- autocommit connection that only waits and never sits inside an open transaction: a listener
-- holding an old transaction lets the notification queue fill, and then NOTIFY makes the
-- PRODUCING transactions fail at commit. Alert when pg_notification_queue_usage() > 0.5; the
-- poll stays as the fallback.
CREATE FUNCTION app.outbox_notify() RETURNS trigger
  LANGUAGE plpgsql SET search_path = pg_catalog
  AS $$ BEGIN PERFORM pg_notify('allwin_outbox', ''); RETURN NULL; END $$;
ALTER FUNCTION app.outbox_notify() OWNER TO allwin_owner;
CREATE TRIGGER outbox_notify AFTER INSERT ON app.outbox
  FOR EACH STATEMENT EXECUTE FUNCTION app.outbox_notify();
