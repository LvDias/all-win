import { localeCookie, localeCookieMaxAge, type Locale } from "./config";

/** Mirrors the chosen language so the proxy can redirect without calling the Core. Only the switcher writes it. */
export function writeLocaleCookie(locale: Locale): void {
  const secure = window.location.protocol === "https:" ? "; Secure" : "";
  document.cookie = `${localeCookie}=${locale}; Max-Age=${localeCookieMaxAge}; Path=/; SameSite=Lax${secure}`;
}
