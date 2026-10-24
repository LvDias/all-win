-- Wave 1 #2b — identity_passwordless (wave1-data-model rev. 6/7/7.1/11 §3, §4.1–§4.3, §8;
-- auth-and-session-policy rev. 2; HTTP contract 2.0.0). Forward change on #2, which is not rewritten
-- (sqlx checksums). Version is provisional: stamped after the latest merged migration at integration.
--
-- DESTRUCTIVE, authorized by the user for development only (2026-10-23): drops the password, TOTP,
-- recovery-code and password-reset objects and deletes every session. No production data exists.
--
-- Notes for DBA review:
--   N1 identity.app_user and identity.session have FORCE RLS, which binds allwin_owner too, so the
--      backfill UPDATE and the session DELETE would match no row. Both run with FORCE lifted for the
--      statement and restored right after, inside this migration's transaction.
--   N2 resolve_session drops mfa_state/authenticated_at and returns device_id and login_channel.
--   N3 allwin_maint gets DELETE on the code, device, change and counter tables for the §6.5 purge;
--      allwin_app loses DELETE on code_send_counter (SELECT, INSERT, UPDATE only, §4.1).
SET lock_timeout = '5s';

-- 1. Functions whose return types change, and the reset lookup that goes away.
DROP FUNCTION identity.resolve_password_reset(bytea);
DROP FUNCTION identity.find_user_for_login(text);
DROP FUNCTION identity.resolve_session(bytea);
DROP FUNCTION identity.admin_list_users();

-- 2. Password, TOTP and recovery objects.
DROP TABLE identity.recovery_code;
DROP TABLE identity.totp_credential;
DROP TABLE identity.password_reset;

-- 3. Users: channels instead of a password.
ALTER TABLE identity.app_user
  DROP COLUMN password_hash,
  ADD COLUMN email_verified_at    timestamptz,                -- signup e-mail code; set again by an e-mail change
  ADD COLUMN whatsapp_e164        text,                       -- verified number only (E.164)
  ADD COLUMN whatsapp_verified_at timestamptz,                -- added after signup by a channel change (S3b)
  ADD COLUMN channel_changed_at   timestamptz;                -- next change refused for 24 h (policy §7)

ALTER TABLE identity.app_user NO FORCE ROW LEVEL SECURITY;   -- N1
UPDATE identity.app_user SET email_verified_at = created_at;  -- dev seeds only
ALTER TABLE identity.app_user FORCE ROW LEVEL SECURITY;

ALTER TABLE identity.app_user
  ALTER COLUMN email_verified_at SET NOT NULL,
  ADD CONSTRAINT app_user_whatsapp_format_check CHECK (whatsapp_e164 ~ '^\+[1-9][0-9]{7,14}$'),
  ADD CONSTRAINT app_user_whatsapp_key UNIQUE (whatsapp_e164),               -- one number, one user (Q32)
  ADD CONSTRAINT app_user_whatsapp_check CHECK ((whatsapp_e164 IS NULL) = (whatsapp_verified_at IS NULL));

-- 4. Devices, send counters, signup codes.
CREATE TABLE identity.known_device (                     -- browsers the user has signed in from
  id              uuid PRIMARY KEY,
  user_id         uuid NOT NULL REFERENCES identity.app_user(id),
  device_key_hash bytea NOT NULL CHECK (octet_length(device_key_hash) = 32),  -- SHA-256 of a 256-bit device cookie
  label           text CHECK (char_length(label) <= 120),    -- from the user agent, e.g. "Chrome on Windows"
  first_seen_at   timestamptz NOT NULL DEFAULT now(),
  last_seen_at    timestamptz NOT NULL DEFAULT now(),
  last_ip         inet,
  revoked_at      timestamptz,                           -- "forget this device": the next sign-in from it is new again
  UNIQUE (user_id, id),
  UNIQUE (user_id, device_key_hash)
);
ALTER TABLE identity.known_device OWNER TO allwin_owner;
ALTER TABLE identity.known_device ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.known_device FORCE  ROW LEVEL SECURITY;
CREATE POLICY known_device_owner ON identity.known_device
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
CREATE POLICY known_device_definer_read ON identity.known_device FOR SELECT TO allwin_owner USING (true);

CREATE TABLE identity.code_send_counter (                -- durable send limits per identifier; no RLS, no user_id
  identifier_hmac bytea NOT NULL CHECK (octet_length(identifier_hmac) = 32),  -- HMAC-SHA-256(k_ident, normalized identifier)
  pepper_version  smallint NOT NULL,
  device_class    text NOT NULL CHECK (device_class IN ('known','unknown')),
  window_seconds  integer NOT NULL CHECK (window_seconds > 0),               -- 60, 900, 86400 (policy §4)
  window_start    timestamptz NOT NULL,                  -- aligned to window_seconds by the Core
  sends           integer NOT NULL CHECK (sends > 0),
  PRIMARY KEY (identifier_hmac, pepper_version, device_class, window_seconds, window_start)
);
ALTER TABLE identity.code_send_counter OWNER TO allwin_owner;
REVOKE DELETE ON identity.code_send_counter FROM allwin_app;  -- N3

