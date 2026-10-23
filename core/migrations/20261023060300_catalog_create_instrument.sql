-- Wave 1 #4 — instrument catalog (wave1-data-model §5.1). Global reference data, no RLS.
-- Instrument support is data (S1 AC-7): enabling NQ/ES/MES is a row change, not code.
SET lock_timeout = '5s';

CREATE TABLE catalog.instrument (
  id           uuid PRIMARY KEY,
  symbol       text NOT NULL UNIQUE CHECK (symbol ~ '^[A-Z0-9]{1,10}$'),   -- neutral root, 'MNQ'
  name         text NOT NULL,
  exchange     text NOT NULL,
  currency     char(3) NOT NULL DEFAULT 'USD',
  tick_size    NUMERIC(22,10) NOT NULL CHECK (tick_size > 0),
  point_value  NUMERIC(18,4)  NOT NULL CHECK (point_value > 0),             -- money per 1.0 price point
  session_code text NOT NULL,                                              -- trading calendar id
  is_enabled   boolean NOT NULL DEFAULT false
);
-- tick value = tick_size × point_value, computed so the two can never disagree

CREATE TABLE catalog.instrument_contract (               -- active contract month (ADR-0003 P5)
  instrument_id  uuid NOT NULL REFERENCES catalog.instrument(id),
  contract_month date NOT NULL CHECK (extract(day FROM contract_month) = 1),   -- 'YYYY-MM' as first day
  active_from    timestamptz NOT NULL,                                       -- rollover instant
  PRIMARY KEY (instrument_id, contract_month),
  UNIQUE (instrument_id, active_from)
);

CREATE TABLE catalog.standard_account_size (             -- vision §8
  size_amount NUMERIC(18,4) PRIMARY KEY CHECK (size_amount > 0)
);

-- Reference rows production needs. MNQ (CME Micro E-mini Nasdaq-100): tick 0.25 index points,
-- US$ 2.00 per point (tick value US$ 0.50).
INSERT INTO catalog.instrument (id, symbol, name, exchange, currency, tick_size, point_value, session_code, is_enabled)
VALUES ('01a166e7-a8d0-700f-bade-6d8493132056', 'MNQ', 'Micro E-mini Nasdaq-100', 'CME', 'USD',
        0.25, 2.0000, 'cme_equity_index', true);

INSERT INTO catalog.standard_account_size (size_amount)
VALUES (25000), (50000), (100000), (150000), (200000), (300000);

-- Default privileges (bootstrap) grant SELECT to allwin_app / allwin_worker; writes are admin
-- operations through migrations until an admin use case exists.
