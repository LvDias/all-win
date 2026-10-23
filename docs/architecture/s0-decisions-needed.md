# S0 — Architecture decisions (resolved)

- **Status:** Resolved (2026-10-23): all recommendations accepted by the user, with the conditions below · Owner: Software Architect
- Record: [decisions.md](../delivery/decisions.md), entry "S0 and V1 plan approved".
- Context: [ADR-0001](adr/0001-technology-stack.md), [ADR-0002](adr/0002-repository-structure.md), and the approved root [CLAUDE.md](../../CLAUDE.md).
- Product questions are tracked by the PO in [slice-plan.md §7](../product/release-1/slice-plan.md). The architecture-relevant ones are only cross-referenced here: **Q6** sign-up mode, **Q7** Pine Script ownership, **Q15** Windows + NinjaTrader 8 machine, **Q16** command TTL.

| # | Question | Decision (2026-10-23) |
|---|---|---|
| A1 | Confirm the technology stack | **Accepted** as in ADR-0001 |
| A2 | Production region | **US central region, near Chicago** (user condition). Dev/staging stay on the current server |
| A3 | Where the TradingView webhook secret travels | **In the webhook URL**, and **masked in every log: Caddy, Core and audit** (user condition). Requirement and verification test in ADR-0001 §5 |
| A4 | Master → follower replication path within one NinjaTrader instance | **Always relay through the Core in V1**. The local fast path is rejected for V1 |
| A5 | Two-factor authentication | **Optional TOTP** in V1 |
| A6 | Git remote and CI | **Private GitHub repository + GitHub Actions**. Push, merge and deploy only with explicit user authorization |

Additional user conditions:

- DevOps may install development tooling on this host. **Opening any firewall port requires explicit user authorization.**
- `safe.directory` for `/srv/www/allwin` was fixed by the user.

## Architecture notes for the related product questions

- **Q15 (Windows + NinjaTrader 8 machine):** accepted option (a). The user asked the Architect to define, in the NinjaTrader 8 spike plan, how agents will build and test the plugin on that machine, and to present options before the machine is provided. Maestri over SSH needs bash/curl and may not work on Windows.
- **Q16 (command TTL):** short default, editable per account. Specified in ADR-0004.
- **Q14 (out-of-sync):** an account flagged out of sync stops receiving new instructions until the user clears it. Specified in ADR-0004.
- **Q1 (conflicting instructions):** rejected and notified; `close` flattens everything; `reverse` while flat opens; no `modify`/`cancel` from TradingView in V1. Specified in ADR-0003.
- **Q7 (Pine Script):** the user shares the source; it lives in `integrations/tradingview/` (ADR-0002).

## What blocks V1 slices (architecture view)

- **Before any code:** DevOps provisioning toolchains. S0 is approved.
- **Before S1/S2:** ADR-0003 (instruction contract v1 + identifiers) and ADR-0004 (adapter protocol v1) with fixtures, plus the NinjaTrader 8 spike. These are the Architect's next deliverables after S0 approval.
- **Before S19/S20:** ADR-0003 and ADR-0004 accepted by the user (A4 is decided: relay through the Core).
