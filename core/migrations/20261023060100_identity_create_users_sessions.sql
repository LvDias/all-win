-- Wave 1 #2 — identity: users, invitations, sessions, password resets, TOTP, recovery codes
-- (wave1-data-model §4.1, auth-and-session-policy, HTTP contract 1.1.0). Builds on #1 app_bootstrap.
--
-- Deltas from wave1-data-model §4.1 (for DBA review):
--   D1 invitation.locale: createInvitation takes a locale and lookupInvitation returns it.
--   D2 session.end_reason adds 'admin_totp_reset' (contract SSE session.ended, adminResetTotp).
--   D3 session.login_at: the absolute timeout counts from the password sign-in and survives
--      rotation (after TOTP or a password change), so it cannot be derived from created_at.
--   D4 SELECT-only policies TO allwin_owner: FORCE RLS applies to the table owner too, and the
--      SECURITY DEFINER lookups below run as allwin_owner (never a runtime login role).
--   D5 identity.email_registered() and identity.admin_list_users(): pre-auth / cross-user reads
--      needed by createInvitation and listUsers, each returning only what its caller needs.
--   D6 invitation.superseded_at: re-inviting an e-mail whose invitation expired closes the old
--      row without falsifying it as revoked; the open-email index ignores superseded rows.
SET lock_timeout = '5s';

CREATE TABLE identity.app_user (
  id                uuid PRIMARY KEY,
  email             text NOT NULL,                       -- as entered
  email_normalized  text NOT NULL,                       -- trimmed + lower-cased by the Core
  password_hash     text NOT NULL,                       -- argon2id PHC string (params inside)
  locale            text NOT NULL DEFAULT 'pt-BR' CHECK (locale IN ('pt-BR','en')),
  time_zone         text NOT NULL,                       -- IANA, validated by the Core (Q23)
  is_platform_admin boolean NOT NULL DEFAULT false,      -- issues invitations (Q19)
  status            text NOT NULL DEFAULT 'active' CHECK (status IN ('active','closed')),
  created_at        timestamptz NOT NULL DEFAULT now(),
  updated_at        timestamptz NOT NULL DEFAULT now(),
  closed_at         timestamptz,                         -- erase after grace (Q13-b)
  CONSTRAINT app_user_email_normalized_key UNIQUE (email_normalized)
);
ALTER TABLE identity.app_user OWNER TO allwin_owner;
ALTER TABLE identity.app_user ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.app_user FORCE  ROW LEVEL SECURITY;
CREATE POLICY app_user_self ON identity.app_user
  USING (id = app.current_user_id())
  WITH CHECK (id = app.current_user_id());
CREATE POLICY app_user_definer_read ON identity.app_user FOR SELECT TO allwin_owner USING (true);

CREATE TABLE identity.invitation (                       -- admin data, no RLS
  id               uuid PRIMARY KEY,
  email_normalized text NOT NULL,
  locale           text NOT NULL CHECK (locale IN ('pt-BR','en')),            -- D1
  token_hash       bytea NOT NULL UNIQUE CHECK (octet_length(token_hash) = 32), -- SHA-256 of 256-bit token
  issued_by        uuid NOT NULL REFERENCES identity.app_user(id),
  created_at       timestamptz NOT NULL DEFAULT now(),
  expires_at       timestamptz NOT NULL,                 -- +7 days (security policy §3)
  accepted_at      timestamptz,
  accepted_user_id uuid REFERENCES identity.app_user(id),
  revoked_at       timestamptz,
  superseded_at    timestamptz,                          -- D6: replaced by a new invitation to the same e-mail
  CONSTRAINT invitation_accepted_check CHECK ((accepted_at IS NULL) = (accepted_user_id IS NULL))
);
ALTER TABLE identity.invitation OWNER TO allwin_owner;
CREATE UNIQUE INDEX invitation_open_email_key ON identity.invitation (email_normalized)
  WHERE accepted_at IS NULL AND revoked_at IS NULL AND superseded_at IS NULL;

