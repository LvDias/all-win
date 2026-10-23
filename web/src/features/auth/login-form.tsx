"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useTranslations } from "next-intl";
import { useRouter, useSearchParams } from "next/navigation";
import { useEffect, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { z } from "zod";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { formatWait } from "@/shared/format";
import { useAppLocale, useLocalePath } from "@/shared/i18n/navigation";
import { useCountdown } from "@/shared/lib/use-countdown";
import { useMe } from "@/shared/session/me";
import { landingPath, safeReturnTo, type LoginReason } from "@/shared/session/redirects";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Checkbox } from "@/shared/ui/checkbox";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { Spinner } from "@/shared/ui/spinner";
import { requestLoginCode } from "./api";
import { AuthCard } from "./auth-card";
import { readPendingLogin, startPendingLogin } from "./pending-login";

const schema = z.object({
  email: z.string().trim().min(1, "emailRequired").regex(/^\S+@\S+\.\S+$/, "emailInvalid"),
  remember: z.boolean(),
});
type Values = z.infer<typeof schema>;

const reasons: readonly LoginReason[] = ["session_expired", "signed_out", "sessions_ended"];

/** S3-01. Whatever the e-mail is, the answer is the same: the code page (non-enumeration, AC-7). */
export function LoginForm() {
  const t = useTranslations();
  const locale = useAppLocale();
  const path = useLocalePath();
  const router = useRouter();
  const search = useSearchParams();
  const me = useMe();
  const [busy, setBusy] = useState(false);
  const [failure, setFailure] = useState<string | null>(null);
  const [deadline, setDeadline] = useState<number | null>(null);
  const waiting = useCountdown(deadline);
  const form = useForm<Values>({ resolver: zodResolver(schema), defaultValues: { email: readPendingLogin()?.email ?? "", remember: false } });
  const returnTo = safeReturnTo(search.get("returnTo"));
  const reasonParam = search.get("reason");
  const reason = reasons.find((candidate) => candidate === reasonParam);

  useEffect(() => {
    if (me.data) router.replace(landingPath(me.data.locale, returnTo));
  }, [me.data, returnTo, router]);

  async function submit(values: Values) {
    if (busy) return;
    setBusy(true);
    setFailure(null);
    try {
      const challenge = await requestLoginCode(values.email);
      startPendingLogin({ email: values.email, remember: values.remember, challenge });
      router.push(`${path("/login/code")}${returnTo ? `?returnTo=${encodeURIComponent(returnTo)}` : ""}`);
    } catch (error) {
      if (error instanceof ApiError && error.code === "auth.rate_limited") {
        const seconds = error.retryAfter ?? 30;
        setDeadline(Date.now() + seconds * 1000);
        setFailure(t("auth.login.rateLimited", { time: formatWait(seconds, locale) }));
      } else {
        const { message, code } = errorMessage(t, error);
        setFailure(`${t("auth.login.failed")} ${message} ${t("common.errors.code", { code })}`);
      }
    } finally {
      setBusy(false);
    }
  }

  const { errors } = form.formState;
  const blocked = waiting > 0;

  return (
    <AuthCard title={t("auth.login.title")} titleId="login-title" footer={t("auth.login.inviteOnly")}>
      <form aria-labelledby="login-title" noValidate onSubmit={(event) => void form.handleSubmit(submit)(event)} className="grid gap-4">
        <p className="text-sm text-muted-foreground">{t("auth.login.intro")}</p>
        {reason ? <Alert tone="info">{t(`auth.login.reason.${reason}`)}</Alert> : null}
        {failure ? (
          <Alert tone="danger" tabIndex={-1} ref={(node) => node?.focus()}>
            {failure}
          </Alert>
        ) : null}
        <div className="grid gap-2">
          <Label htmlFor="login-email">{t("auth.login.email")}</Label>
          <Input
            id="login-email"
            type="email"
            autoComplete="email"
            inputMode="email"
            autoCapitalize="none"
            spellCheck={false}
            autoFocus
            aria-invalid={errors.email ? true : undefined}
            aria-describedby={errors.email ? "login-email-error" : undefined}
            {...form.register("email")}
          />
          {errors.email ? (
            <p id="login-email-error" className="text-sm text-status-danger">
              {t(`auth.login.errors.${errors.email.message}`)}
            </p>
          ) : null}
        </div>
        <div className="flex items-start gap-3">
          <Controller
            control={form.control}
            name="remember"
            render={({ field }) => <Checkbox id="login-remember" checked={field.value} onCheckedChange={(checked) => field.onChange(checked === true)} aria-describedby="login-remember-help" className="mt-1" />}
          />
          <div className="grid gap-1">
            <Label htmlFor="login-remember" className="min-h-6">
              {t("auth.login.remember")}
            </Label>
            <p id="login-remember-help" className="text-sm text-muted-foreground">
              {t("auth.login.rememberHelp")}
            </p>
          </div>
        </div>
        <Button type="submit" disabled={blocked || busy}>
          {busy ? <Spinner /> : null}
          {busy ? t("auth.login.submitting") : blocked ? formatWait(waiting, locale) : t("auth.login.submit")}
        </Button>
      </form>
    </AuthCard>
  );
}
