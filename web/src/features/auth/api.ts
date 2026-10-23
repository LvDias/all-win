"use client";

import { useMutation } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import type { AcceptInvitationBody, InvitationLookup, LoginResult } from "@/shared/api/types";

const post = <T>(path: string, body?: unknown) => apiFetch<T>(path, { method: "POST", body });

export const useLogin = () => useMutation({ mutationFn: (body: { email: string; password: string; remember_me: boolean }) => post<LoginResult>("/api/v1/auth/login", body) });
export const useVerifyTotp = () => useMutation({ mutationFn: (code: string) => post<LoginResult>("/api/v1/auth/login/totp", { code }) });
export const useVerifyRecoveryCode = () => useMutation({ mutationFn: (code: string) => post<LoginResult>("/api/v1/auth/login/recovery-code", { code }) });
export const useLookupInvitation = () => useMutation({ mutationFn: (token: string) => post<InvitationLookup>("/api/v1/auth/invitations/lookup", { token }) });
export const useAcceptInvitation = () =>
  useMutation({ mutationFn: (body: AcceptInvitationBody) => post<LoginResult>("/api/v1/auth/invitations/accept", body) });
export const useRequestPasswordReset = () => useMutation({ mutationFn: (email: string) => post<void>("/api/v1/auth/password-reset/request", { email }) });
export const useConfirmPasswordReset = () => useMutation({ mutationFn: (body: { token: string; new_password: string }) => post<void>("/api/v1/auth/password-reset/confirm", body) });
