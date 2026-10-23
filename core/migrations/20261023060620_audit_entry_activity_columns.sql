-- Wave 1 #7b — S10 activity columns on the partitioned audit.audit_entry
-- (wave1-data-model §6.4 rev. 9/10/12). Every statement runs on the parent and cascades to the
-- partitions. Existing rows cannot be backfilled (the immutability trigger refuses UPDATE and
-- is never disabled): they read outcome = 'info', safe_to_display = false, event_seq = 0.
SET lock_timeout = '5s';

-- 1. Required columns: metadata defaults, then dropped so every new insert sets them
--    (safe_to_display fail-closed: a producer that forgets it fails on insert).
ALTER TABLE audit.audit_entry
  ADD COLUMN outcome text NOT NULL DEFAULT 'info',
  ADD COLUMN safe_to_display boolean NOT NULL DEFAULT false,
  ADD COLUMN event_seq bigint NOT NULL DEFAULT 0;            -- app.outbox.seq of the source event
ALTER TABLE audit.audit_entry
  ALTER COLUMN outcome DROP DEFAULT,
  ALTER COLUMN safe_to_display DROP DEFAULT,
  ALTER COLUMN event_seq DROP DEFAULT;

-- 2. Nullable columns.
ALTER TABLE audit.audit_entry
  ADD COLUMN is_exit boolean,
  ADD COLUMN message_id text,
  ADD COLUMN source_name text,                               -- snapshot, like account_name
  ADD COLUMN actor text;                                     -- who caused it; NULL only before #7b

-- 3. Column checks, validated: only new values can fail them.
ALTER TABLE audit.audit_entry
  ADD CONSTRAINT audit_entry_outcome_check CHECK (outcome IN (
    'received','routed','executed','partial','skipped','refused','rejected','expired','unknown','info')),
  ADD CONSTRAINT audit_entry_is_exit_check CHECK (is_exit IS NULL OR category IN ('routing','execution')),
  ADD CONSTRAINT audit_entry_message_id_check CHECK (message_id ~ '^[A-Za-z0-9._:-]{1,128}$'),
  ADD CONSTRAINT audit_entry_source_name_length_check CHECK (char_length(source_name) BETWEEN 1 AND 60),
  ADD CONSTRAINT audit_entry_actor_value_check CHECK (actor IN ('user','system','platform_admin'));

-- Snapshot and actor rules: NOT VALID and never validated. They bind every new row; rows
-- written before #7b (source_id without source_name, NULL actor) stay exempt.
ALTER TABLE audit.audit_entry
  ADD CONSTRAINT audit_entry_source_name_check CHECK (source_id IS NULL OR source_name IS NOT NULL) NOT VALID,
  ADD CONSTRAINT audit_entry_account_name_check CHECK (account_id IS NULL OR account_name IS NOT NULL) NOT VALID,
  ADD CONSTRAINT audit_entry_actor_check CHECK (actor IS NOT NULL) NOT VALID;

-- No token in details nor in the free-text snapshots or message_id (its pattern allows '_').
ALTER TABLE audit.audit_entry DROP CONSTRAINT audit_entry_no_token;
ALTER TABLE audit.audit_entry
  ADD CONSTRAINT audit_entry_no_token CHECK (
        details::text            !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}'
    AND coalesce(account_name,'') !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}'
    AND coalesce(source_name,'')  !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}'
    AND coalesce(message_id,'')   !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}');

-- The listActivity keyset order (user, occurred_at, event_seq, id), newest first.
DROP INDEX audit.audit_entry_user_time_idx;
CREATE INDEX audit_entry_user_time_idx ON audit.audit_entry (user_id, occurred_at DESC, event_seq DESC, id DESC);
