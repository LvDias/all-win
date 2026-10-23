"use client";

import { NextIntlClientProvider, type AbstractIntlMessages } from "next-intl";
import type { ReactNode } from "react";
import { handleIntlError, messageFallback } from "./missing";

export function IntlProvider({ locale, messages, children }: { locale: string; messages: AbstractIntlMessages; children: ReactNode }) {
  return (
    <NextIntlClientProvider locale={locale} messages={messages} timeZone="UTC" onError={handleIntlError} getMessageFallback={messageFallback}>
      {children}
    </NextIntlClientProvider>
  );
}
