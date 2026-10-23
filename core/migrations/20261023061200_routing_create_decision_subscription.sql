-- Wave 1 #6b — routing configuration (wave1-data-model §5.5) and routing outcomes (§6.2), rev. 15.2.
-- account_id / source_id are cross-module ids (plain uuid, no FK; §1 principle 1).
SET lock_timeout = '5s';

-- Source → account, with a signal-type filter (S7). The S7 endpoints accept only trigger,
-- trade_plan and nq_trigger; master_account stays allowed here for S19 follower subscriptions.
CREATE TABLE routing.subscription (
  id           uuid PRIMARY KEY,
  user_id      uuid NOT NULL REFERENCES identity.app_user(id),
  source_id    uuid NOT NULL,                            -- identity.source
  account_id   uuid NOT NULL,                            -- accounts.account
  signal_types text[] NOT NULL CHECK (
                 cardinality(signal_types) >= 1
                 AND signal_types <@ ARRAY['trigger','trade_plan','nq_trigger','master_account']::text[]),
  is_enabled   boolean NOT NULL DEFAULT true,
  row_version  integer NOT NULL DEFAULT 1 CHECK (row_version >= 1),  -- optimistic concurrency (updateRoute)
  created_at   timestamptz NOT NULL DEFAULT now(),
  updated_at   timestamptz NOT NULL DEFAULT now(),
  UNIQUE (user_id, source_id, account_id)
);
CREATE INDEX subscription_source_idx ON routing.subscription (user_id, source_id) WHERE is_enabled;

-- One decision per (instruction, account), with the account values it used, so the audit trail
-- stays explainable after an edit. Immutable at runtime; purged with its instruction.
CREATE TABLE routing.routing_decision (
  id               uuid PRIMARY KEY,
  user_id          uuid NOT NULL,
  instruction_id   uuid NOT NULL,
  account_id       uuid NOT NULL,                        -- accounts.account
  account_name     text NOT NULL,                        -- snapshot
  outcome          text NOT NULL CHECK (outcome IN ('routed','capped','skipped','rejected','refused','expired')),
  reason_code      text CHECK (reason_code ~ '^routing\.[a-z][a-z0-9_]{0,63}$'),  -- NULL when routed
  multiplier       integer NOT NULL,                     -- snapshots used for the decision
  max_position_qty integer NOT NULL,
  position_before  integer,                              -- signed, the Core's expected state
  requested_qty    integer,                              -- quantity × multiplier
  executed_qty     integer,                              -- after the cap
  decided_at       timestamptz NOT NULL,
  FOREIGN KEY (user_id, instruction_id) REFERENCES routing.instruction (user_id, id) ON DELETE CASCADE,
  UNIQUE (instruction_id, account_id)
);
CREATE INDEX routing_decision_user_account_idx ON routing.routing_decision (user_id, account_id, decided_at DESC);

ALTER TABLE routing.subscription ENABLE ROW LEVEL SECURITY;
ALTER TABLE routing.subscription FORCE  ROW LEVEL SECURITY;
CREATE POLICY subscription_owner ON routing.subscription
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

ALTER TABLE routing.routing_decision ENABLE ROW LEVEL SECURITY;
ALTER TABLE routing.routing_decision FORCE  ROW LEVEL SECURITY;
CREATE POLICY routing_decision_owner ON routing.routing_decision
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

-- Default privileges give allwin_app full DML; decisions are append-only.
REVOKE UPDATE, DELETE ON routing.routing_decision FROM allwin_app;
