-- Wave 1 — identity.resolve_source_key also returns the source's name, the audit
-- `source_name` snapshot that #7b requires with every `source_id` (wave1-data-model §6.4).
-- The return type changes, so the function is dropped and created again in this transaction;
-- callers use it by name (a new OID is fine). The new column goes last.
SET lock_timeout = '5s';

DROP FUNCTION identity.resolve_source_key(text);

-- Webhook authentication (S1): the ingress knows only the key_id before it trusts the request.
-- `revoked` is true when the key or its source is revoked, or the user is not active (a closed
-- account must stop accepting webhooks during its grace period).
CREATE FUNCTION identity.resolve_source_key(p_key_id text)
  RETURNS TABLE (user_id uuid, source_id uuid, source_type text, secret_hash bytea, revoked boolean,
                 source_name text)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT k.user_id, k.source_id, s.source_type, k.secret_hash,
           (k.revoked_at IS NOT NULL OR s.archived_at IS NOT NULL OR u.status <> 'active'),
           s.name
      FROM identity.source_key k
      JOIN identity.source s ON s.user_id = k.user_id AND s.id = k.source_id
      JOIN identity.app_user u ON u.id = k.user_id
     WHERE k.key_id = p_key_id
  $$;
ALTER FUNCTION identity.resolve_source_key(text) OWNER TO allwin_owner;
GRANT EXECUTE ON FUNCTION identity.resolve_source_key(text) TO allwin_app;
