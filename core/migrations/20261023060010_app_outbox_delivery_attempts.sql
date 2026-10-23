-- Wave 1 #1b — outbox delivery attempts and dead-letter (Code Review 5 C2). Expand-only.
-- A failing event is retried with backoff instead of staying at the head of the queue, and
-- after the last attempt it is dead-lettered (kept, never published, alert in the Core log).
-- Runs connected as allwin_owner (see #1).
SET lock_timeout = '5s';

ALTER TABLE app.outbox
  ADD COLUMN attempts         integer NOT NULL DEFAULT 0 CHECK (attempts >= 0),
  ADD COLUMN last_error       text CHECK (char_length(last_error) <= 500),
  ADD COLUMN next_attempt_at  timestamptz NOT NULL DEFAULT now(),
  ADD COLUMN dead_lettered_at timestamptz,
  ADD CONSTRAINT outbox_last_error_no_token CHECK (last_error !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}');

-- Due, undelivered, live events (the dispatcher's only query).
DROP INDEX app.outbox_unpublished_idx;
CREATE INDEX outbox_due_idx ON app.outbox (next_attempt_at, occurred_at)
  WHERE published_at IS NULL AND dead_lettered_at IS NULL;

GRANT UPDATE (attempts, last_error, next_attempt_at, dead_lettered_at) ON app.outbox TO allwin_worker;
