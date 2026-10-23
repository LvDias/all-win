# 2026-10-23 — Docs integration: R2 presets research, R2 conflicts, ADR-0003 §4.1 opt-in gates, delivery record

- **Merges (fast-forward, local, no push), by DevOps:** `5ecba1c` presets research (8 R2 firms + Phidias update) · `646931d` product (`presets/r2-conflicts-for-user.md`) · `6a8f8bf` architecture (ADR-0003 §4.1 automation/copy opt-in gates; auth policy step-up for any opt-in) · `e7154b8` delivery (board, decisions, design log). `main` = `e7154b8`.
- **Scope:** documentation only. R2 covers Topstep, Take Profit Trader, MyFundedFutures, Tradeify, Bulenox, TradeDay, Alpha Futures, Top One Futures (all PARTIAL; official sources with URL and access date).
- **Gates:** owner review (PO reviewed R2 research and owns the conflict list; Architect owns ADR changes); secret/PII scan clean. No code: area QA / Code Reviewer not applicable.
- **Known limitations:** preset conflicts ship as "not confirmed" pending user review (board U6). S-6/S-7/S-8 decided provisionally (board U3–U5).
- **Accepted risks:** none.
- **Follow-ups:** R1 rev. 3 update (Lupa); contracts branch `s/v1-s1-webhook-ingest--contracts` (`45069cc`, OpenAPI S1+S3) awaits Code Reviewer before merge.
