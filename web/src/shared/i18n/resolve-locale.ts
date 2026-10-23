import { defaultLocale, type Locale } from "./config";

/** First `pt*` or `en*` tag by quality order wins; anything else is pt-BR (Q23). */
export function resolveFromAcceptLanguage(header: string | null | undefined): Locale {
  if (!header) return defaultLocale;
  const entries = header
    .split(",")
    .map((part, index) => {
      const [tag = "", ...params] = part.trim().split(";");
      const q = params.map((p) => p.trim()).find((p) => p.startsWith("q="));
      const quality = q ? Number(q.slice(2)) : 1;
      return { tag: tag.trim().toLowerCase(), quality: Number.isFinite(quality) ? quality : 0, index };
    })
    .filter((entry) => entry.tag !== "" && entry.quality > 0)
    .sort((a, b) => b.quality - a.quality || a.index - b.index);

  for (const { tag } of entries) {
    const primary = tag.split("-")[0];
    if (primary === "pt") return "pt-BR";
    if (primary === "en") return "en";
  }
  return defaultLocale;
}
