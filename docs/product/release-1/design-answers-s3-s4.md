# PO answers to design questions — S3/S4

> **Owner:** Product Owner (Bússola @ Dev2) · **Asked by:** Product Designer (Aquarela @ Dev3) · **Date:** 2026-10-23
> Base: [slice-plan.md](slice-plan.md) rev. 3. "user" = open user decision, tracked as Q17–Q23 in slice-plan §7; keep it [Open] with the designer's proposal.

| ID | Topic | Answer |
|---|---|---|
| D1 | Webhook secret reveal | **Confirmed** (S3 AC-3 "view once"): full URL only in the create/rotate dialog (masked, reveal/copy); afterwards only the prefix; the only way out is rotation. |
| D2 | Rotation grace period | **Confirmed:** immediate revocation, with an explicit warning that the current alert stops working. No grace period in V1. |
| D3 | Sources per user | **user (Q17).** Recommendation: several named sources, one URL each. |
| D4 | Revoking a source | **Confirmed:** source shown as "revoked"; nothing deleted (S10 AC-3). |
| D5 | Pairing code validity/format | **Architect** (ADR-0001 §6). No product rule. |
| D6 | Connection name | **Design decision.** Proposal (user-given name, instance id as detail) is fine. |
| D7 | Unpairing an instance in use | **user (Q18).** Recommendation: allow after a confirmation listing affected accounts; with an open position, a strong warning (position stays open in NinjaTrader without platform monitoring/enforcement; accounts flagged out of sync). Nothing is closed automatically. |
| D8 | Invitations | **user (Q19).** Recommendation = designer's proposal (admin role held by the user, single-use link bound to an e-mail, 7 days, manual delivery). |
| D9 | Password reset without e-mail | **user (Q20).** Q8 excluded e-mail *notifications* only. Recommendation: transactional e-mail for invite and reset only; fallback = admin-generated reset link. |
| D10 | Password policy | **Architect/security.** No product rule; no objection to NIST 800-63B (min. 12, breached-password list). |
| D11 | Session length | **user via Architect (Q21).** |
| D12 | TOTP recovery | **user (Q22).** Recommendation = designer's proposal (10 single-use codes; password + code to enable/disable; admin-assisted recovery). |
| D13 | TOTP suggestion banner | **Confirmed:** dismissible banner (S3 AC-1b "strongly suggested"); form is a design decision. |
| D14, D16, D17 | Language, time zone, time format | **user (Q23).** Recommendation: language from browser (pt or en, else pt-BR); profile time zone (default = browser), always shown next to reset/session times; 24-hour clock in both languages. |
| D15 | Currency | **Confirmed:** always USD, no conversion; per-locale formatting as proposed, exact values (X-3). |
| D18 | Risk states | **Confirmed:** normal (<80%), warning (≥80%), limit reached/blocked (until reset); no 90% band (vision §9). Profit target in "notify only": positive color is fine. If the account chose "flatten and block" for the profit target (Q3), the state is blocked but the reason must say it was the target. Consistency is tracked/warned only (Q4), never blocks. Account states (paused, disabled, disconnected, out of sync) are separate from risk states and can coexist with them. |