CREATE TABLE identity.session (
  id                  uuid PRIMARY KEY,
  user_id             uuid NOT NULL REFERENCES identity.app_user(id),
  token_hash          bytea NOT NULL UNIQUE CHECK (octet_length(token_hash) = 32), -- SHA-256 of the cookie secret
  is_persistent       boolean NOT NULL,                  -- "keep me signed in"
  mfa_state           text NOT NULL CHECK (mfa_state IN ('not_required','pending','verified')),
  created_at          timestamptz NOT NULL DEFAULT now(),
  login_at            timestamptz NOT NULL,              -- D3: password sign-in; absolute timeout base
  authenticated_at    timestamptz NOT NULL,              -- last password(+TOTP) proof → 15-min step-up rule
  last_activity_at    timestamptz NOT NULL,              -- user-initiated requests only (SSE excluded)
  idle_expires_at     timestamptz NOT NULL,
  absolute_expires_at timestamptz NOT NULL,
  ended_at            timestamptz,
  end_reason          text CHECK (end_reason IN ('logout','expired','password_reset','password_changed',
                                                 'totp_changed','admin_totp_reset','rotated')),  -- D2
  ip_address          inet,                              -- security; same retention as the activity record
  user_agent          text,
  CONSTRAINT session_end_check CHECK ((ended_at IS NULL) = (end_reason IS NULL))
);
ALTER TABLE identity.session OWNER TO allwin_owner;
CREATE INDEX session_user_active_idx ON identity.session (user_id) WHERE ended_at IS NULL;
CREATE INDEX session_user_created_idx ON identity.session (user_id, created_at);  -- usual IPs, last sign-in
ALTER TABLE identity.session ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.session FORCE  ROW LEVEL SECURITY;
CREATE POLICY session_owner ON identity.session
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
CREATE POLICY session_definer_read ON identity.session FOR SELECT TO allwin_owner USING (true);

CREATE TABLE identity.totp_credential (
  user_id          uuid PRIMARY KEY REFERENCES identity.app_user(id),
  secret_encrypted bytea NOT NULL,                       -- AES-256-GCM: nonce || ciphertext || tag
  key_version      smallint NOT NULL,                    -- encryption key rotation
  created_at       timestamptz NOT NULL DEFAULT now(),
  enabled_at       timestamptz,                          -- NULL until enrolment confirmed
  last_used_step   bigint                                -- replay protection (security policy §2)
);
ALTER TABLE identity.totp_credential OWNER TO allwin_owner;
ALTER TABLE identity.totp_credential ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.totp_credential FORCE  ROW LEVEL SECURITY;
CREATE POLICY totp_credential_owner ON identity.totp_credential
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
CREATE POLICY totp_credential_definer_read ON identity.totp_credential FOR SELECT TO allwin_owner USING (true);

CREATE TABLE identity.recovery_code (
  id             uuid PRIMARY KEY,
  user_id        uuid NOT NULL REFERENCES identity.app_user(id),
  batch_id       uuid NOT NULL,                          -- regeneration replaces the batch
  code_hmac      bytea NOT NULL CHECK (octet_length(code_hmac) = 32),  -- HMAC-SHA-256(pepper, normalized code) (ALIGN-4)
  pepper_version smallint NOT NULL,                      -- codes live for months; survives a pepper rotation
  created_at     timestamptz NOT NULL DEFAULT now(),
  used_at        timestamptz,
  UNIQUE (user_id, code_hmac)
);
ALTER TABLE identity.recovery_code OWNER TO allwin_owner;
ALTER TABLE identity.recovery_code ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.recovery_code FORCE  ROW LEVEL SECURITY;
CREATE POLICY recovery_code_owner ON identity.recovery_code
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

CREATE TABLE identity.password_reset (
  id             uuid PRIMARY KEY,
  user_id        uuid NOT NULL REFERENCES identity.app_user(id),
  token_hash     bytea NOT NULL UNIQUE CHECK (octet_length(token_hash) = 32),
  created_at     timestamptz NOT NULL DEFAULT now(),
  expires_at     timestamptz NOT NULL,                   -- +30 min
  used_at        timestamptz,
  invalidated_at timestamptz                             -- a newer request invalidates older ones
);
ALTER TABLE identity.password_reset OWNER TO allwin_owner;
CREATE INDEX password_reset_user_open_idx ON identity.password_reset (user_id)
  WHERE used_at IS NULL AND invalidated_at IS NULL;
ALTER TABLE identity.password_reset ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.password_reset FORCE  ROW LEVEL SECURITY;
CREATE POLICY password_reset_owner ON identity.password_reset
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
CREATE POLICY password_reset_definer_read ON identity.password_reset FOR SELECT TO allwin_owner USING (true);

-- Pre-authentication lookups (data model §3). Each returns only what its caller needs; the Core
-- then runs user-scoped transactions with app.user_id set to the resolved user.

