import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render } from "@testing-library/react";
import { NextIntlClientProvider } from "next-intl";
import type { ReactElement } from "react";
import en from "../../messages/en.json";
import ptBR from "../../messages/pt-BR.json";
import { LocaleSwitchProvider } from "@/shared/i18n/locale-switch";
import { EventsProvider } from "@/shared/session/events";
import { StepUpProvider } from "@/shared/session/step-up";
import { Toaster } from "@/shared/ui/sonner";
import { TooltipProvider } from "@/shared/ui/tooltip";

const catalogs = { "pt-BR": ptBR, en };

export const navigation = { replace: vi.fn(), push: vi.fn(), pathname: "/pt-BR/settings/preferences", search: "" };

vi.mock("next/navigation", () => ({
  useRouter: () => ({ replace: navigation.replace, push: navigation.push, refresh: vi.fn() }),
  usePathname: () => navigation.pathname,
  useSearchParams: () => new URLSearchParams(navigation.search),
}));

/** Renders inside the same providers as the app, with MSW-backed API calls. */
export function renderApp(ui: ReactElement, locale: keyof typeof catalogs = "pt-BR") {
  const queryClient = new QueryClient({ defaultOptions: { queries: { retry: false }, mutations: { retry: false } } });
  return render(
    <QueryClientProvider client={queryClient}>
      <NextIntlClientProvider locale={locale} messages={catalogs[locale]} timeZone="UTC">
        <TooltipProvider>
          <LocaleSwitchProvider>
            <StepUpProvider>
              <EventsProvider>{ui}</EventsProvider>
            </StepUpProvider>
            <Toaster />
          </LocaleSwitchProvider>
        </TooltipProvider>
      </NextIntlClientProvider>
    </QueryClientProvider>,
  );
}
