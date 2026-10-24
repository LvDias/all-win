-- Wave 1 #8 — identity: adapter connections, pairing codes and device tokens
-- (wave1-data-model rev. 3 §4.1, ADR-0004 §8, S3 AC-4, Q18). Builds on #2.
-- Device token `awd_<key_id>_<secret>`: only key_id and SHA-256 of the 32 decoded secret bytes
-- are stored. Pairing codes (40 bits) are stored as HMAC-SHA-256 with a versioned pepper.
--
-- Note for DBA review: the data model names the pairing lookup `redeem_pairing_code`. Here it is
-- the read-only `identity.resolve_pairing_code(bytea[])`; redemption is a conditional UPDATE
-- (used_at IS NULL AND invalidated_at IS NULL AND expires_at > now) in the same user-scoped
-- transaction that records the instance and swaps the device token, so the function needs no
-- write access under FORCE RLS. It takes an array to match codes across a pepper rotation.
SET lock_timeout = '5s';

-- One connection per NinjaTrader instance (S3 AC-4, ADR-0004 §8)
CREATE TABLE identity.adapter_connection (
  id                uuid PRIMARY KEY,                    -- connection_id (welcome)
  user_id           uuid NOT NULL REFERENCES identity.app_user(id),
  name              text NOT NULL,                       -- "NinjaTrader – Desk A"
  adapter_type      text CHECK (adapter_type ~ '^[a-z][a-z0-9_]{0,31}$'),  -- from pairing ('ninjatrader8')
  instance_id       uuid,                                -- hello.instance_id
  instance_name     text,
  adapter_version   text,
  status            text NOT NULL DEFAULT 'awaiting_pairing'
                    CHECK (status IN ('awaiting_pairing','connected','disconnected','outdated','unpaired')),
  status_changed_at timestamptz NOT NULL DEFAULT now(), -- = created_at until the first change
  paired_at         timestamptz,
  last_seen_at      timestamptz,
  created_at        timestamptz NOT NULL DEFAULT now(),
  archived_at       timestamptz,                         -- unpaired (Q18); row becomes read-only history
  UNIQUE (user_id, id),
  CONSTRAINT adapter_connection_unpaired_check CHECK ((status = 'unpaired') = (archived_at IS NOT NULL)),
  CONSTRAINT adapter_connection_paired_check
    CHECK (status = 'awaiting_pairing' OR status = 'unpaired' OR paired_at IS NOT NULL)
);
ALTER TABLE identity.adapter_connection OWNER TO allwin_owner;
CREATE UNIQUE INDEX adapter_connection_user_name_key ON identity.adapter_connection (user_id, lower(name))
  WHERE archived_at IS NULL;                             -- connections.name_taken, case-insensitive, ignores unpaired
ALTER TABLE identity.adapter_connection ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.adapter_connection FORCE  ROW LEVEL SECURITY;
CREATE POLICY adapter_connection_owner ON identity.adapter_connection
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
CREATE POLICY adapter_connection_definer_read ON identity.adapter_connection FOR SELECT TO allwin_owner USING (true);

CREATE TABLE identity.pairing_code (
  id             uuid PRIMARY KEY,
  user_id        uuid NOT NULL,
  connection_id  uuid NOT NULL,
  code_hmac      bytea NOT NULL UNIQUE CHECK (octet_length(code_hmac) = 32),  -- 40-bit code → HMAC-SHA-256 with pepper (ALIGN-4)
  pepper_version smallint NOT NULL,
  created_at     timestamptz NOT NULL DEFAULT now(),
  expires_at     timestamptz NOT NULL,                   -- +10 min
  used_at        timestamptz,
  invalidated_at timestamptz,                            -- superseded by a new code, or connection unpaired
  FOREIGN KEY (user_id, connection_id) REFERENCES identity.adapter_connection (user_id, id)
);
ALTER TABLE identity.pairing_code OWNER TO allwin_owner;
CREATE UNIQUE INDEX pairing_code_one_active ON identity.pairing_code (connection_id)
  WHERE used_at IS NULL AND invalidated_at IS NULL;
