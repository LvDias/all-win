-- Wave 1 #10d — the inputs of each applied fill's gross (wave1-data-model §5.6, rev. 16.23), so an
-- amend recomputes it from what was applied, without today's catalog or position. NOT NULL without
-- a default: risk.applied_fill only has rows in throwaway S13 test databases (not in any shared
-- environment); such a database fails this file inside its transaction and is recreated.
SET lock_timeout = '5s';

ALTER TABLE risk.applied_fill
  ADD COLUMN instrument_symbol text NOT NULL CHECK (instrument_symbol ~ '^[A-Z0-9]{1,10}$'),
  ADD COLUMN contract_month    date NOT NULL CHECK (extract(day FROM contract_month) = 1),
  ADD COLUMN side              text NOT NULL CHECK (side IN ('buy','sell')),
  ADD COLUMN price             NUMERIC(22,10) NOT NULL CHECK (price > 0),   -- fill price as applied now
  ADD COLUMN position_before   integer,                  -- signed position before this fill; NULL = unknown (M1)
  ADD COLUMN avg_price_before  NUMERIC(22,10) CHECK (avg_price_before > 0),   -- NULL: flat before, or basis unknown
  ADD COLUMN point_value       NUMERIC(18,4) NOT NULL CHECK (point_value > 0),   -- snapshot at the first application
  ADD CONSTRAINT applied_fill_before_check   -- a realized gross needs a known open position and its basis
      CHECK (gross = 0 OR (position_before IS NOT NULL AND position_before <> 0 AND avg_price_before IS NOT NULL));
