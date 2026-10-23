-- Wave 1 #5b — trading accounts (wave1-data-model §5.2, rev. 8–15.2; S5 contract 4.3.0).
-- Binding (ADR-0008, Esquadro 2026-10-23): accounts owns the link; `venue_account_id` is gateway's
-- opaque id (no cross-schema FK), one platform account ↔ at most one All Win account.
-- Cross-module ids are plain uuid; the only cross-module FK is identity.app_user (§1, rule 1).
SET lock_timeout = '5s';

CREATE TABLE accounts.account (
  id                uuid PRIMARY KEY,
  user_id           uuid NOT NULL REFERENCES identity.app_user(id),
  display_name      text NOT NULL CHECK (char_length(display_name) BETWEEN 1 AND 80),
  kind              text NOT NULL CHECK (kind IN ('prop','real')),
  firm_id           uuid,                                -- prop with a preset; catalog (S16)
  firm_name         text CHECK (char_length(firm_name) BETWEEN 1 AND 80),  -- prop, manual mode
  plan_name         text CHECK (char_length(plan_name) BETWEEN 1 AND 80),  -- manual mode
  phase             text CHECK (char_length(phase) BETWEEN 1 AND 80),      -- manual mode
  preset_version_id uuid,                                -- S16
  firm_platform     text CHECK (firm_platform ~ '^[a-z][a-z0-9_]{0,31}$'),  -- with a preset only
  size_amount       NUMERIC(18,4) NOT NULL CHECK (size_amount > 0),
  currency          char(3) NOT NULL DEFAULT 'USD',
  multiplier        integer NOT NULL DEFAULT 1 CHECK (multiplier BETWEEN 1 AND 1000),
  max_position_qty  integer NOT NULL CHECK (max_position_qty BETWEEN 1 AND 10000),
  lifecycle_state   text NOT NULL DEFAULT 'active' CHECK (lifecycle_state IN ('active','paused','disabled')),
  bracket_mode      text NOT NULL DEFAULT 'individual' CHECK (bracket_mode IN ('individual','shared')),
  command_ttl_ms    integer NOT NULL DEFAULT 5000 CHECK (command_ttl_ms BETWEEN 1000 AND 60000),
  venue_account_id  uuid,                                -- gateway.venue_account (cross-module id; ADR-0008)
  bound_at          timestamptz,
  automation_state  text NOT NULL DEFAULT 'not_required'
                    CHECK (automation_state IN ('not_required','chosen','pending_review')),
  automation_choice text CHECK (automation_choice IN ('accepted','declined')),
  automation_rule_version_id uuid,                       -- catalog.opt_in_rule_version (cross-module id)
  automation_pending_reason text CHECK (automation_pending_reason IN ('stance_changed','text_changed','preset_attached')),
  automation_chosen_at timestamptz,
  copy_state        text NOT NULL DEFAULT 'not_required'
                    CHECK (copy_state IN ('not_required','chosen','pending_review')),
  copy_choice       text CHECK (copy_choice IN ('accepted','declined')),
  copy_rule_version_id uuid,                             -- catalog.opt_in_rule_version (cross-module id)
  copy_pending_reason text CHECK (copy_pending_reason IN ('stance_changed','text_changed','preset_attached')),
  copy_chosen_at    timestamptz,
  created_at        timestamptz NOT NULL DEFAULT now(),
  updated_at        timestamptz NOT NULL DEFAULT now(),
  archived_at       timestamptz,                         -- "deleted" (S5 AC-6); history keeps the id
  row_version       integer NOT NULL DEFAULT 1 CHECK (row_version >= 1),
  limits_version    integer NOT NULL DEFAULT 1 CHECK (limits_version >= 1),  -- ADR-0006
  limits_changed_at timestamptz NOT NULL DEFAULT now(),  -- ADR-0006 §3.5
  evaluation_start  date,                                -- S13b; Core default = registration date
  evaluation_earned_before NUMERIC(18,4) CHECK (evaluation_earned_before >= 0),
  create_idempotency_key uuid,                           -- Idempotency-Key of the create
  create_request_hash    bytea CHECK (octet_length(create_request_hash) = 32),  -- SHA-256, step_up excluded
  UNIQUE (user_id, id),
  CONSTRAINT account_create_key UNIQUE (user_id, create_idempotency_key),
  CONSTRAINT account_create_key_hash_check CHECK ((create_idempotency_key IS NULL) = (create_request_hash IS NULL)),
  CONSTRAINT account_binding_check CHECK ((venue_account_id IS NULL) = (bound_at IS NULL)),
  CONSTRAINT account_real_check CHECK (kind = 'prop' OR (firm_id IS NULL AND firm_name IS NULL AND plan_name IS NULL
             AND phase IS NULL AND preset_version_id IS NULL AND firm_platform IS NULL)),
  -- prop: a preset XOR the three manual fields (S5 contract §2.4); with a preset they come from the catalog
  CONSTRAINT account_prop_selection_check CHECK (kind = 'real'
         OR (preset_version_id IS NOT NULL AND firm_name IS NULL AND plan_name IS NULL AND phase IS NULL)
         OR (preset_version_id IS NULL AND firm_id IS NULL AND firm_platform IS NULL
             AND firm_name IS NOT NULL AND plan_name IS NOT NULL AND phase IS NOT NULL)),
  CONSTRAINT account_automation_pending_check CHECK ((automation_state = 'pending_review') = (automation_pending_reason IS NOT NULL)),
  CONSTRAINT account_copy_pending_check CHECK ((copy_state = 'pending_review') = (copy_pending_reason IS NOT NULL)),
  CONSTRAINT account_real_opt_in_check CHECK (kind = 'prop' OR (automation_state = 'not_required' AND copy_state = 'not_required')),
  CONSTRAINT account_automation_chosen_at_check CHECK ((automation_choice IS NULL) = (automation_chosen_at IS NULL)),
  CONSTRAINT account_automation_chosen_check CHECK (automation_state <> 'chosen' OR automation_choice IS NOT NULL),
  CONSTRAINT account_automation_not_required_check CHECK (automation_state <> 'not_required'
             OR (automation_choice IS NULL AND automation_rule_version_id IS NULL)),
  CONSTRAINT account_copy_chosen_at_check CHECK ((copy_choice IS NULL) = (copy_chosen_at IS NULL)),
  CONSTRAINT account_copy_chosen_check CHECK (copy_state <> 'chosen' OR copy_choice IS NOT NULL),
  CONSTRAINT account_copy_not_required_check CHECK (copy_state <> 'not_required'
             OR (copy_choice IS NULL AND copy_rule_version_id IS NULL)),
  CONSTRAINT account_automation_rule_check CHECK (automation_choice IS NULL OR automation_rule_version_id IS NOT NULL),
  CONSTRAINT account_copy_rule_check CHECK (copy_choice IS NULL OR copy_rule_version_id IS NOT NULL)
);
ALTER TABLE accounts.account OWNER TO allwin_owner;
CREATE UNIQUE INDEX account_user_name_key ON accounts.account (user_id, lower(display_name)) WHERE archived_at IS NULL;
CREATE INDEX account_user_live_idx ON accounts.account (user_id) WHERE archived_at IS NULL;
-- 1 platform account ↔ at most 1 All Win account (ADR-0008). Archiving releases the binding
-- (venue_account_id set to NULL), so archived rows never hold one.
CREATE UNIQUE INDEX account_venue_account_key ON accounts.account (venue_account_id) WHERE venue_account_id IS NOT NULL;

ALTER TABLE accounts.account ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounts.account FORCE  ROW LEVEL SECURITY;
CREATE POLICY account_owner ON accounts.account
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

-- Accounts are archived, never deleted at runtime (history keeps the id); Q13-b erasure is a
-- maintenance job with its own grant.
REVOKE DELETE ON accounts.account FROM allwin_app;
