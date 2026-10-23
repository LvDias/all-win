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

/** "3 min ago" / "há 3 min": coarse relative time for status ages. Use `formatDateTime` for the exact value. */
export function formatRelative(value: Instant, locale: Locale, now: Date = new Date()): string {
  const seconds = Math.round((toDate(value).getTime() - now.getTime()) / 1000);
  const formatter = new Intl.RelativeTimeFormat(intlLocales[locale], { numeric: "auto", style: "short" });
  const abs = Math.abs(seconds);
  if (abs < 60) return formatter.format(seconds, "second");
  if (abs < 3600) return formatter.format(Math.round(seconds / 60), "minute");
  if (abs < 86400) return formatter.format(Math.round(seconds / 3600), "hour");
  return formatter.format(Math.round(seconds / 86400), "day");
}

function zoneParts(ms: number, timeZone: string) {
  const values = new Map<string, string>(
    new Intl.DateTimeFormat("en-CA", { timeZone, hourCycle: "h23", year: "numeric", month: "2-digit", day: "2-digit", hour: "2-digit", minute: "2-digit", second: "2-digit" })
      .formatToParts(new Date(ms))
      .map((part): [string, string] => [part.type, part.value]),
  );
  const get = (type: string) => Number(values.get(type));
  return { y: get("year"), mo: get("month"), d: get("day"), h: get("hour"), mi: get("minute"), s: get("second") };
}

/** Offset of `timeZone` from UTC at an instant, in milliseconds. */
function zoneOffset(ms: number, timeZone: string): number {
  const p = zoneParts(ms, timeZone);
  return Date.UTC(p.y, p.mo - 1, p.d, p.h, p.mi, p.s) - Math.floor(ms / 1000) * 1000;
}

/**
 * A wall-clock date (`2026-10-23`) and time (`14:31` or `14:31:05`) typed in `timeZone` → the UTC instant (ISO, `Z`).
 * Returns null when the text is not a real date or time. Times skipped or repeated by a DST change resolve to the earlier instant.
 */
export function zonedToInstant(date: string, time: string, timeZone: string): string | null {
  const d = /^(\d{4})-(\d{2})-(\d{2})$/.exec(date);
  const t = /^([01]\d|2[0-3]):([0-5]\d)(?::([0-5]\d))?$/.exec(time);
  if (!d || !t) return null;
  const [y, mo, day] = [Number(d[1]), Number(d[2]), Number(d[3])];
  const [h, mi, s] = [Number(t[1]), Number(t[2]), Number(t[3] ?? 0)];
  const wall = Date.UTC(y, mo - 1, day, h, mi, s);
  const check = new Date(wall);
  if (check.getUTCFullYear() !== y || check.getUTCMonth() !== mo - 1 || check.getUTCDate() !== day) return null;
  let guess = wall - zoneOffset(wall, timeZone);
  guess = wall - zoneOffset(guess, timeZone);
  return new Date(guess).toISOString().replace(".000Z", "Z");
}

/** The inverse: an instant → the date (`YYYY-MM-DD`) and time (`HH:MM:SS`) shown in `timeZone`, for form fields. */
export function instantToZoned(value: Instant, timeZone: string): { date: string; time: string } {
  const p = zoneParts(toDate(value).getTime(), timeZone);
  const two = (n: number) => String(n).padStart(2, "0");
  return { date: `${String(p.y).padStart(4, "0")}-${two(p.mo)}-${two(p.d)}`, time: `${two(p.h)}:${two(p.mi)}:${two(p.s)}` };
}

/** Start of the calendar day that is `daysBack` days before `now`, in `timeZone`, as a UTC instant. */
export function startOfDayInstant(now: Date, daysBack: number, timeZone: string): string {
  const today = instantToZoned(now, timeZone).date;
  const [y, mo, d] = today.split("-").map(Number) as [number, number, number];
  const back = new Date(Date.UTC(y, mo - 1, d - daysBack));
  const date = `${back.getUTCFullYear()}-${String(back.getUTCMonth() + 1).padStart(2, "0")}-${String(back.getUTCDate()).padStart(2, "0")}`;
  return zonedToInstant(date, "00:00:00", timeZone) ?? now.toISOString();
}
