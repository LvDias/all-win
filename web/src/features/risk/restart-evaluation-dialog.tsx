"use client";

import { useQueryClient } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import { useId, useRef, useState, type FormEvent } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { parseDecimalInput } from "@/shared/format";
import { useDisplay } from "@/shared/session/display";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { Spinner } from "@/shared/ui/spinner";
import { isZeroAmount } from "./amounts";
import { riskQueryKey, useAccountVersion, useRestartEvaluation } from "./api";

type Props = { accountId: string; currentStart: string; currentEarnedBefore: string; onClose: () => void };

/** S13b-02. After creation the start and the amount earned before change only through `restartEvaluation`, so this dialog is a restart. */
export function RestartEvaluationDialog({ accountId, currentStart, currentEarnedBefore, onClose }: Props) {
  const t = useTranslations("risk.evaluation.restart");
  const root = useTranslations();
  const queryClient = useQueryClient();
  const { locale } = useDisplay();
  const version = useAccountVersion(accountId, true);
  const restart = useRestartEvaluation(accountId);
  const startId = useId();
  const earnedId = useId();
  const submitting = useRef(false);
  const [start, setStart] = useState(currentStart);
  const [earned, setEarned] = useState(isZeroAmount(currentEarnedBefore) ? "" : currentEarnedBefore);
  const [confirming, setConfirming] = useState(false);
  const [fieldErrors, setFieldErrors] = useState<{ start?: string; earned?: string }>({});
  const [failure, setFailure] = useState<{ message: string; reload: boolean } | null>(null);

  const dirty = start !== currentStart || earned !== (isZeroAmount(currentEarnedBefore) ? "" : currentEarnedBefore);

  function review(event: FormEvent) {
    event.preventDefault();
    const errors: { start?: string; earned?: string } = {};
    if (!/^\d{4}-\d{2}-\d{2}$/.test(start)) errors.start = t("errors.startRequired");
    if (earned.trim() !== "") {
      const parsed = parseDecimalInput(earned, locale, { scale: 4 });
      if (!parsed.ok) errors.earned = t("errors.earnedInvalid");
    }
    setFieldErrors(errors);
    setFailure(null);
    if (!errors.start && !errors.earned) setConfirming(true);
  }

  async function confirm() {
    if (submitting.current || !version.data) return;
    submitting.current = true;
    const parsed = earned.trim() === "" ? null : parseDecimalInput(earned, locale, { scale: 4 });
    try {
      await restart.mutateAsync({ evaluation_start: start, ...(parsed?.ok ? { evaluation_earned_before: parsed.value } : {}), row_version: version.data.row_version });
      toast.success(t("done"));
      onClose();
    } catch (error) {
      setConfirming(false);
      if (error instanceof ApiError && error.status === 409) setFailure({ message: t("errors.conflict"), reload: true });
      else if (error instanceof ApiError && error.code === "accounts.evaluation_not_applicable") {
        toast.error(root("errors.accounts.evaluation_not_applicable"));
        void queryClient.invalidateQueries({ queryKey: riskQueryKey });
        onClose();
      } else if (error instanceof ApiError && error.pointer === "/evaluation_start") setFieldErrors({ start: errorMessage(root, error).message });
      else if (error instanceof ApiError && error.pointer === "/evaluation_earned_before") setFieldErrors({ earned: errorMessage(root, error).message });
      else setFailure({ message: errorMessage(root, error).message, reload: false });
    } finally {
      submitting.current = false;
    }
  }

  function reload() {
    void version.refetch();
    setFailure(null);
  }

  return (
    <>
      <Dialog open onOpenChange={(open) => !open && !restart.isPending && onClose()}>
        <DialogContent>
          <form onSubmit={review} className="grid gap-4" noValidate>
            <DialogHeader>
              <DialogTitle>{t("title")}</DialogTitle>
              <DialogDescription>{t("description")}</DialogDescription>
            </DialogHeader>
            {failure ? (
              <Alert tone="danger" tabIndex={-1} ref={(node) => node?.focus()}>
                <div className="grid gap-2">
                  <p>{failure.message}</p>
                  {failure.reload ? (
                    <Button type="button" size="sm" variant="outline" onClick={reload}>
                      {t("reload")}
                    </Button>
                  ) : null}
                </div>
              </Alert>
            ) : null}
            <div className="grid gap-2">
              <Label htmlFor={startId}>{t("start")}</Label>
              <Input id={startId} type="date" value={start} onChange={(event) => setStart(event.target.value)} aria-invalid={fieldErrors.start ? true : undefined} aria-describedby={fieldErrors.start ? `${startId}-error` : undefined} autoFocus />
              {fieldErrors.start ? (
                <p id={`${startId}-error`} className="text-sm text-status-danger">
                  {fieldErrors.start}
                </p>
              ) : null}
            </div>
            <div className="grid gap-2">
              <Label htmlFor={earnedId}>{t("earnedBefore")}</Label>
              <Input id={earnedId} inputMode="decimal" value={earned} onChange={(event) => setEarned(event.target.value)} autoComplete="off" aria-invalid={fieldErrors.earned ? true : undefined} aria-describedby={fieldErrors.earned ? `${earnedId}-error` : undefined} />
              {fieldErrors.earned ? (
                <p id={`${earnedId}-error`} className="text-sm text-status-danger">
                  {fieldErrors.earned}
                </p>
              ) : null}
            </div>
            <DialogFooter>
              <Button type="button" variant="outline" onClick={onClose}>
                {root("common.actions.cancel")}
              </Button>
              <Button type="submit" disabled={!dirty || !version.data}>
                {version.isPending ? <Spinner /> : null}
                {t("submit")}
              </Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>
      <AlertDialog open={confirming} onOpenChange={(open) => !open && !restart.isPending && setConfirming(false)}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("confirm.title")}</AlertDialogTitle>
            <AlertDialogDescription>{t("confirm.body")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{t("confirm.back")}</AlertDialogCancel>
            <AlertDialogAction
              disabled={restart.isPending}
              onClick={(event) => {
                event.preventDefault();
                void confirm();
              }}
            >
              {restart.isPending ? <Spinner /> : null}
              {t("confirm.action")}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
}
