# V1-S4 — Language and locale — Screen spec

> **Owner:** Product Designer (Aquarela) · **Status:** final for implementation, pending PO review of rev. 2 · **Revision:** 2.1 · **Date:** 2026-10-23
> **Slice:** [V1-S4](../../product/release-1/slice-plan.md) (ready) · AC-1..AC-4 · cross-cutting X-3, X-6, X-8.
> **Foundations:** [foundations.md](../foundations.md) F§5 glossary, F§6 formatting, F§11 accessibility.
> **Stack:** `next-intl`, locale-prefixed routes `/pt-BR/...` and `/en/...`, default locale `pt-BR`, preference stored in the profile and sent to the plugin in `welcome` (ADR-0001 §9).
> **Decided by the user on 2026-10-23** ([decisions.md](../../delivery/decisions.md)): **Q23**: language from the browser (pt or en, else pt-BR); display time zone from the profile, defaulting to the browser; 24-hour clock in both languages. **Q11**: the plugin follows the web language.
> **Rev. 2.1 (2026-10-23):** references updated for the passwordless S3 rev. 4 (no reset links; code challenges disable the switcher).
> **Rev. 2 changes:** Q23 applied (alternatives removed); plugin language via `welcome.locale` (ADR-0004).
> **Surfaces:** Web (Frontend), plugin language (Plugin Engineer, §6).

---

## 1. Supported locales

| Locale | Label in the switcher (always in its own language) | `lang` | Number/date locale |
|---|---|---|---|
| `pt-BR` | Português (Brasil) | `pt-BR` | `pt-BR` |
| `en` | English | `en` | `en-US` (dates/numbers), 24 h (Q23) |

No flags (languages are not countries). Adding a locale later is a configuration + catalog change, not a layout change.

## 2. Which language is shown (resolution order)

1. **Signed in:** the profile's language. A cookie mirrors it (`NEXT_LOCALE`) so the middleware can redirect without calling the Core.
2. **Not signed in, choice made before on this device:** the `NEXT_LOCALE` cookie (1 year).
3. **First visit (Q23):** browser `Accept-Language`: first `pt*` → `pt-BR`, first `en*` → `en`, anything else → `pt-BR`.
4. Fallback: `pt-BR`.

- `/` and any unprefixed path redirect to the resolved locale, keeping path and query.
- A signed-in user who opens a link with the other prefix is redirected to their profile language (same path), so the persisted choice always wins (AC-1). Switching happens only through the switcher.
- **Exceptions:** the invitation link (S3-03) opens in the locale embedded in the link (chosen at invite creation). The person can still switch on that page **before** the code is requested. Sign-in code and new-device e-mails use the profile language (the invitation language for the sign-up code).
- On sign-in, if the profile language differs from the page's, the redirect after login lands in the profile language.

## 3. Language switcher

### Placements
| Where | Component | Notes |
|---|---|---|
| Public pages (login, invite, forgot/reset) | `DropdownMenu` trigger button, top-right: `Languages` icon + current label ("Português (Brasil)"; ≤ `sm`: "PT" / "EN" visually, full name as accessible name) | Next to the theme switcher |
| App: user menu (`DropdownMenu` in the top bar) | Sub-menu "Idioma" / "Language" with `DropdownMenuRadioGroup` | Quick access |
| App: Settings → Preferences `/[locale]/settings/preferences` | `Select` "Idioma da interface" / "Interface language" + help text (below) | Canonical place; also holds Time zone (§4) and Theme (F§4) |

### Behavior
1. The user picks a language. If signed in, the web calls `me.update_preferences { locale }` (**proposal for the contract**) and waits for success.
2. On success: set the cookie, navigate to the same path with the new prefix (`router.replace`, query string and hash kept, scroll position kept when the page structure allows).
3. While saving: the menu item shows a spinner; further picks are ignored.
4. On failure (signed in): stay in the current language; toast (in the current language) "Não foi possível salvar o idioma. Tente de novo." + error code. Nothing changes.
5. Not signed in: no API call; cookie + navigation only.
6. **Unsaved form data:** if the current page has a dirty form, an `AlertDialog` asks first: "Trocar o idioma recarrega a página e descarta as alterações não salvas." / "Changing the language reloads the page and discards unsaved changes." (`Continuar editando` focused / `Trocar idioma`). Open view-once dialogs (S3-09 step 2, the S3-08a invitation-link result step) **disable the switcher** until closed, so a secret is never lost by a language change. The same applies while a code challenge is pending in memory (S3-02, S3-03 step 2, S3-04, S3-05): the switcher is disabled with the tooltip "Troque o idioma antes de pedir o código." / "Change the language before requesting the code.", because a reload would drop the challenge.
7. After switching, focus goes to the page `h1`, and `<html lang>` and the document title update. Nothing is announced beyond the page change itself.
8. Values never change, only their formatting (AC-2, X-3). Example: `US$ 1.234,50` ⇄ `$1,234.50`; `23/10/2026 17:00` ⇄ `Oct 23, 2026, 17:00`.

### Preferences page help text
- Under language: "Também usado pelo plugin do NinjaTrader a partir da próxima conexão." / "Also used by the NinjaTrader plugin from its next connection." (AC-3, Q11). If ADR-0004 adds a live preference message, the text becomes "…imediatamente" / "…right away".
- States: loading = skeleton for the two selects; error loading = `ErrorState` with retry; save per field (no global Save button), with success toast "Preferência salva." / "Preference saved.".

