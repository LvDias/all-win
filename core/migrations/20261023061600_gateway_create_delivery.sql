-- Wave 1 #9 — gateway: platform facts, command delivery outbox, execution reports, sync state,
-- reliable C→A messages and inbound dedupe (wave1-data-model rev. 16.7 §5.3, §6.3; ADR-0004;
-- ADR-0008: the account ↔ platform binding lives in `accounts`, not here).
--
-- Notes from the DBA review (Esmeril, rev. 16.6):
--   L2  account_sync is written with compare-and-set: the Core bumps `version`
--       (UPDATE … WHERE version = $expected) BEFORE inserting or resolving reason rows in the
--       same transaction; 0 rows → rollback and re-read. No FOR UPDATE.
--   L3  execution_report.seq is identity (allocation) order, not commit order. The Core
--       serializes intake per connection (one adapter session per connection, one intake
--       transaction at a time), so for one account `seq` order is arrival order, which the
--       expected position and `position_ref` follow (ADR-0004 §5).
--   N4  `frame` (the exact envelope text) cannot drift from its columns: the envelope's msg_id,
--       protocol_version (and type) and payload must equal the row's (Lastro C2 on 2bd495344318).
--       jsonb equality ignores key order and spaces; a non-JSON frame fails the cast.
--   N5  inbound_receipt holds only (connection_id, msg_id, received_at): no user data, no RLS.
SET lock_timeout = '5s';

-- Platform accounts reported in hello.accounts (§5.3). Never deleted at runtime (ADR-0008 §3.5).
CREATE TABLE gateway.venue_account (
  id               uuid PRIMARY KEY,
  user_id          uuid NOT NULL REFERENCES identity.app_user(id),
  connection_id    uuid NOT NULL,                        -- identity.adapter_connection (cross-module id)
  platform_account text NOT NULL CHECK (length(platform_account) BETWEEN 1 AND 128),
  connection_name  text,                                 -- hello.accounts[].connection_name
  kind             text NOT NULL CHECK (kind IN ('simulation','playback','live','unknown')),
  oco_location     text NOT NULL CHECK (oco_location IN ('local','server','unknown')),
  first_seen_at    timestamptz NOT NULL DEFAULT now(),
  last_seen_at     timestamptz NOT NULL DEFAULT now(),
  UNIQUE (user_id, id),
  UNIQUE (connection_id, platform_account)
);
ALTER TABLE gateway.venue_account OWNER TO allwin_owner;
ALTER TABLE gateway.venue_account ENABLE ROW LEVEL SECURITY;
ALTER TABLE gateway.venue_account FORCE  ROW LEVEL SECURITY;
CREATE POLICY venue_account_owner ON gateway.venue_account
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

