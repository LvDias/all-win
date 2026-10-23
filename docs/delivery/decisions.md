# Decision Log — All Win (delivery)

> Append-only. Product decisions are canonical in [product-vision.md](../product/product-vision.md); this log records user decisions given during delivery. Reversals are new entries.

## 2026-10-23 — Account usage for agents
- **Decision:** recruit agents on Dev2 and Dev3 only; Dev1 is not to be used until the user says so.
- **Context:** Dev1's Claude Max account has no usage left at the moment.
- **Canonical doc:** this log.

## 2026-10-23 — Plan before implementation; no push/merge/deploy without authorization
- **Decision:** present the architecture proposal and the V1 slice plan to the user before any implementation starts. No push, merge, or deploy without explicit user authorization.
- **Canonical doc:** this log.

## 2026-10-23 — S0 and V1 plan approved; recommendations accepted with conditions
- **Decision:** the user approved the S0 proposal (ADR-0001 stack, ADR-0002 repository structure, root CLAUDE.md) and the V1 slice plan, accepting all recommendations for A1–A6 and Q1–Q16 with these conditions:
  - **A3:** webhook secret in the URL, and it must be masked in every log (Caddy, Core, audit).
  - **A2:** production hosted in the US, central region (near Chicago).
  - **Tooling:** DevOps may install development tooling on the host. Opening firewall ports requires explicit user authorization.
  - **Q15:** before the user provides the Windows + NinjaTrader 8 machine, the Architect defines in the NT8 spike how agents will build and test the plugin there (Maestri over SSH needs bash/curl and may not work on Windows), and presents options.
  - **Git:** the user fixed `safe.directory` for `/srv/www/allwin`.
- **Options rejected:** secret in the message body (A3); local replication shortcut in V1 (A4); open sign-up (Q6); see the canonical docs for the others.
- **Canonical docs:** [ADR-0001](../architecture/adr/0001-technology-stack.md), [ADR-0002](../architecture/adr/0002-repository-structure.md), [s0-decisions-needed.md](../architecture/s0-decisions-needed.md), [slice-plan.md](../product/release-1/slice-plan.md) — to be updated by their owners.

## 2026-10-23 — Prop firms that forbid automation: explicit per-account opt-in (option A)
- **Decision:** keep the plan: TradingView → automatic execution in V1.0, master replication in V1.1. Presets show each firm's official stance on automation and copying, with source and date. When registering or editing an account of a firm that forbids automation, the user must explicitly choose whether the account accepts automated sources (no default), after seeing a warning with the firm's rule text and the risk of losing the account. The choice is recorded in the audit log with date and user.
- **Context:** R1 research found Apex and Phidias officially forbid automation; Lucid allows it; none mentions webhooks.
- **Options rejected:** (B) product-enforced block of automated sources plus master replication moved to V1.0. (C) consulting the firms is deferred, handled by the user.
- **Presets:** items that conflict across a firm's official pages ship marked "not confirmed" and do not block the rest of the preset; the conflict list goes to the user for a later decision. Presets also store the platform (e.g., Rithmic/Tradovate).
- **Risk accepted by:** the user, per account, through the explicit opt-in.
- **Canonical docs:** product-vision.md and slice-plan.md (PO to update S5/S7/S16); presets research format.

