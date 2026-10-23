-- Wave 1 #3 — identity: instruction sources and their keys (wave1-data-model §4.1, ADR-0001 §5,
-- ALIGN-3). Builds on #2. Token `awk_<key_id>_<secret>`: only key_id and SHA-256 of the 32
-- decoded secret bytes are stored. Rotation = revoke + insert in one transaction (immediate).
--
-- Delta from wave1-data-model §4.1 (for DBA review): D4 as in #2 (SELECT policies TO
-- allwin_owner for the SECURITY DEFINER lookup under FORCE RLS).
SET lock_timeout = '5s';

-- Instruction sources: several named sources per user (Q17)
CREATE TABLE identity.source (
  id          uuid PRIMARY KEY,
  user_id     uuid NOT NULL REFERENCES identity.app_user(id),
  source_type text NOT NULL CHECK (source_type ~ '^[a-z][a-z0-9_]{0,31}$'),  -- 'tradingview' (ADR-0003 §3)
  name        text NOT NULL,
  created_at  timestamptz NOT NULL DEFAULT now(),
  archived_at timestamptz,                                                    -- revoked; kept read-only for history
  UNIQUE (user_id, id)
);
ALTER TABLE identity.source OWNER TO allwin_owner;
CREATE UNIQUE INDEX source_user_name_key ON identity.source (user_id, lower(name)) WHERE archived_at IS NULL;
ALTER TABLE identity.source ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.source FORCE  ROW LEVEL SECURITY;
CREATE POLICY source_owner ON identity.source
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
CREATE POLICY source_definer_read ON identity.source FOR SELECT TO allwin_owner USING (true);

CREATE TABLE identity.source_key (                       -- token awk_<key_id>_<secret>
  id          uuid PRIMARY KEY,
  user_id     uuid NOT NULL,
  source_id   uuid NOT NULL,
  key_id      text NOT NULL UNIQUE CHECK (key_id ~ '^[A-Za-z0-9]{12}$'),  -- public part; may appear in audit/logs (ALIGN-3)
  secret_hash bytea NOT NULL CHECK (octet_length(secret_hash) = 32),    -- SHA-256 of the 32-byte secret; never the token
  created_at  timestamptz NOT NULL DEFAULT now(),
  revoked_at  timestamptz,
  FOREIGN KEY (user_id, source_id) REFERENCES identity.source (user_id, id)
);
ALTER TABLE identity.source_key OWNER TO allwin_owner;
CREATE UNIQUE INDEX source_key_one_active ON identity.source_key (source_id) WHERE revoked_at IS NULL;
CREATE INDEX source_key_source_created_idx ON identity.source_key (source_id, created_at);  -- current/last key, rotated_at
ALTER TABLE identity.source_key ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.source_key FORCE  ROW LEVEL SECURITY;
CREATE POLICY source_key_owner ON identity.source_key
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
CREATE POLICY source_key_definer_read ON identity.source_key FOR SELECT TO allwin_owner USING (true);

-- Webhook authentication (S1): the ingress knows only the key_id before it trusts the request.
-- `revoked` is true when the key or its source is revoked, or the user is not active (a closed
-- account must stop accepting webhooks during its grace period).
CREATE FUNCTION identity.resolve_source_key(p_key_id text)
  RETURNS TABLE (user_id uuid, source_id uuid, source_type text, secret_hash bytea, revoked boolean)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT k.user_id, k.source_id, s.source_type, k.secret_hash,
           (k.revoked_at IS NOT NULL OR s.archived_at IS NOT NULL OR u.status <> 'active')
      FROM identity.source_key k
      JOIN identity.source s ON s.user_id = k.user_id AND s.id = k.source_id
      JOIN identity.app_user u ON u.id = k.user_id
     WHERE k.key_id = p_key_id
  $$;
ALTER FUNCTION identity.resolve_source_key(text) OWNER TO allwin_owner;
GRANT EXECUTE ON FUNCTION identity.resolve_source_key(text) TO allwin_app;
