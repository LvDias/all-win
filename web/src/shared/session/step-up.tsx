"use client";

import { useMutation } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import { createContext, useCallback, useContext, useId, useMemo, useRef, useState, type FormEvent, type ReactNode } from "react";
import { ApiError, apiFetch } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { useSecurity } from "@/shared/session/security";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { Label } from "@/shared/ui/label";
import { OtpInput } from "@/shared/ui/otp-input";
import { PasswordField } from "@/shared/ui/password-field";
import { Spinner } from "@/shared/ui/spinner";

/** Raised when the user closes the step-up dialog: the original action was not performed. */
export class StepUpCancelled extends Error {
  constructor() {
    super("step-up cancelled");
    this.name = "StepUpCancelled";
  }
}

type StepUp = {
  /** Runs `action`; on `auth.step_up_required` asks for re-authentication, then retries the action exactly once. */
  run: <T>(action: () => Promise<T>) => Promise<T>;
};

const StepUpContext = createContext<StepUp | null>(null);

export function useStepUp(): StepUp {
  const context = useContext(StepUpContext);
  if (!context) throw new Error("useStepUp must be used inside StepUpProvider");
  return context;
}

export function StepUpProvider({ children }: { children: ReactNode }) {
  const [open, setOpen] = useState(false);
  const pending = useRef<{ promise: Promise<boolean>; resolve: (ok: boolean) => void } | null>(null);

  const request = useCallback((): Promise<boolean> => {
    if (pending.current) return pending.current.promise;
    let resolve: (ok: boolean) => void = () => {};
    const promise = new Promise<boolean>((done) => {
      resolve = done;
    });
    pending.current = { promise, resolve };
    setOpen(true);
    return promise;
  }, []);

  const settle = useCallback((ok: boolean) => {
    pending.current?.resolve(ok);
    pending.current = null;
    setOpen(false);
  }, []);

  const run = useCallback(
    async <T,>(action: () => Promise<T>): Promise<T> => {
      try {
        return await action();
      } catch (error) {
        if (!(error instanceof ApiError && error.code === "auth.step_up_required")) throw error;
        if (!(await request())) throw new StepUpCancelled();
        return action();
      }
    },
    [request],
  );

  const value = useMemo<StepUp>(() => ({ run }), [run]);
  return (
    <StepUpContext.Provider value={value}>
      {children}
      <StepUpDialog open={open} onSettled={settle} />
    </StepUpContext.Provider>
  );
}

function StepUpDialog({ open, onSettled }: { open: boolean; onSettled: (ok: boolean) => void }) {
  return (
    <Dialog open={open} onOpenChange={(next) => !next && onSettled(false)}>
      <DialogContent>
        <StepUpForm onSettled={onSettled} />
      </DialogContent>
    </Dialog>
  );
}

/** Mounted only while the dialog is open, so its fields start empty every time. */
function StepUpForm({ onSettled }: { onSettled: (ok: boolean) => void }) {
  const t = useTranslations();
  const security = useSecurity();
  const [password, setPassword] = useState("");
  const [code, setCode] = useState("");
  const [error, setError] = useState<string | null>(null);
  const passwordId = useId();
  const codeId = useId();
  const totp = security.data?.totp_enabled === true;

  const reauthenticate = useMutation({
    mutationFn: () => apiFetch<void>("/api/v1/auth/step-up", { method: "POST", body: totp ? { password, code } : { password } }),
  });

  async function submit(event: FormEvent) {
    event.preventDefault();
    if (reauthenticate.isPending) return;
    setError(null);
    try {
      await reauthenticate.mutateAsync();
      onSettled(true);
    } catch (failure) {
      if (failure instanceof ApiError && (failure.code === "auth.reauth_failed" || failure.code === "auth.invalid_code")) setError(t("stepUp.wrong"));
      else setError(errorMessage(t, failure).message);
      setPassword("");
      setCode("");
    }
  }

  // Wait for the security state: a TOTP user who submits before it loads would send the password alone.
  const ready = (security.isSuccess || security.isError) && password.length > 0 && (!totp || code.length === 6);

  return (
    <form onSubmit={(event) => void submit(event)} className="grid gap-4">
      <DialogHeader>
        <DialogTitle>{t("stepUp.title")}</DialogTitle>
        <DialogDescription>{t("stepUp.description")}</DialogDescription>
      </DialogHeader>
      {error ? <Alert tone="danger">{error}</Alert> : null}
      <div className="grid gap-2">
        <Label htmlFor={passwordId}>{t("stepUp.password")}</Label>
        <PasswordField id={passwordId} value={password} onChange={(event) => setPassword(event.target.value)} autoComplete="current-password" autoFocus />
      </div>
      {totp ? (
        <div className="grid gap-2">
          <Label htmlFor={codeId}>{t("stepUp.code")}</Label>
          <OtpInput id={codeId} value={code} onChange={setCode} />
        </div>
      ) : null}
      <DialogFooter>
        <Button type="button" variant="outline" onClick={() => onSettled(false)}>
          {t("common.actions.cancel")}
        </Button>
        <Button type="submit" disabled={!ready || reauthenticate.isPending}>
          {reauthenticate.isPending ? <Spinner /> : null}
          {t("stepUp.confirm")}
        </Button>
      </DialogFooter>
    </form>
  );
}
