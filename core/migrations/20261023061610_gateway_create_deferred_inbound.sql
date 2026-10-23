-- Wave 1 #9b — gateway: deferred inbound frames (wave1-data-model rev. 16.10 §6.3, §6.5; Esquadro
-- decision). Reliable A→C frames whose Core consumer belongs to a later slice (risk_event,
-- instruction, trade_completed, notification_read) are schema-validated, stored and acked in one
-- transaction (ack = durably received). The owning slice consumes them with backfill, in `seq`
-- order, and sets `processed_at` (one consumer per type). A stored `instruction` is never routed
-- late: its consumer expires it by TTL. Builds on #9.
SET lock_timeout = '5s';

CREATE TABLE gateway.deferred_inbound (
  connection_id    uuid NOT NULL,
  msg_id           uuid NOT NULL,
  seq              bigint GENERATED ALWAYS AS IDENTITY (CACHE 1) UNIQUE,   -- arrival order (intake is serialized per connection)
  user_id          uuid NOT NULL,
  account_id       uuid,                                 -- from the payload when the type has one (cross-module id)
  type             text NOT NULL CHECK (type IN ('risk_event','instruction','trade_completed','notification_read')),
  protocol_version text NOT NULL CHECK (protocol_version ~ '^1\.(0|[1-9][0-9]*)$'),
  payload          jsonb NOT NULL,                       -- the validated payload
  received_at      timestamptz NOT NULL,
  processed_at     timestamptz,
  PRIMARY KEY (connection_id, msg_id),                   -- same key as inbound_receipt; outlives its 48-h purge
  CONSTRAINT deferred_inbound_no_token CHECK (payload::text !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}')
);
ALTER TABLE gateway.deferred_inbound OWNER TO allwin_owner;
CREATE INDEX deferred_inbound_pending_idx ON gateway.deferred_inbound (type, seq) WHERE processed_at IS NULL;
ALTER TABLE gateway.deferred_inbound ENABLE ROW LEVEL SECURITY;
ALTER TABLE gateway.deferred_inbound FORCE  ROW LEVEL SECURITY;
CREATE POLICY deferred_inbound_owner ON gateway.deferred_inbound
  USING (user_id = app.current_user_id())
  WITH CHECK (user_id = app.current_user_id());

-- allwin_app: INSERT, SELECT and UPDATE (processed_at) only (the consumer slice marks rows in its
-- user-scoped transaction); allwin_worker finds pending rows; allwin_maint purges processed rows
-- 7 days after processed_at (§6.5).
REVOKE UPDATE, DELETE ON gateway.deferred_inbound FROM allwin_app;
GRANT UPDATE (processed_at) ON gateway.deferred_inbound TO allwin_app;
GRANT SELECT ON gateway.deferred_inbound TO allwin_worker;
GRANT SELECT, DELETE ON gateway.deferred_inbound TO allwin_maint;