## 4. Time zone preference (Q23)

- Field on the Preferences page: `Select` with search ("Fuso horário" / "Time zone"), IANA zones grouped by region, each shown as `(GMT-03:00) São Paulo` / `(GMT-05:00) Chicago`, current offset computed by the browser for display only.
- Default: the browser's zone, saved to the profile on first sign-in (the Core needs a `time_zone` profile field: contract item for the Architect). A small note shows when the browser zone differs from the saved one: "Seu navegador está em {zone}. Usar este fuso?" + `Usar` / `Use`.
- Every reset/session time shows its zone (F§6.4).

## 5. Message catalogs and translation rules (for Frontend)

- **Catalogs:** `web/messages/pt-BR.json`, `web/messages/en.json` (path per Frontend scaffold). Keys identical in both, checked in CI (root CLAUDE.md §5).
- **Key naming:** `<feature>.<screen>.<element>[.<variant>]`, e.g. `sources.create.title`, `sources.rotate.confirm`, `auth.login.error.invalid_credentials`. Shared: `common.actions.cancel`, `common.states.loading`. **Core error codes map 1:1** to `errors.<code>` (e.g. `errors.auth.invalid_credentials`).
- **ICU MessageFormat** for plurals and variables: `{count, plural, one {# contrato} other {# contratos}}`. Never build sentences by concatenation; never embed formatted numbers via string concatenation (use the F§6 formatters as arguments).
- **Unknown error code** from the Core: `errors.unknown` with the code as a variable ("Erro inesperado (código: {code})").
- **Text expansion:** layouts tolerate +35 % (en ↔ pt-BR). No fixed widths on buttons, badges or table headers; truncation only with a tooltip and never for status text.
- **Terminology:** F§5 glossary is binding for both catalogs and the plugin.
- Writing style pt-BR: direct, second person "você", sentence case, no exclamation marks except success confirmations, verbs on buttons (`Gerar nova URL`, not `Nova URL`). en: same tone, sentence case.

## 6. Plugin language (AC-3, Q11 decided: follows the web)

- The plugin displays the profile language received in `welcome` (ADR-0004 §3, `welcome` carries the user locale). It applies it at the next connection. ADR-0004 v1 has no live locale message, so the help text in §3 says "a partir da próxima conexão".
- Before the first pairing (no profile known): Windows UI language if `pt*` or `en*`, else pt-BR (S3-11).
- Plugin strings live in `AllWin.Adapter.Core` resources (ADR-0001 §7) and use the F§5 glossary. Number formats follow F§6 with the plugin locale (`decimal` values; NinjaTrader's own grids keep NinjaTrader's formatting).

## 7. Missing translations (AC-4)

| Environment | Behavior |
|---|---|
| Dev / QA / staging | A missing key renders a visible marker `⟦missing: sources.create.title⟧` (dashed `--status-danger` underline) and logs an error to the console. Playwright fails on any `⟦missing:` text or next-intl missing-message error |
| CI | Key-parity check (pt-BR = en) blocks the build; empty strings count as missing |
| Production | **[Proposal, Frontend/Architect to confirm]** Show the default-locale (pt-BR) text and report the missing key (no user data) through the normal error reporting. Never show the raw key |

The user never sees a key name (AC-4: "no silent fallback to the key name").

## 8. States summary

| Situation | UI |
|---|---|
| Switch in progress | Spinner in the menu item; other picks ignored |
| Switch failed (signed in) | Toast with retry, language unchanged |
| Dirty form | Confirmation `AlertDialog` (§3 step 6) |
| View-once dialog open | Switcher disabled with tooltip "Feche a janela atual para trocar o idioma" |
| Preferences loading / error | Skeleton / `ErrorState` with retry |
| Cookie blocked | Works for the current navigation; preference persists only through the profile (signed in) |

## 9. Accessibility

- Switcher trigger accessible name: "Idioma: Português (Brasil)" / "Language: English". Options have `lang` attributes so screen readers pronounce them correctly (WCAG 3.1.2).
- `<html lang>` always matches the rendered locale (3.1.1).
- Keyboard: menus follow the WAI-ARIA menu pattern from Radix (arrow keys, Enter, Esc returns focus to the trigger).

## 10. AC traceability

| AC | Where |
|---|---|
| AC-1 switch + persists across sessions | §2 (profile + cookie), §3 |
| AC-2 all text/errors/e-mails translated; locale formats, values unchanged | §3 step 8, §5, F§6; e-mail language in S3-07/S3-08a and the code e-mails |
| AC-3 plugin follows the web language | §3 help text, §6, S3-11 |
| AC-4 missing translation detectable, no key-name fallback | §7 |

## 11. Open questions (owner)

| ID | Question | Owner | Spec default |
|---|---|---|---|
| — | `me.update_preferences` with `locale`, `time_zone` in the profile contract | Architect | Proposed |
| — | Live language update to the plugin (not in ADR-0004 v1) | Architect | Next connection; a future minor version may add it |
| — | Production behavior for a missing key | Frontend / Architect | pt-BR text + error report |
