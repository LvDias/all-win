import { defaultLocale, isMarkerMode, type Locale } from "./config";

export type Messages = { [key: string]: string | Messages };

/** Overlays `over` on `base`; empty strings count as missing. */
export function mergeMessages(base: Messages, over: Messages): Messages {
  const out: Messages = { ...base };
  for (const [key, value] of Object.entries(over)) {
    const current = out[key];
    if (typeof value === "string") {
      if (value.trim() !== "") out[key] = value;
    } else {
      out[key] = mergeMessages(typeof current === "object" ? current : {}, value);
    }
  }
  return out;
}

/**
 * Marker mode (dev/QA/staging) keeps the locale's own catalog so a missing key shows `⟦missing: …⟧`.
 * Production overlays it on pt-BR, so a missing key reads as pt-BR text instead of a blank (spec V1-S4 §7).
 */
export function buildCatalog(own: Messages, fallback: Messages, markerMode: boolean): Messages {
  return markerMode ? own : mergeMessages(fallback, own);
}

export async function loadMessages(locale: Locale): Promise<Messages> {
  const own = ((await import(`../../../messages/${locale}.json`)) as { default: Messages }).default;
  if (locale === defaultLocale) return own;
  const fallback = ((await import(`../../../messages/${defaultLocale}.json`)) as { default: Messages }).default;
  return buildCatalog(own, fallback, isMarkerMode());
}
