-- Wave 1 #7 — user-visible audit log (wave1-data-model §6.4). Immutable, partitioned monthly.
SET lock_timeout = '5s';

CREATE TABLE audit.audit_entry (
  id              uuid NOT NULL,
  user_id         uuid NOT NULL,
  occurred_at     timestamptz NOT NULL,                  -- from the event (deterministic on redelivery)
  recorded_at     timestamptz NOT NULL DEFAULT now(),
  source_event_id uuid NOT NULL,                         -- app.outbox id → dedupe
  category        text NOT NULL CHECK (category IN ('ingress','routing','execution','risk','connection','security','config')),
  code            text NOT NULL CHECK (code ~ '^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$'),   -- stable code; clients translate
  instruction_id  uuid,
  account_id      uuid,
  account_name    text,                                  -- snapshot
  source_id       uuid,
  source_key_id   text CHECK (source_key_id ~ '^[A-Za-z0-9]{12}$'),   -- key_id only
  command_id      uuid,
  signal_type     text,                                  -- denormalized for S10 filters
  details         jsonb NOT NULL DEFAULT '{}'::jsonb,
  PRIMARY KEY (occurred_at, id),
  UNIQUE (occurred_at, source_event_id),
  CONSTRAINT audit_entry_no_token CHECK (details::text !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}')
) PARTITION BY RANGE (occurred_at);

CREATE INDEX audit_entry_user_time_idx        ON audit.audit_entry (user_id, occurred_at DESC);
CREATE INDEX audit_entry_user_instruction_idx ON audit.audit_entry (user_id, instruction_id) WHERE instruction_id IS NOT NULL;
CREATE INDEX audit_entry_user_account_idx     ON audit.audit_entry (user_id, account_id, occurred_at DESC) WHERE account_id IS NOT NULL;

-- Partition maintenance runs as allwin_maint through these SECURITY DEFINER functions owned
-- by allwin_owner, so maint never owns or alters the table itself. Partitions are monthly in
-- UTC: audit.audit_entry_YYYYMM. The default partition must stay empty (maintenance alerts).

-- Idempotent: creates the partitions from the current UTC month up to p_months_ahead ahead.
CREATE FUNCTION audit.ensure_partitions(p_months_ahead int) RETURNS void
  LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
DECLARE
  m date := date_trunc('month', now() AT TIME ZONE 'UTC')::date;
  first_day date;
BEGIN
  IF p_months_ahead < 0 OR p_months_ahead > 24 THEN
    RAISE EXCEPTION 'p_months_ahead must be between 0 and 24';
  END IF;
  FOR i IN 0..p_months_ahead LOOP
    first_day := (m + make_interval(months => i))::date;
    EXECUTE format(
      'CREATE TABLE IF NOT EXISTS audit.%I PARTITION OF audit.audit_entry FOR VALUES FROM (%L) TO (%L)',
      'audit_entry_' || to_char(first_day, 'YYYYMM'),
      first_day::timestamp AT TIME ZONE 'UTC',
      (first_day + interval '1 month')::timestamp AT TIME ZONE 'UTC');
  END LOOP;
END $$;

-- Retention (Q13 b): detaches and drops whole monthly partitions that end on or before
-- p_month (first day of a month). Names are built here; no table name is ever taken as input.
CREATE FUNCTION audit.drop_partitions_before(p_month date) RETURNS int
  LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp
  AS $$
DECLARE
  part record;
  dropped int := 0;
BEGIN
  IF p_month <> date_trunc('month', p_month)::date THEN
    RAISE EXCEPTION 'p_month must be the first day of a month';
  END IF;
  -- Never the current or the previous month, whatever the caller computes; the retention
  -- window itself (24 months) is the Core's.
  IF p_month > (date_trunc('month', now() AT TIME ZONE 'UTC') - interval '1 month')::date THEN
    RAISE EXCEPTION 'p_month must be at least one month before the current month';
  END IF;
  FOR part IN
    SELECT c.relname
      FROM pg_inherits i
      JOIN pg_class c ON c.oid = i.inhrelid
      JOIN pg_namespace n ON n.oid = c.relnamespace
     WHERE i.inhparent = 'audit.audit_entry'::regclass
       AND n.nspname = 'audit'
       AND c.relname ~ '^audit_entry_[0-9]{6}$'
       AND to_date(substring(c.relname FROM 13 FOR 6), 'YYYYMM') < p_month
  LOOP
    EXECUTE format('ALTER TABLE audit.audit_entry DETACH PARTITION audit.%I', part.relname);
    EXECUTE format('DROP TABLE audit.%I', part.relname);
    dropped := dropped + 1;
  END LOOP;
  RETURN dropped;
END $$;

ALTER FUNCTION audit.ensure_partitions(int) OWNER TO allwin_owner;
ALTER FUNCTION audit.drop_partitions_before(date) OWNER TO allwin_owner;
GRANT EXECUTE ON FUNCTION audit.ensure_partitions(int), audit.drop_partitions_before(date) TO allwin_maint;

SELECT audit.ensure_partitions(3);
CREATE TABLE audit.audit_entry_default PARTITION OF audit.audit_entry DEFAULT;

ALTER TABLE audit.audit_entry ENABLE ROW LEVEL SECURITY;
ALTER TABLE audit.audit_entry FORCE  ROW LEVEL SECURITY;
CREATE POLICY audit_entry_owner ON audit.audit_entry
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

-- Immutable (S10 AC-3): append and read only, and a trigger as a second barrier. The only
-- exception is erasure of a closed account (Q13 b), which runs as allwin_maint; the role is
-- checked with current_user, never with a settable GUC.
CREATE FUNCTION audit.reject_audit_change() RETURNS trigger
  LANGUAGE plpgsql
  AS $$
BEGIN
  IF TG_OP = 'DELETE' AND current_user = 'allwin_maint' THEN
    RETURN OLD;
  END IF;
  RAISE EXCEPTION 'audit.audit_entry is immutable' USING ERRCODE = 'insufficient_privilege';
END $$;
CREATE TRIGGER audit_entry_immutable
  BEFORE UPDATE OR DELETE ON audit.audit_entry
  FOR EACH ROW EXECUTE FUNCTION audit.reject_audit_change();

GRANT SELECT, INSERT ON audit.audit_entry TO allwin_app;
-- Retention drops whole partitions (drop_partitions_before); row deletes only erase closed accounts.
GRANT SELECT, DELETE ON audit.audit_entry TO allwin_maint;
