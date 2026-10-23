# Design — All Win

> **Owner:** Product Designer (Aquarela, design-system owner; other designers request changes through the Maestro). Business rules come from [product-vision.md](../product/product-vision.md) and the [slice plan](../product/release-1/slice-plan.md). Design never adds them.
> Status vocabulary: *proposed* (pending PO review) → *reviewed* (PO ok) → *implemented* / *QA verified* (tracked on the [board](../delivery/board.md)).

| Document | Scope | Status | Implementer · QA |
|---|---|---|---|
| [foundations.md](foundations.md) | shadcn/ui tokens (light/dark), typography, layout, number/money/price/date display, status colors (risk 80 %, account/connection), secrets pattern, step-up by code, mobile browsers, feedback patterns, accessibility, glossary | proposed rev. 4 | Frontend · QA |
| [components.md](components.md) | Shared components: `CodeInput` / `CodeChallenge` (one-time codes, resend cooldown), `Stepper` (+ `FormStepper` for long forms), wave-2 shared widgets (`OptInSection`, `StanceBadge`/`StanceBlock`, `FirmRuleQuote`, `ConflictPicker`, `DistanceFields`, `StepUpDialog`, `RiskMeter`; C§9), `EmptyState` variants, `DataTable` with row actions, `RiskBanner` (80 % / limit reached), `NotificationItem`, `NotificationBell`, `CriticalBanner` (S18) | proposed rev. 4 | Frontend · QA |
| [specs/v1-s3-access-sources-pairing.md](specs/v1-s3-access-sources-pairing.md) | V1-S3 (passwordless): login by code, invite sign-up with e-mail confirmation, step-up by code, change e-mail, security settings, new-device notice, admin invitations, signal sources, NinjaTrader pairing / re-pairing / unpair (web + plugin states); S3b WhatsApp addendum | proposed rev. 4 (auth operations are proposals until the Architect adopts them) | Frontend, Plugin Engineer (S3-11) · QA |
| [specs/v1-s4-language-locale.md](specs/v1-s4-language-locale.md) | V1-S4: locale resolution, language switcher, preferences (language, time zone), catalogs, plugin language, missing translations | final rev. 2 (Q23 applied), PO review | Frontend, Plugin Engineer (§6) · QA |
| [specs/v1-s5-automation-opt-in.md](specs/v1-s5-automation-opt-in.md) | V1-S5 (+S7/S10/S16 touch points): per-account automation opt-in for firms that forbid automation (official rule quote, risk, no default, audit, step-up), suspension after rule change, routing unavailability, firm platform selection, "not confirmed" preset values | proposed rev. 2 (contract-aligned); O1/Q24–Q26 open with the user | Frontend · QA (Backend for routing skip/audit) |

| [handoff-frontend-s3-s4.md](handoff-frontend-s3-s4.md) | Frontend handoff for S4 then S3 web: reading order, rev. 3 contract changes, cross-cutting rules | current | Vitral (Frontend) · QA |

Next design slices (per slice plan §4): S5 full account form, S6, S7, S10, S13 (uses `RiskMeter`), S16, S18, S22, and the plugin panels for S13/S15.
