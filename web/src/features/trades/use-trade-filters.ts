"use client";

import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { useCallback, useMemo, useState } from "react";
import type { AuditSignalType, TradeOrigin } from "@/shared/api/types";
import { startOfDayInstant, zonedToInstant } from "@/shared/format";
import { useDisplay } from "@/shared/session/display";
import type { TradeFilters, TradeQuery, TradeSort } from "./api";

export type Period = "today" | "7d" | "30d" | "custom";

const periods: readonly Period[] = ["today", "7d", "30d", "custom"];
const signalTypes: readonly AuditSignalType[] = ["trigger", "trade_plan", "nq_trigger", "master_account"];
const sorts: readonly TradeSort[] = ["exit_time", "pnl", "duration"];
const daysBack: Record<Exclude<Period, "custom">, number> = { today: 0, "7d": 7, "30d": 30 };
/** "Manual" and "Platform (no signal)" sit in the source filter but are origins for the Core (spec §3.1). */
const originSources: readonly TradeOrigin[] = ["manual", "platform"];

const isDate = (value: string | null): value is string => value !== null && /^\d{4}-\d{2}-\d{2}$/.test(value);

export type UiFilters = { period: Period; fromDate: string; toDate: string; accounts: string[]; sources: string[]; signals: AuditSignalType[]; archived: boolean };

/** Filters live in the URL (`?period=…&account=…&source=…&signal=…`), so a reload or a shared link shows the same list. */
export function useTradeFilters() {
  const params = useSearchParams();
  const pathname = usePathname();
  const router = useRouter();
  const { timeZone } = useDisplay();
  const [now] = useState(() => new Date());

  const search = params.toString();
  const parsed = useMemo(() => {
    const p = new URLSearchParams(search);
    const rawPeriod = p.get("period");
    const rawSort = p.get("sort");
    const fromParam = p.get("from");
    const toParam = p.get("to");
    return {
      period: periods.find((item) => item === rawPeriod) ?? "30d",
      sort: sorts.find((item) => item === rawSort) ?? "exit_time",
      order: p.get("order") === "asc" ? ("asc" as const) : ("desc" as const),
      fromDate: isDate(fromParam) ? fromParam : "",
      toDate: isDate(toParam) ? toParam : "",
      accounts: p.getAll("account"),
      sources: p.getAll("source"),
      archived: p.get("archived") === "1",
      signals: p.getAll("signal").filter((value): value is AuditSignalType => signalTypes.some((type) => type === value)),
    };
  }, [search]);
  const { period, sort, order, fromDate, toDate, accounts, sources, signals, archived } = parsed;

  const ui: UiFilters = { period, fromDate, toDate, accounts, sources, signals, archived };

  const filters = useMemo<TradeFilters>(() => {
    let from: string | undefined;
    let to: string | undefined;
    if (period === "custom") {
      if (fromDate) from = zonedToInstant(fromDate, "00:00:00", timeZone) ?? undefined;
      if (toDate) {
        const [y, m, d] = toDate.split("-").map(Number) as [number, number, number];
        const next = new Date(Date.UTC(y, m - 1, d + 1));
        to = zonedToInstant(`${next.getUTCFullYear()}-${String(next.getUTCMonth() + 1).padStart(2, "0")}-${String(next.getUTCDate()).padStart(2, "0")}`, "00:00:00", timeZone) ?? undefined;
      }
    } else {
      from = startOfDayInstant(now, daysBack[period], timeZone);
    }
    const origins = sources.filter((value): value is TradeOrigin => originSources.some((origin) => origin === value));
    return { from, to, accountIds: accounts, sourceIds: sources.filter((value) => !originSources.some((origin) => origin === value)), origins, signalTypes: signals, archived };
  }, [period, fromDate, toDate, timeZone, now, accounts, sources, signals, archived]);

  const query: TradeQuery = useMemo(() => ({ filters, sort, order }), [filters, sort, order]);

  const update = useCallback(
    (patch: Partial<UiFilters> & { sort?: TradeSort; order?: "asc" | "desc" }) => {
      const next = new URLSearchParams(search);
      const set = (key: string, values: readonly string[]) => {
        next.delete(key);
        for (const value of values) next.append(key, value);
      };
      if (patch.period !== undefined) {
        if (patch.period === "30d") next.delete("period");
        else next.set("period", patch.period);
        if (patch.period !== "custom") {
          next.delete("from");
          next.delete("to");
        }
      }
      const setOrDelete = (key: string, value: string | undefined, drop: boolean) => {
        if (value === undefined) return;
        if (drop) next.delete(key);
        else next.set(key, value);
      };
      setOrDelete("from", patch.fromDate, patch.fromDate === "");
      setOrDelete("to", patch.toDate, patch.toDate === "");
      if (patch.accounts) set("account", patch.accounts);
      if (patch.sources) set("source", patch.sources);
      if (patch.signals) set("signal", patch.signals);
      if (patch.archived !== undefined) {
        if (patch.archived) next.set("archived", "1");
        else next.delete("archived");
      }
      setOrDelete("sort", patch.sort, patch.sort === "exit_time");
      setOrDelete("order", patch.order, patch.order === "desc");
      const target = next.toString();
      router.replace(target ? `${pathname}?${target}` : pathname);
    },
    [search, pathname, router],
  );

  const clear = useCallback(() => update({ accounts: [], sources: [], signals: [], period: "30d", archived: false }), [update]);
  const activeCount = (accounts.length > 0 ? 1 : 0) + (sources.length > 0 ? 1 : 0) + (signals.length > 0 ? 1 : 0) + (archived ? 1 : 0) + (period !== "30d" ? 1 : 0);

  return { ui, filters, query, sort, order, update, clear, activeCount };
}
