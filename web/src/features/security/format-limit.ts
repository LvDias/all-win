import { intlLocales, type Locale } from "@/shared/i18n/config";

/** "12 horas" / "7 dias" for session limits; the values come from the Core. */
export function formatLimit(seconds: number, locale: Locale): string {
  const days = seconds >= 86400 && seconds % 86400 === 0;
  return new Intl.NumberFormat(intlLocales[locale], { style: "unit", unit: days ? "day" : "hour", unitDisplay: "long" }).format(days ? seconds / 86400 : Math.round(seconds / 3600));
}
