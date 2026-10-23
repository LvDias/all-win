-- Wave 1 #11d — the accounts job finds the accounts a firm release affects (S16, Esquadro (B)).
-- The worker only FINDS work, with this narrow column grant (the documented allwin_worker
-- pattern); each user's change runs as allwin_app with app.user_id set. No index: publications
-- are rare and the scan is filtered by archived_at IS NULL.
SET lock_timeout = '5s';

GRANT SELECT (id, user_id, automation_state, automation_rule_version_id, copy_state,
              copy_rule_version_id, preset_version_id, archived_at)
  ON accounts.account TO allwin_worker;
