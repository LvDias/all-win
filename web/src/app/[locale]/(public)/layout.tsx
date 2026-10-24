import { getTranslations } from "next-intl/server";
import type { ReactNode } from "react";
import { LanguageSwitcher } from "@/shared/i18n/language-switcher";

export default async function PublicLayout({ children, params }: { children: ReactNode; params: Promise<{ locale: string }> }) {
  // Nested layouts render alongside the root layout, so the locale is passed explicitly rather than read from the request cache.
  const { locale } = await params;
  const t = await getTranslations({ locale, namespace: "common" });
  return (
    <div className="flex min-h-dvh flex-col bg-muted">
      <header className="flex items-center justify-between gap-4 px-4 py-3">
        <span className="font-semibold">{t("appName")}</span>
        <LanguageSwitcher />
      </header>
      <main id="main" className="mx-auto flex w-full flex-1 flex-col justify-center px-4 py-8">
        {children}
      </main>
    </div>
  );
}
