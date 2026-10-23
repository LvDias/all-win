import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render } from "@testing-library/react";
import { NextIntlClientProvider } from "next-intl";
import type { ReactElement } from "react";
import en from "../../../../messages/en.json";
import ptBR from "../../../../messages/pt-BR.json";
import { LocaleSwitchProvider } from "@/shared/i18n/locale-switch";
import { mergeMessages, type Messages } from "@/shared/i18n/messages";
import { StepUpProvider } from "@/shared/session/step-up";
import { server } from "@/test/server";
import { http, HttpResponse } from "msw";
import { Toaster } from "@/shared/ui/sonner";
import { TooltipProvider } from "@/shared/ui/tooltip";

import accountsEn from "../messages/en.json";
import accountsPtBR from "../messages/pt-BR.json";

// Overlay: the S5 stepper keys not yet merged into messages/*.json (Vitral owns the catalogs).
const catalogs = { "pt-BR": mergeMessages(ptBR as Messages, accountsPtBR), en: mergeMessages(en as Messages, accountsEn) };

const me = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };

export const nav = { push: vi.fn(), replace: vi.fn(), refresh: vi.fn(), pathname: "/pt-BR/accounts", search: "" };

vi.mock("next/navigation", () => ({
  useRouter: () => ({ push: nav.push, replace: nav.replace, refresh: nav.refresh }),
  usePathname: () => nav.pathname,
  useSearchParams: () => new URLSearchParams(nav.search),
}));

export function renderAccounts(ui: ReactElement, locale: keyof typeof catalogs = "pt-BR") {
  // The step-up provider reads the profile; keep a fallback behind any handler a test registers.
  server.use(http.get("/api/v1/me", () => HttpResponse.json(me)));
  const queryClient = new QueryClient({ defaultOptions: { queries: { retry: false }, mutations: { retry: false } } });
  return render(
    <QueryClientProvider client={queryClient}>
      <NextIntlClientProvider locale={locale} messages={catalogs[locale]} timeZone="UTC">
        <TooltipProvider>
          <LocaleSwitchProvider>
            <StepUpProvider>{ui}</StepUpProvider>
          </LocaleSwitchProvider>
          <Toaster />
        </TooltipProvider>
      </NextIntlClientProvider>
    </QueryClientProvider>,
  );
}
