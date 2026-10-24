"use client";

import { keepPreviousData, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { ApiError, apiFetch } from "@/shared/api/client";
import { segment } from "@/shared/api/ids";
import { accountKeys } from "@/shared/api/account-keys";
import type { components } from "@/shared/api/schema.gen";
import { presetKeys } from "./use-presets";

type Schemas = components["schemas"];
export type AccountPresetState = Schemas["AccountPresetState"];
export type PresetDiff = Schemas["PresetDiff"];
export type ApplyPresetVersionRequest = Schemas["ApplyPresetVersionRequest"];
export type RuleField = Schemas["RuleField"];

export const accountPresetKeys = {
  state: (accountId: string) => [...presetKeys.all, "account", accountId] as const,
  diff: (accountId: string, target: string) => [...presetKeys.all, "account", accountId, "diff", target] as const,
};

/** 404 means the account has no preset (free-text prop or real account): the panel simply does not exist. */
export const useAccountPreset = (accountId: string) =>
  useQuery({
    queryKey: accountPresetKeys.state(accountId),
    retry: (count, error) => !(error instanceof ApiError && error.status === 404) && count < 2,
    queryFn: ({ signal }) => apiFetch<AccountPresetState>(`/api/v1/accounts/${segment(accountId)}/preset`, { signal }),
  });

export const usePresetDiff = (accountId: string, target: string) =>
  useQuery({
    queryKey: accountPresetKeys.diff(accountId, target),
    // A diff belongs to one moment of the account and the preset: never reuse a stale one.
    gcTime: 0,
    staleTime: 0,
    // While the diff for a newer target loads, the dialog keeps showing the previous one (greyed, Apply disabled): no blank flash.
    placeholderData: keepPreviousData,
    queryFn: ({ signal }) => apiFetch<PresetDiff>(`/api/v1/accounts/${segment(accountId)}/preset-diff?target_preset_version_id=${encodeURIComponent(target)}`, { signal }),
  });

/** The account and its preset state change together: refresh both caches after a write. */
function useRefreshAccount(accountId: string) {
  const queryClient = useQueryClient();
  return (state?: AccountPresetState) => {
    if (state) queryClient.setQueryData(accountPresetKeys.state(accountId), state);
    else void queryClient.invalidateQueries({ queryKey: accountPresetKeys.state(accountId) });
    void queryClient.invalidateQueries({ queryKey: accountKeys.detail(accountId) });
    void queryClient.invalidateQueries({ queryKey: accountKeys.list() });
  };
}

export function useApplyPresetVersion(accountId: string) {
  const refresh = useRefreshAccount(accountId);
  return useMutation({
    mutationFn: (body: ApplyPresetVersionRequest) => apiFetch<AccountPresetState>(`/api/v1/accounts/${segment(accountId)}/preset-version`, { method: "POST", body }),
    onSuccess: (state) => refresh(state),
  });
}

export function useRestorePresetValue(accountId: string) {
  const refresh = useRefreshAccount(accountId);
  return useMutation({
    mutationFn: ({ field, rowVersion }: { field: RuleField; rowVersion: number }) =>
      apiFetch<AccountPresetState>(`/api/v1/accounts/${segment(accountId)}/preset-values/${segment(field)}/restore`, { method: "POST", body: { row_version: rowVersion } }),
    onSuccess: (state) => refresh(state),
    onError: () => refresh(),
  });
}
