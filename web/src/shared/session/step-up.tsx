"use client";

import { useTranslations } from "next-intl";
import { createContext, useCallback, useContext, useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { ApiError, apiFetch } from "@/shared/api/client";
import type { ChallengeIssued, StepUpAction, StepUpProof } from "@/shared/api/types";
import { CodeChallenge } from "@/shared/components/code-challenge";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { Skeleton } from "@/shared/ui/skeleton";
import { resendChallenge } from "@/shared/api/challenges";
import { errorMessage } from "@/shared/api/error-message";

/** Raised when the user closes the step-up dialog: the original action was not performed. */
export class StepUpCancelled extends Error {
  constructor() {
    super("step-up cancelled");
    this.name = "StepUpCancelled";
  }
}

export type StepUpRequest = {
  action: StepUpAction;
  /** Required by the contract for some actions (see `requestStepUp`), forbidden for the others. */
  targetId?: string;
  /** Names the action in the dialog, for example "unpair connection “VPS”". */
  description: string;
};

type StepUp = {
  /**
   * Runs `send` without proof. When the Core answers `auth.step_up_required`, asks for a new code for this action and
   * sends the request exactly once more, with `{ challenge_id, code }`. Rejects with `StepUpCancelled` if the user closes the dialog.
   */
  run: <T>(request: StepUpRequest, send: (proof?: StepUpProof) => Promise<T>) => Promise<T>;
};

const StepUpContext = createContext<StepUp | null>(null);

export function useStepUp(): StepUp {
  const context = useContext(StepUpContext);
  if (!context) throw new Error("useStepUp must be used inside StepUpProvider");
  return context;
}

type Pending = {
  request: StepUpRequest;
  /** Sends the action with the proof. Resolves the caller's promise on success; throws on failure. */
  submit: (proof: StepUpProof) => Promise<void>;
  cancel: () => void;
};

export function StepUpProvider({ children }: { children: ReactNode }) {
  const [pending, setPending] = useState<Pending | null>(null);
  const busy = useRef(false);

  const run = useCallback(<T,>(request: StepUpRequest, send: (proof?: StepUpProof) => Promise<T>): Promise<T> => {
    return (async () => {
      try {
        return await send();
      } catch (error) {
        if (!(error instanceof ApiError && error.code === "auth.step_up_required")) throw error;
      }
      // One dialog at a time: a second sensitive action while one is open is refused, not queued.
      if (busy.current) throw new StepUpCancelled();
      busy.current = true;
      return new Promise<T>((resolve, reject) => {
        const close = () => {
          busy.current = false;
          setPending(null);
        };
        setPending({
          request,
          submit: async (proof) => {
            try {
              resolve(await send(proof));
              close();
            } catch (failure) {
              // A wrong code keeps the dialog open; any other failure ends the attempt and reaches the caller.
              if (failure instanceof ApiError && (failure.code === "auth.code_invalid" || failure.code === "auth.rate_limited")) throw failure;
              close();
              reject(failure);
            }
          },
          cancel: () => {
            close();
            reject(new StepUpCancelled());
          },
        });
      });
    })();
  }, []);

  const value = useMemo<StepUp>(() => ({ run }), [run]);
  return (
    <StepUpContext.Provider value={value}>
      {children}
      <Dialog open={pending !== null} onOpenChange={(next) => !next && pending?.cancel()}>
        <DialogContent>{pending ? <StepUpForm key={pending.request.action + (pending.request.targetId ?? "")} pending={pending} /> : null}</DialogContent>
      </Dialog>
    </StepUpContext.Provider>
  );
}

/** Mounted only while the dialog is open, so the challenge and the code start fresh every time. */
function StepUpForm({ pending }: { pending: Pending }) {
  const t = useTranslations();
  const { request } = pending;
  const [challenge, setChallenge] = useState<ChallengeIssued | null>(null);
  const [failure, setFailure] = useState<string | null>(null);
  const [attempt, setAttempt] = useState(0);

  const issue = useCallback(async (): Promise<ChallengeIssued> => {
    return apiFetch<ChallengeIssued>("/api/v1/auth/step-up", {
      method: "POST",
      body: { action: request.action, channel: "email", ...(request.targetId ? { target_id: request.targetId } : {}) },
    });
  }, [request.action, request.targetId]);

  // Ask for the code once per attempt.
  useEffect(() => {
    let current = true;
    issue().then(
      (issued) => current && setChallenge(issued),
      (error: unknown) => current && setFailure(errorMessage(t, error).message),
    );
    return () => {
      current = false;
    };
  }, [attempt, issue, t]);

  return (
    <div className="grid gap-4">
      <DialogHeader>
        <DialogTitle>{t("stepUp.title")}</DialogTitle>
        <DialogDescription>{t("stepUp.description", { action: request.description })}</DialogDescription>
      </DialogHeader>
      {failure ? (
        <>
          <Alert tone="danger">{failure}</Alert>
          <DialogFooter>
            <Button type="button" variant="outline" onClick={pending.cancel}>
              {t("common.actions.cancel")}
            </Button>
            <Button type="button" onClick={() => {
                setFailure(null);
                setAttempt((count) => count + 1);
              }}>
              {t("common.actions.retry")}
            </Button>
          </DialogFooter>
        </>
      ) : challenge ? (
        <CodeChallenge
          challenge={challenge}
          description={t("stepUp.sent")}
          submitLabel={t("stepUp.confirm")}
          onVerify={async (code) => {
            await pending.submit({ challenge_id: challenge.challenge_id, code });
          }}
          onResend={async () => {
            const next = await resendChallenge(challenge.challenge_id);
            setChallenge(next);
            return next;
          }}
          onNewChallenge={async () => {
            const next = await issue();
            setChallenge(next);
            return next;
          }}
          secondaryActions={
            <Button type="button" variant="ghost" onClick={pending.cancel}>
              {t("common.actions.cancel")}
            </Button>
          }
        />
      ) : (
        <div className="grid gap-2" aria-busy="true">
          <Skeleton className="h-12 w-full" />
          <p className="sr-only">{t("common.states.loading")}</p>
        </div>
      )}
    </div>
  );
}
