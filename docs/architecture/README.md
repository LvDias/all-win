# Architecture — All Win

Owner: Software Architect. Status vocabulary for ADRs: **Proposed** → **Accepted** (user approved) → **Superseded by ADR-NNNN** / **Deprecated**. Accepted ADRs are never rewritten; changes go into a new ADR.

## ADRs

| ADR | Title | Status |
|---|---|---|
| [0001](adr/0001-technology-stack.md) | Technology stack and runtime architecture | Accepted (2026-10-23) |
| [0002](adr/0002-repository-structure.md) | Repository structure, ownership and contract flow | Accepted (2026-10-23) |
| [0003](adr/0003-instruction-contract-v1.md) | Standard instruction contract v1 and correlation identifiers | Accepted (2026-10-23) |
| [0004](adr/0004-adapter-protocol-v1.md) | Core ↔ platform adapter protocol v1 | Accepted (2026-10-23) |
| [0005](adr/0005-nt8-plugin-implementation.md) | NinjaTrader 8 plugin implementation decisions | Proposed (confirmed by the spike) |

## Other architecture documents

- [Auth, session and credential policy](security/auth-and-session-policy.md): Q21, D10 (Accepted 2026-10-23)
- [NinjaTrader 8 spike plan and Windows build/test access](spikes/nt8-spike-plan.md): W1–W4: Contabo VPS, Tailscale access T1 and setup checklist (rev. 3; T1 awaiting user approval)
- NinjaTrader 8 platform profile: [platforms/ninjatrader8-profile.md](platforms/ninjatrader8-profile.md) (owner: Plugin Researcher)
- Data model: `data/` (owner: Database Architect)

## Pending decisions

- [S0 decisions (resolved 2026-10-23)](s0-decisions-needed.md)
- Open items: W1 revision (Tailscale) and W4 setup (spike plan §6)

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
