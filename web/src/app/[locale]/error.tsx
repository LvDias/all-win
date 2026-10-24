"use client";

import { useTranslations } from "next-intl";
import { ErrorState } from "@/shared/ui/error-state";

/** Render failures (for example a browser without exact decimal formatting) show an error state instead of a blank page. */
export default function LocaleError({ reset }: { error: Error & { digest?: string }; reset: () => void }) {
  const t = useTranslations("common.errors");
  return (
    <main id="main" className="mx-auto w-full max-w-xl p-4">
      <ErrorState title={t("failed")} code="client.render" onRetry={reset} />
    </main>
  );
}
