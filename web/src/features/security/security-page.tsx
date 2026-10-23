"use client";

import { ShieldAlert, ShieldCheck } from "lucide-react";
import { useLocale, useTranslations } from "next-intl";
import { useSearchParams } from "next/navigation";
import { useState } from "react";
import { errorMessage } from "@/shared/api/error-message";
import { intlLocales, isLocale, defaultLocale } from "@/shared/i18n/config";
import { useLogout } from "@/shared/session/logout";
import { useSecurity } from "@/shared/session/security";
import { Badge } from "@/shared/ui/badge";
import { Button } from "@/shared/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/shared/ui/card";
import { ErrorState } from "@/shared/ui/error-state";
import { Skeleton } from "@/shared/ui/skeleton";
import { ChangePasswordDialog } from "./change-password-dialog";
import { DisableTotpDialog } from "./disable-totp-dialog";
import { EnableTotpDialog } from "./enable-totp-dialog";
import { RegenerateCodesDialog } from "./regenerate-codes-dialog";

type Dialogs = "enable" | "disable" | "regenerate" | "password" | null;

function formatTimeout(seconds: number, locale: string): string {
  const days = seconds % 86400 === 0 && seconds >= 86400;
  const loc = isLocale(locale) ? intlLocales[locale] : intlLocales[defaultLocale];
  return new Intl.NumberFormat(loc, { style: "unit", unit: days ? "day" : "hour", unitDisplay: "long" }).format(days ? seconds / 86400 : Math.round(seconds / 3600));
}

export function SecurityPage() {
  const t = useTranslations("security");
  const root = useTranslations();
  const locale = useLocale();
  const search = useSearchParams();
  const security = useSecurity();
  const logout = useLogout();
  const [dialog, setDialog] = useState<Dialogs>(() => (search.get("enable") === "1" ? "enable" : search.get("regenerate") === "1" ? "regenerate" : null));

  if (security.isPending) {
    return (
      <div className="grid gap-4">
        <Skeleton className="h-40 w-full" />
        <Skeleton className="h-32 w-full" />
      </div>
    );
  }
  if (security.isError) {
    const { message, code } = errorMessage(root, security.error);
    return <ErrorState title={message} code={code} onRetry={() => void security.refetch()} />;
  }

  const state = security.data;
  const low = state.recovery_codes_remaining <= 3;
  const close = () => setDialog(null);

  return (
    <div className="grid max-w-2xl gap-6">
      <Card aria-labelledby="totp-title">
        <CardHeader>
          <CardTitle id="totp-title">{t("totp.title")}</CardTitle>
          <CardDescription>{t("totp.recommended")}</CardDescription>
        </CardHeader>
        <CardContent>
          <p>
            {state.totp_enabled ? (
              <Badge tone="success">
                <ShieldCheck aria-hidden="true" />
                {t("totp.on")}
              </Badge>
            ) : (
              <Badge tone="warning">
                <ShieldAlert aria-hidden="true" />
                {t("totp.off")}
              </Badge>
            )}
          </p>
          {state.totp_enabled ? (
            <p className={low ? "text-sm text-status-warning" : "text-sm"}>{t("totp.remaining", { n: state.recovery_codes_remaining })}</p>
          ) : null}
          <div className="flex flex-wrap gap-2">
            {state.totp_enabled ? (
              <>
                <Button variant="outline" onClick={() => setDialog("regenerate")}>
                  {t("recovery.regenerate")}
                </Button>
                <Button variant="destructive" onClick={() => setDialog("disable")}>
                  {t("totp.disable")}
                </Button>
              </>
            ) : (
              <Button onClick={() => setDialog("enable")}>{t("totp.enable")}</Button>
            )}
          </div>
          <p className="text-sm text-muted-foreground">{t("totp.note")}</p>
        </CardContent>
      </Card>

      <Card aria-labelledby="session-title">
        <CardHeader>
          <CardTitle id="session-title">{t("session.title")}</CardTitle>
        </CardHeader>
        <CardContent>
          <Button variant="outline" className="self-start" disabled={logout.isPending} onClick={() => logout.mutate()}>
            {t("session.signOut")}
          </Button>
          <p className="text-sm">
            {t("session.limits", { idle: formatTimeout(state.session.idle_timeout_s, locale), absolute: formatTimeout(state.session.absolute_timeout_s, locale) })}
          </p>
          <p className="text-sm text-muted-foreground">{t("session.keepAlive")}</p>
        </CardContent>
      </Card>

      <Card aria-labelledby="password-title">
        <CardHeader>
          <CardTitle id="password-title">{t("password.cardTitle")}</CardTitle>
        </CardHeader>
        <CardContent>
          <Button variant="outline" className="self-start" onClick={() => setDialog("password")}>
            {t("password.change")}
          </Button>
        </CardContent>
      </Card>

      <EnableTotpDialog open={dialog === "enable"} onClose={close} />
      <DisableTotpDialog open={dialog === "disable"} onClose={close} />
      <RegenerateCodesDialog open={dialog === "regenerate"} onClose={close} />
      <ChangePasswordDialog open={dialog === "password"} onClose={close} />
    </div>
  );
}
