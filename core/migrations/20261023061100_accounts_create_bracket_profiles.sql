-- Wave 1 #5c — stop/target profiles (wave1-data-model §5.4, rev. 8–15.2; S6 contract 4.3.0).
-- Depends on 20261023061000 (accounts.account with UNIQUE (user_id, id)).
-- Cross-module ids are plain uuid (§1, rule 1): instrument_id is validated by the Core through the
-- catalog query port, size_amount against catalog.standard_account_size in the domain.
-- USD distances: the contract takes 2 decimals; the CHECK keeps 4 as headroom (Q-S6-6).
SET lock_timeout = '5s';

CREATE TABLE accounts.default_bracket_profile (          -- per user × standard size × instrument
  user_id         uuid NOT NULL REFERENCES identity.app_user(id),
  size_amount     NUMERIC(18,4) NOT NULL,
  instrument_id   uuid NOT NULL,
  distance_unit   text NOT NULL CHECK (distance_unit IN ('points','ticks','usd')),
  stop_distance   NUMERIC(22,10) NOT NULL CHECK (stop_distance > 0),
  target_distance NUMERIC(22,10) NOT NULL CHECK (target_distance > 0),
  updated_at      timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (user_id, size_amount, instrument_id),
  CONSTRAINT default_bracket_profile_ticks_check CHECK (distance_unit <> 'ticks'
             OR (stop_distance = trunc(stop_distance) AND target_distance = trunc(target_distance))),
  CONSTRAINT default_bracket_profile_usd_check CHECK (distance_unit <> 'usd'
             OR (stop_distance = round(stop_distance, 4) AND target_distance = round(target_distance, 4)))
);
ALTER TABLE accounts.default_bracket_profile OWNER TO allwin_owner;

CREATE TABLE accounts.account_bracket_profile (          -- the account's own copy (S6 AC-2)
  user_id          uuid NOT NULL,
  account_id       uuid NOT NULL,
  instrument_id    uuid NOT NULL,
  distance_unit    text NOT NULL CHECK (distance_unit IN ('points','ticks','usd')),
  stop_distance    NUMERIC(22,10) NOT NULL CHECK (stop_distance > 0),
  target_distance  NUMERIC(22,10) NOT NULL CHECK (target_distance > 0),
  copied_from_size NUMERIC(18,4),                        -- provenance (nearest standard size for real accounts)
  origin           text NOT NULL CHECK (origin IN ('size_default','nearest_size_default','set_by_user')),
  copied_at        timestamptz,                          -- initial copy or last restore
  customized       boolean NOT NULL DEFAULT false,       -- edited after the copy; false again on restore
  row_version      integer NOT NULL DEFAULT 1 CHECK (row_version >= 1),
  updated_at       timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (account_id, instrument_id),
  FOREIGN KEY (user_id, account_id) REFERENCES accounts.account (user_id, id),
  CONSTRAINT account_bracket_profile_provenance_check CHECK ((origin = 'set_by_user') = (copied_from_size IS NULL)),
  CONSTRAINT account_bracket_profile_copied_check CHECK ((copied_from_size IS NULL) = (copied_at IS NULL)),
  CONSTRAINT account_bracket_profile_customized_check CHECK (origin <> 'set_by_user' OR NOT customized),
  CONSTRAINT account_bracket_profile_ticks_check CHECK (distance_unit <> 'ticks'
             OR (stop_distance = trunc(stop_distance) AND target_distance = trunc(target_distance))),
  CONSTRAINT account_bracket_profile_usd_check CHECK (distance_unit <> 'usd'
             OR (stop_distance = round(stop_distance, 4) AND target_distance = round(target_distance, 4)))
);
ALTER TABLE accounts.account_bracket_profile OWNER TO allwin_owner;
-- No extra index: execution reads `account_id = ANY(..) AND instrument_id = ..`, served by the PK.

ALTER TABLE accounts.default_bracket_profile ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounts.default_bracket_profile FORCE  ROW LEVEL SECURITY;
CREATE POLICY default_bracket_profile_owner ON accounts.default_bracket_profile
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

ALTER TABLE accounts.account_bracket_profile ENABLE ROW LEVEL SECURITY;
ALTER TABLE accounts.account_bracket_profile FORCE  ROW LEVEL SECURITY;
CREATE POLICY account_bracket_profile_owner ON accounts.account_bracket_profile
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

-- No endpoint removes a profile; Q13-b erasure is a maintenance job with its own grant.
REVOKE DELETE ON accounts.default_bracket_profile FROM allwin_app;
REVOKE DELETE ON accounts.account_bracket_profile FROM allwin_app;
