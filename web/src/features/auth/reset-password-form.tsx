"use client";

import { useTranslations } from "next-intl";
import Link from "next/link";
import { useState, type FormEvent } from "react";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { useLocalePath } from "@/shared/i18n/navigation";
import { Alert } from "@/shared/ui/alert";
import { Button, buttonVariants } from "@/shared/ui/button";
import { Label } from "@/shared/ui/label";
import { PasswordField } from "@/shared/ui/password-field";
import { Spinner } from "@/shared/ui/spinner";
import { useConfirmPasswordReset } from "./api";
import { AuthCard } from "./auth-card";
import { passwordMax, passwordMin } from "./password-rules";
import { useFragmentToken } from "./use-fragment-token";

export function ResetPasswordForm() {
  const t = useTranslations();
  const path = useLocalePath();
  const { checked, token } = useFragmentToken();
  const confirm = useConfirmPasswordReset();
  const [password, setPassword] = useState("");
  const [state, setState] = useState<"form" | "done" | "invalid">("form");
  const [failure, setFailure] = useState<string | null>(null);

  async function submit(event: FormEvent) {
    event.preventDefault();
    if (confirm.isPending || !token) return;
    setFailure(null);
    try {
      await confirm.mutateAsync({ token, new_password: password });
      setPassword("");
      setState("done");
    } catch (error) {
      if (error instanceof ApiError && error.code === "auth.reset_token_invalid") setState("invalid");
      else {
        const { message, code } = errorMessage(t, error);
        setFailure(`${message} ${t("common.errors.code", { code })}`);
      }
    }
  }

  if (!checked) return null;

  if (state === "done") {
    return (
      <AuthCard title={t("auth.reset.doneTitle")} titleId="reset-title" focusTitle>
        <p className="text-sm">{t("auth.reset.done")}</p>
        <Link href={path("/login")} className={buttonVariants()}>
          {t("auth.reset.signIn")}
        </Link>
        <Alert tone="info">
          <div className="grid gap-2">
            <p>{t("auth.reset.review")}</p>
            <p className="flex flex-wrap gap-x-4">
              <Link className="underline underline-offset-4" href={`${path("/login")}?returnTo=${encodeURIComponent(path("/connections/sources"))}`}>
                {t("auth.reset.reviewSources")}
              </Link>
              <Link className="underline underline-offset-4" href={`${path("/login")}?returnTo=${encodeURIComponent(path("/connections/ninjatrader"))}`}>
                {t("auth.reset.reviewConnections")}
              </Link>
            </p>
          </div>
        </Alert>
      </AuthCard>
    );
  }

  if (state === "invalid" || !token) {
    return (
      <AuthCard title={t("auth.reset.title")} titleId="reset-title">
        <Alert tone="danger">{t("auth.reset.invalid")}</Alert>
        <Link href={path("/forgot-password")} className={buttonVariants({ variant: "outline" })}>
          {t("auth.reset.requestNew")}
        </Link>
      </AuthCard>
    );
  }

  const length = [...password].length;
  return (
    <AuthCard title={t("auth.reset.title")} titleId="reset-title">
      <form aria-labelledby="reset-title" onSubmit={(event) => void submit(event)} className="grid gap-4">
        {failure ? <Alert tone="danger" tabIndex={-1} ref={(node) => node?.focus()}>{failure}</Alert> : null}
        <div className="grid gap-2">
          <Label htmlFor="reset-password">{t("auth.reset.password")}</Label>
          <PasswordField id="reset-password" autoComplete="new-password" autoFocus value={password} onChange={(event) => setPassword(event.target.value)} aria-describedby="reset-password-help" />
          <p id="reset-password-help" className="text-sm text-muted-foreground">
            {t("auth.invite.passwordHelp")} {t("auth.invite.passwordCount", { count: length })}
          </p>
        </div>
        <Button type="submit" disabled={confirm.isPending || length < passwordMin || length > passwordMax}>
          {confirm.isPending ? <Spinner /> : null}
          {confirm.isPending ? t("auth.reset.submitting") : t("auth.reset.submit")}
        </Button>
      </form>
    </AuthCard>
  );
}
