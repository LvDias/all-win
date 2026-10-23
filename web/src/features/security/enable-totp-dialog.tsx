"use client";

import { useTranslations } from "next-intl";
import { useCallback, useEffect, useRef, useState } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { useViewOnceLock } from "@/shared/i18n/locale-switch";
import { StepUpCancelled, useStepUp } from "@/shared/session/step-up";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { Label } from "@/shared/ui/label";
import { OtpInput } from "@/shared/ui/otp-input";
import { QrCode } from "@/shared/ui/qr-code";
import { SecretField } from "@/shared/ui/secret-field";
import { Skeleton } from "@/shared/ui/skeleton";
import { Spinner } from "@/shared/ui/spinner";
import { useConfirmTotpSetup, useStartTotpSetup, type TotpSetup } from "./api";
import { RecoveryCodesView } from "./recovery-codes-view";

type Step = "loading" | "scan" | "confirm" | "codes";

/** Groups of four, e.g. `ABCD EFGH IJKL` (F§8.1). */
export function groupKey(secret: string): string {
  return secret.replace(/\s+/g, "").replace(/(.{4})/g, "$1 ").trim();
}

export function EnableTotpDialog({ open, onClose }: { open: boolean; onClose: () => void }) {
  return open ? <EnableFlow onClose={onClose} /> : null;
}

function EnableFlow({ onClose }: { onClose: () => void }) {
  const t = useTranslations();
  const stepUp = useStepUp();
  const start = useStartTotpSetup();
  const confirm = useConfirmTotpSetup();
  const started = useRef(false);
  const [step, setStep] = useState<Step>("loading");
  const [setup, setSetup] = useState<TotpSetup | null>(null);
  const [codes, setCodes] = useState<string[]>([]);
  const [code, setCode] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [expired, setExpired] = useState(false);
  const [acknowledged, setAcknowledged] = useState(false);
  const [guard, setGuard] = useState(false);
  useViewOnceLock(true);

  const begin = useCallback(async () => {
    try {
      const result = await stepUp.run(() => start.mutateAsync());
      setSetup(result);
      setStep("scan");
    } catch (failure) {
      if (failure instanceof StepUpCancelled) onClose();
      else if (failure instanceof ApiError && failure.code === "auth.totp_already_enabled") {
        toast.info(t("security.totp.alreadyEnabled"));
        onClose();
      } else setError(errorMessage(t, failure).message);
    }
    // `start` and `t` are stable enough for a one-shot start; re-running would create a second secret.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [stepUp, onClose]);

  useEffect(() => {
    if (started.current) return;
    started.current = true;
    void begin();
  }, [begin]);

  function restart() {
    setError(null);
    setExpired(false);
    setSetup(null);
    setCode("");
    setStep("loading");
    void begin();
  }

  async function submitCode(value: string) {
    if (confirm.isPending) return;
    setError(null);
    try {
      const result = await confirm.mutateAsync(value);
      setCodes(result.recovery_codes);
      setSetup(null);
      setStep("codes");
      toast.success(t("security.totp.enabledToast"));
    } catch (failure) {
      setCode("");
      if (failure instanceof ApiError && failure.code === "auth.totp_setup_expired") setExpired(true);
      else if (failure instanceof ApiError && failure.code === "auth.invalid_code") setError(t("security.totp.wrongCode"));
      else setError(errorMessage(t, failure).message);
    }
  }

  function requestClose() {
    if (step === "codes" && !acknowledged) setGuard(true);
    else onClose();
  }

  const stepNumber = step === "confirm" ? 2 : step === "codes" ? 3 : 1;

  return (
    <>
      <Dialog open onOpenChange={(next) => !next && requestClose()}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("security.totp.enableTitle")}</DialogTitle>
            <DialogDescription>{t("security.totp.step", { current: stepNumber, total: 3 })}</DialogDescription>
          </DialogHeader>
          {expired ? (
            <div className="grid gap-3">
              <Alert tone="warning">{t("errors.auth.totp_setup_expired")}</Alert>
              <Button onClick={restart}>{t("security.totp.restart")}</Button>
            </div>
          ) : error && step === "loading" ? (
            <div className="grid gap-3">
              <Alert tone="danger">{error}</Alert>
              <Button onClick={restart}>{t("common.actions.retry")}</Button>
            </div>
          ) : step === "loading" ? (
            <div className="grid gap-3">
              <p className="sr-only" role="status">
                {t("common.states.loading")}
              </p>
              <Skeleton className="mx-auto size-48" />
            </div>
          ) : step === "scan" && setup ? (
            <div className="grid gap-4">
              <QrCode value={setup.otpauth_uri} label={t("security.totp.qrAlt")} className="mx-auto size-48 rounded-md border" />
              <p className="text-sm">{t("security.totp.cannotScan")}</p>
              <SecretField value={setup.secret} revealedDisplay={groupKey(setup.secret)} maskedDisplay="•••• •••• •••• ••••" label={t("security.totp.setupKey")} />
              <Button onClick={() => setStep("confirm")}>{t("security.totp.next")}</Button>
            </div>
          ) : step === "confirm" ? (
            <form
              className="grid gap-4"
              onSubmit={(event) => {
                event.preventDefault();
                if (code.length === 6) void submitCode(code);
              }}
            >
              {error ? <Alert tone="danger">{error}</Alert> : null}
              <div className="grid gap-2">
                <Label htmlFor="totp-confirm">{t("security.totp.codeLabel")}</Label>
                <OtpInput id="totp-confirm" value={code} onChange={setCode} onComplete={(value) => void submitCode(value)} readOnly={confirm.isPending} autoFocus />
              </div>
              <div className="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
                <Button type="button" variant="outline" onClick={() => setStep("scan")}>
                  {t("common.actions.back")}
                </Button>
                <Button type="submit" disabled={code.length !== 6 || confirm.isPending}>
                  {confirm.isPending ? <Spinner /> : null}
                  {t("security.totp.verify")}
                </Button>
              </div>
            </form>
          ) : step === "codes" ? (
            <RecoveryCodesView codes={codes} warning={t("security.recovery.warning")} onAcknowledgedChange={setAcknowledged} onDone={onClose} />
          ) : null}
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
