-- Wave 1 — app.outbox.seq (wiring for audit event_seq; wave1-data-model §6.4 rev. 10).
-- Drawn when the producer inserts the event: causal across transactions, insert order within
-- one. CACHE 1 keeps a single order across sessions. Adding the identity column rewrites the
-- table under ACCESS EXCLUSIVE and fills existing rows (dev/test only; no production rows).
SET lock_timeout = '5s';

-- UNIQUE guards the order that audit event_seq / listActivity rely on, and is the index the
-- outbox tracking uses (start_seq = max(seq) + 1, seq-batched purges).
ALTER TABLE app.outbox
  ADD COLUMN seq bigint GENERATED ALWAYS AS IDENTITY (CACHE 1),
  ADD CONSTRAINT outbox_seq_key UNIQUE (seq);
