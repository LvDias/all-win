"use client";

import { useTranslations } from "next-intl";
import { useState } from "react";
import { useViewOnceLock } from "@/shared/i18n/locale-switch";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { useRegenerateRecoveryCodes } from "./api";
import { reauthErrorMessage } from "./disable-totp-dialog";
import { PasswordCodeForm } from "./password-code-form";
import { RecoveryCodesView } from "./recovery-codes-view";

export function RegenerateCodesDialog({ open, onClose }: { open: boolean; onClose: () => void }) {
  return open ? <RegenerateFlow onClose={onClose} /> : null;
}

function RegenerateFlow({ onClose }: { onClose: () => void }) {
  const t = useTranslations();
  const regenerate = useRegenerateRecoveryCodes();
  const [codes, setCodes] = useState<string[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [acknowledged, setAcknowledged] = useState(false);
  const [guard, setGuard] = useState(false);
  useViewOnceLock(codes !== null);

  async function submit(values: { password: string; code: string }) {
    setError(null);
    try {
      const result = await regenerate.mutateAsync(values);
      setCodes(result.recovery_codes);
    } catch (failure) {
      setError(reauthErrorMessage(t, failure));
    }
  }

  function requestClose() {
    if (codes && !acknowledged) setGuard(true);
    else onClose();
  }

  return (
    <>
      <Dialog open onOpenChange={(next) => !next && requestClose()}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("security.recovery.regenerateTitle")}</DialogTitle>
            <DialogDescription>{codes ? t("security.recovery.warning") : t("security.recovery.regenerateBody")}</DialogDescription>
          </DialogHeader>
          {codes ? (
            <RecoveryCodesView codes={codes} warning={t("security.recovery.replacedWarning")} onAcknowledgedChange={setAcknowledged} onDone={onClose} />
          ) : (
            <PasswordCodeForm confirmLabel={t("security.recovery.regenerateConfirm")} pending={regenerate.isPending} error={error} onCancel={onClose} onSubmit={(values) => void submit(values)} />
          )}
        </DialogContent>
      </Dialog>
      <AlertDialog open={guard} onOpenChange={setGuard}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("security.recovery.guardTitle")}</AlertDialogTitle>
            <AlertDialogDescription>{t("security.recovery.guard")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{t("security.recovery.guardBack")}</AlertDialogCancel>
            <AlertDialogAction onClick={onClose}>{t("security.recovery.guardClose")}</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
}
