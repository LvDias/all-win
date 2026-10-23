-- Wave 1 #5a — opt-in rule versions (wave1-data-model §5.1, rev. 8; S5 contract §2.5, Q40).
-- Global reference data, no RLS. Append-only: a published text the user saw never changes; a
-- correction is a new version with `change_cause`. S16 adds the firm/release/preset foreign keys
-- and the non-generic uniqueness (forward migration #11).
SET lock_timeout = '5s';

CREATE TABLE catalog.opt_in_rule_version (
  id                uuid PRIMARY KEY,
  kind              text NOT NULL CHECK (kind IN ('automation','copy')),
  firm_stance       text NOT NULL CHECK (firm_stance IN ('allowed','forbidden','conditional','unknown','not_confirmed')),
  is_generic        boolean NOT NULL,
  firm_id           uuid,                                -- S16: FK to catalog.prop_firm
  firm_platform     text CHECK (firm_platform ~ '^[a-z][a-z0-9_]{0,31}$'),  -- when the rule is platform-specific
  version_number    integer NOT NULL CHECK (version_number >= 1),
  published_at      timestamptz NOT NULL DEFAULT now(),
  quotes            jsonb NOT NULL DEFAULT '[]' CHECK (jsonb_typeof(quotes) = 'array'),   -- RuleQuote[], validated by the Core
  research          jsonb CHECK (research IS NULL OR jsonb_typeof(research) = 'object'),
  change_cause      text CHECK (change_cause IN ('stance_changed','text_changed','not_material')),
  change_reason     jsonb CHECK (change_reason IS NULL OR jsonb_typeof(change_reason) = 'object'),  -- {"pt-BR": …, "en": …}
  previous_id       uuid REFERENCES catalog.opt_in_rule_version(id),
  UNIQUE (id, kind),
  CONSTRAINT rule_version_generic_check CHECK (NOT is_generic OR (firm_stance = 'unknown' AND quotes = '[]'::jsonb
             AND research IS NULL AND firm_id IS NULL AND firm_platform IS NULL)),
  CONSTRAINT rule_version_firm_check CHECK (is_generic OR firm_id IS NOT NULL),
  CONSTRAINT rule_version_change_check CHECK ((previous_id IS NULL) = (change_cause IS NULL)),
  CONSTRAINT rule_version_reason_check CHECK (change_cause IS DISTINCT FROM 'text_changed' OR change_reason IS NOT NULL)
);
ALTER TABLE catalog.opt_in_rule_version OWNER TO allwin_owner;
CREATE UNIQUE INDEX opt_in_rule_version_generic_key ON catalog.opt_in_rule_version (kind, version_number) WHERE is_generic;
-- The current version of a selection is the highest version_number.

-- The two generic versions (Q40): no text (the UI shows its own generic warning). Fixed literal
-- ids, so the Core and tests can name them (accounts_enterprise::GENERIC_*_RULE_VERSION_ID).
INSERT INTO catalog.opt_in_rule_version (id, kind, firm_stance, is_generic, version_number, published_at)
VALUES ('019a1000-0000-7000-8000-000000000001', 'automation', 'unknown', true, 1, '2026-10-23T00:00:00Z'),
       ('019a1000-0000-7000-8000-000000000002', 'copy',       'unknown', true, 1, '2026-10-23T00:00:00Z');

-- Grants: the bootstrap default privileges give SELECT to allwin_app / allwin_worker. INSERT for
-- allwin_app comes with the S16 admin publish use case; nobody gets UPDATE/DELETE at runtime.
