-- Wave 1 #6d — duplicate comparison (HTTP 5.4.0 `ingress.duplicate_conflict`; data model 16.8.1,
-- Esquadro's final rule). Expand-only.
--
-- `body_sha256` is the sha256 of the canonical JSON of the instruction's compared fields (closed
-- list in contracts/instruction/v1/README.md). It lives only as long as the content it
-- summarizes: the Q13 b purge deletes routing.instruction and, in the same batch, sets
-- body_sha256 = NULL and content_purged_at = now(), so no fingerprint of the content outlives
-- it. The receipt itself (the dedupe key) is kept forever.
SET lock_timeout = '5s';

ALTER TABLE routing.message_receipt
  ADD COLUMN body_sha256       bytea CHECK (octet_length(body_sha256) = 32),
  ADD COLUMN content_purged_at timestamptz;

-- Enforced on every new or updated row; NOT VALID because older DEV rows have no hash, and it
-- is never validated.
ALTER TABLE routing.message_receipt
  ADD CONSTRAINT message_receipt_body_hash_check
  CHECK (body_sha256 IS NOT NULL OR content_purged_at IS NOT NULL) NOT VALID;

-- The retention purge runs as allwin_maint and clears only these two columns.
GRANT UPDATE (body_sha256, content_purged_at) ON routing.message_receipt TO allwin_maint;

-- Every path that deletes routing.instruction (the retention job, or any other) clears the
-- matching receipts' hash in the same statement, so no receipt keeps a fingerprint of content
-- that is gone (Esmeril nit (a)). Statement level with a transition table: one UPDATE join per
-- batch. Invoker rights: only allwin_maint deletes instructions (BYPASSRLS), with the column
-- grant above; on erasure, receipts already deleted make this a no-op.
CREATE FUNCTION routing.instruction_purge_receipt_hash() RETURNS trigger
  LANGUAGE plpgsql SET search_path = pg_catalog
  AS $$
BEGIN
  UPDATE routing.message_receipt r
     SET body_sha256 = NULL, content_purged_at = now()
    FROM old_rows o
   WHERE r.user_id = o.user_id AND r.message_id = o.message_id AND r.body_sha256 IS NOT NULL;
  RETURN NULL;
END $$;
CREATE TRIGGER instruction_purge_receipt_hash
  AFTER DELETE ON routing.instruction
  REFERENCING OLD TABLE AS old_rows
  FOR EACH STATEMENT EXECUTE FUNCTION routing.instruction_purge_receipt_hash();
