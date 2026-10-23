"use client";

import { useTranslations } from "next-intl";
import { useId, useState, type FormEvent } from "react";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Label } from "@/shared/ui/label";
import { OtpInput } from "@/shared/ui/otp-input";
import { PasswordField } from "@/shared/ui/password-field";
import { Spinner } from "@/shared/ui/spinner";

type Props = {
  confirmLabel: string;
  destructive?: boolean;
  pending: boolean;
  error: string | null;
  onCancel: () => void;
  onSubmit: (values: { password: string; code: string }) => void;
};

/** Current password plus a current authenticator code: re-authentication for disabling TOTP or new recovery codes (Q22). */
export function PasswordCodeForm({ confirmLabel, destructive = false, pending, error, onCancel, onSubmit }: Props) {
  const t = useTranslations();
  const [password, setPassword] = useState("");
  const [code, setCode] = useState("");
  const passwordId = useId();
  const codeId = useId();

  function submit(event: FormEvent) {
    event.preventDefault();
    if (pending || !password || code.length !== 6) return;
    onSubmit({ password, code });
    setCode("");
  }

  return (
    <form onSubmit={submit} className="grid gap-4">
      {error ? <Alert tone="danger">{error}</Alert> : null}
      <div className="grid gap-2">
        <Label htmlFor={passwordId}>{t("stepUp.password")}</Label>
        <PasswordField id={passwordId} value={password} onChange={(event) => setPassword(event.target.value)} autoComplete="current-password" autoFocus />
      </div>
      <div className="grid gap-2">
        <Label htmlFor={codeId}>{t("stepUp.code")}</Label>
        <OtpInput id={codeId} value={code} onChange={setCode} />
      </div>
      <div className="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
        <Button type="button" variant="outline" onClick={onCancel}>
          {t("common.actions.cancel")}
        </Button>
        <Button type="submit" variant={destructive ? "destructive" : "default"} disabled={pending || !password || code.length !== 6}>
          {pending ? <Spinner /> : null}
          {confirmLabel}
        </Button>
      </div>
    </form>
  );
}
