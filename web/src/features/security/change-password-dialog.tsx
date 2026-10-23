"use client";

import { useTranslations } from "next-intl";
import { useId, useState, type FormEvent } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { StepUpCancelled, useStepUp } from "@/shared/session/step-up";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { Label } from "@/shared/ui/label";
import { PasswordField } from "@/shared/ui/password-field";
import { Spinner } from "@/shared/ui/spinner";
import { useChangePassword } from "./api";

const min = 12;
const max = 128;

export function ChangePasswordDialog({ open, onClose }: { open: boolean; onClose: () => void }) {
  return open ? <ChangePasswordForm onClose={onClose} /> : null;
}

function ChangePasswordForm({ onClose }: { onClose: () => void }) {
  const t = useTranslations();
  const stepUp = useStepUp();
  const change = useChangePassword();
  const [current, setCurrent] = useState("");
  const [next, setNext] = useState("");
  const [error, setError] = useState<string | null>(null);
  const currentId = useId();
  const nextId = useId();
  const length = [...next].length;

  async function submit(event: FormEvent) {
    event.preventDefault();
    if (change.isPending) return;
    setError(null);
    try {
      await stepUp.run(() => change.mutateAsync({ current_password: current, new_password: next }));
      toast.success(t("security.password.changedToast"));
      onClose();
    } catch (failure) {
      if (failure instanceof StepUpCancelled) return;
      setCurrent("");
      setError(failure instanceof ApiError && failure.code === "auth.reauth_failed" ? t("errors.auth.reauth_failed") : errorMessage(t, failure).message);
    }
  }

  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <form onSubmit={(event) => void submit(event)} className="grid gap-4">
          <DialogHeader>
            <DialogTitle>{t("security.password.title")}</DialogTitle>
            <DialogDescription>{t("security.password.description")}</DialogDescription>
          </DialogHeader>
          {error ? <Alert tone="danger">{error}</Alert> : null}
          <div className="grid gap-2">
            <Label htmlFor={currentId}>{t("security.password.current")}</Label>
            <PasswordField id={currentId} value={current} onChange={(event) => setCurrent(event.target.value)} autoComplete="current-password" autoFocus />
          </div>
          <div className="grid gap-2">
            <Label htmlFor={nextId}>{t("security.password.new")}</Label>
            <PasswordField id={nextId} value={next} onChange={(event) => setNext(event.target.value)} autoComplete="new-password" />
            <p className="text-sm text-muted-foreground">
              {t("auth.invite.passwordHelp")} {t("auth.invite.passwordCount", { count: length })}
            </p>
          </div>
          <DialogFooter>
            <Button type="button" variant="outline" onClick={onClose}>
              {t("common.actions.cancel")}
            </Button>
            <Button type="submit" disabled={change.isPending || !current || length < min || length > max}>
              {change.isPending ? <Spinner /> : null}
              {t("security.password.submit")}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
