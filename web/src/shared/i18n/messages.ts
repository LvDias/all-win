import { defaultLocale, type Locale } from "./config";

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

export async function loadMessages(locale: Locale): Promise<Messages> {
  const own = ((await import(`../../../messages/${locale}.json`)) as { default: Messages }).default;
  if (locale === defaultLocale) return own;
  const fallback = ((await import(`../../../messages/${defaultLocale}.json`)) as { default: Messages }).default;
  return mergeMessages(fallback, own);
}
