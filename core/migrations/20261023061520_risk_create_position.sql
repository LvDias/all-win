-- Wave 1 #10c — each account's position per contract and its cost basis (wave1-data-model §5.6,
-- rev. 16.23). Risk realizes each fill's gross P&L on AVERAGE COST against it; the platform's
-- `avg_price_after` wins whenever a report carries it. `basis_known = false` is an open position
-- whose basis is unknown: its reducing fills realize 0 and the account's P&L is stale (fail closed).
SET lock_timeout = '5s';

CREATE TABLE risk.position (
  user_id           uuid NOT NULL,
  account_id        uuid NOT NULL,
  instrument_symbol text NOT NULL CHECK (instrument_symbol ~ '^[A-Z0-9]{1,10}$'),   -- wire form, as gateway G3
  contract_month    date NOT NULL CHECK (extract(day FROM contract_month) = 1),
  qty               integer NOT NULL,                  -- signed; 0 = flat (the row stays)
  avg_price         NUMERIC(22,10) CHECK (avg_price > 0),   -- NULL when flat or when the basis is unknown
  basis_known       boolean NOT NULL DEFAULT true,     -- false: open position whose cost basis is unknown
  last_fill_id      text CHECK (char_length(last_fill_id) BETWEEN 1 AND 128),
  last_event_id     uuid,                              -- gateway execution_report.event_id applied last (trace)
  updated_at        timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (account_id, instrument_symbol, contract_month),
  CONSTRAINT position_flat_check  CHECK (qty <> 0 OR (avg_price IS NULL AND basis_known)),
  CONSTRAINT position_basis_check CHECK (qty = 0 OR (avg_price IS NOT NULL) = basis_known),
  FOREIGN KEY (user_id, account_id) REFERENCES risk.account_limits (user_id, account_id)
);
ALTER TABLE risk.position OWNER TO allwin_owner;

ALTER TABLE risk.position ENABLE ROW LEVEL SECURITY;
ALTER TABLE risk.position FORCE  ROW LEVEL SECURITY;
CREATE POLICY position_owner ON risk.position
  USING (user_id = app.current_user_id()) WITH CHECK (user_id = app.current_user_id());

-- allwin_app reads and writes it (SELECT, INSERT, UPDATE from the bootstrap defaults); no runtime
-- path deletes a position row (a flat position keeps its row). Q13-b erasure runs as allwin_maint.
-- The worker gets nothing: positions are updated per user on the allwin_app pool.
REVOKE DELETE ON risk.position FROM allwin_app;
GRANT SELECT, DELETE ON risk.position TO allwin_maint;
