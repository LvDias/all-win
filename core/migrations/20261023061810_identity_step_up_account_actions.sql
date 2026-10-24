-- Wave 1 #2f — two account step-up actions (user decisions Q95/Q96, auth policy rev. 3.3 §6.2;
-- data model rev. 16.18): account_sync_clear ("clear out of sync") and trading_day_restart
-- ("restart today's count"), both with an account target. Widens the two one_time_code CHECKs
-- as 061800 left them (admin_recovery included); the session check is unchanged. CHECK only, no
-- data changed. Runs connected as allwin_owner (see #1).
SET lock_timeout = '5s';

ALTER TABLE identity.one_time_code
  DROP CONSTRAINT one_time_code_action_check,
  ADD CONSTRAINT one_time_code_action_check
    CHECK (action IN ('source_create','source_rotate','connection_create','pairing_code_new',
                      'connection_unpair','account_opt_in_accept','account_create_opt_in_accept',
                      'admin_invitation_create','route_enable','account_binding_change','admin_recovery',
                      'account_sync_clear','trading_day_restart')),
  DROP CONSTRAINT one_time_code_step_up_check,
  ADD CONSTRAINT one_time_code_step_up_check
    CHECK ((purpose = 'step_up') = (action IS NOT NULL)
           AND (target_id IS NOT NULL) = COALESCE(action IN ('source_rotate','pairing_code_new','connection_unpair',
                                                            'account_opt_in_accept','route_enable',
                                                            'account_binding_change','admin_recovery',
                                                            'account_sync_clear','trading_day_restart'), false));
