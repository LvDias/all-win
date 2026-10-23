import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render } from "@testing-library/react";
import { NextIntlClientProvider } from "next-intl";
import type { ReactElement } from "react";
import en from "../../../../messages/en.json";
import ptBR from "../../../../messages/pt-BR.json";
import { Toaster } from "@/shared/ui/sonner";
import { TooltipProvider } from "@/shared/ui/tooltip";

const catalogs = { "pt-BR": ptBR, en };

export const nav = { push: vi.fn(), replace: vi.fn(), refresh: vi.fn(), pathname: "/pt-BR/accounts", search: "" };

vi.mock("next/navigation", () => ({
  useRouter: () => ({ push: nav.push, replace: nav.replace, refresh: nav.refresh }),
  usePathname: () => nav.pathname,
  useSearchParams: () => new URLSearchParams(nav.search),
}));

export function renderAccounts(ui: ReactElement, locale: keyof typeof catalogs = "pt-BR") {
  const queryClient = new QueryClient({ defaultOptions: { queries: { retry: false }, mutations: { retry: false } } });
  return render(
    <QueryClientProvider client={queryClient}>
      <NextIntlClientProvider locale={locale} messages={catalogs[locale]} timeZone="UTC">
        <TooltipProvider>
          {ui}
          <Toaster />
        </TooltipProvider>
      </NextIntlClientProvider>
    </QueryClientProvider>,
  );
}
