"use client";

import { useTranslations } from "next-intl";
import { useState } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { AlertDialog, AlertDialogContent, AlertDialogDescription, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { useDisableTotp } from "./api";
import { PasswordCodeForm } from "./password-code-form";

/** Wrong re-authentication answers share one message so the dialog never says which factor failed. */
export function reauthErrorMessage(t: ReturnType<typeof useTranslations>, error: unknown): string {
  if (error instanceof ApiError && (error.code === "auth.reauth_failed" || error.code === "auth.invalid_code")) return t("stepUp.wrong");
  return errorMessage(t, error).message;
}

export function DisableTotpDialog({ open, onClose }: { open: boolean; onClose: () => void }) {
  const t = useTranslations();
  const disable = useDisableTotp();
  const [error, setError] = useState<string | null>(null);

  async function submit(values: { password: string; code: string }) {
    setError(null);
    try {
      await disable.mutateAsync(values);
      toast.success(t("security.totp.disabledToast"));
      onClose();
    } catch (failure) {
      setError(reauthErrorMessage(t, failure));
    }
  }

  return (
    <AlertDialog open={open} onOpenChange={(next) => !next && onClose()}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{t("security.totp.disableTitle")}</AlertDialogTitle>
          <AlertDialogDescription>{t("security.totp.disableBody")}</AlertDialogDescription>
        </AlertDialogHeader>
        <PasswordCodeForm confirmLabel={t("security.totp.disableConfirm")} destructive pending={disable.isPending} error={error} onCancel={onClose} onSubmit={(values) => void submit(values)} />
      </AlertDialogContent>
    </AlertDialog>
  );
}