ALTER TABLE identity.pairing_code ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.pairing_code FORCE  ROW LEVEL SECURITY;
CREATE POLICY pairing_code_owner ON identity.pairing_code
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
CREATE POLICY pairing_code_definer_read ON identity.pairing_code FOR SELECT TO allwin_owner USING (true);

CREATE TABLE identity.device_token (                     -- token awd_<key_id>_<secret>
  id            uuid PRIMARY KEY,
  user_id       uuid NOT NULL,
  connection_id uuid NOT NULL,
  key_id        text NOT NULL UNIQUE CHECK (key_id ~ '^[A-Za-z0-9]{12}$'),  -- ALIGN-3
  token_hash    bytea NOT NULL CHECK (octet_length(token_hash) = 32),    -- SHA-256 of the 32-byte secret
  created_at    timestamptz NOT NULL DEFAULT now(),
  revoked_at    timestamptz,
  FOREIGN KEY (user_id, connection_id) REFERENCES identity.adapter_connection (user_id, id)
);
ALTER TABLE identity.device_token OWNER TO allwin_owner;
CREATE UNIQUE INDEX device_token_one_active ON identity.device_token (connection_id) WHERE revoked_at IS NULL;
-- re-pairing revokes the previous token in the same transaction (ADR-0004 §8)
ALTER TABLE identity.device_token ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.device_token FORCE  ROW LEVEL SECURITY;
CREATE POLICY device_token_owner ON identity.device_token
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
CREATE POLICY device_token_definer_read ON identity.device_token FOR SELECT TO allwin_owner USING (true);

-- Adapter WebSocket authentication (S2). `revoked` is true when the token is revoked, the
-- connection is unpaired, or the user is not active.
CREATE FUNCTION identity.resolve_device_token(p_key_id text)
  RETURNS TABLE (user_id uuid, connection_id uuid, token_hash bytea, revoked boolean)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT t.user_id, t.connection_id, t.token_hash,
           (t.revoked_at IS NOT NULL OR c.archived_at IS NOT NULL OR u.status <> 'active')
      FROM identity.device_token t
      JOIN identity.adapter_connection c ON c.user_id = t.user_id AND c.id = t.connection_id
      JOIN identity.app_user u ON u.id = t.user_id
     WHERE t.key_id = p_key_id
  $$;

-- Pairing (POST /adapter/v1/pairings): the request carries only the code. Returns the matching
-- code of an active user under any accepted pepper version; the Core checks redeemability.
CREATE FUNCTION identity.resolve_pairing_code(p_code_hmacs bytea[])
  RETURNS TABLE (id uuid, user_id uuid, connection_id uuid, code_hmac bytea, pepper_version smallint,
                 created_at timestamptz, expires_at timestamptz, used_at timestamptz, invalidated_at timestamptz)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT p.id, p.user_id, p.connection_id, p.code_hmac, p.pepper_version, p.created_at,
           p.expires_at, p.used_at, p.invalidated_at
      FROM identity.pairing_code p
      JOIN identity.app_user u ON u.id = p.user_id
     WHERE p.code_hmac = ANY (p_code_hmacs)
       AND u.status = 'active'
  $$;

-- Core startup, before adapter connections are accepted: no adapter session survives a restart.
-- Lists every connection still marked live; the Core then, per user, sets them 'disconnected'
-- and writes audit + SSE events to app.outbox in one user-scoped transaction (idempotent: users
-- not yet processed are still listed at the next startup). Executable by allwin_worker only.
CREATE FUNCTION identity.list_live_connections()
  RETURNS TABLE (user_id uuid, connection_id uuid)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT c.user_id, c.id
      FROM identity.adapter_connection c
     WHERE c.status IN ('connected','outdated')
     ORDER BY c.user_id, c.id
  $$;

ALTER FUNCTION identity.list_live_connections() OWNER TO allwin_owner;
GRANT EXECUTE ON FUNCTION identity.list_live_connections() TO allwin_worker;
ALTER FUNCTION identity.resolve_device_token(text)    OWNER TO allwin_owner;
ALTER FUNCTION identity.resolve_pairing_code(bytea[]) OWNER TO allwin_owner;
GRANT EXECUTE ON FUNCTION identity.resolve_device_token(text)    TO allwin_app;
GRANT EXECUTE ON FUNCTION identity.resolve_pairing_code(bytea[]) TO allwin_app;
