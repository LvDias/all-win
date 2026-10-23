-- Wave 1 #10 — risk core (wave1-data-model §5.6, rev. 16.3; contract s13-risk.md). Every table
-- is user-owned (RLS) and written only by the risk module. The rule inputs live in accounts;
-- account_limits / scheduled_limit / evaluation_progress are DERIVED from them (ADR-0006, option B).
-- The schema and allwin_app's default DML grants come from the bootstrap (#1).
SET lock_timeout = '5s';

CREATE TABLE risk.account_limits (                       -- derived, typed copy of the account's tracked rules
  user_id                   uuid NOT NULL REFERENCES identity.app_user(id),
  account_id                uuid NOT NULL,               -- accounts.account (cross-module id, no FK)
  version                   integer NOT NULL CHECK (version >= 1),   -- monotonic; sent in plugin `limits`
  derived_from_version      integer NOT NULL CHECK (derived_from_version >= 1),  -- account.limits_version of the consistent read
  effective_as_of           timestamptz NOT NULL,        -- the trading-day start these effective values cover
  enforcement_enabled       boolean NOT NULL DEFAULT false,  -- S15 AC-1; never touched by the derivation upsert
  daily_loss_kind           text CHECK (daily_loss_kind IN ('hard','soft_pause','none')),   -- NULL = unset
  daily_loss_limit          NUMERIC(18,4) CHECK (daily_loss_limit > 0),
  drawdown_type             text CHECK (drawdown_type IN ('trailing_intraday','trailing_eod','static')),  -- 'other' is untracked → NULL
  drawdown_amount           NUMERIC(18,4) CHECK (drawdown_amount > 0),
  daily_profit_target       NUMERIC(18,4) CHECK (daily_profit_target > 0),
  profit_target_on_reach    text NOT NULL DEFAULT 'notify' CHECK (profit_target_on_reach IN ('notify','flatten_and_block')),
  consistency_threshold_pct NUMERIC(4,1) CHECK (consistency_threshold_pct BETWEEN 0 AND 100),  -- tracked, never enforced
  evaluation_profit_target  NUMERIC(18,4) CHECK (evaluation_profit_target > 0),   -- notice only (Q46)
  commission_per_contract_side NUMERIC(18,4) CHECK (commission_per_contract_side >= 0),  -- the rate in force today; NULL = none
  evaluation_start          date,                        -- copy of account.evaluation_start
  evaluation_earned_before  NUMERIC(18,4),               -- copy of account.evaluation_earned_before
  reset_source              text NOT NULL CHECK (reset_source IN ('firm','cme_session_close')),
  reset_local_time          time,                        -- firm reset (account copy; Q-S13-2)
  reset_time_zone           text,                        -- IANA
  updated_at                timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (account_id),
  UNIQUE (user_id, account_id),
  CONSTRAINT account_limits_daily_loss_check
    CHECK ((daily_loss_kind IS NULL OR daily_loss_kind = 'none') = (daily_loss_limit IS NULL)),
  CONSTRAINT account_limits_drawdown_check CHECK ((drawdown_type IS NULL) = (drawdown_amount IS NULL)),
  CONSTRAINT account_limits_reset_check
    CHECK ((reset_source = 'firm') = (reset_local_time IS NOT NULL AND reset_time_zone IS NOT NULL))
);
-- 'cme_session_close' (real accounts): the Core takes the close from the exchange calendar.

