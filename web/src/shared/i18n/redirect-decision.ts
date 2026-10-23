import { isLocale, type Locale } from "./config";
import { isLocaleExempt, splitLocalePath, withLocale } from "./paths";
import { resolveFromAcceptLanguage } from "./resolve-locale";

export type RedirectInput = {
  pathname: string;
  search: string;
  cookieLocale: string | undefined;
  hasSession: boolean;
  acceptLanguage: string | null;
};

/**
 * Returns the path to redirect to, or null when the request is served as is (spec V1-S4 §2).
 * Unprefixed paths go to the resolved locale. A signed-in user (session cookie) whose
 * persisted locale differs from the prefix is redirected to the persisted one, except on
 * invite and reset links. Visiting never writes the cookie; only the switcher does.
 */
export function decideRedirect(input: RedirectInput): string | null {
  const { locale, rest } = splitLocalePath(input.pathname);
  const preferred: Locale | null = isLocale(input.cookieLocale) ? input.cookieLocale : null;

  if (locale === null) {
    const target = preferred ?? resolveFromAcceptLanguage(input.acceptLanguage);
    return `${withLocale(target, rest)}${input.search}`;
  }
  if (input.hasSession && preferred !== null && preferred !== locale && !isLocaleExempt(rest)) {
    return `${withLocale(preferred, rest)}${input.search}`;
  }
  return null;
}
