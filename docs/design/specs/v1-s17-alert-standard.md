# V1-S17 — TradingView alert standard and daily routine — Design note

> **Owner:** Product Designer (Aquarela) · **Status:** proposed, pending PO review · **Revision:** 1 · **Date:** 2026-10-23
> **Slice:** [V1-S17](../../product/release-1/slice-plan.md) AC-1 … AC-3. **Surfaces per the slice plan:** TradingView input and documentation. **No Web screen is stated by the slice.**

## 1. Verdict: no screen of its own

The slice plan states three deliverables, none of them a Web screen:

| AC | Deliverable | Where it lives | Design involvement |
|---|---|---|---|
| AC-1 | Message templates for `trigger`, `trade_plan`, `nq_trigger` in the standard JSON, each with `signal_type` and a unique id | `integrations/tradingview/` (Plugin Engineer; starts when the user hands over the Pine source, Q7) | none |
| AC-2 | pt-BR / en guide of the daily routine (paste levels, delete yesterday's alert, one alert per chart with "any alert() function call", recreate after level or indicator changes) | documentation | copy review only (F§5 terms, tone) |
| AC-3 | A test alert of each type reaches the Core and is accepted | S1 / QA | none |

Nothing here needs a layout, component or state. **Not invented:** a template picker, a "copy template" button, a guide page inside the Web, or a test-alert button. If the PO wants any of them, they need a slice change first.

## 2. Existing Web touch points (already specified, unchanged)

- S3 sources: the empty state already links "to the alert guide" and the pairing steps text carries a copy marked "validate with PO/S17 against TradingView's current UI" ([S3 spec](v1-s3-access-sources-pairing.md) sources page). When the guide exists, that link points to it; the text of the steps is validated against the guide.
- The webhook URL secret rules of F§8.1 apply to anything the guide shows: never print a real URL, only a placeholder.

## 3. Handoff and open question

- **Guide copy (AC-2):** the Plugin Engineer / PO write it; Aquarela reviews terms and screenshots policy (no secrets, F§8.1) when a draft exists.
- **Q-S17-1 (answered by the PO, 2026-10-23):** the slice plan does not specify a guide link or a copy-template button on the source dialog and S3 has none, so it is **not a requirement and not drawn**. If wanted it is a **proposal for the user** (candidate S17 AC-4): a `Guia do alerta` link and a `Copiar modelo` button on the source dialog, where the template **never contains the secret URL** (the secret is only in the URL, not in the message; X-4, A3).
- **Blocked** until the user hands over the Pine source (slice note).
- **Responsible:** Plugin Engineer (templates), PO (guide), QA (AC-3). Web: none.
