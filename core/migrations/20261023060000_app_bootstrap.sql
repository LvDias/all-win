-- Wave 1 #1 — bootstrap (wave1-data-model §1, §3, §6.4).
-- Every migration MUST run connected as allwin_owner (never a superuser): the default
-- privileges below apply only to objects created by allwin_owner, so tables created by
-- another role would miss the allwin_app grants. The roles allwin_owner / allwin_app /
-- allwin_worker / allwin_maint are created outside migrations (DevOps; test harness), with
-- per-environment secrets; migrations only grant.
SET lock_timeout = '5s';

-- One schema per Core module, plus `app` for shared infrastructure.
CREATE SCHEMA app      AUTHORIZATION allwin_owner;
CREATE SCHEMA identity AUTHORIZATION allwin_owner;
CREATE SCHEMA catalog  AUTHORIZATION allwin_owner;
CREATE SCHEMA accounts AUTHORIZATION allwin_owner;
CREATE SCHEMA routing  AUTHORIZATION allwin_owner;
CREATE SCHEMA gateway  AUTHORIZATION allwin_owner;
CREATE SCHEMA audit    AUTHORIZATION allwin_owner;
CREATE SCHEMA risk     AUTHORIZATION allwin_owner;

REVOKE ALL ON SCHEMA public FROM PUBLIC;

GRANT USAGE ON SCHEMA app, identity, catalog, accounts, routing, gateway, audit, risk
  TO allwin_app, allwin_worker, allwin_maint;

-- Functions are not executable by PUBLIC unless granted explicitly (resolve_* are SECURITY DEFINER).
ALTER DEFAULT PRIVILEGES FOR ROLE allwin_owner REVOKE EXECUTE ON FUNCTIONS FROM PUBLIC;

-- Runtime DML on module tables; RLS (enabled per table) is the second barrier.
-- Narrower rights (audit append-only, worker queues) are set by the migration that creates the table.
ALTER DEFAULT PRIVILEGES FOR ROLE allwin_owner IN SCHEMA identity, accounts, routing, gateway, risk
  GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO allwin_app;
ALTER DEFAULT PRIVILEGES FOR ROLE allwin_owner IN SCHEMA catalog
  GRANT SELECT ON TABLES TO allwin_app, allwin_worker;

-- Per-transaction user context. NULLIF: once a session has set the GUC, it reverts to ''
-- (not NULL) after the transaction; a missing context makes every policy false.
CREATE FUNCTION app.current_user_id() RETURNS uuid
  LANGUAGE sql STABLE PARALLEL SAFE
  AS $$ SELECT NULLIF(current_setting('app.user_id', true), '')::uuid $$;
ALTER FUNCTION app.current_user_id() OWNER TO allwin_owner;
GRANT EXECUTE ON FUNCTION app.current_user_id() TO allwin_app, allwin_worker, allwin_maint;

-- Domain events written in the producer's transaction (ALIGN-2); delivered at least once
-- by the dispatcher (allwin_worker), consumers dedupe on the event id.
CREATE TABLE app.outbox (
  id           uuid PRIMARY KEY,
  user_id      uuid NOT NULL,
  event_type   text NOT NULL CHECK (event_type ~ '^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$'),
  occurred_at  timestamptz NOT NULL,
  payload      jsonb NOT NULL,
  published_at timestamptz,
  CONSTRAINT outbox_no_token CHECK (payload::text !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}')
);
ALTER TABLE app.outbox OWNER TO allwin_owner;
CREATE INDEX outbox_unpublished_idx ON app.outbox (occurred_at) WHERE published_at IS NULL;

ALTER TABLE app.outbox ENABLE ROW LEVEL SECURITY;
ALTER TABLE app.outbox FORCE  ROW LEVEL SECURITY;
CREATE POLICY outbox_owner ON app.outbox
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

-- Producers append in their user-scoped transaction; the dispatcher (BYPASSRLS) finds and
-- marks events; maintenance purges published rows after 7 days.
GRANT INSERT, SELECT ON app.outbox TO allwin_app;
GRANT SELECT, UPDATE (published_at) ON app.outbox TO allwin_worker;
GRANT SELECT, DELETE ON app.outbox TO allwin_maint;
