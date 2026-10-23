-- Wave 1 #11b — opt-in rule versions per firm release (wave1-data-model §5.7 forward change; S16).
-- #5a (20261023060900) is never edited. A non-generic rule version belongs to one firm release ×
-- firm_platform × kind; its firm and release stay inside `catalog`. The only existing rows are the
-- two generic ones (release_id NULL, firm_id NULL), so the new constraints validate at once.
SET lock_timeout = '5s';

ALTER TABLE catalog.opt_in_rule_version
  ADD COLUMN release_id uuid,
  ADD CONSTRAINT opt_in_rule_version_release_check CHECK (is_generic = (release_id IS NULL)),
  ADD CONSTRAINT opt_in_rule_version_firm_id_fkey FOREIGN KEY (firm_id) REFERENCES catalog.prop_firm(id),
  -- The release belongs to the same firm (as preset_version). Generic rows have both NULL, which
  -- MATCH SIMPLE skips.
  ADD CONSTRAINT opt_in_rule_version_release_fkey FOREIGN KEY (firm_id, release_id)
    REFERENCES catalog.firm_release (firm_id, id);

-- One text per (release, firm platform, kind); a firm whose stance does not vary by platform has
-- firm_platform NULL, which NULLS NOT DISTINCT keeps unique too (PG ≥ 15).
CREATE UNIQUE INDEX opt_in_rule_version_release_key
  ON catalog.opt_in_rule_version (release_id, firm_platform, kind) NULLS NOT DISTINCT
  WHERE NOT is_generic;

-- Grants: allwin_publisher (publish-release CLI) inserts new versions; nobody updates or deletes.
-- The #5a note about INSERT for allwin_app is superseded: the app never writes rule versions.
GRANT SELECT, INSERT ON catalog.opt_in_rule_version TO allwin_publisher;
