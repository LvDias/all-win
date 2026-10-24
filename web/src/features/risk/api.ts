"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import { accountKeys } from "@/shared/api/account-keys";
import { segment } from "@/shared/api/ids";
import { accountUpdateBody } from "./account-update";
import type { Account, AccountRisk, AccountRiskRules, RiskRulesPreview, RiskRulesRequest, RiskSummaryList } from "@/shared/api/types";

/** Every risk query lives under ["risk"], so one invalidation (SSE `risk.state_changed`, reconnect) refreshes them all. */
export const riskQueryKey = ["risk"] as const;

export function useAccountRisk(accountId: string) {
  return useQuery({
    queryKey: [...riskQueryKey, "state", accountId],
    queryFn: ({ signal }) => apiFetch<AccountRisk>(`/api/v1/accounts/${segment(accountId)}/risk`, { signal }),
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
    queryFn: ({ signal }) => apiFetch<AccountRiskRules>(`/api/v1/accounts/${segment(accountId)}/risk-rules`, { signal }),
  });
}

/** Validate only: stores and audits nothing. */
export const usePreviewRiskRules = (accountId: string) =>
  useMutation({ mutationFn: (body: RiskRulesRequest) => apiFetch<RiskRulesPreview>(`/api/v1/accounts/${segment(accountId)}/risk-rules/preview`, { method: "POST", body }) });

/** No invalidation here: the form saves rules and commission in two calls and refreshes once both are done. */
export const useSaveRiskRules = (accountId: string) =>
  useMutation({ mutationFn: (body: RiskRulesRequest) => apiFetch<AccountRiskRules>(`/api/v1/accounts/${segment(accountId)}/risk-rules`, { method: "PUT", body }) });

/** Changes the commission floor through `updateAccount` (U20 a): a higher floor applies at once, a lower one from the next trading day. */
export const useUpdateCommission = (accountId: string) =>
  useMutation({
    mutationFn: ({ account, commission }: { account: Account; commission: string | null }) =>
      apiFetch<Account>(`/api/v1/accounts/${segment(accountId)}`, { method: "PUT", body: accountUpdateBody(account, commission) }),
  });

/** Refresh everything that shows risk or the account after the edit. */
export function useRefreshAfterEdit() {
  const queryClient = useQueryClient();
  return () => {
    void queryClient.invalidateQueries({ queryKey: riskQueryKey });
    void queryClient.invalidateQueries({ queryKey: accountKeys.all });
  };
}

/** The account, for its `row_version` and its commission floor. Shares the one account key family (`accountKeys`). */
export function useAccount(accountId: string, enabled = true) {
  return useQuery({
    queryKey: accountKeys.detail(accountId),
    enabled,
    queryFn: ({ signal }) => apiFetch<Account>(`/api/v1/accounts/${segment(accountId)}`, { signal }),
  });
}

export function useRestartEvaluation(accountId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: { evaluation_start: string; evaluation_earned_before?: string; row_version: number }) =>
      apiFetch<Account>(`/api/v1/accounts/${segment(accountId)}/evaluation/restart`, { method: "POST", body }),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: riskQueryKey });
      void queryClient.invalidateQueries({ queryKey: accountKeys.all });
    },
  });
}
