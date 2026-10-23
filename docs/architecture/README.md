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
| [0006](adr/0006-derived-state-across-modules.md) | Derived state across modules: versioned events and a staleness gate | Accepted (2026-10-23) |
| [0007](adr/0007-access-port.md) | Access port: authenticated caller and step-up across modules | Accepted (2026-10-23) |
| [0008](adr/0008-account-platform-binding.md) | Account ↔ platform account binding is owned by `accounts` | Accepted (2026-10-23) |

## Other architecture documents

- [Auth, session and credential policy](security/auth-and-session-policy.md): passwordless sign-in by one-time code, sessions, devices, step-up (rev. 2 Proposed 2026-10-23; rev. 1 password + TOTP superseded)
- [NinjaTrader 8 spike plan and Windows build/test access](spikes/nt8-spike-plan.md): W1–W4: Contabo VPS, Tailscale access T1 and setup checklist (rev. 4: T1 with Tailnet Lock approved, U1; final VPS configuration, U2)
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
