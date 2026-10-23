import { startOfDayInstant, zonedToInstant } from "./datetime";

export type Period = "today" | "7d" | "30d" | "custom";

export const periods: readonly Period[] = ["today", "7d", "30d", "custom"];
const daysBack: Record<Exclude<Period, "custom">, number> = { today: 0, "7d": 7, "30d": 30 };

export const isDateText = (value: string | null): value is string => value !== null && /^\d{4}-\d{2}-\d{2}$/.test(value);

/**
 * The exit-time window of a period filter, as UTC instants, in the user's display zone (Q53). The interval is half-open
 * `[from, to)`: a custom period ending on day D sends D+1 at 00:00 local as `to`. S21 and S22 both use this, so the same
 * filter selects the same trades on both screens (S22 AC-2).
 */
export function periodWindow(period: Period, fromDate: string, toDate: string, now: Date, timeZone: string): { from?: string; to?: string } {
  if (period !== "custom") return { from: startOfDayInstant(now, daysBack[period], timeZone) };
  const window: { from?: string; to?: string } = {};
  if (fromDate) window.from = zonedToInstant(fromDate, "00:00:00", timeZone) ?? undefined;
  if (toDate) {
    const [y, m, d] = toDate.split("-").map(Number) as [number, number, number];
    const next = new Date(Date.UTC(y, m - 1, d + 1));
    window.to = zonedToInstant(`${next.getUTCFullYear()}-${String(next.getUTCMonth() + 1).padStart(2, "0")}-${String(next.getUTCDate()).padStart(2, "0")}`, "00:00:00", timeZone) ?? undefined;
  }
  return window;
}
