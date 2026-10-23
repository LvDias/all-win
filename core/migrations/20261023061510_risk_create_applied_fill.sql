-- Wave 1 #10b — the fills the risk module has applied to its trading-day sums (wave1-data-model
-- §5.6, rev. 16.6). The primary key makes "never count a fill twice" a database fact: a redelivered
-- execution report is a no-op, an amend applies the difference against this row, and a remove
-- zeroes it and keeps it as the dedupe record.
SET lock_timeout = '5s';

CREATE TABLE risk.applied_fill (
  user_id              uuid NOT NULL,
  account_id           uuid NOT NULL,
  fill_id              text NOT NULL CHECK (char_length(fill_id) BETWEEN 1 AND 128),   -- platform execution id
  trading_date         date NOT NULL,                    -- fixed at the first application; never moves
  qty                  integer NOT NULL CHECK (qty >= 0),          -- as applied now (0 after a remove)
  gross                NUMERIC(18,4) NOT NULL,                      -- realized gross P&L this fill contributed
  commission_effective NUMERIC(18,4) NOT NULL CHECK (commission_effective >= 0),
  commission_gap       NUMERIC(18,4) NOT NULL CHECK (commission_gap >= 0),
  last_event_id        uuid NOT NULL,                    -- gateway execution_report.event_id applied last (trace)
  applied_at           timestamptz NOT NULL DEFAULT now(),
  removed_at           timestamptz,                      -- fill.operation = remove; the row stays as the dedupe record
  PRIMARY KEY (account_id, fill_id),                     -- fill_id is unique per account, not globally (ADR-0004 §4)
  CONSTRAINT applied_fill_removed_check CHECK (removed_at IS NULL
             OR (qty = 0 AND gross = 0 AND commission_effective = 0 AND commission_gap = 0)),
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id),
  FOREIGN KEY (account_id, trading_date) REFERENCES risk.trading_day (account_id, trading_date)
);
CREATE INDEX applied_fill_trading_date_idx ON risk.applied_fill (trading_date);   -- retention purge

ALTER TABLE risk.applied_fill OWNER TO allwin_owner;

ALTER TABLE risk.applied_fill ENABLE ROW LEVEL SECURITY;
ALTER TABLE risk.applied_fill FORCE  ROW LEVEL SECURITY;
CREATE POLICY applied_fill_owner ON risk.applied_fill
  USING (user_id = app.current_user_id()) WITH CHECK (user_id = app.current_user_id());

-- No runtime path deletes a fill row. The purge (30 days after its trading_date) and Q13-b erasure
-- run as allwin_maint.
REVOKE DELETE ON risk.applied_fill FROM allwin_app;
GRANT SELECT, DELETE ON risk.applied_fill TO allwin_maint;
