"use client";

import { useTranslations } from "next-intl";
import { useState } from "react";
import { toast } from "sonner";
import { errorMessage } from "@/shared/api/error-message";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { useSetLifecycle } from "./api/use-accounts";
import type { Account } from "./api/types";

export type LifecycleAction = "pause" | "disable";

/** Pause and disable confirm; resume and enable are one click (see `useLifecycleQuickAction`). */
export function LifecycleDialog({ account, action, onClose }: { account: Account; action: LifecycleAction; onClose: () => void }) {
  const t = useTranslations();
  const setLifecycle = useSetLifecycle();
  const [failure, setFailure] = useState<string | null>(null);
  const paused = action === "pause";

  function confirm() {
    setFailure(null);
    setLifecycle.mutate(
      { id: account.id, state: paused ? "paused" : "disabled", rowVersion: account.row_version },
      {
        onSuccess: () => {
          toast.success(t(paused ? "accounts.lifecycle.paused" : "accounts.lifecycle.disabled"));
          onClose();
        },
        onError: (error) => setFailure(errorMessage(t, error).message),
      },
    );
  }

  return (
    <AlertDialog open onOpenChange={(open) => (open ? undefined : onClose())}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{t(paused ? "accounts.lifecycle.pauseTitle" : "accounts.lifecycle.disableTitle", { name: account.display_name })}</AlertDialogTitle>
          <AlertDialogDescription>
            {t("accounts.lifecycle.body", { state: t(paused ? "accounts.lifecycle.statePaused" : "accounts.lifecycle.stateDisabled") })}
          </AlertDialogDescription>
          <p className="text-sm text-muted-foreground">{t(paused ? "accounts.lifecycle.pauseHelp" : "accounts.lifecycle.disableHelp")}</p>
        </AlertDialogHeader>
        {account.has_open_position ? <Alert tone="warning">{t("accounts.lifecycle.openPosition")}</Alert> : null}
        {failure ? <Alert tone="danger">{failure}</Alert> : null}
        <AlertDialogFooter>
          <AlertDialogCancel autoFocus disabled={setLifecycle.isPending}>{t("common.actions.cancel")}</AlertDialogCancel>
          <AlertDialogAction
            variant="destructive"
            disabled={setLifecycle.isPending}
            onClick={(event) => {
              event.preventDefault();
              confirm();
            }}
          >
            {setLifecycle.isPending ? t("common.states.saving") : t(paused ? "accounts.lifecycle.pauseConfirm" : "accounts.lifecycle.disableConfirm")}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}

/** Resume (paused → active) and enable (disabled → active): no confirmation. */
export function useLifecycleQuickAction() {
  const t = useTranslations();
  const setLifecycle = useSetLifecycle();
  return {
    pending: setLifecycle.isPending,
    run(account: Account) {
      const message = account.lifecycle_state === "paused" ? "accounts.lifecycle.resumed" : "accounts.lifecycle.enabled";
      setLifecycle.mutate(
        { id: account.id, state: "active", rowVersion: account.row_version },
        {
          onSuccess: () => toast.success(t(message)),
          onError: (error) => toast.error(errorMessage(t, error).message),
        },
      );
    },
  };
}