-- Commands; also the delivery outbox (ADR-0004 §4). `frame` is retransmitted byte for byte.
CREATE TABLE gateway.command (
  id             uuid PRIMARY KEY,                       -- command_id = UUIDv5(instruction_id, account_id, step)
  user_id        uuid NOT NULL,
  instruction_id uuid NOT NULL,
  account_id     uuid NOT NULL,
  connection_id  uuid NOT NULL,
  step           text NOT NULL CHECK (step ~ '^(place|flatten|(modify|cancel):(0|[1-9][0-9]{0,3})|orphan_flatten:[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})$'),
  kind           text NOT NULL CHECK (kind IN ('place','modify','cancel','flatten')),
  position_effect text CHECK (position_effect IN ('open','reduce')),
  instrument_symbol text NOT NULL CHECK (instrument_symbol ~ '^[A-Z0-9]{1,10}$'),
  contract_month    date NOT NULL CHECK (extract(day FROM contract_month) = 1),
  msg_id         uuid NOT NULL UNIQUE,
  protocol_version text NOT NULL CHECK (protocol_version ~ '^1\.(0|[1-9][0-9]*)$'),
  frame          text NOT NULL,
  depends_on     uuid REFERENCES gateway.command(id) ON DELETE SET NULL,
  expires_at     timestamptz,
  state          text NOT NULL CHECK (state IN ('pending','sent','acked','completed','rejected','expired','failed','unknown','undelivered')),
  payload        jsonb NOT NULL,
  created_at     timestamptz NOT NULL,                   -- = the envelope sent_at
  sent_at        timestamptz,
  acked_at       timestamptz,
  finished_at    timestamptz,
  UNIQUE (instruction_id, account_id, step),
  CONSTRAINT command_place_effect_check CHECK ((kind = 'place') = (position_effect IS NOT NULL)),
  CONSTRAINT command_open_expires_check CHECK (position_effect IS DISTINCT FROM 'open' OR expires_at IS NOT NULL),
  CONSTRAINT command_frame_no_token CHECK (frame !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}'),
  CONSTRAINT command_frame_envelope_check CHECK (                                           -- N4
    (frame::jsonb ->> 'msg_id')::uuid = msg_id
    AND frame::jsonb ->> 'protocol_version' = protocol_version
    AND (frame::jsonb -> 'payload') = payload),
  CONSTRAINT command_step_kind_check
    CHECK (CASE split_part(step, ':', 1) WHEN 'orphan_flatten' THEN 'flatten' ELSE split_part(step, ':', 1) END = kind)
);
ALTER TABLE gateway.command OWNER TO allwin_owner;
CREATE INDEX command_open_idx ON gateway.command (connection_id, created_at) WHERE state IN ('pending','sent');
CREATE INDEX command_user_instruction_idx ON gateway.command (user_id, instruction_id);
CREATE INDEX command_pending_reduce_idx ON gateway.command (account_id)
  WHERE position_effect = 'reduce' AND state IN ('pending','sent');
CREATE INDEX command_in_flight_idx ON gateway.command (connection_id) WHERE state IN ('sent','acked');
ALTER TABLE gateway.command ENABLE ROW LEVEL SECURITY;
ALTER TABLE gateway.command FORCE  ROW LEVEL SECURITY;
CREATE POLICY command_owner ON gateway.command
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

-- Execution reports: immutable history; corrections are new rows (fill.operation).
CREATE TABLE gateway.execution_report (
  event_id          uuid PRIMARY KEY,
  user_id           uuid NOT NULL,
  account_id        uuid NOT NULL,
  connection_id     uuid NOT NULL,
  command_id        uuid,
  seq               bigint GENERATED ALWAYS AS IDENTITY (CACHE 1) UNIQUE,          -- L3
  origin            text NOT NULL CHECK (origin IN ('all_win','local_enforcement','external')),
  instrument_symbol text CHECK (instrument_symbol ~ '^[A-Z0-9]{1,10}$'),
  contract_month    date CHECK (extract(day FROM contract_month) = 1),
  child_role        text CHECK (child_role IN ('entry','stop','target','exit','flatten')),   -- rev. 16.7
  status            text NOT NULL CHECK (status IN ('received','working','partially_filled','filled','cancelled','rejected','expired','duplicate','unknown')),
  platform_order_id text CHECK (length(platform_order_id) <= 128),
  fill_id           text CHECK (length(fill_id) BETWEEN 1 AND 128),
  fill_qty          integer CHECK (fill_qty >= 1),
  fill_price        NUMERIC(22,10),
  fill_operation    text CHECK (fill_operation IN ('add','amend','remove')),
  commission        NUMERIC(18,4) CHECK (commission >= 0),             -- platform-reported; NULL = not reported
  commission_rate      NUMERIC(18,4) CHECK (commission_rate >= 0),
  commission_effective NUMERIC(18,4) CHECK (commission_effective >= 0),
  commission_source    text CHECK (commission_source IN ('platform','configured','none')),
  position_after    integer,
  avg_price_after   NUMERIC(22,10),
  reason_code       text CHECK (reason_code ~ '^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$'),  -- rev. 16.7: protocol $defs/code
  occurred_at       timestamptz NOT NULL,
  platform_time     timestamptz,
  received_at       timestamptz NOT NULL DEFAULT now(),
  payload           jsonb NOT NULL,
  CONSTRAINT execution_report_fill_pair_check
    CHECK ((fill_id IS NULL) = (fill_qty IS NULL) AND (fill_id IS NULL) = (fill_price IS NULL)
           AND (fill_id IS NULL) = (fill_operation IS NULL)),
  CONSTRAINT execution_report_commission_pair_check
    CHECK ((fill_id IS NULL) = (commission_effective IS NULL) AND (fill_id IS NULL) = (commission_source IS NULL)),
  CONSTRAINT execution_report_commission_none_check
    CHECK (commission_source IS DISTINCT FROM 'none' OR commission_effective = 0),
  CONSTRAINT execution_report_instrument_check CHECK ((instrument_symbol IS NULL) = (contract_month IS NULL)),
  CONSTRAINT execution_report_position_check CHECK (position_after IS NULL OR instrument_symbol IS NOT NULL),
  CONSTRAINT execution_report_no_token CHECK (payload::text !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}')
);
ALTER TABLE gateway.execution_report OWNER TO allwin_owner;
CREATE UNIQUE INDEX execution_report_fill_key ON gateway.execution_report (account_id, fill_id) WHERE fill_operation = 'add';
CREATE INDEX execution_report_fill_history_idx ON gateway.execution_report (account_id, fill_id, occurred_at)
  WHERE fill_id IS NOT NULL;
