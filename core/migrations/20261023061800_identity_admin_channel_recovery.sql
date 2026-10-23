-- Wave 1 #2e — identity_admin_channel_recovery (wave1-data-model rev. 16.8 §4; auth-and-session-policy §9;
-- HTTP contract 5.4.0; S3 AC-12). Expand-only: existing rows keep their values (kind defaults to
-- user_change and every one has a session), so no data changes.
--
-- An admin recovery is a channel_change of kind admin_recovery with no requesting session. Two codes
-- have no session either: the new-value code of the recovery, and the admin's own step-up
-- (action admin_recovery) asked by the operator command. A CHECK cannot read channel_change.kind, so
-- the Core allows a session-less new-value code only for an admin_recovery change.
SET lock_timeout = '5s';

ALTER TABLE identity.channel_change
  ADD COLUMN kind text NOT NULL DEFAULT 'user_change' CHECK (kind IN ('user_change','admin_recovery')),
  ALTER COLUMN requested_session_id DROP NOT NULL,
  ADD COLUMN admin_user_id    uuid,                      -- the admin who acted (no FK: erasure stays decoupled; audit has the actor)
  ADD COLUMN verified_by      text CHECK (verified_by IN ('video_call','phone_known_number','in_person','other')),
  ADD COLUMN verification_ref text CHECK (char_length(verification_ref) <= 200),  -- never the new value or a document number (Core)
  ADD COLUMN break_glass      boolean NOT NULL DEFAULT false,
  ADD CONSTRAINT channel_change_session_kind_check CHECK ((kind = 'user_change') = (requested_session_id IS NOT NULL)),
  ADD CONSTRAINT channel_change_kind_fields CHECK (
        (kind = 'admin_recovery' AND admin_user_id IS NOT NULL AND verified_by IS NOT NULL AND verification_ref IS NOT NULL)
     OR (kind = 'user_change' AND admin_user_id IS NULL AND verified_by IS NULL AND verification_ref IS NULL AND NOT break_glass)),
  -- I3: an admin recovers themself only through break-glass, and break-glass is only that.
  ADD CONSTRAINT channel_change_break_glass_check CHECK (kind <> 'admin_recovery' OR break_glass = (admin_user_id = user_id)),
  ADD CONSTRAINT channel_change_verification_ref_no_token CHECK (verification_ref !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}');

ALTER TABLE identity.one_time_code
  DROP CONSTRAINT one_time_code_action_check,
  ADD CONSTRAINT one_time_code_action_check
    CHECK (action IN ('source_create','source_rotate','connection_create','pairing_code_new',
                      'connection_unpair','account_opt_in_accept','account_create_opt_in_accept',
                      'admin_invitation_create','route_enable','account_binding_change','admin_recovery')),
  DROP CONSTRAINT one_time_code_step_up_check,
  ADD CONSTRAINT one_time_code_step_up_check
    CHECK ((purpose = 'step_up') = (action IS NOT NULL)
           AND (target_id IS NOT NULL) = COALESCE(action IN ('source_rotate','pairing_code_new','connection_unpair',
                                                            'account_opt_in_accept','route_enable',
                                                            'account_binding_change','admin_recovery'), false)),
  DROP CONSTRAINT one_time_code_session_check,
  ADD CONSTRAINT one_time_code_session_check
    CHECK (CASE purpose
             WHEN 'login'   THEN session_id IS NULL
             WHEN 'step_up' THEN (session_id IS NULL) = (action = 'admin_recovery')
             ELSE session_id IS NOT NULL OR code_role = 'new_value'
           END);
