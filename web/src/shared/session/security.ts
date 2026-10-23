"use client";

import { useQuery } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import type { SecurityState } from "@/shared/api/types";

export const securityQueryKey = ["me", "security"] as const;

/** Channels, session limits and the pending channel change of the signed-in user. */
export function useSecurity(enabled = true) {
  return useQuery({ queryKey: securityQueryKey, queryFn: ({ signal }) => apiFetch<SecurityState>("/api/v1/me/security", { signal }), enabled, staleTime: 30_000 });
}