CREATE TABLE risk.scheduled_limit (                      -- DERIVED (ADR-0006 §3.7): a looser value in force from a later trading day
  user_id               uuid NOT NULL,
  account_id            uuid NOT NULL,
  kind                  text NOT NULL CHECK (kind IN ('daily_loss','drawdown','daily_profit_target','consistency',
                                                      'profit_target_on_reach','trading_day_reset',
                                                      'commission_per_contract_side')),
  new_value             jsonb,
  is_unset              boolean NOT NULL DEFAULT false,  -- the scheduled change removes the rule
  effective_from        timestamptz NOT NULL,            -- start of the trading day after the field's saved_at, under the current reset
  derived_from_version  integer NOT NULL CHECK (derived_from_version >= 1),
  PRIMARY KEY (account_id, kind),                        -- latest save wins → at most one schedule per kind
  CONSTRAINT scheduled_limit_value_check CHECK ((new_value IS NULL) = is_unset),
  CONSTRAINT scheduled_limit_object_check CHECK (new_value IS NULL OR (jsonb_typeof(new_value) = 'object'
             AND NOT jsonb_path_exists(new_value, '$.**.amount ? (@.type() == "number")'))),  -- money as strings
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id)
);
CREATE INDEX scheduled_limit_due_idx ON risk.scheduled_limit (effective_from);

CREATE TABLE risk.trading_day (                          -- one row per account and trading day; P&L inputs, not percentages
  user_id             uuid NOT NULL,
  account_id          uuid NOT NULL,
  trading_date        date NOT NULL,                     -- the day label in the reset's time zone
  starts_at           timestamptz NOT NULL,
  ends_at             timestamptz NOT NULL,              -- = next reset
  source              text NOT NULL CHECK (source IN ('firm','cme_session_close')),
  start_balance       NUMERIC(18,4),                     -- balance at the reset, from the platform snapshot
  realized_pnl_gross  NUMERIC(18,4) NOT NULL DEFAULT 0,
  realized_commission NUMERIC(18,4) NOT NULL DEFAULT 0 CHECK (realized_commission >= 0),  -- Σ fills' commission_effective
  commission_gap      NUMERIC(18,4) NOT NULL DEFAULT 0 CHECK (commission_gap >= 0),       -- Σ (effective − reported)
  realized_pnl_net    NUMERIC(18,4) GENERATED ALWAYS AS (realized_pnl_gross - realized_commission) STORED,
  unrealized_pnl      NUMERIC(18,4),                     -- last known; NULL when unknown
  as_of               timestamptz,                       -- time of the last P&L input (staleness)
  PRIMARY KEY (account_id, trading_date),
  CONSTRAINT trading_day_range_check CHECK (ends_at > starts_at),
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id)
);

CREATE TABLE risk.evaluation_progress (                  -- DERIVED (ADR-0006): "how much is left to pass" (Q46)
  user_id               uuid NOT NULL,
  account_id            uuid NOT NULL PRIMARY KEY,
  evaluation_start      date NOT NULL,
  baseline_balance      NUMERIC(18,4) NOT NULL,          -- net balance at the start of evaluation_start's trading day
  baseline_source       text NOT NULL CHECK (baseline_source IN ('platform_snapshot','account_size')),
  derived_from_version  integer NOT NULL CHECK (derived_from_version >= 1),
  target_reached_at     timestamptz,                     -- notice + notification once; no automatic action
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id)
);

CREATE TABLE risk.drawdown_tracker (                     -- trailing drawdown state (Q-S13-5)
  user_id             uuid NOT NULL,
  account_id          uuid NOT NULL PRIMARY KEY,
  high_water_balance  NUMERIC(18,4) NOT NULL,            -- intraday or end-of-day per drawdown_type
  floor_balance       NUMERIC(18,4) NOT NULL,            -- balance at which the drawdown is breached, now
  stopped_trailing_at timestamptz,                       -- when the firm's "stops trailing at" condition was met
  as_of               timestamptz NOT NULL,
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id)
);

CREATE TABLE risk.account_block (                        -- blocked until the reset (S15 writes it; no early unlock)
  id          uuid PRIMARY KEY,
  user_id     uuid NOT NULL,
  account_id  uuid NOT NULL,
  cause       text NOT NULL CHECK (cause IN ('daily_loss','drawdown','daily_profit_target')),
  blocked_at  timestamptz NOT NULL DEFAULT now(),
  until       timestamptz NOT NULL,                      -- the trading-day end
  ended_at    timestamptz,                               -- set at the reset
  CONSTRAINT account_block_until_check CHECK (until > blocked_at),
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id)
);
CREATE UNIQUE INDEX account_block_active_key ON risk.account_block (account_id) WHERE ended_at IS NULL;

