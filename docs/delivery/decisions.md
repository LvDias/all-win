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

## 2026-10-23 — Morning confirmations: U1–U15, GitHub remote, VPS OS
- **Confirmed (provisional → final):** U3 Alpha Futures manual-tracking-only preset; U4 MFF "Studio" excluded; U5 All Win classified as automated trading (TradingView sources) and trade copier (master replication); U7 a material change of the firm's rule text also returns an opt-in to pending review; U8 page security headers in the Next proxy, HSTS at Caddy; U9 browser floor Chrome 106+, Safari 15.4+, Firefox 116+.
- **Approved:** U1 Tailscale T1 (user RDP and agents' SSH over the tailnet; this dev server joins the tailnet) with Tailnet Lock on; U10 login lockout keyed by HMAC(pepper, normalized identifier) for any identifier; U11 remove the global pairing lock, per-IP and per-/24 or /64 limits, global threshold becomes an alert; U12 recovery code accepted in disableTotp only, plus a warning e-mail on 2FA removal; U13 option (A) position-identity guard on late `reduce` (`flatten` unchanged); U15 local "allow live accounts" opt-in in the NT8 plugin release UI in addition to the Core flag.
- **Options rejected:** U1 T2/T3; U13 (B) expiring reduce and (C) keep T1; U11 keep the global lock; U12 admin reset only / also in stepUp.
- **Pending:** U2 Windows VPS — before contracting, the Architect confirms whether NinjaTrader 8 supports Windows Server 2022 or Windows 10/11 must be requested, and delivers the final configuration. U6 preset conflicts and the questions to firms stay with the user, non-blocking.
- **Standing authorization (GitHub):** the user creates the private repository `allwin`. Push of `main` and branches is authorized only to that private repository, as an off-server copy. Still forbidden: making the repository public, deploy, any production access.
- **Working rule:** after every dispatch, the Maestro confirms delivery in the agent's terminal.
- **Canonical docs:** auth-and-session-policy.md and contracts (Architect) for U10–U12; ADR-0004 / adapter protocol (Architect) for U13 and U15; nt8-spike-plan.md (Architect) for U1/U2; slice-plan.md / presets (PO) for U3–U5, U7, U9.

## 2026-10-23 — Requirement change: passwordless authentication (supersedes Q21/Q22)
- **Decision:** no passwords. Invite-only sign-up: the user gives an e-mail and a WhatsApp number and confirms each with a code sent to it. Login: the user chooses to receive the code by e-mail or WhatsApp. Code: single use, expires in 10 minutes, invalidated after 5 wrong attempts, send limits per user and per IP. A notice e-mail on every login from a new device. Sensitive actions (enable automation on an account, unpair NT8, change e-mail or WhatsApp) require a fresh code. Changing e-mail or WhatsApp requires codes on both the old and the new channel. WhatsApp may ship after e-mail.
- **Open (user):** code alphabet — 8 uppercase alphanumerics without ambiguous characters, or mixed case + digits + specials (U16). WhatsApp provider choice (cost; options with cost and lead time to be brought by the team) (U17).
- **Superseded:** Q21/Q22 password + TOTP model; U10 (password-login lockout) and U12 (TOTP recovery code) approvals no longer apply as written; the Architect maps their intent (anti-enumeration, per-identifier throttling) to code delivery and verification.
- **Canonical docs:** slice-plan.md / S3 spec (PO); security/auth-and-session-policy.md, HTTP contract, data model (Architect, Database Architect).

## 2026-10-23 — User decisions on all open recommendations (U16–U19, M2/M3, Q28–Q68, migration, VPS, GitHub, WhatsApp)
- **Accepted as recommended:** U16 (8-char Crockford code, `XXXX-XXXX`); M2 (72 h hold for a channel change requested from a new device); M3 (on a new device, persistence actions need the e-mail code); Q28–Q36 as recommended, including the Q35 step-up set (the user's 3 actions + source create/rotate, connection create, pairing code, admin invitation, route_enable, account_binding_change; server setting on) and proposals (i) two-channel change needs the other channel's code and (ii) 24 h freeze; U18 (pre-deploy versioning: breaking → document major, `/api/v1` kept); U19 (a) + U19.1 (b), and the user ACCEPTS the residual risk (a flatten that fails after cancelling legs → top-severity alert + out_of_sync); Q37–Q45, Q47–Q53, Q55–Q68 as recommended.
- **Migration:** the user AUTHORIZES the destructive forward migration #2b (drop password/TOTP/reset tables and columns, delete dev sessions) on the development environment only (no production exists).
- **Exceptions:**
  - Commissions (recorded by the user as "Q55"; it is the commissions question, Q54): **net P&L (with commissions) is the main value, gross shown beside it**, because firms measure drawdown and limits on net. Impact on S13/S15 risk to be analysed by the Architect.
  - Q46: **include in V1 the evaluation profit target (how much is left to pass)**; minimum days and payout later. The team reports the cost if it is complex.
- **VPS:** the user contracts Contabo Cloud VPS 8 in St. Louis with Windows Server 2025 (2022 if unavailable) and will hand over access; the team says if Windows tests are needed earlier.
- **GitHub:** private repository under account `LvDias`; the user registers the deploy key (standing push authorization from earlier today applies only to that private repository).
- **WhatsApp (U17):** the user starts the Meta business verification (Meta Cloud API direct, as recommended).
- **Canonical docs:** slice-plan / product-vision (PO); auth-and-session-policy, ADR-0004/0005, contracts (Architect); data model (Database Architect).

## 2026-10-23 — Net P&L, evaluation target, commission floor (U20), GitHub remote
- **Approved:** net P&L as the main value (gross beside it) and the V1 evaluation profit target (how much is left to pass), cost ≈ 4–6 dev-days, with **risk computed on net** (daily loss, drawdown, target, S15 enforcement).
- **U20:** option (a) — per-account `commission_per_contract_side` (prefilled by the S16 preset when the firm publishes it, editable); effective commission per fill = max(platform-reported, configured × qty); warning when neither exists. Option (b) platform-only rejected.
- **Evaluation target (Q46 details):** start = date given by the user, default = the account's registration date; reaching the target = notice + notification only, no automatic action. Minimum days and payout later.
- **GitHub:** the private repository is `git@github.com:LvDias/all-win.git` with the deploy key registered; the user asks to configure the remote, test the connection and push `main` and the branches (standing authorization: push only to this private repository; never public, no deploy, no production access).
- **Canonical docs:** slice-plan (PO); ADR-0004/0005 + protocol 1.3, auth policy (Architect); HTTP contract (Compasso); data model (Database Architect).

## 2026-10-23 — First push done; routine pushes
- **Fact:** the user ran the initial push; `main` (188ed53) and the `s/*` / `d/*` branches are on the private repository `git@github.com:LvDias/all-win.git`, using dev2's deploy key.
- **Decision:** from now on the team pushes to that private repository under the standing authorization (main after each local integration, and the slice/docs branches). If the Claude Code filter blocks a push, the Maestro gives the user the ready command. Still forbidden: making the repository public, force pushes, deploy, production access.
- **Canonical doc:** this log.

## 2026-10-23 — Q71 manual-trade commission
- **Decision:** when a manual trade's commission is left blank, it is filled automatically from the account's configured `commission_per_contract_side` (U20); if the account has no configured commission, the recommended option applies (zero, net = gross, labelled "comissões não informadas"). Manual trades still never feed risk.
- **Note:** the amount is computed by the Core (the browser does no money arithmetic); the UI shows the Core's value/preview and the user may override it.
- **Canonical docs:** slice-plan S21 AC-7 (PO); HTTP contract trades module (Compasso).

## 2026-10-23 — UX rule: long forms become step-by-step
- **Decision (user):** when a form gets long, use a step-by-step (stepper) flow, so the user adding something is not tired by seeing many fields at once. The user likes the current visual design.
- **Application:** the Product Designer (design-system owner) defines the threshold and the stepper pattern in the foundations; specs and web forms follow it (first case: S5 "Nova conta": type → account data → platform → automation).
- **Canonical docs:** docs/design/foundations.md (Aquarela); product-vision (PO).
