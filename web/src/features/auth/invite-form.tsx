"use client";

import { useQueryClient } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import { useRouter } from "next/navigation";
import { useEffect, useRef, useState, type FormEvent } from "react";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { isLocale, localeNames, locales, type Locale } from "@/shared/i18n/config";
import { useAppLocale } from "@/shared/i18n/navigation";
import { useMe, meQueryKey } from "@/shared/session/me";
import { landingPath } from "@/shared/session/redirects";
import { offsetLabel } from "@/shared/time-zone/time-zones";
import { TimeZoneCombobox } from "@/shared/time-zone/time-zone-combobox";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { PasswordField } from "@/shared/ui/password-field";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import { Skeleton } from "@/shared/ui/skeleton";
import { Spinner } from "@/shared/ui/spinner";
import { useLogout } from "@/shared/session/logout";
import { useAcceptInvitation, useLookupInvitation } from "./api";
import { AuthCard } from "./auth-card";
import { passwordMax, passwordMin } from "./password-rules";
import { useFragmentToken } from "./use-fragment-token";

function browserTimeZone(): string {
  try {
    return Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";
  } catch {
    return "UTC";
  }
}

export function InviteForm() {
  const t = useTranslations();
  const current = useAppLocale();
  const router = useRouter();
  const queryClient = useQueryClient();
  const me = useMe();
  const { checked, token } = useFragmentToken();
  const lookup = useLookupInvitation();
  const accept = useAcceptInvitation();
  const logout = useLogout();
  const looked = useRef(false);
  const [rejected, setRejected] = useState(false);
  const [password, setPassword] = useState("");
  const [locale, setLocale] = useState<Locale>(current);
  const [timeZone, setTimeZone] = useState(browserTimeZone);
  const [editingZone, setEditingZone] = useState(false);
  const [zoneError, setZoneError] = useState(false);
  const [failure, setFailure] = useState<string | null>(null);

  useEffect(() => {
    if (!checked || looked.current) return;
    looked.current = true;
    if (!token) return;
    lookup.mutate(token, { onSuccess: (invitation) => invitation.locale && setLocale(invitation.locale) });
    // eslint-disable-next-line react-hooks/exhaustive-deps -- runs once per mounted page, after the fragment was read
  }, [checked, token]);

  async function submit(event: FormEvent) {
    event.preventDefault();
    if (accept.isPending || !token) return;
    setFailure(null);
    setZoneError(false);
    try {
      const result = await accept.mutateAsync({ token, password, locale, time_zone: timeZone });
      setPassword("");
      if (result.user) {
        queryClient.clear();
        queryClient.setQueryData(meQueryKey, result.user);
        router.replace(landingPath(result.user.locale, null));
      }
    } catch (error) {
      if (error instanceof ApiError && error.code === "auth.invitation_invalid") setRejected(true);
      else if (error instanceof ApiError && error.code === "me.invalid_time_zone") {
        setZoneError(true);
        setEditingZone(true);
      } else {
        const { message, code } = errorMessage(t, error);
        setFailure(`${message} ${t("common.errors.code", { code })}`);
      }
    }
  }

  const invalid = rejected || lookup.isError || (checked && !token);
  const title = t("auth.invite.title");
  if (me.data) {
    return (
      <AuthCard title={title} titleId="invite-title">
        <Alert tone="info">{t("auth.invite.signedInAs", { email: me.data.email })}</Alert>
        <Button
          variant="outline"
          disabled={logout.isPending}
          onClick={() => logout.mutate()}
        >
          {t("auth.invite.signOut")}
        </Button>
      </AuthCard>
    );
  }
  if (invalid) {
    return (
      <AuthCard title={title} titleId="invite-title">
        <Alert tone="danger">{t("auth.invite.invalid")}</Alert>
      </AuthCard>
    );
  }
  if (!lookup.data) {
    return (
      <AuthCard title={title} titleId="invite-title">
        <p className="sr-only" role="status">{t("auth.invite.validating")}</p>
        <Skeleton className="h-11 w-full" />
        <Skeleton className="h-11 w-full" />
        <Skeleton className="h-11 w-full" />
      </AuthCard>
    );
  }

  const length = [...password].length;
  return (
    <AuthCard title={title} titleId="invite-title">
      <form aria-labelledby="invite-title" onSubmit={(event) => void submit(event)} className="grid gap-4">
        {failure ? <Alert tone="danger" tabIndex={-1} ref={(node) => node?.focus()}>{failure}</Alert> : null}
        <div className="grid gap-2">
          <Label htmlFor="invite-email">{t("auth.invite.email")}</Label>
          <Input id="invite-email" type="email" readOnly autoComplete="username" value={lookup.data.email} />
        </div>
        <div className="grid gap-2">
          <Label htmlFor="invite-password">{t("auth.invite.password")}</Label>
          <PasswordField id="invite-password" autoComplete="new-password" autoFocus value={password} onChange={(event) => setPassword(event.target.value)} aria-describedby="invite-password-help" />
          <p id="invite-password-help" className="text-sm text-muted-foreground">
            {t("auth.invite.passwordHelp")} {t("auth.invite.passwordCount", { count: length })}
          </p>
        </div>
        <div className="grid gap-2">
          <Label htmlFor="invite-language">{t("auth.invite.language")}</Label>
          <Select value={locale} onValueChange={(value) => isLocale(value) && setLocale(value)}>
            <SelectTrigger id="invite-language">
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
        </div>
        <div className="grid gap-2">
          {editingZone ? (
            <>
              <Label htmlFor="invite-zone">{t("settings.preferences.timeZone.label")}</Label>
              <TimeZoneCombobox id="invite-zone" value={timeZone} onChange={(zone) => { setTimeZone(zone); setZoneError(false); }} invalid={zoneError} />
              {zoneError ? <p className="text-sm text-status-danger">{t("errors.me.invalid_time_zone")}</p> : null}
            </>
          ) : (
            <p className="flex flex-wrap items-center gap-x-2 text-sm">
              {t("auth.invite.timeZone", { zone: timeZone, offset: offsetLabel(timeZone, new Date()) })}
              <Button type="button" variant="link" size="sm" onClick={() => setEditingZone(true)}>
                {t("auth.invite.timeZoneChange")}
              </Button>
            </p>
          )}
        </div>
        <Button type="submit" disabled={accept.isPending || length < passwordMin || length > passwordMax}>
          {accept.isPending ? <Spinner /> : null}
          {accept.isPending ? t("auth.invite.submitting") : t("auth.invite.submit")}
        </Button>
      </form>
    </AuthCard>
  );
}
