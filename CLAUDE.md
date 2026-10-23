# All Win — Project rules for all agents

> **Approved by the user on 2026-10-23 (S0).** Owner: Software Architect; changes need user approval.

## 1. Read first

1. [docs/product/product-vision.md](docs/product/product-vision.md) — the product bible. Business rules come from the user; never invent them. Surface gaps as questions through the Maestro.
2. [docs/architecture/](docs/architecture/README.md) — ADRs (ADR-0001 stack, ADR-0002 repository structure, ADR-0003 instruction contract, ADR-0004 adapter protocol) and `contracts/`.
3. [docs/delivery/board.md](docs/delivery/board.md) and [decisions.md](docs/delivery/decisions.md) — current slice state and user decisions.
4. The `CLAUDE.md` / README of the component you touch.

Read only what your task needs. Search before reading long files.

## 2. Safety — non-negotiable

- **Simulation only.** Agents develop and test only with simulation, demo or playback accounts (NinjaTrader Sim/Playback, adapter-sim, Tradovate demo). Never connect, configure or trade a live account. Live use is done exclusively by the user.
- **No secrets in code, logs, fixtures, screenshots or messages.** The TradingView webhook URL contains a secret token: mask it everywhere (Caddy, Core, audit); only its `key_id` may appear (ADR-0001 §5). Ingress secrets, device tokens, passwords and third-party credentials are stored only as hashes or encrypted. Logs pass through redaction. `.env` files are never committed; only `.env.example` with placeholders.
- **Exact decimals.** Prices, P&L and money use `NUMERIC` (PostgreSQL), `rust_decimal` (Rust), `decimal` (C#), and **strings** on the wire. Never floats. NinjaTrader `double` prices are converted to `decimal` at the plugin boundary with tick rounding. The browser does no money arithmetic.
- **Per-user isolation.** Every user-owned query is scoped by the authenticated `user_id`. Every endpoint has a cross-user access test.
- **Never execute twice, never execute late.** Idempotency by `message_id` / `command_id` / `event_id`. Expired commands are reported, not executed. After a disconnect: alert and wait for the user, never auto-resync.
- **Transparency.** Every automatic action (route, skip, cap, flatten, block, expire) is written to the user-visible audit log with its reason.
- **No push, merge or deploy** without explicit user authorization. **No firewall port is opened** (this host, production, or the user's Windows machine) without explicit user authorization. Agents do not commit unless assigned. DevOps integrates.

## 3. Architecture boundaries (ADR-0001, ADR-0002)

- **Input → Core → Output.** Platforms connect only through adapters. Nothing in the Core names a specific platform (NinjaTrader, Tradovate, TradingView). Platform logic lives in `core/adapters/*` (server-side) or `plugins/*` (client-side).
- **Rust Core** (`core/`): modular monolith. Each business module has `enterprise` / `application` / `infra` crates with inward-only dependencies. Modules communicate only via events and query ports; wiring happens in `crates/app`. No microservices, queues, caches or new infrastructure without an ADR.
- **Web** (`web/`): Next.js + TypeScript + shadcn/ui. The Next.js server renders and routes only. Business rules and authorization live in the Core. Features in `src/features/<feature>` import only `src/shared` and themselves.
- **NinjaTrader 8 plugin** (`plugins/ninjatrader8/`): logic in `AllWin.Adapter.Core` (`netstandard2.0`, no NinjaTrader references, tested on Linux); NinjaTrader API mapping and UI only in the `net48` AddOn host. NinjaTrader DLLs are never committed.
- **Contracts are spec-first** (`contracts/`): JSON Schema (instruction, adapter protocol) and OpenAPI. Change the contract first, then the code. Every language passes the shared fixtures. Generated TypeScript is committed and checked for drift. Breaking changes need a new major version and Architect approval.

## 4. Ownership

Single writer per path (full table in ADR-0002 §3):

| Path | Owner |
|---|---|
| `docs/product/` | Product Owner |
| `docs/delivery/` | Maestro |
| `docs/architecture/`, `contracts/`, `CLAUDE.md` | Software Architect |
| `core/` (`migrations/` reviewed by Database Architect) | Backend Rust |
| `web/` | Frontend |
| `plugins/ninjatrader8/`, `integrations/tradingview/` | Plugin Engineer |
| `tests/e2e/` | QA |
| `infra/`, root config, CI | DevOps |

Never edit, stage, reformat or revert files you do not own. Request changes through the Maestro. Work in your own worktree (`/srv/www/allwin.worktrees/<branch>`) on a slice branch `s/<slice-id>-<name>`; never directly on `main`.

## 5. Verification gates

These commands become effective once scaffolded; DevOps wires them into `make check`.

- **Core:** `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (including PostgreSQL integration tests), contract fixture tests, OpenAPI drift check.
- **Web:** `pnpm lint`, `pnpm typecheck`, `pnpm test`, i18n key parity (pt-BR = en), generated-types drift check; Playwright E2E for user-facing flows.
- **Plugin:** `dotnet test` on `AllWin.Adapter.Core`; NinjaTrader 8 Sim/Playback checklist on the Windows machine for host-level changes.
- **Security:** no secrets in diffs or logs; authorization and isolation tests for every new endpoint.
- Run the checks relevant to your change. Never claim a check passed without running it. Report failures with their output. A slice is "done" only after independent QA verification.

## 6. Language

- Code, identifiers, commits and project documents: **English**.
- UI: **pt-BR and en** for every user-visible string (web and plugin); no hard-coded UI text. The Core returns stable error codes, and clients translate them. Display formats are locale-aware; parsing of numbers is exact.
- Conversation with the user: **pt-BR**.

## 7. Reporting

End every task with: **Status** · **Scope/revision** · **Changed files** · **Validation** (run → result; not run → why) · **Open questions/blockers** · **Next owner**. Separate proposed, approved and verified states.
