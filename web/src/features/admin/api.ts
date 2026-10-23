"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { ApiError, apiFetch } from "@/shared/api/client";
import type { AdminUser, Invitation } from "@/shared/api/types";

export const invitationsKey = ["admin", "invitations"] as const;
export const usersKey = ["admin", "users"] as const;

import type { InvitationCreated, ResetLink } from "@/shared/api/types";

export type { InvitationCreated, ResetLink };

/** `auth.forbidden` on an admin call is shown as the generic not-found screen (the route is not revealed). */
export function isForbidden(error: unknown): boolean {
  return error instanceof ApiError && error.code === "auth.forbidden";
}

export function useInvitations() {
  return useQuery({ queryKey: invitationsKey, queryFn: ({ signal }) => apiFetch<Invitation[]>("/api/v1/admin/invitations", { signal }), retry: false });
}

export function useCreateInvitation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: { email: string; locale: "pt-BR" | "en" }) => apiFetch<InvitationCreated>("/api/v1/admin/invitations", { method: "POST", body: { ...body, send_email: true } }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: invitationsKey }),
  });
}

export function useRevokeInvitation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => apiFetch<void>(`/api/v1/admin/invitations/${id}/revoke`, { method: "POST" }),
    onSettled: () => queryClient.invalidateQueries({ queryKey: invitationsKey }),
  });
}

export function useUsers() {
  return useQuery({ queryKey: usersKey, queryFn: ({ signal }) => apiFetch<AdminUser[]>("/api/v1/admin/users", { signal }), retry: false });
}

export const useCreateResetLink = () => useMutation({ mutationFn: (userId: string) => apiFetch<ResetLink>(`/api/v1/admin/users/${userId}/password-reset-link`, { method: "POST" }) });

export function useResetTotp() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (userId: string) => apiFetch<void>(`/api/v1/admin/users/${userId}/totp-reset`, { method: "POST" }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: usersKey }),
  });
}
