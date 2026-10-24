-- Wave 1 #2d — identity.source.name shape (data model §8 #2d; Crivo L1 on 196a30a; user-authorized
-- 2026-10-24). The audit copies the source name into every ingress event of the source: an empty
-- or token-shaped name would dead-letter all of them (audit_entry_no_token). The Core already
-- refuses such names (DisplayName: trimmed, 1..60 chars, not blank, no control character, no
-- token shape → 400), so this CHECK only guards the paths that bypass the API (seed, script, bug).
-- Narrowing, CHECK only, no data changed (0 violations measured in the dev databases).
-- Runs connected as allwin_owner (see #1).
SET lock_timeout = '5s';

ALTER TABLE identity.source
  ADD CONSTRAINT source_name_shape_check
  CHECK (char_length(name) BETWEEN 1 AND 60 AND name ~ '\S'
         AND name !~ 'aw[kd]_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}') NOT VALID;
ALTER TABLE identity.source VALIDATE CONSTRAINT source_name_shape_check;
