# 2026-10-23 — V1-S2 (prep) Plugin adapter core on Linux

- **Merge (local, no push), by DevOps:** contracts and ADR first — `da4accf`, `97ca844` (reason codes, rebased from `4fe3957`/`bb6b8ba`), `8f249cd`, `ec504b2` (ADR-0004, rebased from `46ffa80`/`d10c9a0`) — then merge commit `6579023` of the plugin at the reviewed SHA `1009999`.
- **Scope:** `plugins/ninjatrader8/**` — `AllWin.Adapter.Core` (netstandard2.0; net48, no NinjaTrader references): adapter protocol v1/1.1 engine (hello/welcome, acks, outbox, dedupe of msg_id/command_id/fill_id, stale commands, expiry of entries only, write-ahead and crash recovery, mandatory reconnect snapshot, individual/shared brackets with protection never larger than the position, orphan protection and orphan flatten (Q27), live-fill rule for brackets, simulation guard on every platform write, durable CRC journal), simulated platform, scripted fake Core with 11 scenarios. Contributes to S2, S9, S11, S15 ACs; NT8 host not built.
- **Gate verdicts (exact SHA `1009999`, clean QA worktrees):**
  - QA [Plugins] (Pêndulo): APPROVED, Linux scope — 421 tests + 23 adversarial probes. Earlier rounds: 3 high + 1 medium + low findings, all fixed.
  - Code Reviewer (Bisturi): APPROVED — earlier 3 high (stuck flatten after crash, pruned bracket owner, startup prune with absent account), 2 medium, lows: all fixed.
  - QA [Security] (Vigia): APPROVED — summary: 0 critical, 0 high, 0 medium, 2 low, 3 info (lows fixed afterwards in the hardening follow-up).
  - Software Architect: rules decided in ADR-0004 (answers A–G, R1 live-fill rule, L3 torn tail, 1.0 rollback).
- **Known limitations:** NOT verified on NinjaTrader 8 — the net48 AddOn host, UTC conversion of execution times, Playback clock, OCO behavior, NTFS durability (spike E4/E9/E11/E12) need the Windows VPS. The slice is not "done" until those checks pass. No CI pipeline yet.
- **Open user decisions touching this slice:** U13 (late partial exit guard), U15 (local live opt-in).
- **Accepted risks:** none.
- **Follow-ups:** hardening branch `9a2ece9` (NuGet lock files, token 0600, ws only in Development) in verification; host slice after the VPS exists.
