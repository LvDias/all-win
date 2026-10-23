> **Product Owner note (2026-10-23):** English translation (CLAUDE.md §6) of the original research by Farol (pt-BR, `/tmp/farol-whatsapp-providers.md`). Content, figures, source labels and links are preserved; labels translated as **[OFFICIAL] / [THIRD-PARTY] / [NOT CONFIRMED]**. **Every price and delivery-time figure not marked [OFFICIAL] is unconfirmed and must not be used as a requirement.** This is input to **U17** (supplier and cost: a **user decision**, not made here) and to [slice V1-S3b](../release-1/slice-plan.md#v1-s3b--whatsapp-as-a-second-code-channel-blocked). Points relevant to the product: the supplier must let the Core keep generating and verifying the code (S3 AC-5; section 2.3 notes Twilio Verify would hold the code outside the Core, an Architect decision); e-mail as the fallback channel (S3b AC-5); the dedicated number and business verification depend on the user and take weeks (critical path for S3b).

# WhatsApp OTP providers — research (All Win)

Research date: 2026-10-23. Read-only research: no account created, nothing purchased, no personal data entered.
Legend: **[OFFICIAL]** = read on a provider/Meta page during this research · **[THIRD-PARTY]** = blogs/aggregators only · **[NOT CONFIRMED]** = no usable source found.

## 0. Explicit assumptions

- Base: 100 to 1,000 users; 2 to 5 codes/user/week; 4.33 weeks/month.
- Destination mix: 50% Brazil / 50% US (adjustable; Brazil is twice the price per message).
- Each code = 1 delivered AUTHENTICATION template message. Resends and undelivered failures are not charged by Meta (charged on delivery only) [OFFICIAL].
- Reference exchange rate: US$ 1 = R$ 5.00 (used only for Zenvia).
- Taxes (ISS/PIS/COFINS in Brazil, IOF on USD charges) are **excluded** from all calculations.

## 1. Meta's current billing model

- **Per delivered message billing** (no longer per 24-hour conversation) since 1 Jul 2025. The rate varies by category and recipient country. [OFFICIAL] https://developers.facebook.com/documentation/business-messaging/whatsapp/pricing
- The AUTHENTICATION (and UTILITY) category has **volume tiers** per market, reset every month; only charged messages count. For our volume, assume the entry tier (no discount). [OFFICIAL, same page]
- **Authentication-international**: higher rate applied to up to 18 markets (9 new on 1 Oct 2026: Bangladesh, Iraq, Kazakhstan, Kuwait, Morocco, Nepal, Oman, Sri Lanka, Ukraine). **Brazil and North America are not in the list of new ones**; the full list of the 9 earlier markets was not verified. A business whose Main Business Location is in the market may keep the standard rate. Relevant if there are users in other countries. [OFFICIAL; full list NOT CONFIRMED]
- **Local billing in BRL** (Facebook Brasil) available since 1 Jul 2026; accounts in another currency must migrate by 30 Jun 2027. Today only new BRL accounts can be created. [OFFICIAL]
- Meta updates the rate card quarterly (CSV/PDF cards per currency on the page above). [OFFICIAL]
- Service messages (replies to the user) stopped being free on 1 Oct 2026, with 1,000 free per number/month. Does not affect OTP (we only send templates). [OFFICIAL]

### AUTHENTICATION rate (list price, USD, entry tier)

| Destination | Value | Status |
|---|---|---|
| US / North America | US$ 0.0034 | **[THIRD-PARTY]** — several sites and the Twilio calculator (https://www.twilio.com/en-us/whatsapp/pricing, "Pricing current as of September 2026") show exactly 0.0034 as the authentication Meta fee. Meta confirms only that the rate dropped on 1 Jan 2026 (https://developers.facebook.com/docs/whatsapp/pricing/updates-to-pricing/). |
| Brazil | US$ 0.0068 (~R$ 0.034 at R$ 5.00) | **[THIRD-PARTY]** — 0.0068 appears on several sites; Meta's updates page cites "about 0.68 ¢ per utility or authentication message" in Brazil (official hint, not the rate card). Third-party sources also cite R$ 0.03 to R$ 0.035. |

**Discrepancies to ignore**: US$ 0.0315 and R$ 0.16 for Brazil, and US$ 0.0135 for the US, are values from the old per-conversation model or have no source. US$ 0.003876/0.007752 comes from a single guide (+14%).
**Mandatory action before any contract**: download the USD and BRL rate CSVs at https://developers.facebook.com/docs/whatsapp/pricing (section "Rate cards and volume tiers") and check both lines. I could not obtain the files with this tool (links not exposed in the text).

## 2. Comparison by provider

### 2.1 Meta WhatsApp Cloud API (direct)
- **Price**: only Meta's rate (table above); no intermediary fee, no monthly fee or minimum. [OFFICIAL as to the model]
- **Production entry**: Meta Business account (portfolio) + WABA + dedicated number + business verification (Business Verification) + display name approval + AUTHENTICATION template + payment method (card or credit line) on the WABA. Without verification: low initial limit (hundreds of recipients/24 h, and 250 business-initiated messages/24 h if the display name is rejected, per Twilio). [OFFICIAL via Twilio docs https://www.twilio.com/docs/whatsapp/self-sign-up]
- **Lead time**: business verification "can take several weeks" depending on region [OFFICIAL via Twilio]; an authentication template is usually approved in minutes to hours [NOT CONFIRMED, no lead time in Meta's docs]. Realistic estimate: 1 to 4 weeks, dominated by verification.
- **Number**: dedicated, must not be registered in the WhatsApp/Business app (the account must be deleted), must receive SMS or voice for the registration OTP; cannot be an IVR/outbound-only line. [OFFICIAL via Twilio]
- **Authentication template**: fixed text ("<code> is your verification code." + optional "For your security, do not share this code." + optional expiry from 1 to 90 min), mandatory OTP button (copy code; Android one-tap autofill; Android zero-tap; iOS 26+ suggests it from the keyboard). No free-form content: the same template wording is translated by Meta per language (`languages` field). [OFFICIAL https://developers.facebook.com/docs/whatsapp/business-management-api/authentication-templates/]
- **SMS/e-mail fallback**: none; must be done in our backend (suggestion: e-mail first, SMS through a separate provider).
- **API**: REST/JSON + webhooks; works with plain HTTP from Rust. [OFFICIAL]
- **Data/LGPD**: the contract is directly with Meta (Meta Platforms Ireland / Facebook Brasil for BRL billing). Residency: **[NOT CONFIRMED]**; Meta announces data storage localization for some countries, but I did not verify Brazil. The user's phone number goes to Meta (controller/processor to be defined with legal).
- **Risks**: no dedicated support (Business Support tickets only); number/WABA blocking for quality reasons (reports, many sends without opt-in); display name rejection; Meta may reclassify a template; dependence on a single Meta account (a ban stops all WhatsApp sign-in).

### 2.2 Twilio — WhatsApp (Programmable Messaging)
- **Price**: Meta (pass-through, table above) + **US$ 0.005 per message** Twilio, inbound or outbound. Calculator: 0.0034 + 0.005 = **US$ 0.0084 (US)**. Brazil: 0.0068 + 0.005 = 0.0118 [THIRD-PARTY for the Meta part]. "Pricing current as of September 2026". [OFFICIAL https://www.twilio.com/en-us/whatsapp/pricing]
- **Monthly fee/minimum**: none listed (pay-as-you-go). The Twilio account must be "upgraded" (paid). [OFFICIAL]
- **Entry**: Self Sign-up in the Console (WABA + profile + number verification by OTP in minutes); display name reviewed by Meta; business verification is the long step ("several weeks"). Template via Content Template Builder. [OFFICIAL https://www.twilio.com/docs/whatsapp/self-sign-up]
- **Fallback**: SMS via Twilio (Brazil US$ 0.0599/segment, Oct 2026 page; US US$ 0.0083 per the Verify page) and e-mail via SendGrid, all in the same account, but automatic fallback comes only through Verify. [OFFICIAL https://www.twilio.com/en-us/sms/pricing/br]
- **API**: REST and webhooks; no official Rust SDK (plain HTTP). Excellent docs.
- **Data/LGPD**: Twilio offers a DPA; regional residency (Twilio Regions: US1, IE1, AU1) for account data; WhatsApp not verified. **[NOT CONFIRMED]**.
- **Risks**: the same set of Meta account risks (the WABA stays in our Meta Business, portable only via migration); cost per message 2.5x the direct one.

### 2.3 Twilio Verify (WhatsApp + SMS + e-mail)
- **Price**: **US$ 0.05 per successful verification** + channel fee. WhatsApp: + US$ 0.0034 per template (US); SMS: + US$ 0.0083 (US); e-mail: only the 0.05. The page also shows US$ 0.0147 per template in the FAQ — two figures on the same page; **treat the higher one as a risk** until confirmed. Volume discounts only by contacting sales. No monthly minimum listed. [OFFICIAL https://twilio.com/verify/pricing]
- **Strengths**: generates, delivers, verifies and rate-limits attempts; automatic SMS/voice/e-mail fallback; no need to implement OTP storage. Does it conflict with our own ledger model? Yes — Verify holds the code; assess with the Architect (ADR) whether sign-in keeps the OTP in the Core.
- **Risk**: charges only on success, but at ~14x the direct channel. The authentication template follows Meta's rules.

### 2.4 Zenvia
- **Price**: **[NOT CONFIRMED]**. The official billing-rules page (https://zenvia.com/novas-regras-cobranca-whatsapp-2026/) says API traffic is "invoiced and intermediated by Zenvia", **does not separate the Meta rate from the margin**, and does not publish an authentication price. The simulator uses R$ 0.05/message "taxes already included" (simulation input, not an official table). A third-party comparison (tenessihub.com.br) cites channel packages from R$ 100/month and templates between R$ 0.47 and R$ 0.55 — a figure that does not match the above and **must not be used** without confirmation.
- **Strengths**: billing and invoicing in BRL, support in Portuguese, SMS/RCS/e-mail on the same platform, local CNPJ (useful for LGPD and procurement).
- **Production entry**: BSP process (Zenvia acts as a Solution Partner); same Meta prerequisites; lead time and contract requirements **[NOT CONFIRMED]**; probably requires a CNPJ and a commercial contract.
- **API**: REST; **[NOT CONFIRMED]** details.
- **Risk**: lack of price transparency; the contract may have a monthly minimum; outside Brazil, support is smaller.

### 2.5 Infobip
- **Price**: pay-as-you-go with a per-country/volume calculator on its own page; I could not open the page (HTTP 403). Third parties: no fixed plans [THIRD-PARTY]; YCloud (a competitor) claims ~39% average markup and AgentOS from US$ 530/month — **partial, unverified, biased**. **[NOT CONFIRMED]**.
- **Fallback**: SMS, e-mail, voice and RCS on the same platform (general knowledge, not verified in this research).
- **API**: REST. **Data**: has DPAs and data centers in several regions **[NOT CONFIRMED]**.
- **Risk**: oriented to corporate contracts; high friction for a small base.

### 2.6 Vonage (Messages API)
- **Price**: Meta's rate per delivered message + **Vonage platform fee "from €0.0001"** (per message; the real value depends on volume and contract) — a search-result snippet of the official page https://www.vonage.com/communications-apis/messages/features/whatsapp/pricing/, which returned 403 when I tried to open it. **[PARTIAL/NOT CONFIRMED]**. Example in the Telekom/Vonage doc (Germany, marketing): Meta €0.1131 + Vonage €0.0754.
- **Entry**: Meta partner; same requirements. **[NOT CONFIRMED]** as to lead time.
- **API**: REST (Messages API), with failover between channels. **Data/LGPD**: **[NOT CONFIRMED]**.
- **Risk**: little focus on Brazil; price depends on negotiation.

### 2.7 Gupshup (self-serve)
- **Price**: **US$ 0.001 per message** (template and session) Gupshup fee, plus Meta's rate charged at cost. The page table shows "Authentication 0.0000 USD" for the Meta rate, which is **placeholder/confusing** (the pass-through is probably charged separately); monthly fee not listed; no date on the page. [OFFICIAL partial https://www.gupshup.ai/isv-partners/whatsapp-api/pricing]
- **Entry**: ISV/BSP with self-serve onboarding; lead times equal to Meta's. **Fallback**: SMS via Gupshup **[NOT CONFIRMED]**.
- **API**: REST. **Data/LGPD**: based in India/Singapore/US; **[NOT CONFIRMED]** as to residency in Brazil; little PT-BR support.
- **Risk**: the WABA account stays under the partner; support and reputation vary; the payment page uses a prepaid wallet.

## 3. Estimated monthly cost (messages only, USD, no taxes)

Volume: A = 100 users × 2/week ≈ **866 msg/month**; B = 500 × 3/week ≈ **6,495**; C = 1,000 × 5/week ≈ **21,650**.
50/50 BR/US mix → average Meta cost US$ 0.0051/msg [THIRD-PARTY].

| Provider | US$/msg (average) | A (866) | B (6,495) | C (21,650) | Note |
|---|---|---|---|---|---|
| Meta Cloud API direct | 0.0051 | **4.4** | **33** | **110** | + taxes; no fallback |
| Gupshup | 0.0061 | 5.3 | 40 | 132 | + monthly fee [NOT CONFIRMED] |
| Twilio WhatsApp | 0.0101 | 8.7 | 66 | 219 | no minimum listed |
| Zenvia | ~0.010 (R$ 0.05) | ~8.7 | ~65 | ~217 | **[NOT CONFIRMED]**; order of magnitude only |
| Twilio Verify (WhatsApp) | 0.0551 | 48 | 358 | 1,193 | with fallback and verification; the US$ 0.0147/msg scenario would add ~US$ 0.01 to each line |
| Infobip / Vonage | — | — | — | — | no verifiable price |

Summary: the cost is negligible in every scenario (< US$ 220/month outside Verify). What decides is **operations** (fallback, support, Meta account risk), not price.
SMS fallback (if used in 10% of sends): Brazil US$ 0.0599/segment at Twilio — ~US$ 130/month in scenario C for Brazil alone; **the fallback route for Brazil should be e-mail**, not SMS, for cost reasons.

## 4. Recommendation

1. **Meta WhatsApp Cloud API direct** to start: lowest cost, no monthly fee, a single contract with Meta, plain REST compatible with Rust, and the WABA account stays ours (no partner lock-in). Weak points: no fallback and weak support.
2. **Fallback in the Core**, not in the provider: e-mail as the first resort (free), SMS optionally later, everything recorded in the user's audit log, as the transparency rule in CLAUDE.md §2 requires.
3. If the team prefers to **delegate operations** (no dealing with the Meta account or webhooks), the second choice is **Twilio WhatsApp** (+US$ 0.005/msg, published price, no minimum, good docs) — not Verify, which costs ~11x more and would store the OTP outside the Core.
4. Zenvia is only worth it if billing in BRL with invoices and support in Portuguese is required, and **only after** receiving a written table separating Meta and margin. Infobip/Vonage: not recommended for the current size (no verified public price, contract-oriented).

## 5. Pending items before the decision

- Download Meta's official CSV rate cards (USD and BRL) and confirm US$ 0.0034 (US) and US$ 0.0068 (BR).
- Confirm the list of the 9 original *authentication-international* markets and whether users in other countries fall under it.
- Ask Zenvia, Infobip and Vonage for a written table (Meta + fee + monthly fee/minimum + taxes).
- Data residency / LGPD (Meta, Twilio): validate with legal; legal basis and role (processor/controller) for the user's phone number.
- Warning: the dedicated number and business verification (CNPJ) depend on the **user**; start early, as they are the critical path (several weeks).
- Security: no tokens or keys in the repository; always in encrypted secrets (CLAUDE.md §2). Development only with Meta's test number (sandbox), never production.

## Sources

- Meta, pricing: https://developers.facebook.com/documentation/business-messaging/whatsapp/pricing
- Meta, pricing updates: https://developers.facebook.com/docs/whatsapp/pricing/updates-to-pricing/
- Meta, authentication templates: https://developers.facebook.com/docs/whatsapp/business-management-api/authentication-templates/
- Twilio WhatsApp pricing: https://www.twilio.com/en-us/whatsapp/pricing
- Twilio Verify pricing: https://twilio.com/verify/pricing
- Twilio SMS Brazil: https://www.twilio.com/en-us/sms/pricing/br
- Twilio Self Sign-up: https://www.twilio.com/docs/whatsapp/self-sign-up
- Gupshup: https://www.gupshup.ai/isv-partners/whatsapp-api/pricing
- Zenvia, billing rules: https://zenvia.com/novas-regras-cobranca-whatsapp-2026/
- Vonage (search snippet only; page returned 403): https://www.vonage.com/communications-apis/messages/features/whatsapp/pricing/
- Infobip (page returned 403; third parties only)
- Third parties (hints only): getkanal.com, flowcall.co, authgear.com, zernio.com, ycloud.com/vs-infobip
