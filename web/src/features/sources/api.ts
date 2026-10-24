"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import type { Source, SourceWithUrl, StepUpProof } from "@/shared/api/types";

export const sourcesQueryKey = ["sources"] as const;

export function useSources(includeRevoked: boolean) {
  return useQuery({
    queryKey: [...sourcesQueryKey, { includeRevoked }],
    queryFn: ({ signal }) => apiFetch<Source[]>(`/api/v1/sources${includeRevoked ? "?include_revoked=true" : ""}`, { signal }),
  });
}

/** The create and rotate answers carry the full URL: they are returned to the caller only and never enter the query cache. */
export function useCreateSource() {
  return useMutation({ mutationFn: ({ name, proof }: { name: string; proof?: StepUpProof }) => apiFetch<SourceWithUrl>("/api/v1/sources", { method: "POST", body: { name, source_type: "tradingview", step_up: proof } }), gcTime: 0 });
}

export function useRotateSource() {
  return useMutation({ mutationFn: ({ id, proof }: { id: string; proof?: StepUpProof }) => apiFetch<SourceWithUrl>(`/api/v1/sources/${id}/rotate`, { method: "POST", body: { step_up: proof } }), gcTime: 0 });
}

export function useRenameSource() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, name }: { id: string; name: string }) => apiFetch<Source>(`/api/v1/sources/${id}`, { method: "PATCH", body: { name } }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: sourcesQueryKey }),
  });
}

export function useRevokeSource() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => apiFetch<void>(`/api/v1/sources/${id}/revoke`, { method: "POST" }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: sourcesQueryKey }),
  });
}
