"use client";

import { MutationCache, QueryCache, QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useLocale } from "next-intl";
import { useRouter } from "next/navigation";
import { useEffect, useState, type ReactNode } from "react";
import { ApiError } from "@/shared/api/client";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { EventsProvider } from "@/shared/session/events";
import { notifySessionExpired, onSessionExpired } from "@/shared/session/expiry";
import { loginPath } from "@/shared/session/redirects";
import { StepUpProvider } from "@/shared/session/step-up";
import { LocaleSwitchProvider } from "@/shared/i18n/locale-switch";
import { LocaleSync } from "@/shared/i18n/locale-sync";
import { Toaster } from "@/shared/ui/sonner";
import { TooltipProvider } from "@/shared/ui/tooltip";

/** Any `auth.unauthenticated` answer means the session is gone: forget the user and go to login (F§10). */
export function createQueryClient(): QueryClient {
  const handle = (error: unknown) => {
    if (error instanceof ApiError && error.code === "auth.unauthenticated" && !window.location.pathname.includes("/login")) {
      // A new session must never see the previous one's data: drop everything, then go to login.
      client.clear();
      notifySessionExpired();
    }
  };
  const client: QueryClient = new QueryClient({
    queryCache: new QueryCache({ onError: handle }),
    mutationCache: new MutationCache({ onError: handle }),
    // Mutation results can carry secrets (webhook URLs, pairing codes, invite links): they are dropped as soon as they settle.
    defaultOptions: { queries: { refetchOnWindowFocus: false }, mutations: { gcTime: 0 } },
  });
  return client;
}

export function Providers({ children }: { children: ReactNode }) {
  const router = useRouter();
  const current = useLocale();
  const locale = isLocale(current) ? current : defaultLocale;
  useEffect(
    () =>
      onSessionExpired(() => {
        const returnTo = `${window.location.pathname}${window.location.search}`;
        router.replace(loginPath(locale, { reason: "session_expired", returnTo }));
      }),
    [router, locale],
  );

  const [queryClient] = useState(() => createQueryClient());
  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider>
        <LocaleSwitchProvider>
          <LocaleSync />
          <StepUpProvider>
            <EventsProvider>{children}</EventsProvider>
          </StepUpProvider>
          <Toaster />
        </LocaleSwitchProvider>
      </TooltipProvider>
    </QueryClientProvider>
  );
}
