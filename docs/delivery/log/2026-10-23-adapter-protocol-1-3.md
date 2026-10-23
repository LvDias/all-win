# 2026-10-23 — Adapter protocol 1.3 / 1.3.1 integrated

- **Scope:** adapter protocol 1.3 (notifications, trade_completed, notification_read, welcome display fields, commission in risk_state) and 1.3.1 (documentation only); 1.2 (U13 position-identity guard) and the live_consent registry were already on main.
- **Revision:** `main` 5bde605 (merge --no-ff of s/v1-protocol-1-3--contracts @ 31f0821; 1.3 = 2b45236, diff be6c238..2b45236 = 92d4b5111016; 1.3.1 diff 2b45236..31f0821 = 18e0c87dfcbb). ADR parts already on main (1473aa73811d, 731d119).
- **Gates:** 1.3 — Code Review (Crivo) APPROVED, QA [Security] (Vigia) APPROVED, QA [Plugins] (Pêndulo) APPROVED with 2 text lows, Database Architect (Lastro) APPROVED; 1.3.1 — Crivo + Pêndulo APPROVED. DevOps: validate_fixtures on the merge ALL OK (adapter 47 valid / 45 invalid; instruction 13/29).
- **Known limitations:** follow-up text deltas queued (already_flat cancels every working order on the contract; 1.3.2 reason-codes note for Q75; canon of 20 sync.* codes; resolved reasons when none active; stored params limit; accounts_changed message in the queue).
- **Accepted risks:** none.
- **Follow-ups:** gateway (Bigorna), plugin (Elo) and QA (Agulha) rebase/merge main; HTTP wave2 merge must not regress contracts/adapter-protocol.
