"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import type { ChannelChange, Device } from "@/shared/api/types";
import { securityQueryKey } from "@/shared/session/security";

export const devicesQueryKey = ["me", "devices"] as const;

export const useDevices = () => useQuery({ queryKey: devicesQueryKey, queryFn: ({ signal }) => apiFetch<Device[]>("/api/v1/me/devices", { signal }) });

export function useForgetDevice() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => apiFetch<void>(`/api/v1/me/devices/${id}/forget`, { method: "POST" }),
    onSettled: () => queryClient.invalidateQueries({ queryKey: devicesQueryKey }),
  });
}

/** Ends every session, this one included; the caller leaves for the login page. */
export const signOutEverywhere = () => apiFetch<void>("/api/v1/auth/logout-everywhere", { method: "POST" });

export const startEmailChange = (newValue: string) => apiFetch<ChannelChange>("/api/v1/me/channel-changes", { method: "POST", body: { channel: "email", new_value: newValue } });
export const verifyChannelChangeCode = (changeId: string, body: { challenge_id: string; code: string }) =>
  apiFetch<ChannelChange>(`/api/v1/me/channel-changes/${changeId}/verify`, { method: "POST", body });

export function useCancelChannelChange() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (changeId: string) => apiFetch<void>(`/api/v1/me/channel-changes/${changeId}/cancel`, { method: "POST" }),
    onSettled: () => queryClient.invalidateQueries({ queryKey: securityQueryKey }),
  });
}

/** Active sources and paired connections, for the "sign out of all devices" reminder. Same cache keys as their own pages. */
export function useActiveCounts() {
  const sources = useQuery({ queryKey: ["sources", { includeRevoked: false }], queryFn: ({ signal }) => apiFetch<{ status: string }[]>("/api/v1/sources", { signal }) });
  const connections = useQuery({ queryKey: ["connections", { includeUnpaired: false }], queryFn: ({ signal }) => apiFetch<{ status: string }[]>("/api/v1/connections", { signal }) });
  return {
    sources: sources.data?.filter((source) => source.status === "active").length ?? 0,
    connections: connections.data?.filter((connection) => connection.status !== "awaiting_pairing" && connection.status !== "unpaired").length ?? 0,
  };
}
