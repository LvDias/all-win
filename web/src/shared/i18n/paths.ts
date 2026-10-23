import { isLocale, localeExemptSegments, type Locale } from "./config";

export function splitLocalePath(pathname: string): { locale: Locale | null; rest: string } {
  const [, first = "", ...others] = pathname.split("/");
  if (isLocale(first)) return { locale: first, rest: others.length > 0 ? `/${others.join("/")}` : "/" };
  return { locale: null, rest: pathname };
}

export function withLocale(locale: Locale, rest: string): string {
  return rest === "/" ? `/${locale}` : `/${locale}${rest.startsWith("/") ? rest : `/${rest}`}`;
}

export function isLocaleExempt(rest: string): boolean {
  const first = rest.split("/")[1] ?? "";
  return (localeExemptSegments as readonly string[]).includes(first);
}