CREATE INDEX execution_report_command_idx ON gateway.execution_report (command_id) WHERE command_id IS NOT NULL;
CREATE INDEX execution_report_user_account_idx ON gateway.execution_report (user_id, account_id, occurred_at DESC);
CREATE INDEX execution_report_position_idx ON gateway.execution_report (account_id, instrument_symbol, contract_month, seq DESC)
  WHERE position_after IS NOT NULL;
ALTER TABLE gateway.execution_report ENABLE ROW LEVEL SECURITY;
ALTER TABLE gateway.execution_report FORCE  ROW LEVEL SECURITY;
CREATE POLICY execution_report_owner ON gateway.execution_report
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
REVOKE UPDATE, DELETE ON gateway.execution_report FROM allwin_app;   -- history is append-only

-- Q14 sticky flag; single writer = gateway; compare-and-set on `version` (L2).
CREATE TABLE gateway.account_sync (
  user_id     uuid NOT NULL,
  account_id  uuid PRIMARY KEY,
  sync_state  text NOT NULL CHECK (sync_state IN ('in_sync','out_of_sync')),
  flagged_at  timestamptz,
  cleared_at  timestamptz,
  cleared_by  uuid,
  version     integer NOT NULL DEFAULT 1 CHECK (version >= 1),
  UNIQUE (user_id, account_id),
  CONSTRAINT account_sync_flagged_check CHECK ((sync_state = 'out_of_sync') = (flagged_at IS NOT NULL))
);
ALTER TABLE gateway.account_sync OWNER TO allwin_owner;
ALTER TABLE gateway.account_sync ENABLE ROW LEVEL SECURITY;
ALTER TABLE gateway.account_sync FORCE  ROW LEVEL SECURITY;
CREATE POLICY account_sync_owner ON gateway.account_sync
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

CREATE TABLE gateway.account_sync_reason (
  id          uuid PRIMARY KEY,
  user_id     uuid NOT NULL,
  account_id  uuid NOT NULL,
  reason_code text NOT NULL CHECK (reason_code ~ '^sync\.[a-z][a-z0-9_]*$'),
  dedupe_key  text NOT NULL CHECK (char_length(dedupe_key) <= 200),
  raised_at   timestamptz NOT NULL,
  details     jsonb NOT NULL DEFAULT '{}',
  resolved_at timestamptz,
  CONSTRAINT account_sync_reason_details_check
    CHECK (jsonb_typeof(details) = 'object'
           AND NOT jsonb_path_exists(details, '$.**.fill_price ? (@.type() == "number")')),
  CONSTRAINT account_sync_reason_details_no_token CHECK (details::text !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}'),
  FOREIGN KEY (user_id, account_id) REFERENCES gateway.account_sync (user_id, account_id) ON DELETE CASCADE
);
ALTER TABLE gateway.account_sync_reason OWNER TO allwin_owner;
CREATE UNIQUE INDEX account_sync_reason_active_key ON gateway.account_sync_reason (account_id, dedupe_key)
  WHERE resolved_at IS NULL;
