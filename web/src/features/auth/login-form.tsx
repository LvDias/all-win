"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useQueryClient } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { useRouter, useSearchParams } from "next/navigation";
import { useEffect, useRef, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { z } from "zod";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { formatWait } from "@/shared/format";
import { useAppLocale, useLocalePath } from "@/shared/i18n/navigation";
import { useCountdown } from "@/shared/lib/use-countdown";
import { useMe, meQueryKey } from "@/shared/session/me";
import { landingPath, safeReturnTo, type LoginReason } from "@/shared/session/redirects";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Checkbox } from "@/shared/ui/checkbox";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { PasswordField } from "@/shared/ui/password-field";
import { Spinner } from "@/shared/ui/spinner";
import { useLogin } from "./api";
import { AuthCard } from "./auth-card";

const schema = z.object({
  email: z.string().trim().min(1, "emailRequired").regex(/^\S+@\S+\.\S+$/, "emailInvalid"),
  password: z.string().min(1, "passwordRequired"),
  remember: z.boolean(),
});
type Values = z.infer<typeof schema>;

const reasons: readonly LoginReason[] = ["session_expired", "signed_out", "sessions_ended", "pending_expired"];

export function LoginForm() {
  const t = useTranslations();
  const locale = useAppLocale();
  const path = useLocalePath();
  const router = useRouter();
  const search = useSearchParams();
  const queryClient = useQueryClient();
  const me = useMe();
  const login = useLogin();
  const passwordRef = useRef<HTMLInputElement | null>(null);
  const [failure, setFailure] = useState<string | null>(null);
  const [deadline, setDeadline] = useState<number | null>(null);
  const waiting = useCountdown(deadline);

  const form = useForm<Values>({ resolver: zodResolver(schema), defaultValues: { email: "", password: "", remember: false } });
  const returnTo = safeReturnTo(search.get("returnTo"));
  const reasonParam = search.get("reason");
  const reason = reasons.find((candidate) => candidate === reasonParam);

  useEffect(() => {
    if (me.data) router.replace(landingPath(me.data.locale, returnTo));
  }, [me.data, returnTo, router]);

  async function submit(values: Values) {
    if (login.isPending) return;
    setFailure(null);
    try {
      const result = await login.mutateAsync({ email: values.email, password: values.password, remember_me: values.remember });
      if (result.state === "pending_2fa") {
        router.push(`${path("/login/verify")}${returnTo ? `?returnTo=${encodeURIComponent(returnTo)}` : ""}`);
        return;
      }
      if (result.user) {
        queryClient.setQueryData(meQueryKey, result.user);
        router.replace(landingPath(result.user.locale, returnTo));
      }
    } catch (error) {
      form.resetField("password");
      if (error instanceof ApiError && error.code === "auth.rate_limited") {
        const seconds = error.retryAfter ?? 30;
        setDeadline(Date.now() + seconds * 1000);
        setFailure(t("auth.login.rateLimited", { time: formatWait(seconds, locale) }));
      } else if (error instanceof ApiError && error.code === "auth.invalid_credentials") {
        setFailure(errorMessage(t, error).message);
        passwordRef.current?.focus();
      } else {
        const { message, code } = errorMessage(t, error);
        setFailure(`${t("auth.login.failed")} ${message} ${t("common.errors.code", { code })}`);
      }
    }
  }

  const errorKey = (message: string | undefined) => (message ? t(`auth.login.errors.${message}`) : undefined);
  const { errors } = form.formState;
  const blocked = waiting > 0;

  return (
    <AuthCard title={t("auth.login.title")} titleId="login-title" footer={t("auth.login.inviteOnly")}>
      <form aria-labelledby="login-title" noValidate onSubmit={(event) => void form.handleSubmit(submit)(event)} className="grid gap-4">
        {reason ? <Alert tone="info">{t(`auth.login.reason.${reason}`)}</Alert> : null}
        {failure ? (
          <Alert tone="danger" tabIndex={-1} ref={(node) => node?.focus()}>
            {failure}
          </Alert>
        ) : null}
        <div className="grid gap-2">
          <Label htmlFor="login-email">{t("auth.login.email")}</Label>
          <Input id="login-email" type="email" autoComplete="username" autoFocus aria-invalid={errors.email ? true : undefined} aria-describedby={errors.email ? "login-email-error" : undefined} {...form.register("email")} />
          {errors.email ? (
            <p id="login-email-error" className="text-sm text-status-danger">
              {errorKey(errors.email.message)}
            </p>
          ) : null}
        </div>
        <div className="grid gap-2">
          <Label htmlFor="login-password">{t("auth.login.password")}</Label>
          <PasswordField
            id="login-password"
            autoComplete="current-password"
            aria-invalid={errors.password ? true : undefined}
            aria-describedby={errors.password ? "login-password-error" : undefined}
            {...form.register("password")}
            ref={(node) => {
              form.register("password").ref(node);
              passwordRef.current = node;
            }}
          />
          {errors.password ? (
            <p id="login-password-error" className="text-sm text-status-danger">
              {errorKey(errors.password.message)}
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
        <Button type="submit" disabled={blocked || login.isPending}>
          {login.isPending ? <Spinner /> : null}
          {login.isPending ? t("auth.login.submitting") : blocked ? formatWait(waiting, locale) : t("auth.login.submit")}
        </Button>
        <Link href={path("/forgot-password")} className="inline-flex min-h-11 items-center justify-center text-sm underline underline-offset-4">
          {t("auth.login.forgot")}
        </Link>
      </form>
    </AuthCard>
  );
}
