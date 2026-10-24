"use client";

import { useQueryClient } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import { useRouter } from "next/navigation";
import { useEffect, useRef, useState, type FormEvent } from "react";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import type { ChallengeDelivery, ChallengeIssued } from "@/shared/api/types";
import { challengeDelivery, resendChallenge } from "@/shared/api/challenges";
import { CodeChallenge } from "@/shared/components/code-challenge";
import { lifetimeMinutes } from "@/shared/format";
import { isLocale, localeNames, locales, type Locale } from "@/shared/i18n/config";
import { useViewOnceLock } from "@/shared/i18n/locale-switch";
import { useAppLocale } from "@/shared/i18n/navigation";
import { useLogout } from "@/shared/session/logout";
import { meQueryKey, useMe } from "@/shared/session/me";
import { landingPath } from "@/shared/session/redirects";
import { offsetLabel } from "@/shared/time-zone/time-zones";
import { TimeZoneCombobox } from "@/shared/time-zone/time-zone-combobox";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Checkbox } from "@/shared/ui/checkbox";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import { Skeleton } from "@/shared/ui/skeleton";
import { Spinner } from "@/shared/ui/spinner";
import { Stepper } from "@/shared/ui/stepper";
import { focusOnMount } from "@/shared/lib/focus";
import { completeSignup, startSignup, useLookupInvitation } from "./api";
import { AuthCard } from "./auth-card";
import { useFragmentToken } from "./use-fragment-token";

function browserTimeZone(): string {
  try {
    return Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";
  } catch {
    return "UTC";
  }
}

