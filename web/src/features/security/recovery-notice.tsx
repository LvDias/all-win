"use client";

import { useTranslations } from "next-intl";
import Link from "next/link";
import { useEffect, useState } from "react";
import { useLocalePath } from "@/shared/i18n/navigation";
import { Alert } from "@/shared/ui/alert";
import { buttonVariants } from "@/shared/ui/button";

const storageKey = "aw.recovery-remaining";

/** One-time warning after signing in with a recovery code (S3-02); the count comes from the sign-in response. */
export function RecoveryNotice() {
  const t = useTranslations("auth.verify");
  const path = useLocalePath();
  const [remaining, setRemaining] = useState<number | null>(null);

  useEffect(() => {
    /* Session storage is browser-only, so the notice is read once after mount and consumed. */
    /* eslint-disable react-hooks/set-state-in-effect */
    try {
      const stored = sessionStorage.getItem(storageKey);
      if (stored === null) return;
      sessionStorage.removeItem(storageKey);
      const count = Number(stored);
      if (Number.isInteger(count) && count >= 0) setRemaining(count);
    } catch {
      // Storage blocked: no notice.
    }
    /* eslint-enable react-hooks/set-state-in-effect */
  }, []);

  if (remaining === null) return null;
  return (
    <Alert tone="warning" className="items-center">
      <p className="flex-1">{t("recoveryUsed", { n: remaining })}</p>
      {remaining <= 3 ? (
        <Link href={`${path("/settings/security")}?regenerate=1`} className={buttonVariants({ size: "sm", variant: "outline" })}>
          {t("regenerate")}
        </Link>
      ) : null}
    </Alert>
  );
}
