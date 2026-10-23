"use client";

import { useTranslations } from "next-intl";
import Link from "next/link";
import { useState, type FormEvent } from "react";
import { errorMessage } from "@/shared/api/error-message";
import { useLocalePath } from "@/shared/i18n/navigation";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { Spinner } from "@/shared/ui/spinner";
import { useRequestPasswordReset } from "./api";
import { AuthCard } from "./auth-card";

export function ForgotPasswordForm() {
  const t = useTranslations();
  const path = useLocalePath();
  const request = useRequestPasswordReset();
  const [email, setEmail] = useState("");
  const [sent, setSent] = useState(false);
  const [failure, setFailure] = useState<string | null>(null);

  async function submit(event: FormEvent) {
    event.preventDefault();
    if (request.isPending || email.trim() === "") return;
    setFailure(null);
    try {
      await request.mutateAsync(email.trim());
      setSent(true);
    } catch (error) {
      // The Core always answers 202; only transport failures reach here. Throttling is not told apart (AC-5).
      const { message, code } = errorMessage(t, error);
      setFailure(`${message} ${t("common.errors.code", { code })}`);
    }
  }

  return (
    <AuthCard title={t("auth.forgot.title")} titleId="forgot-title">
      {sent ? (
        <>
          <Alert tone="success">{t("auth.forgot.sent")}</Alert>
          <Link href={path("/login")} className="inline-flex min-h-11 items-center justify-center text-sm underline underline-offset-4">
            {t("auth.forgot.backToLogin")}
          </Link>
        </>
      ) : (
        <form aria-labelledby="forgot-title" onSubmit={(event) => void submit(event)} className="grid gap-4">
          <p className="text-sm text-muted-foreground">{t("auth.forgot.intro")}</p>
          {failure ? <Alert tone="danger">{failure}</Alert> : null}
          <div className="grid gap-2">
            <Label htmlFor="forgot-email">{t("auth.forgot.email")}</Label>
            <Input id="forgot-email" type="email" autoComplete="username" autoFocus required value={email} onChange={(event) => setEmail(event.target.value)} />
          </div>
          <Button type="submit" disabled={request.isPending}>
            {request.isPending ? <Spinner /> : null}
            {request.isPending ? t("auth.forgot.submitting") : t("auth.forgot.submit")}
          </Button>
          <Link href={path("/login")} className="inline-flex min-h-11 items-center justify-center text-sm underline underline-offset-4">
            {t("auth.forgot.backToLogin")}
          </Link>
        </form>
      )}
    </AuthCard>
  );
}
