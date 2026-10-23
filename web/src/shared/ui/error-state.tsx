"use client";

import { AlertCircle } from "lucide-react";
import { useTranslations } from "next-intl";
import type { ReactNode } from "react";
import { Button } from "@/shared/ui/button";

type ErrorStateProps = {
  title: ReactNode;
  /** Stable error code, shown in small mono text for support. */
  code?: string;
  onRetry?: () => void;
};

/** Request error for a whole page or card (F§10): what failed, retry, and the stable code. */
export function ErrorState({ title, code, onRetry }: ErrorStateProps) {
  const t = useTranslations("common");
  return (
    <div role="alert" className="flex flex-col items-start gap-3 rounded-lg border border-status-danger/30 bg-status-danger-soft p-4 text-status-danger">
      <div className="flex items-center gap-2">
        <AlertCircle aria-hidden="true" className="size-5 shrink-0" />
        <p className="text-sm font-medium">{title}</p>
      </div>
      {code ? <p className="font-mono text-xs">{t("errors.code", { code })}</p> : null}
      {onRetry ? (
        <Button variant="outline" size="sm" onClick={onRetry}>
          {t("actions.retry")}
        </Button>
      ) : null}
    </div>
  );
}
