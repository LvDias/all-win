"use client";

import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import type { Performance, TradesSummary } from "@/shared/api/types";

export const performanceQueryKey = ["performance"] as const;

export type GroupBy = "source_signal" | "source" | "signal_type";
export type PerformanceSort = "total_pnl" | "trade_count" | "avg_pnl" | "win_rate";

export type PerformanceQuery = { from?: string; to?: string; accountIds: string[]; archived: boolean; groupBy: GroupBy; sort: PerformanceSort; order: "asc" | "desc" };

function params(query: PerformanceQuery): URLSearchParams {
  const p = new URLSearchParams({ group_by: query.groupBy, sort: query.sort, order: query.order });
  if (query.from) p.set("from", query.from);
  if (query.to) p.set("to", query.to);
  for (const id of query.accountIds) p.append("account_id", id);
  if (query.archived) p.set("include_archived", "true");
  return p;
}

/** One call for the whole page. The previous figures stay (dimmed by the caller) while a changed filter loads, so no old number looks current. */
export function usePerformance(query: PerformanceQuery) {
  return useQuery({
    queryKey: [...performanceQueryKey, query],
    placeholderData: keepPreviousData,
    queryFn: ({ signal }) => apiFetch<Performance>(`/api/v1/performance?${params(query).toString()}`, { signal }),
  });
}

/** Whether the user has any trade at all (archived accounts included), to tell "first use" from "this filter matches nothing". Only asked when the page is empty. */
export function useHasAnyTrade(enabled: boolean) {
  return useQuery({
    queryKey: ["trades", "any"],
    enabled,
    queryFn: ({ signal }) => apiFetch<TradesSummary>("/api/v1/trades/summary?include_archived=true", { signal }),
  });
}

export const useAccountsForFilter = () =>
  useQuery({ queryKey: ["accounts", "list"], queryFn: ({ signal }) => apiFetch<{ accounts: { id: string; display_name: string }[] }>("/api/v1/accounts", { signal }) });
