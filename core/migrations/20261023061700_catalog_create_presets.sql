-- Wave 1 #11 — prop firm presets (wave1-data-model §5.7; S16; preset release schema v1 6523253759bd).
-- Global catalog, no RLS. Written only by the publish-release CLI as allwin_publisher (Esquadro,
-- 2026-10-23): published content is immutable (SELECT/INSERT only); identity rows can be renamed
-- (column UPDATE only); nothing is deleted. All foreign keys stay inside `catalog` (§1, rule 1).
-- The publisher writes only `catalog` (no outbox, Esquadro (B)): an accounts worker job reads the
-- releases by `publish_seq` through a catalog query port and suspends the affected opt-ins per user.
-- Deltas from §5.7 rev. 15.2, all from the frozen release schema:
--   firm_plan.key and preset.size_key (the release file's stable keys);
--   firm_release: publish_seq (accounts job cursor, Esquadro (B)), source_sha256 (idempotent import), research_file, published_by; excluded_plans as
--   jsonb ({name, reason}); firm_release_platform.manual_only (S-R3-2/4); personal_device may be
--   not_confirmed (only accepted by the import with HTTP 5.4.0); preset_rule_set.stored_only (AC-10).
SET lock_timeout = '5s';

CREATE TABLE catalog.prop_firm (
  id          uuid PRIMARY KEY,
  slug        text NOT NULL UNIQUE CHECK (slug ~ '^[a-z0-9]+(-[a-z0-9]+)*$' AND char_length(slug) <= 64),
  name        text NOT NULL CHECK (char_length(name) BETWEEN 1 AND 120),
  created_at  timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE catalog.firm_release (                      -- versioning unit: one researched, published firm (S16 §2.1)
  id             uuid PRIMARY KEY,
  -- Publication order read by the accounts job (cursor). The CLI, the only writer, takes
  -- pg_advisory_xact_lock(hashtext('catalog'), hashtext('publish')) (two-key space, apart from the
  -- one-key outbox consumer locks) as the first statement of its transaction,
  -- so publications are serial and commit in publish_seq order: a reader never sees seq n+1 before
  -- seq n, and a rollback only leaves a gap (the cursor reads > last). No table lock, so the
  -- publisher needs no UPDATE/DELETE privilege.
  publish_seq    bigint GENERATED ALWAYS AS IDENTITY UNIQUE,
  firm_id        uuid NOT NULL REFERENCES catalog.prop_firm(id),
  version_number integer NOT NULL CHECK (version_number >= 1),
  published_at   timestamptz NOT NULL DEFAULT now(),
  verified_on    date NOT NULL,
  research_file  text NOT NULL CHECK (research_file ~ '^docs/product/presets/research/[a-z0-9-]+\.md$'),
  source_sha256  bytea NOT NULL CHECK (octet_length(source_sha256) = 32),  -- canonical release file
  published_by   text NOT NULL CHECK (published_by ~ '^[a-z0-9][a-z0-9._-]{0,63}$'),  -- operator label (CLI), never an e-mail
  changes        jsonb NOT NULL DEFAULT '[]' CHECK (jsonb_typeof(changes) = 'array'),       -- ReleaseChange[]
  manual_only    jsonb CHECK (manual_only IS NULL OR jsonb_typeof(manual_only) = 'object'), -- RuleQuote (S16 AC-7)
  excluded_plans jsonb NOT NULL DEFAULT '[]' CHECK (jsonb_typeof(excluded_plans) = 'array'),  -- S16 AC-9 {name, reason}
  UNIQUE (firm_id, version_number),
  UNIQUE (firm_id, source_sha256),                      -- a re-run of the same file conflicts: "already published"
  UNIQUE (firm_id, id)
);

CREATE TABLE catalog.firm_release_platform (             -- FirmDetail.platforms
  release_id  uuid NOT NULL REFERENCES catalog.firm_release(id),
  code        text NOT NULL CHECK (code ~ '^[a-z][a-z0-9_]{0,31}$'),
  label       text NOT NULL CHECK (char_length(label) BETWEEN 1 AND 60),
  manual_only jsonb CHECK (manual_only IS NULL OR jsonb_typeof(manual_only) = 'object'),  -- S-R3-2/4: {reason}
  PRIMARY KEY (release_id, code)
);

CREATE TABLE catalog.firm_stance (                       -- vps / personal_device per release (automation/copy = opt_in_rule_version)
  release_id uuid NOT NULL REFERENCES catalog.firm_release(id),
  kind       text NOT NULL CHECK (kind IN ('vps','personal_device')),
  value      text NOT NULL,
  quotes     jsonb NOT NULL DEFAULT '[]' CHECK (jsonb_typeof(quotes) = 'array'),   -- RuleQuote[], verbatim
  research   jsonb CHECK (research IS NULL OR jsonb_typeof(research) = 'object'),
  PRIMARY KEY (release_id, kind),
  CONSTRAINT firm_stance_value_check CHECK (
         (kind = 'vps' AND value IN ('allowed','forbidden','conditional','unknown','not_confirmed'))
      OR (kind = 'personal_device' AND value IN ('required','not_required','unknown','not_confirmed')))
);

CREATE TABLE catalog.firm_plan (                         -- stable across releases (plan_id on the wire)
  id      uuid PRIMARY KEY,
  firm_id uuid NOT NULL REFERENCES catalog.prop_firm(id),
  key     text NOT NULL CHECK (key ~ '^[a-z0-9]+(-[a-z0-9]+)*$' AND char_length(key) <= 64),  -- release-file key
  name    text NOT NULL CHECK (char_length(name) BETWEEN 1 AND 120),
  UNIQUE (firm_id, id),
  UNIQUE (firm_id, key)
);
CREATE UNIQUE INDEX firm_plan_name_key ON catalog.firm_plan (firm_id, lower(name));

CREATE TABLE catalog.preset (                            -- firm × plan × phase × size; stable across releases
  id          uuid PRIMARY KEY,
  firm_id     uuid NOT NULL,
  plan_id     uuid NOT NULL,
  phase       text NOT NULL CHECK (phase IN ('evaluation','funded','live')),
  size_amount NUMERIC(18,4) NOT NULL CHECK (size_amount > 0),
  size_key    text NOT NULL CHECK (size_key ~ '^[a-z0-9]+(-[a-z0-9]+)*$' AND char_length(size_key) <= 64),  -- release-file key
  size_label  text CHECK (char_length(size_label) <= 32),
  UNIQUE (plan_id, phase, size_amount),
  UNIQUE (plan_id, phase, size_key),
  UNIQUE (firm_id, id),
  FOREIGN KEY (firm_id, plan_id) REFERENCES catalog.firm_plan (firm_id, id)
);

CREATE TABLE catalog.preset_version (                    -- one per preset per release that offers it
  id          uuid PRIMARY KEY,
  preset_id   uuid NOT NULL,
  firm_id     uuid NOT NULL,
  release_id  uuid NOT NULL,
  manual_only jsonb CHECK (manual_only IS NULL OR jsonb_typeof(manual_only) = 'object'),  -- RuleQuote; risk only, no routing
  UNIQUE (preset_id, release_id),
  FOREIGN KEY (firm_id, preset_id)  REFERENCES catalog.preset (firm_id, id),
  FOREIGN KEY (firm_id, release_id) REFERENCES catalog.firm_release (firm_id, id)   -- same firm, by construction
);
-- superseded_by is derived (the same preset's version in a later release), not stored, so rows stay immutable.

CREATE TABLE catalog.preset_rule_set (                   -- PresetVersion.rule_sets
  preset_version_id uuid NOT NULL REFERENCES catalog.preset_version(id),
  firm_platform     text CHECK (firm_platform ~ '^[a-z][a-z0-9_]{0,31}$'),  -- NULL = one set for every platform
  rules             jsonb NOT NULL,                      -- PresetRules with provenance per field (verified / not_confirmed)
  stored_only       jsonb,                               -- AC-10: min trading days, payout rules; never exposed in V1
  CONSTRAINT preset_rule_set_rules_check CHECK (jsonb_typeof(rules) = 'object'
         AND rules ?& ARRAY['max_position_qty','drawdown','daily_loss_limit','trading_day_reset']),  -- S16 AC-6 fields
  CONSTRAINT preset_rule_set_stored_only_check CHECK (stored_only IS NULL OR jsonb_typeof(stored_only) = 'object'),
  -- money only as decimal strings (X-3); every money field is named `amount`
  CONSTRAINT preset_rule_set_amount_check CHECK (NOT jsonb_path_exists(rules, '$.**.amount ? (@.type() == "number")')
         AND (stored_only IS NULL OR NOT jsonb_path_exists(stored_only, '$.**.amount ? (@.type() == "number")')))
);
CREATE UNIQUE INDEX preset_rule_set_key ON catalog.preset_rule_set (preset_version_id, coalesce(firm_platform, ''));
-- "One set without a platform OR one per platform" is a publishing rule (Core, import check 5) plus a §9 test.

ALTER TABLE catalog.prop_firm             OWNER TO allwin_owner;
ALTER TABLE catalog.firm_release          OWNER TO allwin_owner;
ALTER TABLE catalog.firm_release_platform OWNER TO allwin_owner;
ALTER TABLE catalog.firm_stance           OWNER TO allwin_owner;
ALTER TABLE catalog.firm_plan             OWNER TO allwin_owner;
ALTER TABLE catalog.preset                OWNER TO allwin_owner;
ALTER TABLE catalog.preset_version        OWNER TO allwin_owner;
ALTER TABLE catalog.preset_rule_set       OWNER TO allwin_owner;

-- ---- grants ---------------------------------------------------------------------------------
-- allwin_app / allwin_worker read the catalog through the bootstrap default privileges (SELECT).
-- allwin_publisher (NOBYPASSRLS, publish-release CLI only; created by DevOps in every environment
-- before this migration): insert-only on published content, column renames, no DELETE anywhere.
GRANT USAGE ON SCHEMA catalog TO allwin_publisher;
GRANT SELECT, INSERT ON catalog.prop_firm, catalog.firm_release, catalog.firm_release_platform,
  catalog.firm_stance, catalog.firm_plan, catalog.preset, catalog.preset_version,
  catalog.preset_rule_set TO allwin_publisher;
GRANT UPDATE (name)       ON catalog.prop_firm TO allwin_publisher;
GRANT UPDATE (name)       ON catalog.firm_plan TO allwin_publisher;
GRANT UPDATE (size_label) ON catalog.preset    TO allwin_publisher;
-- The publisher also reads the instruments and standard sizes it validates against.
GRANT SELECT ON catalog.instrument, catalog.standard_account_size TO allwin_publisher;
