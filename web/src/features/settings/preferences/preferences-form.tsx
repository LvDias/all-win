"use client";

import { useLocale, useTranslations } from "next-intl";
import { useId, useState, useSyncExternalStore } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { isLocale, localeNames, locales } from "@/shared/i18n/config";
import { useLocaleSwitch } from "@/shared/i18n/locale-switch";
import { useMe, useUpdateMe } from "@/shared/session/me";
import { Button } from "@/shared/ui/button";
import { ErrorState } from "@/shared/ui/error-state";
import { Label } from "@/shared/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import { Skeleton } from "@/shared/ui/skeleton";
import { TimeZoneCombobox } from "@/shared/time-zone/time-zone-combobox";

const subscribeNever = () => () => {};
const browserTimeZone = () => Intl.DateTimeFormat().resolvedOptions().timeZone;

export function PreferencesForm() {
  const t = useTranslations();
  const locale = useLocale();
  const me = useMe();
  const updateMe = useUpdateMe();
  const { switchLocale, pending: switching, locked } = useLocaleSwitch();
  const languageId = useId();
  const languageHelpId = useId();
  const zoneId = useId();
  const zoneHelpId = useId();
  const zoneErrorId = useId();
  const [zoneError, setZoneError] = useState<string | null>(null);
  const detectedZone = useSyncExternalStore(subscribeNever, browserTimeZone, () => null);

  if (me.isPending) {
    return (
      <div className="space-y-6" aria-busy="true" role="status">
        <span className="sr-only">{t("common.states.loading")}</span>
        <Skeleton className="h-11 w-full max-w-md" />
        <Skeleton className="h-11 w-full max-w-md" />
      </div>
    );
  }
  if (me.isError) {
    const { code } = errorMessage(t, me.error);
    return <ErrorState title={t("settings.preferences.loadError")} code={code} onRetry={() => void me.refetch()} />;
  }
  if (me.data === null) {
    return <p className="text-sm text-muted-foreground">{t("settings.preferences.signInRequired")}</p>;
  }

  const profile = me.data;

  function saveTimeZone(timeZone: string) {
    setZoneError(null);
    updateMe.mutate(
      { time_zone: timeZone },
      {
        onSuccess: () => toast.success(t("settings.preferences.saved")),
        onError: (error) => {
          if (error instanceof ApiError && error.code === "me.invalid_time_zone") setZoneError(t("errors.me.invalid_time_zone"));
          else {
            const { message, code } = errorMessage(t, error);
            toast.error(t("settings.preferences.timeZone.saveFailed"), { description: `${message} (${code})` });
          }
        },
      },
    );
  }

  const savingZone = updateMe.isPending;
  const browserDiffers = detectedZone !== null && detectedZone !== profile.time_zone;

  return (
    <div className="max-w-md space-y-8">
      <div className="space-y-2">
        <Label htmlFor={languageId}>{t("settings.preferences.language.label")}</Label>
        <Select value={locale} onValueChange={(value) => isLocale(value) && switchLocale(value)} disabled={switching || locked}>
          <SelectTrigger id={languageId} aria-describedby={languageHelpId}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {locales.map((option) => (
              <SelectItem key={option} value={option} lang={option}>
                {localeNames[option].name}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <p id={languageHelpId} className="text-sm text-muted-foreground">
          {t("settings.preferences.language.help")}
        </p>
      </div>

      <div className="space-y-2">
        <Label htmlFor={zoneId}>{t("settings.preferences.timeZone.label")}</Label>
        <TimeZoneCombobox
          id={zoneId}
          value={profile.time_zone}
          onChange={saveTimeZone}
          disabled={savingZone}
          invalid={zoneError !== null}
          describedBy={zoneError ? `${zoneHelpId} ${zoneErrorId}` : zoneHelpId}
        />
        <p id={zoneHelpId} className="text-sm text-muted-foreground">
          {t("settings.preferences.timeZone.help")}
        </p>
        {zoneError ? (
          <p id={zoneErrorId} role="alert" className="text-sm text-status-danger">
            {zoneError}
          </p>
        ) : null}
        {browserDiffers ? (
          <div className="flex flex-wrap items-center gap-2 text-sm">
            <span>{t("settings.preferences.timeZone.browserNote", { zone: detectedZone })}</span>
            <Button variant="outline" size="sm" disabled={savingZone} onClick={() => saveTimeZone(detectedZone)}>
              {t("common.actions.use")}
            </Button>
          </div>
        ) : null}
      </div>
    </div>
  );
}
