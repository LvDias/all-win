"use client";

import { useInfiniteQuery, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import { segment } from "@/shared/api/ids";
import type { Account, AuditSignalType, Instrument, ManualTradeFields, ManualTradePreview, Source, Trade, TradeOrigin, TradePage, TradesSummary } from "@/shared/api/types";

export const tradesQueryKey = ["trades"] as const;

export type TradeSort = "exit_time" | "pnl" | "duration";

/** What the list asks the Core for. `from`/`to` are exit-time instants (Q53); the arrays repeat as query parameters. */
export type TradeFilters = {
  from?: string;
  to?: string;
  accountIds: string[];
  sourceIds: string[];
  origins: TradeOrigin[];
  signalTypes: AuditSignalType[];
  /** Includes trades of archived accounts; the Core hides them by default (Q79). Part of the cursor binding. */
  archived: boolean;
};

export type TradeQuery = { filters: TradeFilters; sort: TradeSort; order: "asc" | "desc" };

function filterParams(filters: TradeFilters): URLSearchParams {
  const params = new URLSearchParams();
  if (filters.from) params.set("from", filters.from);
  if (filters.to) params.set("to", filters.to);
  for (const id of filters.accountIds) params.append("account_id", id);
  for (const id of filters.sourceIds) params.append("source_id", id);
  for (const origin of filters.origins) params.append("origin", origin);
  for (const type of filters.signalTypes) params.append("signal_type", type);
  if (filters.archived) params.set("include_archived", "true");
  return params;
}

const withQuery = (path: string, params: URLSearchParams) => (params.size > 0 ? `${path}?${params.toString()}` : path);

/** Cursor paging, 50 per page. The cursor is opaque and bound to the filters and the sort, so both are part of the key. */
export function useTrades(query: TradeQuery) {
  return useInfiniteQuery({
    queryKey: [...tradesQueryKey, "list", query],
    initialPageParam: undefined as string | undefined,
    queryFn: ({ pageParam, signal }) => {
      const params = filterParams(query.filters);
      params.set("sort", query.sort);
      params.set("order", query.order);
      params.set("limit", "50");
      if (pageParam) params.set("cursor", pageParam);
      return apiFetch<TradePage>(withQuery("/api/v1/trades", params), { signal });
    },
    getNextPageParam: (last) => last.next_cursor,
  });
}

/** Whole filtered set, from the Core (the same figures S22 uses). */
export function useTradesSummary(filters: TradeFilters) {
  return useQuery({
    queryKey: [...tradesQueryKey, "summary", filters],
    queryFn: ({ signal }) => apiFetch<TradesSummary>(withQuery("/api/v1/trades/summary", filterParams(filters)), { signal }),
  });
}

export function useTrade(id: string | null) {
  return useQuery({
    queryKey: [...tradesQueryKey, "one", id],
    enabled: id !== null,
    queryFn: ({ signal }) => apiFetch<Trade>(`/api/v1/trades/${segment(id ?? "")}`, { signal }),
  });
}

/** Accounts and sources feed the filters and the manual form. Archived accounts come from the trades themselves (`account_deleted`). */
export const useTradeAccounts = () => useQuery({ queryKey: ["accounts", "list"], queryFn: ({ signal }) => apiFetch<{ accounts: Account[] }>("/api/v1/accounts", { signal }) });
export const useTradeSources = () => useQuery({ queryKey: ["sources"], queryFn: ({ signal }) => apiFetch<Source[]>("/api/v1/sources?include_revoked=true", { signal }) });
export const useInstruments = () => useQuery({ queryKey: ["instruments"], queryFn: ({ signal }) => apiFetch<{ instruments: Instrument[] }>("/api/v1/instruments", { signal }), staleTime: 3_600_000 });

export type PreviewInput = Pick<ManualTradeFields, "account_id" | "instrument_id" | "direction" | "quantity" | "entry_time" | "entry_price" | "exit_time" | "exit_price"> & { commission?: string };

/** Stateless Core arithmetic: the browser never computes P&L, duration or commission (X-3). */
export function previewManualTrade(input: PreviewInput, signal?: AbortSignal): Promise<ManualTradePreview> {
  const params = new URLSearchParams({
    account_id: input.account_id,
    instrument_id: input.instrument_id,
    direction: input.direction,
    quantity: String(input.quantity),
    entry_time: input.entry_time,
    entry_price: input.entry_price,
    exit_time: input.exit_time,
    exit_price: input.exit_price,
  });
  if (input.commission) params.set("commission", input.commission);
  return apiFetch<ManualTradePreview>(`/api/v1/trade-preview?${params.toString()}`, { signal });
}

export function useCreateManualTrade() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ body, key }: { body: ManualTradeFields; key: string }) => apiFetch<Trade>("/api/v1/trades", { method: "POST", body, headers: { "Idempotency-Key": key } }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: tradesQueryKey }),
  });
}

export function useUpdateManualTrade(id: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: ManualTradeFields & { row_version: number }) => apiFetch<Trade>(`/api/v1/trades/${segment(id)}`, { method: "PUT", body }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: tradesQueryKey }),
  });
}

export function useDeleteManualTrade() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, rowVersion }: { id: string; rowVersion: number }) => apiFetch<void>(`/api/v1/trades/${segment(id)}?row_version=${rowVersion}`, { method: "DELETE" }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: tradesQueryKey }),
  });
}
