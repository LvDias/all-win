"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { ApiError, apiFetch } from "@/shared/api/client";
import type { Invitation, StepUpProof } from "@/shared/api/types";

export const invitationsKey = ["admin", "invitations"] as const;

import type { InvitationCreated } from "@/shared/api/types";

export type { InvitationCreated };

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
    mutationFn: ({ proof, ...body }: { email: string; locale: "pt-BR" | "en"; proof?: StepUpProof }) => apiFetch<InvitationCreated>("/api/v1/admin/invitations", { method: "POST", body: { ...body, send_email: true, step_up: proof } }),
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
