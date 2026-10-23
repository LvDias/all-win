"use client";

import { ShieldAlert, X } from "lucide-react";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { useState } from "react";
import { useLocalePath } from "@/shared/i18n/navigation";
import { useSecurity } from "@/shared/session/security";
import { Alert } from "@/shared/ui/alert";
import { Button, buttonVariants } from "@/shared/ui/button";

const storageKey = "aw.totp-banner-dismissed";

function readDismissed(): boolean {
  try {
    return localStorage.getItem(storageKey) === "1";
  } catch {
    return false;
  }
}

/** Suggests two-step verification while it is off; dismissal is remembered per device only (D13). */
export function TotpBanner() {
  const t = useTranslations("security.banner");
  const path = useLocalePath();
  const security = useSecurity();
  const [dismissed, setDismissed] = useState(readDismissed);
  if (security.data?.totp_enabled !== false || dismissed) return null;

  function dismiss() {
    try {
      localStorage.setItem(storageKey, "1");
    } catch {
      // Storage blocked: the banner shows again next time.
    }
    setDismissed(true);
  }

  return (
    <Alert tone="info" className="items-center">
      <ShieldAlert aria-hidden="true" />
      <p className="flex-1">{t("text")}</p>
      <Link href={`${path("/settings/security")}?enable=1`} className={buttonVariants({ size: "sm", variant: "outline" })}>
        {t("enable")}
      </Link>
      <Button type="button" variant="ghost" size="icon" aria-label={t("dismiss")} onClick={dismiss}>
        <X aria-hidden="true" />
      </Button>
    </Alert>
  );
}
