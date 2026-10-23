export const locales = ["pt-BR", "en"] as const;
export type Locale = (typeof locales)[number];

export const defaultLocale: Locale = "pt-BR";
export const localeCookie = "NEXT_LOCALE";
export const sessionCookie = "__Host-aw_session";
export const localeCookieMaxAge = 60 * 60 * 24 * 365;

/** Locale used by `Intl` for numbers and dates. English uses en-US with a 24-hour clock (Q23). */
export const intlLocales: Record<Locale, string> = { "pt-BR": "pt-BR", en: "en-US" };

/** Names are always shown in their own language; `short` is the compact label below `sm`. */
export const localeNames: Record<Locale, { name: string; short: string }> = {
  "pt-BR": { name: "Português (Brasil)", short: "PT" },
  en: { name: "English", short: "EN" },
};

/** Pages that open in the locale embedded in the link (invitation and password reset). */
export const localeExemptSegments = ["invite", "reset-password"] as const;

export function isLocale(value: unknown): value is Locale {
  return typeof value === "string" && (locales as readonly string[]).includes(value);
}

/** Missing-translation marker is enabled for dev, QA and staging builds (spec V1-S4 §7). */
export function isMarkerMode(): boolean {
  return process.env.NEXT_PUBLIC_I18N_MISSING === "marker" || process.env.NODE_ENV === "development";
}
