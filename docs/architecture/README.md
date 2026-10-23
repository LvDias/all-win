# Architecture — All Win

Owner: Software Architect. Status vocabulary for ADRs: **Proposed** → **Accepted** (user approved) → **Superseded by ADR-NNNN** / **Deprecated**. Accepted ADRs are never rewritten; changes go into a new ADR.

## ADRs

| ADR | Title | Status |
|---|---|---|
| [0001](adr/0001-technology-stack.md) | Technology stack and runtime architecture | Accepted (2026-10-23) |
| [0002](adr/0002-repository-structure.md) | Repository structure, ownership and contract flow | Accepted (2026-10-23) |
| [0003](adr/0003-instruction-contract-v1.md) | Standard instruction contract v1 and correlation identifiers | Proposed |
| [0004](adr/0004-adapter-protocol-v1.md) | Core ↔ platform adapter protocol v1 | Proposed |

## Other architecture documents

- [Auth, session and credential policy](security/auth-and-session-policy.md): Q21, D10 (Proposed)
- [NinjaTrader 8 spike plan and Windows build/test access](spikes/nt8-spike-plan.md): Q15 options (Proposed)
- Data model: `data/` (owner: Database Architect)

## Pending decisions

- [S0 decisions (resolved 2026-10-23)](s0-decisions-needed.md)
- Open items: ADR-0003 §10 (P1–P5), ADR-0004 §11 (T1–T3), Q21 (auth policy §1), W1–W4 (spike plan §7)

## ADR template

```markdown
# ADR-NNNN — Title
- **Status:** Proposed (YYYY-MM-DD)
- **Owner:** …
## Context
## Options considered
## Decision
## Consequences
## Adoption / migration plan
## Decisions needed
```
