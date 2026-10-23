"use client";

import { useQuery } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import type { SecurityState } from "@/shared/api/types";

export const securityQueryKey = ["me", "security"] as const;

export function useSecurity(options?: { enabled?: boolean }) {
  return useQuery({
    queryKey: securityQueryKey,
    queryFn: ({ signal }) => apiFetch<SecurityState>("/api/v1/me/security", { signal }),
    enabled: options?.enabled ?? true,
    staleTime: 0,
  });
}
