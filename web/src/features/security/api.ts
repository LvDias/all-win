"use client";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import type { RecoveryCodes, TotpSetup } from "@/shared/api/types";
import { securityQueryKey } from "@/shared/session/security";

const post = <T>(path: string, body?: unknown) => apiFetch<T>(path, { method: "POST", body });

export type { TotpSetup } from "@/shared/api/types";

export const useStartTotpSetup = () => useMutation({ mutationFn: () => post<TotpSetup>("/api/v1/me/totp/setup") });

export function useConfirmTotpSetup() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (code: string) => post<RecoveryCodes>("/api/v1/me/totp/confirm", { code }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: securityQueryKey }),
  });
}

export function useDisableTotp() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: { password: string; code: string }) => post<void>("/api/v1/me/totp/disable", body),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: securityQueryKey }),
  });
}

export function useRegenerateRecoveryCodes() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: { password: string; code: string }) => post<RecoveryCodes>("/api/v1/me/recovery-codes/regenerate", body),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: securityQueryKey }),
  });
}

export const useChangePassword = () => useMutation({ mutationFn: (body: { current_password: string; new_password: string }) => post<void>("/api/v1/me/password", body) });
