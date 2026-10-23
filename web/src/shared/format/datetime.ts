import { intlLocales, type Locale } from "@/shared/i18n/config";

type Instant = string | Date;

function parts(locale: Locale) {
  const intl = intlLocales[locale];
  return {
    date: (timeZone: string) =>
      new Intl.DateTimeFormat(intl, locale === "en" ? { year: "numeric", month: "short", day: "numeric", timeZone } : { year: "numeric", month: "2-digit", day: "2-digit", timeZone }),
    time: (timeZone: string, seconds: boolean, zone: boolean) =>
      new Intl.DateTimeFormat(intl, {
        hour: "2-digit",
        minute: "2-digit",
        ...(seconds ? { second: "2-digit" as const } : {}),
        hourCycle: "h23",
        ...(zone ? { timeZoneName: "short" as const } : {}),
        timeZone,
      }),
  };
}

function toDate(value: Instant): Date {
  const date = typeof value === "string" ? new Date(value) : value;
  if (Number.isNaN(date.getTime())) throw new RangeError(`Invalid instant: ${String(value)}`);
  return date;
}

/** `23/10/2026` · `Oct 23, 2026`, in the user's display time zone. */
export function formatDate(value: Instant, locale: Locale, timeZone: string): string {
  return parts(locale).date(timeZone).format(toDate(value));
}

/** `14:31` or `14:31:05`; 24-hour clock in both languages (Q23). `zone` appends the short zone name (F§6.4). */
export function formatTime(value: Instant, locale: Locale, timeZone: string, options: { seconds?: boolean; zone?: boolean } = {}): string {
  return parts(locale).time(timeZone, options.seconds ?? false, options.zone ?? false).format(toDate(value));
}

/** `23/10/2026 14:31:05` · `Oct 23, 2026, 14:31:05`. */
export function formatDateTime(value: Instant, locale: Locale, timeZone: string, options: { seconds?: boolean; zone?: boolean } = {}): string {
  const date = formatDate(value, locale, timeZone);
  const time = formatTime(value, locale, timeZone, options);
  return locale === "en" ? `${date}, ${time}` : `${date} ${time}`;
}
