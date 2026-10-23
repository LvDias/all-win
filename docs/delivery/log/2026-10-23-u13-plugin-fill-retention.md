# 2026-10-23 — U13 plugin delta: fill retention for open positions + reference validator thread safety

- **Scope:** plugin `AllWin.Adapter.Core` keeps fills of the still-open position (same PositionSeq, non-flat) past retention so the U13/N11 row-6 cap survives restarts; flat positions are pruned; records without an instrument fall back to the old pruning. Test harness: reference schema validator with its own registry and lock (flaky T1 resolved).
- **Revision:** `s/v1-u13-reduce-guard--plugin` de98e86 (parent 8b9f3e5; reviewed patch hash acc038806e60); integrated in `main` aee304e (merge --no-ff).
- **Gates:**
  - Code Review (Bisturi): APPROVED acc038806e60.
  - QA [Plugins] (Pêndulo): APPROVED d40cd3e = de98e86 tree; 677/677 × 10 runs, 20/20 under load, 36/36 probes, mutation of the InOpenPosition clause killed.
  - QA [Security] (Guarita): APPROVED; 0 critical / 0 high / 0 medium / 1 low (delta) + 1 low (pre-existing).
  - DevOps: plugin tests 677/677 on the merge before push.
- **Known limitations:** Linux scope only; net48 host, dialog/DPAPI, kind detection on NT8, E4/K15, E14 untested until the Windows VPS (checklist drafted by Pêndulo).
- **Accepted risks:** none.
- **Follow-ups:** Elo — L-a `InOpenPosition` calls `GetPositions` unwrapped in periodic Prune (availability, fail-closed); L-b (pre-existing) `SafeExecutions` returns empty on failure so Prune may drop records the platform still holds.
