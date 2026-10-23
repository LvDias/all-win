"use client";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useState, type ReactNode } from "react";
import { LocaleSwitchProvider } from "@/shared/i18n/locale-switch";
import { LocaleSync } from "@/shared/i18n/locale-sync";
import { Toaster } from "@/shared/ui/sonner";
import { TooltipProvider } from "@/shared/ui/tooltip";

export function Providers({ children }: { children: ReactNode }) {
  const [queryClient] = useState(() => new QueryClient({ defaultOptions: { queries: { refetchOnWindowFocus: false } } }));
  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider>
        <LocaleSwitchProvider>
          <LocaleSync />
          {children}
          <Toaster />
        </LocaleSwitchProvider>
      </TooltipProvider>
    </QueryClientProvider>
  );
}
