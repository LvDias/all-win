# 2026-10-23 — Architecture docs: ADR-0007/0008, auth policy rev. 3.1, ADR-0004 1.3/1.3.1, ADR-0005 N11, ADR-0006 §3.7, spike plan rev. 5

- **Merge:** local `main` 0606af3 = main 3b65471 + `d/architecture` a1bd0e5 (`--no-ff`). Documentation only; no code or contract files.
- **Scope:**
  - ADR-0006 §3.7 saved-but-not-yet-effective limits (S13 Q48) and commission/evaluation inputs.
  - ADR-0005 N11 (U19 a / U19.1 b).
  - ADR-0004 protocol 1.3 and 1.3.1 (close 4100, net P&L rules, evaluation, commission allocation).
  - Auth and session policy rev. 3 (user decisions on passwordless S3) and rev. 3.1 (step-up code consumed before the action).
  - ADR-0007 neutral access/step-up port; ADR-0008 account ↔ platform binding owned by accounts (user approved 2026-10-23).
  - NT8 spike plan rev. 5 (OpenSSH installed before the firewall lock, dev server joins the tailnet without an auth key).

## Gate verdicts (exact diff hashes, sha256[:12])

| Commit | Content | Verdicts |
|---|---|---|
| f1c959b | ADR-0006 §3.7 | Code Review APPROVED d9456a8957a1 |
| 6ea9980 | ADR-0005 N11 | Code Review, QA Security, QA Plugins APPROVED 1cc4cd465663 |
| 7f9a921 | ADR-0004 1.3, ADR-0006 inputs | Code Review, QA Security, QA Plugins, Database Architect APPROVED 1473aa73811d |
| e1f0ae3 | Auth policy rev. 3 | Code Review APPROVED 8d5356090006; QA Security covered by the cumulative verdict below |
| 731d119 | ADR-0004 1.3.1 | Code Review, QA Plugins APPROVED 5f22126222da |
| ea72f8d | ADR-0007/0008 round 1, policy rev. 3.1 | Code Review APPROVED 7b028d91d343; QA Security covered by the cumulative verdict below |
| 36705d1 | ADR-0007/0008 round 2 | Code Review, QA Security APPROVED 356e23ba24ae |
| a1bd0e5 | Spike plan rev. 5, ADR-0007/0008 lows | Code Review, QA Security APPROVED 3a8e747ed903 |

- **QA Security cumulative:** `git diff 3b65471 a1bd0e5` on the auth policy, ADR-0007 and ADR-0008 = 7ade345b8878, APPROVED. 0 findings open.
- **Author/architecture:** Software Architect (Esquadro). Docs link check on the 9 changed files: 0 broken (DevOps).

## Known limitations and follow-ups

- Contracts for protocol 1.3/1.3.1 and HTTP 4.x stay on their slice branches and land with the implementing code.
- Spike plan rev. 6.1 (2ca46b4, Code Review APPROVED 095fa227a617) is not in this merge; it lands with the next architecture merge.
- ADR-0003 S7 gates delta (Q74) is in review.
