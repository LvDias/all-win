-- Wave 1 #12 (moved to S5, Lastro 2026-10-23) — the account's own rule values (wave1-data-model
-- §5.7, rev. 15.2). Only deviations from the preset are stored; the absence of a row means "from
-- the preset" (or "unset" without one). Inputs of the derived risk limits (ADR-0006): every write
-- bumps account.limits_version in the same transaction (Core rule).
SET lock_timeout = '5s';

CREATE TABLE accounts.account_rule_value (
  user_id     uuid NOT NULL,
  account_id  uuid NOT NULL,
  field       text NOT NULL CHECK (field IN ('max_position_qty','drawdown','daily_loss_limit',
                                             'daily_profit_target','trading_day_reset','consistency_rule',
                                             'profit_target_on_reach','consistency_threshold_pct',
                                             'commission_per_contract_side','evaluation_profit_target')),
  origin      text NOT NULL CHECK (origin IN ('edited','chosen_from_conflict','set_by_user')),
  value       jsonb,                                     -- typed by field (PresetRuleValues); NULL for max_position_qty
  chose       text CHECK (chose IN ('published_value','other_value')),
  chosen_at   timestamptz,
  is_unset    boolean NOT NULL DEFAULT false,            -- the user removed this rule; a tombstone that keeps saved_at
  saved_at    timestamptz NOT NULL DEFAULT now(),        -- this field's own save time (ADR-0006 §3.7)
  updated_at  timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (account_id, field),
  FOREIGN KEY (user_id, account_id) REFERENCES accounts.account (user_id, id),
  CONSTRAINT account_rule_value_value_check CHECK (CASE WHEN field = 'max_position_qty' THEN value IS NULL AND NOT is_unset
              ELSE (value IS NULL) = is_unset END),
  CONSTRAINT account_rule_value_object_check CHECK (value IS NULL OR jsonb_typeof(value) = 'object'),
  CONSTRAINT account_rule_value_amount_check CHECK (value IS NULL
             OR NOT jsonb_path_exists(value, '$.**.amount ? (@.type() == "number")')),
  CONSTRAINT account_rule_value_conflict_check CHECK ((origin = 'chosen_from_conflict') = (chose IS NOT NULL)),
  CONSTRAINT account_rule_value_chosen_at_check CHECK ((chose IS NULL) = (chosen_at IS NULL))
);
ALTER TABLE accounts.account_rule_value OWNER TO allwin_owner;

ALTER TABLE accounts.account_rule_value ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounts.account_rule_value FORCE  ROW LEVEL SECURITY;
CREATE POLICY account_rule_value_owner ON accounts.account_rule_value
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

-- allwin_app keeps the bootstrap default (SELECT, INSERT, UPDATE, DELETE): restore and apply
-- delete rows.
