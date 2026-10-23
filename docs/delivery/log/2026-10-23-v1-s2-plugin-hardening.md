# 2026-10-23 — V1-S2 plugin hardening (QA Security follow-ups)

- **Scope:** follow-ups from the S2 QA [Security] review of `1009999`: NuGet lock files with locked-mode restore and full audit (L1), owner-only fake-Core token file (L2), WebSocket listener on loopback only in development builds (I2), a malformed but checksum-valid store record is treated as corruption (I3). No contract change.
- **Revision:** `9a2ece9` (branch `s/v1-s2-plugin-hardening--plugin`, owner Elo). Integrated into `main` as merge commit `a9a95a7` (local, no push).
- **Gates (exact SHA `9a2ece9`):**
  - QA [Plugins] (Pêndulo): APPROVED — locked restore enforced (a tampered lock file fails the restore).
  - QA [Security] (Vigia): APPROVED — the 2 low and 2 info findings are fixed, 0 new findings.
  - Code Review (Bisturi): APPROVED.
- **Integration check on main:** `dotnet restore --locked-mode` + `dotnet test plugins/ninjatrader8` (DevOps).
- **Known limitations:** host-level NT8 behavior is still untested until the Windows VPS (U1/U2) exists.
- **Accepted risks:** none.
- **Follow-ups:** CI (`make check` in GitHub Actions) after the private GitHub remote is set up (user authorization recorded in decisions.md, 2026-10-23).
