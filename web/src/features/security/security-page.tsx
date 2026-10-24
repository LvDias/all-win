"use client";

import { CircleCheck } from "lucide-react";
import { useTranslations } from "next-intl";
import { useState } from "react";
import { toast } from "sonner";
import { errorMessage } from "@/shared/api/error-message";
import { formatDateTime } from "@/shared/format";
import { useDisplay } from "@/shared/session/display";
import { useLogout } from "@/shared/session/logout";
import { useSecurity } from "@/shared/session/security";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Badge } from "@/shared/ui/badge";
import { Button } from "@/shared/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/shared/ui/card";
import { ErrorState } from "@/shared/ui/error-state";
import { Skeleton } from "@/shared/ui/skeleton";
import { useCancelChannelChange } from "./api";
import { ChangeEmailDialog } from "./change-email-dialog";
import { DevicesCard } from "./devices-card";
import { formatLimit } from "./format-limit";

export function SecurityPage() {
  const t = useTranslations("security");
  const root = useTranslations();
  const { locale, timeZone } = useDisplay();
  const security = useSecurity();
  const logout = useLogout();
  const cancel = useCancelChannelChange();
  const [changing, setChanging] = useState(false);
  const [cancelling, setCancelling] = useState(false);
  const [now] = useState(() => Date.now());

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
  const frozenUntil = Date.parse(state.channel_change_allowed_at) > now ? state.channel_change_allowed_at : null;
  const pendingChange = state.pending_channel_change;
  const held = pendingChange?.status === "on_hold" ? pendingChange : null;
  const session = state.session;

  return (
    <div className="grid max-w-2xl gap-6">
      <Card aria-labelledby="channels-title">
        <CardHeader>
          <CardTitle id="channels-title">{t("channels.title")}</CardTitle>
          <CardDescription>{t("channels.description")}</CardDescription>
        </CardHeader>
        <CardContent className="grid gap-4">
          {held ? (
            <Alert tone="warning">
              <div className="grid gap-2">
                <p>{t("channels.pending", { email: held.masked_new_value, date: held.hold_until ? formatDateTime(held.hold_until, locale, timeZone, { zone: true }) : "" })}</p>
                <div>
                  <Button variant="outline" size="sm" disabled={cancel.isPending} onClick={() => setCancelling(true)}>
                    {t("channels.cancelChange")}
                  </Button>
                </div>
              </div>
            </Alert>
          ) : null}
          <div className="flex flex-wrap items-center justify-between gap-3">
            <div className="grid gap-1">
              <p className="text-sm text-muted-foreground">{t("channels.email")}</p>
              <p className="flex flex-wrap items-center gap-2 font-medium">
                {state.email}
                <Badge tone="success">
                  <CircleCheck aria-hidden="true" />
                  {t("channels.confirmed")}
                </Badge>
              </p>
            </div>
            <Button variant="outline" disabled={frozenUntil !== null || pendingChange !== undefined} onClick={() => setChanging(true)}>
              {t("channels.changeEmail")}
            </Button>
          </div>
          {frozenUntil ? <p className="text-sm text-muted-foreground">{t("channels.frozen", { date: formatDateTime(frozenUntil, locale, timeZone, { zone: true }) })}</p> : null}
          <p className="text-sm text-muted-foreground">{t("channels.note")}</p>
          <p className="text-sm text-muted-foreground">{t("channels.advice")}</p>
        </CardContent>
      </Card>

      <Card aria-labelledby="session-title">
        <CardHeader>
          <CardTitle id="session-title">{t("session.title")}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-3">
          <p className="text-sm">{t("session.limits", { idle: formatLimit(session.idle_timeout_s, locale), absolute: formatLimit(session.absolute_timeout_s, locale)})}</p>
          <p className="text-sm text-muted-foreground">{t("session.keepAlive")}</p>
          <div>
            <Button variant="outline" disabled={logout.isPending} onClick={() => logout.mutate()}>
              {t("session.signOut")}
            </Button>
          </div>
        </CardContent>
      </Card>

      <DevicesCard />

      <ChangeEmailDialog open={changing} onClose={() => setChanging(false)} currentEmail={state.email} />
      <AlertDialog open={cancelling} onOpenChange={setCancelling}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("channels.cancelTitle")}</AlertDialogTitle>
            <AlertDialogDescription>{t("channels.cancelBody")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{root("common.actions.cancel")}</AlertDialogCancel>
            <AlertDialogAction
              variant="destructive"
              onClick={() => {
                if (held) cancel.mutate(held.id, { onSuccess: () => toast.success(t("channels.cancelled")), onError: (error) => toast.error(errorMessage(root, error).message) });
              }}
            >
              {t("channels.cancelChange")}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
