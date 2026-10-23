"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { ApiError, apiFetch } from "@/shared/api/client";
import type { Me, UpdateMeBody } from "@/shared/api/types";

export const meQueryKey = ["me"] as const;

/** The signed-in user, or `null` when there is no session (401). */
export function useMe() {
  return useQuery({
    queryKey: meQueryKey,
    queryFn: async ({ signal }): Promise<Me | null> => {
      try {
        return await apiFetch<Me>("/api/v1/me", { signal });
      } catch (error) {
        if (error instanceof ApiError && error.status === 401) return null;
        throw error;
      }
    },
    retry: false,
    staleTime: 60_000,
  });
}

export function useUpdateMe() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: UpdateMeBody) => apiFetch<Me>("/api/v1/me", { method: "PATCH", body }),
    onSuccess: (me) => queryClient.setQueryData(meQueryKey, me),
  });
}
