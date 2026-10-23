# 2026-10-23 — Docs integration: product, data model 16.5/16.6, design specs and system, presets research

- **Scope:** slice-plan update (Q75/Q76 in S7 AC-8b with side effects pending user confirmation, S12 AC-2 aligned to T1 + U13 guard, S18 AC-5 critical = Q27 incl. bracket double fill + U19, Q77–Q86 pending rows, Q78 withdrawn); R3 preset decision list for the user; data model rev. 16.5 (outbox per-consumer tracking + purge) and 16.6 (risk tables + applied_fill, gateway #9, S11 sync storage); design specs S8, S11, S13 (+S13b), S17 note; design system C§9 wave-2 shared components; presets research R3/R4 + consolidation + questions to firms.
- **Revision:** `main` 4eb433c (merges 8fc62da d/product@98178d8, c8853e9 d/data@11dfba5, 4b4d9e1 d/design-specs@ad4bdf4, 2abb981 d/design-system@2e72898, 4eb433c d/presets-research@8df42fa).
- **Gates:**
  - slice-plan 386b41d4c98e26ee: Code Review (Bisturi → Grosa) APPROVED after two rounds.
  - data model 16.5 f0ff1dcd53c4 and 16.6 d05eccbcf8fb: Code Review (Esmeril) APPROVED (16.4 CHANGES REQUIRED: 2 medium fixed).
  - design specs 5a84f794931a, 5299f814a601, 894c500af80b: Code Review (Esmeril) + PO (Rumo) APPROVED.
  - design system C§9 e637f6921c77: Code Review (Esmeril, 3 rounds) + PO APPROVED.
  - presets research: PO-reviewed research files (status PARTIAL/PROPOSED; nothing presented as verified).
- **Known limitations:** commit f9dd0cb (S22 net P&L spec) and a80ad64 (R3 research batch 1) entered under branch tips without a recorded verdict; post-merge review of f9dd0cb requested (Esmeril). Q75 side effects in S7 AC-8b await the user's confirmation; ADR-0003/0004 r4 not merged (held).
- **Accepted risks:** none.
- **Follow-ups:** user answers Q75-1/2, Q77, Q79–Q87, S-R3-1..5; d/architecture r4 merge after the user's answer; README rev bump for components.md.