ALTER TABLE risk.account_limits      OWNER TO allwin_owner;
ALTER TABLE risk.scheduled_limit     OWNER TO allwin_owner;
ALTER TABLE risk.trading_day         OWNER TO allwin_owner;
ALTER TABLE risk.evaluation_progress OWNER TO allwin_owner;
ALTER TABLE risk.drawdown_tracker    OWNER TO allwin_owner;
ALTER TABLE risk.account_block       OWNER TO allwin_owner;

ALTER TABLE risk.account_limits      ENABLE ROW LEVEL SECURITY;
ALTER TABLE risk.account_limits      FORCE  ROW LEVEL SECURITY;
ALTER TABLE risk.scheduled_limit     ENABLE ROW LEVEL SECURITY;
ALTER TABLE risk.scheduled_limit     FORCE  ROW LEVEL SECURITY;
ALTER TABLE risk.trading_day         ENABLE ROW LEVEL SECURITY;
ALTER TABLE risk.trading_day         FORCE  ROW LEVEL SECURITY;
ALTER TABLE risk.evaluation_progress ENABLE ROW LEVEL SECURITY;
ALTER TABLE risk.evaluation_progress FORCE  ROW LEVEL SECURITY;
ALTER TABLE risk.drawdown_tracker    ENABLE ROW LEVEL SECURITY;
ALTER TABLE risk.drawdown_tracker    FORCE  ROW LEVEL SECURITY;
ALTER TABLE risk.account_block       ENABLE ROW LEVEL SECURITY;
ALTER TABLE risk.account_block       FORCE  ROW LEVEL SECURITY;

CREATE POLICY account_limits_owner ON risk.account_limits
  USING (user_id = app.current_user_id()) WITH CHECK (user_id = app.current_user_id());
CREATE POLICY scheduled_limit_owner ON risk.scheduled_limit
  USING (user_id = app.current_user_id()) WITH CHECK (user_id = app.current_user_id());
CREATE POLICY trading_day_owner ON risk.trading_day
  USING (user_id = app.current_user_id()) WITH CHECK (user_id = app.current_user_id());
CREATE POLICY evaluation_progress_owner ON risk.evaluation_progress
  USING (user_id = app.current_user_id()) WITH CHECK (user_id = app.current_user_id());
CREATE POLICY drawdown_tracker_owner ON risk.drawdown_tracker
  USING (user_id = app.current_user_id()) WITH CHECK (user_id = app.current_user_id());
CREATE POLICY account_block_owner ON risk.account_block
  USING (user_id = app.current_user_id()) WITH CHECK (user_id = app.current_user_id());

-- allwin_app keeps the bootstrap default (SELECT, INSERT, UPDATE, DELETE) only on scheduled_limit,
-- whose due rows the reset step deletes. Everywhere else no runtime path deletes: trading_day is
-- kept for the account's lifetime (consistency) and a block row never vanishes (no early unlock).
-- Q13-b erasure runs as allwin_maint (its grant comes in a later migration).
REVOKE DELETE ON risk.account_limits, risk.trading_day, risk.evaluation_progress,
                 risk.drawdown_tracker, risk.account_block FROM allwin_app;

-- allwin_worker (BYPASSRLS) only FINDS due work; each item's business work then runs on the
-- allwin_app pool with app.user_id set.
GRANT SELECT (user_id, account_id, effective_from) ON risk.scheduled_limit TO allwin_worker;  -- due schedules
GRANT SELECT (user_id, account_id, until, ended_at) ON risk.account_block TO allwin_worker;    -- blocks to end
GRANT SELECT (user_id, account_id, effective_as_of, reset_source, reset_local_time, reset_time_zone)
  ON risk.account_limits TO allwin_worker;                                                    -- stuck-reset scan
