import { intlLocales, type Locale } from "@/shared/i18n/config";

/** "30 s" / "2 min" for waits such as `Retry-After`, in the active language. */
export function formatWait(seconds: number, locale: Locale): string {
  const minutes = seconds >= 60;
  return new Intl.NumberFormat(intlLocales[locale], { style: "unit", unit: minutes ? "minute" : "second", unitDisplay: "short" }).format(minutes ? Math.ceil(seconds / 60) : Math.max(1, Math.ceil(seconds)));
}

/** "9:41" countdown clock (minutes:seconds). */
export function formatClock(totalSeconds: number): string {
  const safe = Math.max(0, Math.floor(totalSeconds));
  return `${Math.floor(safe / 60)}:${String(safe % 60).padStart(2, "0")}`;
}