CREATE TABLE identity.signup_code (                      -- signup challenges; no user yet. Admin-side data: no RLS
  id                 uuid PRIMARY KEY,                   -- challenge_id; never the code
  invitation_id      uuid NOT NULL REFERENCES identity.invitation(id),
  channel            text NOT NULL DEFAULT 'email' CHECK (channel = 'email'),  -- S3: e-mail only (AC-3)
  code_hmac          bytea NOT NULL CHECK (octet_length(code_hmac) = 32),  -- HMAC-SHA-256(k_code, challenge_id ‖ code)
  pepper_version     smallint NOT NULL,
  failed_attempts    smallint NOT NULL DEFAULT 0 CHECK (failed_attempts BETWEEN 0 AND 5),
  created_at         timestamptz NOT NULL DEFAULT now(),
  sent_at            timestamptz NOT NULL DEFAULT now(),
  expires_at         timestamptz NOT NULL,
  consumed_at        timestamptz,
  invalidated_at     timestamptz,
  invalidated_reason text CHECK (invalidated_reason IN ('attempts','invitation_closed')),
  request_ip         inet,
  delivery_state     text NOT NULL DEFAULT 'pending' CHECK (delivery_state IN ('pending','sent','failed')),
  CONSTRAINT signup_code_expiry_check
    CHECK (sent_at >= created_at AND expires_at > sent_at AND expires_at <= sent_at + interval '10 minutes'),
  CONSTRAINT signup_code_invalidated_check CHECK ((invalidated_at IS NULL) = (invalidated_reason IS NULL)),
  CONSTRAINT signup_code_attempts_check CHECK (failed_attempts < 5 OR invalidated_reason = 'attempts'),
  CONSTRAINT signup_code_final_check CHECK (consumed_at IS NULL OR invalidated_at IS NULL)
);
ALTER TABLE identity.signup_code OWNER TO allwin_owner;
CREATE INDEX signup_code_open_idx ON identity.signup_code (invitation_id, created_at)
  WHERE consumed_at IS NULL AND invalidated_at IS NULL;

-- 5. Sessions: void every password session, then bind sessions to a device and a login channel.
ALTER TABLE identity.session NO FORCE ROW LEVEL SECURITY;    -- N1
DELETE FROM identity.session;
ALTER TABLE identity.session FORCE ROW LEVEL SECURITY;

ALTER TABLE identity.session
  DROP COLUMN mfa_state,
  DROP COLUMN authenticated_at,
  ADD COLUMN device_id     uuid NOT NULL,
  ADD COLUMN login_channel text NOT NULL CHECK (login_channel IN ('email','whatsapp')),
  ADD CONSTRAINT session_user_id_key UNIQUE (user_id, id),
  ADD CONSTRAINT session_device_fkey FOREIGN KEY (user_id, device_id)
    REFERENCES identity.known_device (user_id, id),
  DROP CONSTRAINT session_end_reason_check,
  ADD CONSTRAINT session_end_reason_check
    CHECK (end_reason IN ('logout','expired','rotated','channel_changed','device_revoked','signed_out_everywhere'));
CREATE INDEX session_user_device_idx ON identity.session (user_id, device_id) WHERE ended_at IS NULL;  -- forget device

-- 6. Channel changes and one-time codes (both reference session (user_id, id)).
CREATE TABLE identity.channel_change (                   -- HTTP 2.0.0 ChannelChange; id = change_id
  id                      uuid PRIMARY KEY,
  user_id                 uuid NOT NULL REFERENCES identity.app_user(id),
  channel                 text NOT NULL CHECK (channel IN ('email','whatsapp')),
  new_value               text NOT NULL,                 -- normalized; shown only masked; applied when completed
  requested_session_id    uuid NOT NULL,                 -- the session that started it (codes are bound to it)
  status                  text NOT NULL DEFAULT 'awaiting_codes'
                          CHECK (status IN ('awaiting_codes','on_hold','completed','cancelled','expired')),
  created_at              timestamptz NOT NULL DEFAULT now(),
  expires_at              timestamptz NOT NULL,          -- every code confirmed within 10 min (policy §7)
  hold_until              timestamptz,                   -- on_hold: effective time (M2, policy §9)
  completed_at            timestamptz,
  cancelled_at            timestamptz,
  cancelled_by_session_id uuid,                          -- NULL = cancelled by the Core, generic reason
  UNIQUE (user_id, id),
  CONSTRAINT channel_change_whatsapp_check CHECK (channel <> 'whatsapp' OR new_value ~ '^\+[1-9][0-9]{7,14}$'),
  CONSTRAINT channel_change_expiry_check CHECK (expires_at > created_at AND expires_at <= created_at + interval '10 minutes'),
  CONSTRAINT channel_change_hold_check CHECK (status <> 'on_hold' OR hold_until IS NOT NULL),
  CONSTRAINT channel_change_completed_check CHECK ((status = 'completed') = (completed_at IS NOT NULL)),
  CONSTRAINT channel_change_cancelled_check CHECK ((status = 'cancelled') = (cancelled_at IS NOT NULL)),
  CONSTRAINT channel_change_cancelled_by_check CHECK (cancelled_by_session_id IS NULL OR cancelled_at IS NOT NULL),
  FOREIGN KEY (user_id, requested_session_id)    REFERENCES identity.session (user_id, id),
  FOREIGN KEY (user_id, cancelled_by_session_id) REFERENCES identity.session (user_id, id)
);
ALTER TABLE identity.channel_change OWNER TO allwin_owner;
CREATE UNIQUE INDEX channel_change_one_open ON identity.channel_change (user_id)
  WHERE status IN ('awaiting_codes','on_hold');