CREATE INDEX account_sync_reason_account_idx ON gateway.account_sync_reason (account_id);
ALTER TABLE gateway.account_sync_reason ENABLE ROW LEVEL SECURITY;
ALTER TABLE gateway.account_sync_reason FORCE  ROW LEVEL SECURITY;
CREATE POLICY account_sync_reason_owner ON gateway.account_sync_reason
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

-- Reliable C→A messages other than commands (limits, instruction_result).
CREATE TABLE gateway.outbound_message (
  msg_id        uuid PRIMARY KEY,
  user_id       uuid NOT NULL,
  connection_id uuid NOT NULL,
  type          text NOT NULL CHECK (type ~ '^[a-z][a-z_]*$'),
  payload       jsonb NOT NULL,
  protocol_version text NOT NULL CHECK (protocol_version ~ '^1\.(0|[1-9][0-9]*)$'),
  frame         text NOT NULL,
  created_at    timestamptz NOT NULL,
  acked_at      timestamptz,
  CONSTRAINT outbound_message_frame_no_token CHECK (frame !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}'),
  CONSTRAINT outbound_message_frame_envelope_check CHECK (                                 -- N4
    (frame::jsonb ->> 'msg_id')::uuid = msg_id
    AND frame::jsonb ->> 'protocol_version' = protocol_version
    AND frame::jsonb ->> 'type' = type
    AND (frame::jsonb -> 'payload') = payload)
);
ALTER TABLE gateway.outbound_message OWNER TO allwin_owner;
CREATE INDEX outbound_message_unacked_idx ON gateway.outbound_message (connection_id, created_at) WHERE acked_at IS NULL;
ALTER TABLE gateway.outbound_message ENABLE ROW LEVEL SECURITY;
ALTER TABLE gateway.outbound_message FORCE  ROW LEVEL SECURITY;
CREATE POLICY outbound_message_owner ON gateway.outbound_message
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

-- msg_id dedupe ≥ 24 h (ADR-0004 §4); purged after 48 h. N5: no user data, so no RLS.
CREATE TABLE gateway.inbound_receipt (
  connection_id uuid NOT NULL,
  msg_id        uuid NOT NULL,
  received_at   timestamptz NOT NULL,
  PRIMARY KEY (connection_id, msg_id)
);
ALTER TABLE gateway.inbound_receipt OWNER TO allwin_owner;
REVOKE UPDATE, DELETE ON gateway.inbound_receipt FROM allwin_app;

-- Runtime deletes exist only for account_sync_reason (the clear and the aggregate rewrite).
-- Venue accounts are never deleted at runtime (ADR-0008 §3.5); commands and outbound messages
-- leave only through the maintenance purge; account_sync is updated, never deleted (Lastro C1).
REVOKE DELETE ON gateway.venue_account, gateway.command, gateway.outbound_message, gateway.account_sync
  FROM allwin_app;

-- G8: the worker (BYPASSRLS) only finds work and moves delivery columns; it never edits a
-- payload or a frame. Maintenance purges per §6.5.
GRANT SELECT, UPDATE (state, sent_at, acked_at, finished_at) ON gateway.command TO allwin_worker;
GRANT SELECT, UPDATE (acked_at) ON gateway.outbound_message TO allwin_worker;
GRANT SELECT, DELETE ON gateway.command, gateway.execution_report, gateway.outbound_message,
  gateway.inbound_receipt TO allwin_maint;
