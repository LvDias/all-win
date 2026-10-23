-- Wave 1 #6 (S1 part) — idempotent ingest (wave1-data-model §6.1).
-- routing_decision + subscription follow in 6b routing_create_decision_subscription (M1/S7).
SET lock_timeout = '5s';

-- Dedupe key, unique per user forever (ADR-0003 §5); never purged by retention, erased with the user.
CREATE TABLE routing.message_receipt (
  user_id        uuid NOT NULL REFERENCES identity.app_user(id),
  message_id     text NOT NULL CHECK (message_id ~ '^[A-Za-z0-9._:-]{1,128}$'),
  instruction_id uuid NOT NULL,
  received_at    timestamptz NOT NULL,
  PRIMARY KEY (user_id, message_id)
);

-- Accepted instructions; purged by retention (Q13 b).
CREATE TABLE routing.instruction (
  id                uuid PRIMARY KEY,                    -- instruction_id (UUIDv7)
  user_id           uuid NOT NULL REFERENCES identity.app_user(id),
  source_id         uuid NOT NULL,                       -- identity.source, resolved through a query port
  source_type       text NOT NULL CHECK (source_type ~ '^[a-z][a-z0-9_]{0,31}$'),
  source_key_id     text CHECK (source_key_id ~ '^[A-Za-z0-9]{12}$'),   -- key_id only (webhook)
  connection_id     uuid,                                -- master input via adapter
  message_id        text NOT NULL CHECK (message_id ~ '^[A-Za-z0-9._:-]{1,128}$'),
  schema_version    text NOT NULL CHECK (schema_version ~ '^1\.(0|[1-9][0-9]*)$'),
  signal_type       text NOT NULL CHECK (signal_type IN ('trigger','trade_plan','nq_trigger','master_account')),
  action            text NOT NULL CHECK (action IN ('buy','sell','close','reverse','modify','cancel')),
  instrument_symbol text NOT NULL CHECK (instrument_symbol ~ '^[A-Z0-9]{1,10}$'),
  contract_month    date CHECK (extract(day FROM contract_month) = 1),   -- resolved by the Core if omitted (P5)
  quantity          integer CHECK (quantity BETWEEN 1 AND 1000),          -- NULL / ignored on close
  position_ref      text,
  order_ref         text,
  sent_at           timestamptz NOT NULL,
  received_at       timestamptz NOT NULL,
  expiry_base       timestamptz NOT NULL,                -- min(sent_at, received_at), ADR-0003 §7
  payload           jsonb NOT NULL,                      -- validated instruction; no credentials
  status            text NOT NULL DEFAULT 'accepted' CHECK (status IN ('accepted','routing','routed','failed')),
  UNIQUE (user_id, id),
  UNIQUE (user_id, message_id),
  CHECK (expiry_base = LEAST(sent_at, received_at)),
  CONSTRAINT instruction_no_token CHECK (payload::text !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}')
);
CREATE INDEX instruction_user_received_idx ON routing.instruction (user_id, received_at DESC);
CREATE INDEX instruction_pending_idx ON routing.instruction (received_at) WHERE status IN ('accepted','routing');

-- The receipt points at the instruction it created while that row exists; the instruction may be
-- purged later, so there is no FK from receipt to instruction.

ALTER TABLE routing.message_receipt ENABLE ROW LEVEL SECURITY;
ALTER TABLE routing.message_receipt FORCE  ROW LEVEL SECURITY;
CREATE POLICY message_receipt_owner ON routing.message_receipt
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

ALTER TABLE routing.instruction ENABLE ROW LEVEL SECURITY;
ALTER TABLE routing.instruction FORCE  ROW LEVEL SECURITY;
CREATE POLICY instruction_owner ON routing.instruction
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

-- Receipts are append-only at runtime (dedupe forever); retention and erasure run as allwin_maint.
REVOKE UPDATE, DELETE ON routing.message_receipt FROM allwin_app;
REVOKE DELETE ON routing.instruction FROM allwin_app;
GRANT SELECT, DELETE ON routing.message_receipt, routing.instruction TO allwin_maint;