/** S3-03. The account is created, and the invitation consumed, only after the code is confirmed. */
export function InviteForm() {
  const t = useTranslations();
  const current = useAppLocale();
  const router = useRouter();
  const queryClient = useQueryClient();
  const me = useMe();
  const { checked, token } = useFragmentToken();
  const lookup = useLookupInvitation();
  const logout = useLogout();
  const looked = useRef(false);
  const [rejected, setRejected] = useState(false);
  const [locale, setLocale] = useState<Locale>(current);
  const [timeZone, setTimeZone] = useState(browserTimeZone);
  const [editingZone, setEditingZone] = useState(false);
  const [zoneError, setZoneError] = useState(false);
  const [remember, setRemember] = useState(false);
  const [challenge, setChallenge] = useState<ChallengeIssued | null>(null);
  const [minutes, setMinutes] = useState(10);
  const [delivery, setDelivery] = useState<ChallengeDelivery | undefined>(undefined);
  const [busy, setBusy] = useState(false);
  const [failure, setFailure] = useState<string | null>(null);
  const [unknownOutcome, setUnknownOutcome] = useState(false);
  // Going back or switching language would drop the challenge, so the switcher stays off while one is pending.
  useViewOnceLock(challenge !== null);

  useEffect(() => {
    if (!checked || looked.current) return;
    looked.current = true;
    if (!token) return;
    lookup.mutate(token, { onSuccess: (invitation) => invitation.locale && setLocale(invitation.locale), onError: () => setRejected(true) });
    // eslint-disable-next-line react-hooks/exhaustive-deps -- runs once per mounted page, after the fragment was read
  }, [checked, token]);

  // Back drops the code step, so focus goes to the first field of step 1 instead of the page body. Done in an effect
  // (no timer), so nothing can fire after the component is gone.
  const refocusEmail = useRef(false);
  useEffect(() => {
    if (challenge === null && refocusEmail.current) {
      refocusEmail.current = false;
      document.getElementById("invite-email")?.focus();
    }
  }, [challenge]);

  const challengeId = challenge?.challenge_id;
  useEffect(() => {
    if (!challengeId) return;
    let live = true;
    const poll = () => challengeDelivery(challengeId).then((state) => live && setDelivery(state), () => undefined);
    void poll();
    const timer = window.setInterval(() => void poll(), 5000);
    return () => {
      live = false;
      window.clearInterval(timer);
    };
  }, [challengeId]);

  async function sendCode(event?: FormEvent) {
    event?.preventDefault();
    if (busy || !token) return;
    setBusy(true);
    setFailure(null);
    setZoneError(false);
    try {
      const issued = await startSignup(token);
      setMinutes(lifetimeMinutes(issued.expires_at));
      setChallenge(issued);
    } catch (error) {
      if (error instanceof ApiError && error.code === "auth.invitation_invalid") setRejected(true);
      else {
        const { message, code } = errorMessage(t, error);
        setFailure(`${message} ${t("common.errors.code", { code })}`);
      }
    } finally {
      setBusy(false);
    }
  }

  const invalid = rejected || (checked && !token);
  const title = t("auth.invite.title");
  const steps = [
    { id: "details", label: t("auth.invite.stepDetails") },
    { id: "confirm", label: t("auth.invite.stepConfirm") },
  ];

  if (me.data) {
    return (
      <AuthCard title={title} titleId="invite-title">
        <Alert tone="info">{t("auth.invite.signedInAs", { email: me.data.email })}</Alert>
        <Button variant="outline" disabled={logout.isPending} onClick={() => logout.mutate()}>
          {t("auth.invite.signOut")}
        </Button>
      </AuthCard>
    );
  }
  if (invalid) {
    return (
      <AuthCard title={title} titleId="invite-title">
        <Alert tone="danger">{t("auth.invite.invalid")}</Alert>
        {checked && !token ? <p className="mt-3 text-sm text-muted-foreground">{t("auth.invite.reloaded")}</p> : null}
      </AuthCard>
    );
  }
  if (unknownOutcome) {
    return (
      <AuthCard title={title} titleId="invite-title">
        <Alert tone="warning">{t("auth.invite.unknownOutcome")}</Alert>
      </AuthCard>
    );
  }
  if (!lookup.data) {
    return (
      <AuthCard title={title} titleId="invite-title">
        <p className="sr-only" role="status">
          {t("auth.invite.validating")}
        </p>
        <Skeleton className="h-11 w-full" />
        <Skeleton className="mt-3 h-11 w-full" />
        <Skeleton className="mt-3 h-11 w-full" />
      </AuthCard>
    );
  }

  const email = lookup.data.email;
  if (challenge) {
    return (
      <AuthCard title={title} titleId="invite-title">
        <Stepper steps={steps} current={1} className="mb-4" />
        <CodeChallenge
          challenge={challenge}
          delivery={delivery}
          description={t("auth.invite.sent", { email, minutes })}
          extra={
            <div className="flex items-start gap-3">
              <Checkbox id="invite-remember" checked={remember} onCheckedChange={(checked) => setRemember(checked === true)} className="mt-1" />
              <Label htmlFor="invite-remember" className="min-h-6">
                {t("auth.login.remember")}
              </Label>
            </div>
          }
          onVerify={async (code) => {
            try {
              const result = await completeSignup({ challenge_id: challenge.challenge_id, code, locale, time_zone: timeZone, remember_me: remember });
              queryClient.clear();
              queryClient.setQueryData(meQueryKey, result.user);
              router.replace(landingPath(result.user.locale, null));
            } catch (error) {
              if (error instanceof ApiError && error.code === "auth.invitation_invalid") setRejected(true);
              // The outcome is unknown when the answer never arrived: the Core's confirm is not assumed idempotent.
              else if (error instanceof ApiError && error.status === 0) setUnknownOutcome(true);
              throw error;
            }
          }}
          onResend={async () => {
            const next = await resendChallenge(challenge.challenge_id);
            setChallenge(next);
            return next;
          }}
          onNewChallenge={async () => {
            if (!token) throw new ApiError({ code: "auth.invitation_invalid", status: 404 });
            const next = await startSignup(token);
            setChallenge(next);
            return next;
          }}
          secondaryActions={
            <Button
              type="button"
              variant="ghost"
              onClick={() => {
                refocusEmail.current = true;
                setChallenge(null);
              }}
            >
              {t("auth.invite.back")}
            </Button>
          }
        />
      </AuthCard>
    );
  }

  return (
    <AuthCard title={title} titleId="invite-title">
      <form aria-labelledby="invite-title" onSubmit={(event) => void sendCode(event)} className="grid gap-4">
        <Stepper steps={steps} current={0} />
        <p className="text-sm text-muted-foreground">{t("auth.invite.intro")}</p>
        {failure ? (
          <Alert tone="danger" tabIndex={-1} ref={focusOnMount}>
            {failure}
          </Alert>
        ) : null}
        <div className="grid gap-2">
          <Label htmlFor="invite-email">{t("auth.invite.email")}</Label>
          <Input id="invite-email" type="email" readOnly autoComplete="email" value={email} aria-describedby="invite-email-help" />
          <p id="invite-email-help" className="text-sm text-muted-foreground">
            {t("auth.invite.emailHelp")}
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
              <TimeZoneCombobox
                id="invite-zone"
                value={timeZone}
                onChange={(zone) => {
                  setTimeZone(zone);
                  setZoneError(false);
                }}
                invalid={zoneError}
              />
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
        <Button type="submit" disabled={busy}>
          {busy ? <Spinner /> : null}
          {busy ? t("auth.invite.submitting") : t("auth.invite.submit")}
        </Button>
      </form>
    </AuthCard>
  );
}