CREATE INDEX channel_change_hold_due_idx ON identity.channel_change (hold_until) WHERE status = 'on_hold';
ALTER TABLE identity.channel_change ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.channel_change FORCE  ROW LEVEL SECURITY;
CREATE POLICY channel_change_owner ON identity.channel_change
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
GRANT SELECT ON identity.channel_change TO allwin_worker;     -- finds due holds; the effect runs as allwin_app

CREATE TABLE identity.one_time_code (                    -- login, step-up and channel-change codes
  id                 uuid PRIMARY KEY,                   -- uuid inside the sealed challenge token; never the code
  user_id            uuid NOT NULL REFERENCES identity.app_user(id),
  channel_change_id  uuid,
  purpose            text NOT NULL CHECK (purpose IN ('login','step_up','channel_change')),
  channel            text NOT NULL CHECK (channel IN ('email','whatsapp')),
  destination        text NOT NULL,                      -- normalized address the code was sent to
  action             text CHECK (action IN ('source_create','source_rotate','connection_create','pairing_code_new',
                                        'connection_unpair','account_opt_in_accept','account_create_opt_in_accept',
                                        'admin_invitation_create','route_enable','account_binding_change')),
  target_id          uuid,
  session_id         uuid,                               -- step_up / channel_change: only this session may use it
  code_role          text CHECK (code_role IN ('current_value','new_value','other_channel')),
  code_hmac          bytea NOT NULL CHECK (octet_length(code_hmac) = 32),  -- HMAC-SHA-256(k_code, challenge_id ‖ code)
  pepper_version     smallint NOT NULL,
  failed_attempts    smallint NOT NULL DEFAULT 0 CHECK (failed_attempts BETWEEN 0 AND 5),
  created_at         timestamptz NOT NULL DEFAULT now(),
  sent_at            timestamptz NOT NULL DEFAULT now(),  -- last (re)send
  expires_at         timestamptz NOT NULL,
  consumed_at        timestamptz,
  invalidated_at     timestamptz,
  invalidated_reason text CHECK (invalidated_reason IN ('attempts','channel_changed','change_closed','user_closed')),
  request_ip         inet,
  delivery_state     text NOT NULL DEFAULT 'pending' CHECK (delivery_state IN ('pending','sent','failed')),
  CONSTRAINT one_time_code_expiry_check
    CHECK (sent_at >= created_at AND expires_at > sent_at AND expires_at <= sent_at + interval '10 minutes'),
  CONSTRAINT one_time_code_invalidated_check CHECK ((invalidated_at IS NULL) = (invalidated_reason IS NULL)),
  CONSTRAINT one_time_code_attempts_check CHECK (failed_attempts < 5 OR invalidated_reason = 'attempts'),
  CONSTRAINT one_time_code_final_check CHECK (consumed_at IS NULL OR invalidated_at IS NULL),
  CONSTRAINT one_time_code_step_up_check
    CHECK ((purpose = 'step_up') = (action IS NOT NULL)
           AND (target_id IS NOT NULL) = COALESCE(action IN ('source_rotate','pairing_code_new','connection_unpair',
                                                            'account_opt_in_accept','route_enable',
                                                            'account_binding_change'), false)),
  CONSTRAINT one_time_code_role_check
    CHECK ((purpose = 'channel_change') = (code_role IS NOT NULL) AND (code_role IS NULL) = (channel_change_id IS NULL)),
  CONSTRAINT one_time_code_session_check CHECK ((purpose IN ('step_up','channel_change')) = (session_id IS NOT NULL)),
  CONSTRAINT one_time_code_whatsapp_check CHECK (channel <> 'whatsapp' OR destination ~ '^\+[1-9][0-9]{7,14}$'),
  FOREIGN KEY (user_id, session_id) REFERENCES identity.session (user_id, id),
  FOREIGN KEY (user_id, channel_change_id) REFERENCES identity.channel_change (user_id, id)
);
ALTER TABLE identity.one_time_code OWNER TO allwin_owner;
CREATE INDEX one_time_code_user_created_idx ON identity.one_time_code (user_id, created_at);
CREATE UNIQUE INDEX one_time_code_change_role_key ON identity.one_time_code (channel_change_id, code_role)
  WHERE channel_change_id IS NOT NULL;
