"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import type { Connection, PairingCode } from "@/shared/api/types";

export const connectionsQueryKey = ["connections"] as const;

export function useConnections(includeUnpaired: boolean) {
  return useQuery({
    queryKey: [...connectionsQueryKey, { includeUnpaired }],
    queryFn: ({ signal }) => apiFetch<Connection[]>(`/api/v1/connections${includeUnpaired ? "?include_unpaired=true" : ""}`, { signal }),
  });
}

export type CreatedConnection = { connection: Connection; pairing: PairingCode };

/** Create and new-code answers carry the pairing code: returned to the caller only, never cached. */
export const useCreateConnection = () => useMutation({ mutationFn: (name: string) => apiFetch<CreatedConnection>("/api/v1/connections", { method: "POST", body: { name } }), gcTime: 0 });
export const useNewPairingCode = () => useMutation({ mutationFn: (id: string) => apiFetch<PairingCode>(`/api/v1/connections/${id}/pairing-code`, { method: "POST" }), gcTime: 0 });

export function useRenameConnection() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, name }: { id: string; name: string }) => apiFetch<Connection>(`/api/v1/connections/${id}`, { method: "PATCH", body: { name } }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: connectionsQueryKey }),
  });
}

export function useUnpairConnection() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, acknowledgeOpenPositions }: { id: string; acknowledgeOpenPositions: boolean }) =>
      apiFetch<void>(`/api/v1/connections/${id}/unpair`, { method: "POST", body: { acknowledge_open_positions: acknowledgeOpenPositions } }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: connectionsQueryKey }),
  });
}
