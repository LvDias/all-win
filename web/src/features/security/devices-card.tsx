"use client";

import { useTranslations } from "next-intl";
import { useState } from "react";
import { toast } from "sonner";
import { errorMessage } from "@/shared/api/error-message";
import type { Device } from "@/shared/api/types";
import { formatDateTime } from "@/shared/format";
import { useDisplay } from "@/shared/session/display";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Badge } from "@/shared/ui/badge";
import { Button } from "@/shared/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/shared/ui/card";
import { ErrorState } from "@/shared/ui/error-state";
import { Skeleton } from "@/shared/ui/skeleton";
import { useDevices, useForgetDevice } from "./api";
import { SignOutEverywhereDialog } from "./sign-out-everywhere-dialog";
import { useActiveCounts } from "./api";

/** S3-06 Card C. Provisional until Q34 is decided (the Core publishes the operations). */
export function DevicesCard() {
  const t = useTranslations("security.devices");
  const root = useTranslations();
  const { locale, timeZone } = useDisplay();
  const devices = useDevices();
  const forget = useForgetDevice();
  const counts = useActiveCounts();
  const [target, setTarget] = useState<Device | null>(null);
  const [signOutAll, setSignOutAll] = useState(false);

  async function confirmForget(device: Device) {
    try {
      await forget.mutateAsync(device.id);
      toast.success(t("forgotten"));
      setTarget(null);
    } catch (error) {
      toast.error(errorMessage(root, error).message);
      setTarget(null);
    }
  }

  return (
    <Card aria-labelledby="devices-title">
      <CardHeader>
        <CardTitle id="devices-title">{t("title")}</CardTitle>
        <CardDescription>{t("description")}</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        {devices.isPending ? <Skeleton className="h-16 w-full" /> : null}
        {devices.isError ? <ErrorState title={errorMessage(root, devices.error).message} code={errorMessage(root, devices.error).code} onRetry={() => void devices.refetch()} /> : null}
        {devices.data ? (
          <ul className="grid gap-3">
            {devices.data.map((device) => (
              <li key={device.id} className="flex flex-wrap items-center justify-between gap-3 rounded-md border p-3">
                <div className="grid gap-1 text-sm">
                  <p className="flex flex-wrap items-center gap-2 font-medium">
                    {device.label}
                    {device.current ? <Badge tone="info">{t("current")}</Badge> : null}
                  </p>
                  <p className="text-muted-foreground">{t("firstSeen", { date: formatDateTime(device.first_seen_at, locale, timeZone, { zone: true }) })}</p>
                  <p className="text-muted-foreground">{t("lastSeen", { date: formatDateTime(device.last_seen_at, locale, timeZone, { zone: true }) })}</p>
                </div>
                {device.current ? null : (
                  <Button variant="outline" size="sm" aria-label={t("forgetNamed", { name: device.label })} onClick={() => setTarget(device)}>
                    {t("forget")}
                  </Button>
                )}
              </li>
            ))}
          </ul>
        ) : null}
        <div>
          <Button variant="destructive" onClick={() => setSignOutAll(true)}>
            {t("signOutAll")}
          </Button>
        </div>
      </CardContent>
      <AlertDialog open={target !== null} onOpenChange={(next) => !next && setTarget(null)}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("forgetTitle")}</AlertDialogTitle>
            <AlertDialogDescription>{t("forgetBody")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{root("common.actions.cancel")}</AlertDialogCancel>
            <AlertDialogAction
              variant="destructive"
              onClick={(event) => {
                event.preventDefault();
                if (target) void confirmForget(target);
              }}
            >
              {t("forget")}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
      <SignOutEverywhereDialog open={signOutAll} onClose={() => setSignOutAll(false)} sources={counts.sources} connections={counts.connections} />
    </Card>
  );
}