## 2026-10-23 — Consolidated open questions answered (items 1–22 accepted)
- **Decision:** the user accepted every recommendation:
  - ADR-0003 (instruction contract v1) and ADR-0004 (adapter protocol v1) accepted.
  - P1 exits bypass risk/out-of-sync gates (pause/disable block all); P2 FIFO on partial exits in individual mode; P3 master protective cancel replicated with warnings; P4 expiry base = min(sent_at, received_at); P5/Q-A5 global rollover catalog maintained by the admin with advance warning.
  - T1 only entries expire, late exits audited; T2 default command TTL 5 s (editable 1–60 s per account); T3 external order marks master-replication destinations out of sync, audit only for TradingView-only destinations.
  - Q13 activity record retained 24 months rolling, configurable (trades/configuration for the account's life); Q13-b data erased 30 days after the user closes the account (privacy policy).
  - Q-A1 one max position per account in V1 (MNQ); Q-A2 stop/target profiles keyed by instrument; Q-A4 signal filter per (source, account); Q-N1 dollar distances rounded toward the entry, adjustment shown in the audit.
  - Q17 multiple named TradingView sources; Q18 unpairing allowed with confirmation, accounts become out of sync, nothing closed; Q19 admin-only invites, single-use, email-bound, 7 days, delivered manually; Q20 transactional email only for invite/reset, admin-generated reset link as fallback; Q21 12 h idle / 24 h absolute, remember-me 7 d / 30 d, step-up after 15 min; Q22 10 recovery codes, password + code to toggle TOTP, admin-assisted reset; Q23 browser language (else pt-BR), profile time zone defaulting to browser, 24-hour clock.
- **User-owned pending item:** review NinjaTrader ToS §5(a)(viii) and EULA (or consult NinjaTrader/legal) before offering the product to other people. Does not block development on simulation.
- **Still open:** W1–W3 (Windows machine access) pending the Architect's consolidated options; preset conflict list pending PO review.
- **Canonical docs:** ADR-0003, ADR-0004, auth-and-session-policy.md, q13-audit-retention.md, wave1-data-model.md (owners mark Accepted); slice-plan.md and product-vision.md (PO).

## 2026-10-23 — Standing authorization for the development environment
- **Decision:** agents have passwordless sudo on this server (group `devs`). DevOps may install, configure and fix the development environment on its own, including finishing `infra/scripts/provision-dev-host.sh`, without asking the user.
- **Still requires explicit user approval:** firewall (ufw), SSH configuration, creating or removing users, anything that exposes ports to the internet, push/merge/deploy, and any access to production.
- **Escalation rule:** call the user only for business or structural decisions (architecture, scope, costs, security). Technical environment problems are solved by the team.
- **Canonical doc:** this log.

## 2026-10-23 — Windows test machine, per-account opt-ins, presets, model tiers
- **Decision:** accepted W1 (reverse SSH tunnel from Windows; no inbound port), W3 (user types the NT8 password on restarts; Windows auto-logon off), Q24 (separate copy opt-in for followers of firms that forbid/condition copying), Q25 (opt-in also for conditional/unknown/not-confirmed stances), Q26 (new preset version changing the stance suspends automated sources until reconfirmed), Q27 (double execution with NT8 down: auto-flatten only the position created by the orphan leg, with maximum alert), and the R1 conflict-list recommendations with Apex Legacy and Phidias LIVE excluded. The user reviews the 31 conflicts later; until then conflicting items ship as "not confirmed".
- **W2 changed:** the user prefers a Contabo Windows VPS without GPU if a GPU is not indispensable for building and testing the plugin on Sim/Playback; minimum requirements to be confirmed by the team. Two machines will be needed later for S19 cross-instance tests.
- **Model tiers:** the user authorized creating a Sonnet preset for routine tasks.
- **Canonical docs:** nt8-spike-plan.md (Architect), slice-plan.md / product-vision.md (PO).

## 2026-10-23 — R2 scope findings; remote access to the Windows VPS
- **Decision (presets):** S-1 Topstep: option (a), preset in V1 for manual tracking only (rules, warnings, manual trades; no routing), with the limitation explained; (c) a ProjectX adapter on the user's PC stays for future evaluation. S-2 Take Profit Trader: copy stance "conditional" with the official text shown; the per-account opt-in applies; asking the firm is the user's task. S-3/S-4: Topstep Labs Challenges/Pro Account and TPT PRO+/PRO+ Development are excluded.
- **Options rejected:** Topstep excluded from V1 presets (b).
- **Decision (Windows VPS access):** the team may configure the VPS's remote access under one rule: RDP must never be open to the internet. The user's proposal is Tailscale between the user's Mac, the Linux server and the Windows VPS, with RDP allowed only over the tailnet. If there is a reason not to use Tailscale, the team brings the alternative to the user before applying it. The user's home IP may change, so IP allow-lists are not suitable.
- **Canonical docs:** slice-plan.md / presets research (PO); nt8-spike-plan.md (Architect).

## 2026-10-23 — Working rules after the night audit
- **Decision:** no agent writes directly in the main checkout; every change goes through a worktree and the verification gates. Scripts that run with sudo get code review and security review before running.
- **Standing authorization:** the Maestro may have DevOps merge locally into `main` any slice whose required gates are approved for the exact revision, always without push, recording each merge in `docs/delivery/log/`.
- **Models:** use the Sonnet preset ("Claude Sonnet") for routine tasks from now on.
- **Git:** the initial commit's author stays as the user's name. The user set system `safe.directory = *` because worktrees live outside `/srv/www/allwin`.
- **Delivery record:** keep `board.md` updated at every status change.
- **Canonical doc:** this log.

## 2026-10-23 — Overnight protocol and model economy
- **Decision (overnight):** do not wait for the user. User decisions go to `board.md` as open questions with options and a recommendation, and all independent work continues. Reversible, low-risk decisions proceed with the recommendation, marked "decided provisionally, awaiting user confirmation". Irreversible, security or money decisions stay open. "Trust this folder" and permission prompts inside the project and worktrees may be accepted by the team. If Dev2 or Dev3 hits its usage limit, move work to the other account; if both do, save state in the board and stop in an orderly way. Morning summary: done, provisional decisions, waiting for the user.
- **Decision (model economy):** Sonnet for routine work (research, DevOps, routine QA, frontend with ready spec); Opus only for architecture, risk/order logic, security and code review. Dismiss idle agents and recruit again with a short brief when needed; prefer a fresh agent per new slice; `/compact` long sessions; normal effort for Architect/DBA after architecture decisions; summarized test/build output showing only failures.
- **Canonical doc:** this log.
