"use client";

import { useQueryClient } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { useRouter, useSearchParams } from "next/navigation";
import { useState, type FormEvent } from "react";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { formatWait } from "@/shared/format";
import { useAppLocale, useLocalePath } from "@/shared/i18n/navigation";
import { useCountdown } from "@/shared/lib/use-countdown";
import { meQueryKey } from "@/shared/session/me";
import { landingPath, loginPath, safeReturnTo } from "@/shared/session/redirects";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { OtpInput } from "@/shared/ui/otp-input";
import { Spinner } from "@/shared/ui/spinner";
import { useVerifyRecoveryCode, useVerifyTotp } from "./api";
import { AuthCard } from "./auth-card";

export const recoveryRemainingKey = "aw.recovery-remaining";

/** Crockford Base32 without separators: the Core ignores case, spaces and hyphens. */
export function normalizeRecoveryCode(input: string): string {
  return input.replace(/[\s-]/g, "").toUpperCase();
}
const recoveryPattern = /^[0-9A-HJKMNP-TV-Z]{10}$/;

export function VerifyForm() {
  const t = useTranslations();
  const locale = useAppLocale();
  const path = useLocalePath();
  const router = useRouter();
  const search = useSearchParams();
  const queryClient = useQueryClient();
  const totp = useVerifyTotp();
  const recovery = useVerifyRecoveryCode();
  const [mode, setMode] = useState<"app" | "recovery">("app");
  const [code, setCode] = useState("");
  const [recoveryCode, setRecoveryCode] = useState("");
  const [failure, setFailure] = useState<string | null>(null);
  const [deadline, setDeadline] = useState<number | null>(null);
  const waiting = useCountdown(deadline);
  const returnTo = safeReturnTo(search.get("returnTo"));
  const pending = totp.isPending || recovery.isPending;

  async function verify(value: string) {
    if (pending) return;
    setFailure(null);
    try {
      const result = mode === "app" ? await totp.mutateAsync(value) : await recovery.mutateAsync(value);
      if (!result.user) return;
      queryClient.setQueryData(meQueryKey, result.user);
      if (mode === "recovery" && result.recovery_codes_remaining !== undefined) {
        try {
          sessionStorage.setItem(recoveryRemainingKey, String(result.recovery_codes_remaining));
        } catch {
          // Storage blocked: the one-time warning is skipped; the Security page still shows the count.
        }
      }
      router.replace(landingPath(result.user.locale, returnTo));
    } catch (error) {
      setCode("");
      setRecoveryCode("");
      if (error instanceof ApiError && (error.code === "auth.pending_expired" || error.code === "auth.unauthenticated")) {
        router.replace(loginPath(locale, { reason: "pending_expired", returnTo: returnTo ?? undefined }));
      } else if (error instanceof ApiError && error.code === "auth.invalid_code") {
        setFailure(t("auth.verify.invalid"));
      } else if (error instanceof ApiError && error.code === "auth.rate_limited") {
        const seconds = error.retryAfter ?? 30;
        setDeadline(Date.now() + seconds * 1000);
        setFailure(t("auth.login.rateLimited", { time: formatWait(seconds, locale) }));
      } else {
        const { message, code: errorCode } = errorMessage(t, error);
        setFailure(`${message} ${t("common.errors.code", { code: errorCode })}`);
      }
    }
  }

  function submit(event: FormEvent) {
    event.preventDefault();
    if (mode === "app") {
      if (code.length === 6) void verify(code);
    } else {
      const normalized = normalizeRecoveryCode(recoveryCode);
      if (recoveryPattern.test(normalized)) void verify(normalized);
      else setFailure(t("auth.verify.invalid"));
    }
  }

  const ready = mode === "app" ? code.length === 6 : recoveryCode.trim().length > 0;

  return (
    <AuthCard title={t("auth.verify.title")} titleId="verify-title">
      <form aria-labelledby="verify-title" onSubmit={submit} className="grid gap-4">
        <p className="text-sm text-muted-foreground">{mode === "app" ? t("auth.verify.intro") : t("auth.verify.introRecovery")}</p>
        {failure ? (
          <Alert tone="danger" tabIndex={-1} ref={(node) => node?.focus()}>
            {failure}
          </Alert>
        ) : null}
        {mode === "app" ? (
          <div className="grid gap-2">
            <Label htmlFor="verify-code">{t("auth.verify.code")}</Label>
            <OtpInput id="verify-code" value={code} onChange={setCode} onComplete={(value) => void verify(value)} readOnly={pending} autoFocus />
          </div>
        ) : (
          <div className="grid gap-2">
            <Label htmlFor="verify-recovery">{t("auth.verify.recoveryCode")}</Label>
            <Input
              id="verify-recovery"
              className="font-mono uppercase"
              value={recoveryCode}
              onChange={(event) => setRecoveryCode(event.target.value)}
              placeholder={t("auth.verify.recoveryPlaceholder")}
              autoComplete="off"
              autoCapitalize="characters"
              spellCheck={false}
              readOnly={pending}
              autoFocus
            />
          </div>
        )}
        <Button type="submit" disabled={!ready || pending || waiting > 0}>
          {pending ? <Spinner /> : null}
          {waiting > 0 ? formatWait(waiting, locale) : t("auth.verify.submit")}
        </Button>
        <Button type="button" variant="link" onClick={() => { setMode(mode === "app" ? "recovery" : "app"); setFailure(null); }}>
          {mode === "app" ? t("auth.verify.useRecovery") : t("auth.verify.useApp")}
        </Button>
        <Link href={path("/login")} className="inline-flex min-h-11 items-center justify-center text-sm underline underline-offset-4">
          {t("auth.verify.back")}
        </Link>
        <p className="text-sm text-muted-foreground">{t("auth.verify.lost")}</p>
      </form>
    </AuthCard>
  );
}