CREATE FUNCTION identity.find_user_for_login(p_email_normalized text)
  RETURNS TABLE (id uuid, email text, email_normalized text, password_hash text, locale text,
                 time_zone text, is_platform_admin boolean, status text, created_at timestamptz)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT u.id, u.email, u.email_normalized, u.password_hash, u.locale, u.time_zone,
           u.is_platform_admin, u.status, u.created_at
      FROM identity.app_user u
     WHERE u.email_normalized = p_email_normalized
  $$;

CREATE FUNCTION identity.email_registered(p_email_normalized text)
  RETURNS boolean
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$ SELECT EXISTS (SELECT 1 FROM identity.app_user u WHERE u.email_normalized = p_email_normalized) $$;

CREATE FUNCTION identity.resolve_session(p_token_hash bytea)
  RETURNS TABLE (id uuid, user_id uuid, is_persistent boolean, mfa_state text, login_at timestamptz,
                 authenticated_at timestamptz, last_activity_at timestamptz, idle_expires_at timestamptz,
                 absolute_expires_at timestamptz, ended_at timestamptz, end_reason text)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT s.id, s.user_id, s.is_persistent, s.mfa_state, s.login_at, s.authenticated_at,
           s.last_activity_at, s.idle_expires_at, s.absolute_expires_at, s.ended_at, s.end_reason
      FROM identity.session s
     WHERE s.token_hash = p_token_hash
  $$;

CREATE FUNCTION identity.resolve_invitation(p_token_hash bytea)
  RETURNS TABLE (id uuid, email_normalized text, locale text, issued_by uuid, created_at timestamptz,
                 expires_at timestamptz, accepted_at timestamptz, accepted_user_id uuid, revoked_at timestamptz,
                 superseded_at timestamptz)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT i.id, i.email_normalized, i.locale, i.issued_by, i.created_at, i.expires_at,
           i.accepted_at, i.accepted_user_id, i.revoked_at, i.superseded_at
      FROM identity.invitation i
     WHERE i.token_hash = p_token_hash
  $$;

CREATE FUNCTION identity.resolve_password_reset(p_token_hash bytea)
  RETURNS TABLE (id uuid, user_id uuid, created_at timestamptz, expires_at timestamptz,
                 used_at timestamptz, invalidated_at timestamptz)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT r.id, r.user_id, r.created_at, r.expires_at, r.used_at, r.invalidated_at
      FROM identity.password_reset r
     WHERE r.token_hash = p_token_hash
  $$;

-- listUsers: only for an active platform admin, who must be the transaction's app.user_id.
CREATE FUNCTION identity.admin_list_users()
  RETURNS TABLE (id uuid, email text, status text, is_platform_admin boolean, created_at timestamptz,
                 totp_enabled boolean, last_sign_in_at timestamptz)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT u.id, u.email, u.status, u.is_platform_admin, u.created_at,
           EXISTS (SELECT 1 FROM identity.totp_credential t WHERE t.user_id = u.id AND t.enabled_at IS NOT NULL),
           (SELECT max(s.login_at) FROM identity.session s WHERE s.user_id = u.id AND s.mfa_state <> 'pending')
      FROM identity.app_user u
     WHERE EXISTS (SELECT 1 FROM identity.app_user a
                    WHERE a.id = app.current_user_id() AND a.is_platform_admin AND a.status = 'active')
     ORDER BY u.created_at
  $$;

ALTER FUNCTION identity.find_user_for_login(text)    OWNER TO allwin_owner;
ALTER FUNCTION identity.email_registered(text)       OWNER TO allwin_owner;
ALTER FUNCTION identity.resolve_session(bytea)        OWNER TO allwin_owner;
ALTER FUNCTION identity.resolve_invitation(bytea)     OWNER TO allwin_owner;
ALTER FUNCTION identity.resolve_password_reset(bytea) OWNER TO allwin_owner;
ALTER FUNCTION identity.admin_list_users()            OWNER TO allwin_owner;

GRANT EXECUTE ON FUNCTION identity.find_user_for_login(text)    TO allwin_app;
GRANT EXECUTE ON FUNCTION identity.email_registered(text)       TO allwin_app;
GRANT EXECUTE ON FUNCTION identity.resolve_session(bytea)        TO allwin_app;
GRANT EXECUTE ON FUNCTION identity.resolve_invitation(bytea)     TO allwin_app;
GRANT EXECUTE ON FUNCTION identity.resolve_password_reset(bytea) TO allwin_app;
GRANT EXECUTE ON FUNCTION identity.admin_list_users()            TO allwin_app;
