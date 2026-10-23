"use client";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { apiFetch } from "@/shared/api/client";
import { useAppLocale } from "@/shared/i18n/navigation";
import { loginPath } from "./redirects";

/** Ends the session on the server, drops every cached query, and goes to login (`signed_out`). */
export function useLogout() {
  const queryClient = useQueryClient();
  const router = useRouter();
  const locale = useAppLocale();
  return useMutation({
    mutationFn: () => apiFetch<void>("/api/v1/auth/logout", { method: "POST" }),
    onSettled: () => {
      queryClient.clear();
      router.replace(loginPath(locale, { reason: "signed_out" }));
    },
  });
}
