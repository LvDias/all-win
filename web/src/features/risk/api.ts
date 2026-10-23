"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import type { Account, AccountRisk, AccountRiskRules, RiskRulesPreview, RiskRulesRequest, RiskSummaryList } from "@/shared/api/types";

/** Every risk query lives under ["risk"], so one invalidation (SSE `risk.state_changed`, reconnect) refreshes them all. */
export const riskQueryKey = ["risk"] as const;

export function useAccountRisk(accountId: string) {
  return useQuery({
    queryKey: [...riskQueryKey, "state", accountId],
    queryFn: ({ signal }) => apiFetch<AccountRisk>(`/api/v1/accounts/${accountId}/risk`, { signal }),
  });
}

/** One summary per live account, for the account list column (S13-03). */
export function useRiskSummaries() {
  return useQuery({
    queryKey: [...riskQueryKey, "summaries"],
    queryFn: ({ signal }) => apiFetch<RiskSummaryList>("/api/v1/risk-summaries", { signal }),
  });
}

export function useAccountRiskRules(accountId: string) {
  return useQuery({
    queryKey: [...riskQueryKey, "rules", accountId],
    queryFn: ({ signal }) => apiFetch<AccountRiskRules>(`/api/v1/accounts/${accountId}/risk-rules`, { signal }),
  });
}

/** Validate only: stores and audits nothing. */
export const usePreviewRiskRules = (accountId: string) =>
  useMutation({ mutationFn: (body: RiskRulesRequest) => apiFetch<RiskRulesPreview>(`/api/v1/accounts/${accountId}/risk-rules/preview`, { method: "POST", body }) });

export function useSaveRiskRules(accountId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: RiskRulesRequest) => apiFetch<AccountRiskRules>(`/api/v1/accounts/${accountId}/risk-rules`, { method: "PUT", body }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: riskQueryKey }),
  });
}

/** The account's `row_version`, needed by `restartEvaluation`. */
export function useAccountVersion(accountId: string, enabled: boolean) {
  return useQuery({
    queryKey: ["accounts", accountId, "version"],
    enabled,
    queryFn: ({ signal }) => apiFetch<Account>(`/api/v1/accounts/${accountId}`, { signal }),
  });
}

export function useRestartEvaluation(accountId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: { evaluation_start: string; evaluation_earned_before?: string; row_version: number }) =>
      apiFetch<Account>(`/api/v1/accounts/${accountId}/evaluation/restart`, { method: "POST", body }),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: riskQueryKey });
      void queryClient.invalidateQueries({ queryKey: ["accounts", accountId] });
    },
  });
}
