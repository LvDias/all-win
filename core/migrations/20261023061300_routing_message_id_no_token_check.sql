-- Wave 1 #6c — storage barrier: no webhook token in a routing message_id (Crivo, data model rev. 9;
-- AUTHORIZED by the user 2026-10-24, CHECK only, no data deleted). The format CHECK
-- `^[A-Za-z0-9._:-]{1,128}$` admits a whole `awk_<12>_<43>` token; the S1 credential guard refuses
-- it at ingress, and this keeps a future bug from storing a secret in a key that is never purged
-- (message_receipt is kept forever). Same pattern as outbox_no_token and instruction_no_token.
-- NOT VALID + VALIDATE in one file: sqlx runs it in one transaction, so the ADD CONSTRAINT lock is
-- held through the scan (fine for these small tables); a violating row fails the whole migration.
SET lock_timeout = '5s';

ALTER TABLE routing.message_receipt
  ADD CONSTRAINT message_receipt_message_id_no_token
  CHECK (message_id !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}') NOT VALID;
ALTER TABLE routing.message_receipt VALIDATE CONSTRAINT message_receipt_message_id_no_token;

ALTER TABLE routing.instruction
  ADD CONSTRAINT instruction_message_id_no_token
  CHECK (message_id !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}') NOT VALID;
ALTER TABLE routing.instruction VALIDATE CONSTRAINT instruction_message_id_no_token;
