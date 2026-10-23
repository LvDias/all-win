-- Wave 1 #11c — the accounts job's cursor over published firm releases (S16, Esquadro (B)).
-- A system table: one row, no user_id and no RLS. Only the worker job reads and advances it, by
-- compare-and-set (UPDATE … SET last_publish_seq = $new WHERE last_publish_seq = $old), never
-- backwards; it advances only after every user of a release is processed.
SET lock_timeout = '5s';

CREATE TABLE accounts.catalog_release_cursor (
  singleton        boolean PRIMARY KEY DEFAULT true CHECK (singleton),
  last_publish_seq bigint NOT NULL DEFAULT 0 CHECK (last_publish_seq >= 0),  -- catalog.firm_release.publish_seq
  updated_at       timestamptz NOT NULL DEFAULT now()
);
ALTER TABLE accounts.catalog_release_cursor OWNER TO allwin_owner;
INSERT INTO accounts.catalog_release_cursor DEFAULT VALUES;

-- The accounts default privileges would give allwin_app full DML: the app never touches it.
REVOKE ALL ON accounts.catalog_release_cursor FROM allwin_app;
GRANT SELECT, UPDATE (last_publish_seq, updated_at) ON accounts.catalog_release_cursor TO allwin_worker;
