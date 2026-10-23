# Design — All Win

> **Owner:** Product Designer (Aquarela @ Dev3). Business rules come from [product-vision.md](../product/product-vision.md) and the [slice plan](../product/release-1/slice-plan.md). Design never adds them.
> Status vocabulary: *proposed* (pending PO review) → *reviewed* (PO ok) → *implemented* / *QA verified* (tracked on the [board](../delivery/board.md)).

| Document | Scope | Status | Implementer · QA |
|---|---|---|---|
| [foundations.md](foundations.md) | shadcn/ui tokens (light/dark), typography, layout, number/money/price/date display, status colors (risk 80 %, account/connection), secrets pattern, mobile browsers, feedback patterns, accessibility, glossary | proposed rev. 1 | Frontend · QA |
| [specs/v1-s3-access-sources-pairing.md](specs/v1-s3-access-sources-pairing.md) | V1-S3: login, TOTP, invite sign-up, password reset, security settings, invitations (admin), signal sources (webhook URL view-once, rotate, revoke), NinjaTrader pairing (web + plugin states) | proposed rev. 1 | Frontend, Plugin Engineer (S3-11) · QA |
| [specs/v1-s4-language-locale.md](specs/v1-s4-language-locale.md) | V1-S4: locale resolution, language switcher, preferences (language, time zone), catalogs, plugin language, missing translations | proposed rev. 1 | Frontend, Plugin Engineer (§6) · QA |

Next design slices (per slice plan §4): S5, S6, S7, S10, S13 (uses `RiskMeter`), S16, S18, S22, and the plugin panels for S13/S15.
