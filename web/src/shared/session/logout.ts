"use client";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import { useRouter } from "next/navigation";
import { toast } from "sonner";
import { ApiError, apiFetch } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { useAppLocale } from "@/shared/i18n/navigation";
import { loginPath } from "./redirects";

/**
 * Ends the session on the server. On success (or 401, already signed out) it drops every cached query and goes to
 * login with `signed_out`. If the request fails the session may still be alive: the cache is dropped anyway, nothing
 * claims the user signed out, and a retry is offered.
 */
export function useLogout() {
  const queryClient = useQueryClient();
  const router = useRouter();
  const locale = useAppLocale();
  const t = useTranslations();
  const mutation = useMutation({
    mutationFn: () => apiFetch<void>("/api/v1/auth/logout", { method: "POST" }),
    onSuccess: () => {
      queryClient.clear();
      router.replace(loginPath(locale, { reason: "signed_out" }));
    },
    onError: (error) => {
      queryClient.clear();
      if (error instanceof ApiError && error.status === 401) {
        router.replace(loginPath(locale, { reason: "signed_out" }));
        return;
      }
      toast.error(t("nav.signOutFailed"), { description: errorMessage(t, error).message, action: { label: t("common.actions.retry"), onClick: () => mutation.mutate() } });
    },
  });
  return mutation;
}