ALTER TABLE identity.one_time_code ENABLE ROW LEVEL SECURITY;
ALTER TABLE identity.one_time_code FORCE  ROW LEVEL SECURITY;
CREATE POLICY one_time_code_owner ON identity.one_time_code
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());
CREATE POLICY one_time_code_definer_read ON identity.one_time_code FOR SELECT TO allwin_owner USING (true);

-- 6b/1. Pre-authentication lookups (data model §3).

CREATE FUNCTION identity.find_user_for_login(p_email_normalized text)
  RETURNS TABLE (id uuid, status text, locale text, email text, whatsapp_e164 text)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT u.id, u.status, u.locale, u.email, u.whatsapp_e164
      FROM identity.app_user u
     WHERE u.email_normalized = p_email_normalized
  $$;

CREATE FUNCTION identity.resolve_session(p_token_hash bytea)
  RETURNS TABLE (id uuid, user_id uuid, device_id uuid, is_persistent boolean, login_channel text,
                 login_at timestamptz, last_activity_at timestamptz, idle_expires_at timestamptz,
                 absolute_expires_at timestamptz, ended_at timestamptz, end_reason text)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT s.id, s.user_id, s.device_id, s.is_persistent, s.login_channel, s.login_at,
           s.last_activity_at, s.idle_expires_at, s.absolute_expires_at, s.ended_at, s.end_reason
      FROM identity.session s
     WHERE s.token_hash = p_token_hash
  $$;

-- listUsers: only for an active platform admin, who must be the transaction's app.user_id.
CREATE FUNCTION identity.admin_list_users()
  RETURNS TABLE (id uuid, email text, status text, is_platform_admin boolean, created_at timestamptz,
                 whatsapp_verified boolean, last_sign_in_at timestamptz)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT u.id, u.email, u.status, u.is_platform_admin, u.created_at,
           u.whatsapp_verified_at IS NOT NULL,
           (SELECT max(s.login_at) FROM identity.session s WHERE s.user_id = u.id)
      FROM identity.app_user u
     WHERE EXISTS (SELECT 1 FROM identity.app_user a
                    WHERE a.id = app.current_user_id() AND a.is_platform_admin AND a.status = 'active')
     ORDER BY u.created_at
  $$;

-- Shared browser (C4): a non-revoked device of ANY user holds this cookie hash.
CREATE FUNCTION identity.device_key_in_use(p_device_key_hash bytea)
  RETURNS boolean
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT EXISTS (SELECT 1 FROM identity.known_device d
                    WHERE d.device_key_hash = p_device_key_hash AND d.revoked_at IS NULL)
  $$;

-- Verification: the owner and purpose only; the row is then read and locked under RLS (§4.3).
CREATE FUNCTION identity.resolve_one_time_code(p_id uuid)
  RETURNS TABLE (user_id uuid, purpose text)
  LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
    SELECT c.user_id, c.purpose FROM identity.one_time_code c WHERE c.id = p_id
  $$;

ALTER FUNCTION identity.find_user_for_login(text)     OWNER TO allwin_owner;
ALTER FUNCTION identity.resolve_session(bytea)         OWNER TO allwin_owner;
ALTER FUNCTION identity.admin_list_users()             OWNER TO allwin_owner;
ALTER FUNCTION identity.device_key_in_use(bytea)       OWNER TO allwin_owner;
ALTER FUNCTION identity.resolve_one_time_code(uuid)    OWNER TO allwin_owner;

GRANT EXECUTE ON FUNCTION identity.find_user_for_login(text)  TO allwin_app;
GRANT EXECUTE ON FUNCTION identity.resolve_session(bytea)      TO allwin_app;
GRANT EXECUTE ON FUNCTION identity.admin_list_users()          TO allwin_app;
GRANT EXECUTE ON FUNCTION identity.device_key_in_use(bytea)    TO allwin_app;
GRANT EXECUTE ON FUNCTION identity.resolve_one_time_code(uuid) TO allwin_app;

-- 7. Purge (§6.5, nightly allwin_maint).  N3
GRANT SELECT, DELETE ON identity.one_time_code, identity.signup_code, identity.channel_change,
                        identity.known_device, identity.code_send_counter TO allwin_maint;
