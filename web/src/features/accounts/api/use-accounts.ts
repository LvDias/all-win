"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import {
  accountKeys,
  archiveAccount,
  createAccount,
  getAccount,
  getArchiveImpact,
  getOptInRules,
  getRuleVersion,
  listAccounts,
  setLifecycle,
  setOptIn,
  updateAccount,
} from "./accounts-api";
import type { Account, AccountCreateBody, AccountUpdateBody, LifecycleState, OptInChoice, OptInKind } from "./types";

export const useAccounts = () => useQuery({ queryKey: accountKeys.list(), queryFn: ({ signal }) => listAccounts(signal) });

export const useAccount = (id: string) =>
  useQuery({ queryKey: accountKeys.detail(id), queryFn: ({ signal }) => getAccount(id, signal), retry: false });

export const useArchiveImpact = (id: string, enabled: boolean) =>
  useQuery({ queryKey: accountKeys.impact(id), queryFn: ({ signal }) => getArchiveImpact(id, signal), enabled, gcTime: 0 });

/** Rule versions for the current prop selection; refetched on every selection change (spec §5.2). */
export const useOptInRules = (presetVersionId: string | null, firmPlatform: string | null, enabled = true) =>
  useQuery({
    queryKey: accountKeys.rules(presetVersionId, firmPlatform),
    queryFn: ({ signal }) => getOptInRules({ presetVersionId, firmPlatform }, signal),
    enabled,
    retry: false,
  });

export const useRuleVersion = (id: string | undefined) =>
  useQuery({ queryKey: accountKeys.ruleVersion(id ?? ""), queryFn: ({ signal }) => getRuleVersion(id as string, signal), enabled: id !== undefined, staleTime: Infinity });

/** Writes return the account: put it in the cache and refresh the list. */
function useCacheAccount() {
  const queryClient = useQueryClient();
  return (account: Account) => {
    queryClient.setQueryData(accountKeys.detail(account.id), account);
    void queryClient.invalidateQueries({ queryKey: accountKeys.list() });
  };
}

export function useCreateAccount() {
  const cache = useCacheAccount();
  return useMutation({
    mutationFn: (v: { body: AccountCreateBody; idempotencyKey: string }) => createAccount(v.body, v.idempotencyKey),
    onSuccess: cache,
  });
}

export function useUpdateAccount(id: string) {
  const cache = useCacheAccount();
  return useMutation({ mutationFn: (body: AccountUpdateBody) => updateAccount(id, body), onSuccess: cache });
}

export function useSetLifecycle() {
  const cache = useCacheAccount();
  return useMutation({
    mutationFn: (v: { id: string; state: LifecycleState; rowVersion: number }) => setLifecycle(v.id, v.state, v.rowVersion),
    onSuccess: cache,
  });
}

export function useSetOptIn(id: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (v: { kind: OptInKind; choice: OptInChoice; ruleVersionId: string; stepUp?: { challenge_id: string; code: string } }) =>
      setOptIn(id, v.kind, { choice: v.choice, rule_version_id: v.ruleVersionId, step_up: v.stepUp }),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: accountKeys.detail(id) });
      void queryClient.invalidateQueries({ queryKey: accountKeys.list() });
    },
  });
}

export function useArchiveAccount() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => archiveAccount(id),
    onSuccess: (_void, id) => {
      queryClient.removeQueries({ queryKey: accountKeys.detail(id) });
      void queryClient.invalidateQueries({ queryKey: accountKeys.list() });
    },
  });
}
