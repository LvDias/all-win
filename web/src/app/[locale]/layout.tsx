import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import { notFound } from "next/navigation";
import { connection } from "next/server";
import { hasLocale } from "next-intl";
import { getMessages, getTranslations, setRequestLocale } from "next-intl/server";
import type { ReactNode } from "react";
import { Providers } from "@/app/providers";
import { locales } from "@/shared/i18n/config";
import { IntlProvider } from "@/shared/i18n/intl-provider";
import { LanguageSwitcher } from "@/shared/i18n/language-switcher";
import { cn } from "@/shared/lib/utils";
import "../globals.css";

const geistSans = Geist({ subsets: ["latin"], variable: "--font-geist-sans" });
const geistMono = Geist_Mono({ subsets: ["latin"], variable: "--font-geist-mono" });

type LayoutProps = { children: ReactNode; params: Promise<{ locale: string }> };

export function generateStaticParams() {
  return locales.map((locale) => ({ locale }));
}

export async function generateMetadata({ params }: Pick<LayoutProps, "params">): Promise<Metadata> {
  const { locale } = await params;
  const t = await getTranslations({ locale, namespace: "common" });
  return { title: t("appName") };
}

export default async function LocaleLayout({ children, params }: LayoutProps) {
  // Pages carry a per-request CSP nonce, so they render per request, never from the static cache.
  await connection();
  const { locale } = await params;
  if (!hasLocale(locales, locale)) notFound();
  setRequestLocale(locale);
  const [messages, t] = await Promise.all([getMessages(), getTranslations("common")]);

  return (
    <html lang={locale} className={cn(geistSans.variable, geistMono.variable)}>
      <body className="min-h-dvh font-sans">
        <IntlProvider locale={locale} messages={messages}>
          <Providers>
            <a href="#main" className="sr-only focus:not-sr-only focus:absolute focus:top-2 focus:left-2 focus:z-50 focus:rounded-md focus:bg-background focus:p-3">
              {t("skipToContent")}
            </a>
            <header className="flex items-center justify-between gap-4 border-b px-4 py-3">
              <span className="font-semibold">{t("appName")}</span>
              <LanguageSwitcher />
            </header>
            <main id="main" className="mx-auto w-full max-w-5xl px-4 py-8">
              {children}
            </main>
          </Providers>
        </IntlProvider>
      </body>
    </html>
  );
}
