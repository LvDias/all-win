"use client";

import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { useCallback, useMemo, useState } from "react";
import { isDateText, periods, periodWindow, type Period } from "@/shared/format";
import { useDisplay } from "@/shared/session/display";
import type { GroupBy, PerformanceQuery, PerformanceSort } from "./api";

const groups: readonly GroupBy[] = ["source_signal", "source", "signal_type"];
const sorts: readonly PerformanceSort[] = ["total_pnl", "trade_count", "avg_pnl", "win_rate"];

export type UiFilters = { period: Period; fromDate: string; toDate: string; accounts: string[]; archived: boolean };

/** The same URL parameters and the same exit-time window as the trade list (S21), so a filter selects the same trades on both. */
export function usePerformanceFilters() {
  const params = useSearchParams();
  const pathname = usePathname();
  const router = useRouter();
  const { timeZone } = useDisplay();
  const [now] = useState(() => new Date());
  const search = params.toString();

  const parsed = useMemo(() => {
    const p = new URLSearchParams(search);
    const rawPeriod = p.get("period");
    const rawGroup = p.get("group");
    const rawSort = p.get("sort");
    const fromParam = p.get("from");
    const toParam = p.get("to");
    return {
      period: periods.find((item) => item === rawPeriod) ?? "30d",
      groupBy: groups.find((item) => item === rawGroup) ?? "source_signal",
      sort: sorts.find((item) => item === rawSort) ?? "total_pnl",
      order: p.get("order") === "asc" ? ("asc" as const) : ("desc" as const),
      fromDate: isDateText(fromParam) ? fromParam : "",
      toDate: isDateText(toParam) ? toParam : "",
      accounts: p.getAll("account"),
      archived: p.get("archived") === "1",
    };
  }, [search]);
  const { period, groupBy, sort, order, fromDate, toDate, accounts, archived } = parsed;
  const ui: UiFilters = { period, fromDate, toDate, accounts, archived };

  const query = useMemo<PerformanceQuery>(() => {
    const { from, to } = periodWindow(period, fromDate, toDate, now, timeZone);
    return { from, to, accountIds: accounts, archived, groupBy, sort, order };
  }, [period, fromDate, toDate, accounts, archived, groupBy, sort, order, now, timeZone]);

  const update = useCallback(
    (patch: Partial<UiFilters> & { groupBy?: GroupBy; sort?: PerformanceSort; order?: "asc" | "desc" }) => {
      const next = new URLSearchParams(search);
      const setOrDelete = (key: string, value: string | undefined, drop: boolean) => {
        if (value === undefined) return;
        if (drop) next.delete(key);
        else next.set(key, value);
      };
      if (patch.period !== undefined) {
        setOrDelete("period", patch.period, patch.period === "30d");
        if (patch.period !== "custom") {
          next.delete("from");
          next.delete("to");
        }
      }
      setOrDelete("from", patch.fromDate, patch.fromDate === "");
      setOrDelete("to", patch.toDate, patch.toDate === "");
      if (patch.accounts) {
        next.delete("account");
        for (const id of patch.accounts) next.append("account", id);
      }
      if (patch.archived !== undefined) {
        if (patch.archived) next.set("archived", "1");
        else next.delete("archived");
      }
      setOrDelete("group", patch.groupBy, patch.groupBy === "source_signal");
      setOrDelete("sort", patch.sort, patch.sort === "total_pnl");
      setOrDelete("order", patch.order, patch.order === "desc");
      const target = next.toString();
      router.replace(target ? `${pathname}?${target}` : pathname);
    },
    [search, pathname, router],
  );

  const clear = useCallback(() => update({ period: "30d", accounts: [], archived: false }), [update]);
  const activeCount = (accounts.length > 0 ? 1 : 0) + (archived ? 1 : 0) + (period !== "30d" ? 1 : 0);
  /** The filter as the trade list reads it: `?period=…&from=…&to=…&account=…`. */
  const tradesQuery = useMemo(() => {
    const p = new URLSearchParams();
    if (period !== "30d") p.set("period", period);
    if (fromDate) p.set("from", fromDate);
    if (toDate) p.set("to", toDate);
    for (const id of accounts) p.append("account", id);
    if (archived) p.set("archived", "1");
    return p;
  }, [period, fromDate, toDate, accounts, archived]);

  return { ui, query, groupBy, sort, order, update, clear, activeCount, tradesQuery };
}
