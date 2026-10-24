import type { Locale } from "@/shared/i18n/config";

const parts = (zone: string, instant: number) =>
  new Intl.DateTimeFormat("en-US", { timeZone: zone, hourCycle: "h23", year: "numeric", month: "2-digit", day: "2-digit", hour: "2-digit", minute: "2-digit", second: "2-digit" }).formatToParts(new Date(instant));

/** Offset of `zone` from UTC at `instant`, in minutes. */
function offsetMinutes(zone: string, instant: number): number {
  const p = parts(zone, instant);
  const n = (type: string) => Number(p.find((part) => part.type === type)?.value);
  return Math.round((Date.UTC(n("year"), n("month") - 1, n("day"), n("hour"), n("minute"), n("second")) - Math.floor(instant / 1000) * 1000) / 60000);
}

/**
 * The firm's trading-day reset (`HH:MM` in the firm's zone) as the same moment on the clock of the user's zone, for display
 * only (F§6.4). The date used is `at`'s; null when a zone name is unknown to the browser or the time is malformed.
 */
export function resetInUserZone(localTime: string, firmZone: string, userZone: string, locale: Locale, at: Date): { firm: string; user: string } | null {
  const match = /^([01][0-9]|2[0-3]):([0-5][0-9])$/.exec(localTime);
  if (!match) return null;
  try {
    const guess = Date.UTC(at.getUTCFullYear(), at.getUTCMonth(), at.getUTCDate(), Number(match[1]), Number(match[2]));
    const instant = guess - offsetMinutes(firmZone, guess) * 60000;
    const format = (zone: string, style: "short" | "shortOffset") => new Intl.DateTimeFormat(locale, { timeZone: zone, hour: "2-digit", minute: "2-digit", timeZoneName: style }).format(new Date(instant));
    return { firm: format(firmZone, "short"), user: format(userZone, "shortOffset") };
  } catch {
    return null;
  }
}
