"use client";

import { useTranslations } from "next-intl";
import { useEffect, useId, useRef, useState, type FormEvent, type ReactNode } from "react";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import type { ChallengeDelivery, ChallengeIssued } from "@/shared/api/types";
import { formatClock, formatWait } from "@/shared/format/duration";
import { useAppLocale } from "@/shared/i18n/navigation";
import { useCountdown } from "@/shared/lib/use-countdown";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { CodeInput, defaultCodeLength } from "@/shared/ui/code-input";
import { Label } from "@/shared/ui/label";
import { Spinner } from "@/shared/ui/spinner";

/** The UI's own count of failed attempts per challenge (a resend does not reset it); the server never reveals exhaustion. */
const maxAttempts = 5;

type CodeChallengeProps = {
  challenge: ChallengeIssued;
  /** Text that says where the code went. */
  description: ReactNode;
  /** Resolves when the code was accepted. Throws the `ApiError` otherwise. The challenge id is the caller's current one. */
  onVerify: (code: string) => Promise<void>;
  /** Replaces the code of the same challenge; returns the new timing. */
  onResend: () => Promise<ChallengeIssued>;
  /** Starts a new challenge when this one is closed. */
  onNewChallenge: () => Promise<ChallengeIssued>;
  /** Only for sign-up, step-up and channel flows; never for login. */
  delivery?: ChallengeDelivery;
  length?: number;
  autoFocus?: boolean;
  secondaryActions?: ReactNode;
  /** Rendered between the code field and the buttons (for example "keep me signed in"). */
  extra?: ReactNode;
  submitLabel?: string;
};

/**
 * The verification block shared by sign-in, sign-up, step-up and channel change (components.md C§1.3).
 * The code lives only in this component's state and is cleared after every attempt.
 */
export function CodeChallenge({ challenge, description, onVerify, onResend, onNewChallenge, delivery, length = defaultCodeLength, autoFocus = true, secondaryActions, extra, submitLabel }: CodeChallengeProps) {
  const t = useTranslations();
  const locale = useAppLocale();
  const inputId = useId();
  const errorId = useId();
  const [code, setCode] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [attempts, setAttempts] = useState(0);
  const [resent, setResent] = useState(false);
  const [cooldownUntil, setCooldownUntil] = useState(() => Date.now() + challenge.resend_after_s * 1000);
  const [expiresAt, setExpiresAt] = useState(() => Date.parse(challenge.expires_at));
  const [announce, setAnnounce] = useState("");
  const input = useRef<HTMLDivElement>(null);

  const cooldown = useCountdown(cooldownUntil);
  const remaining = useCountdown(expiresAt);
  const expired = remaining === 0;
  const locked = attempts >= maxAttempts || expired;

  // Refocus from an effect after a failed attempt (no timer that could fire after unmount).
  const [focusTick, setFocusTick] = useState(0);
  useEffect(() => {
    if (focusTick > 0) input.current?.querySelector("input")?.focus();
  }, [focusTick]);
  const focusInput = () => setFocusTick((tick) => tick + 1);

  function apply(next: ChallengeIssued) {
    setCooldownUntil(Date.now() + next.resend_after_s * 1000);
    setExpiresAt(Date.parse(next.expires_at));
  }

  async function submit(value: string) {
    if (busy || locked || value.length !== length) return;
    setBusy(true);
    setError(null);
    try {
      await onVerify(value);
    } catch (failure) {
      setCode("");
      if (failure instanceof ApiError && failure.code === "auth.rate_limited" && failure.retryAfter !== undefined) {
        setError(t("codeChallenge.rateLimited", { time: formatWait(failure.retryAfter, locale) }));
        setCooldownUntil(Date.now() + failure.retryAfter * 1000);
      } else {
        if (failure instanceof ApiError && failure.code === "auth.code_invalid") setAttempts((count) => count + 1);
        setError(errorMessage(t, failure).message);
      }
      focusInput();
    } finally {
      setBusy(false);
    }
  }

  async function resend() {
    if (cooldown > 0 || busy) return;
    setBusy(true);
    setError(null);
    try {
      apply(await onResend());
      setResent(true);
      setAnnounce(t("codeChallenge.resentAnnounce"));
      setCode("");
    } catch (failure) {
      if (failure instanceof ApiError && failure.code === "auth.rate_limited" && failure.retryAfter !== undefined) {
        setError(t("codeChallenge.rateLimited", { time: formatWait(failure.retryAfter, locale) }));
        setCooldownUntil(Date.now() + failure.retryAfter * 1000);
      } else setError(errorMessage(t, failure).message);
    } finally {
      setBusy(false);
    }
  }

  async function requestNew() {
    if (cooldown > 0 || busy) return;
    setBusy(true);
    setError(null);
    try {
      apply(await onNewChallenge());
      setAttempts(0);
      setResent(false);
      setCode("");
      focusInput();
    } catch (failure) {
      setError(errorMessage(t, failure).message);
    } finally {
      setBusy(false);
    }
  }

  const waiting = cooldown > 0;
  return (
    <form
      onSubmit={(event: FormEvent) => {
        event.preventDefault();
        void submit(code);
      }}
      className="grid gap-4"
      noValidate
    >
      <div className="text-sm text-muted-foreground">{description}</div>
      <div className="grid gap-2" ref={input}>
        <Label htmlFor={inputId}>
          {t("codeChallenge.label")} <span className="font-normal text-muted-foreground">{t("codeChallenge.length", { length })}</span>
        </Label>
        <CodeInput id={inputId} value={code} onChange={setCode} onComplete={(value) => void submit(value)} length={length} readOnly={busy} disabled={locked} invalid={error !== null} describedBy={error ? errorId : undefined} autoFocus={autoFocus} />
        {error ? (
          <p id={errorId} role="alert" className="text-sm text-status-danger">
            {error}
          </p>
        ) : null}
      </div>
      {extra}
      {locked ? <Alert tone="info">{expired ? t("codeChallenge.expired") : t("codeChallenge.locked")}</Alert> : null}
      {delivery === "failed" ? <Alert tone="warning">{t("codeChallenge.deliveryFailed")}</Alert> : null}
      {resent && !locked ? <p className="text-sm text-muted-foreground">{t("codeChallenge.resent")}</p> : null}
      <div className="flex flex-col gap-2 sm:flex-row">
        {locked ? null : (
          <Button type="submit" disabled={code.length !== length || busy}>
            {busy ? <Spinner /> : null}
            {submitLabel ?? t("codeChallenge.verify")}
          </Button>
        )}
        <Button type="button" variant={locked ? "default" : "outline"} aria-disabled={waiting || busy} onClick={() => void (locked ? requestNew() : resend())} className={waiting ? "opacity-60" : undefined}>
          {locked ? t("codeChallenge.newCode") : t("codeChallenge.resend")}
          {waiting ? <span className="font-normal"> {t("codeChallenge.resendIn", { time: formatClock(cooldown) })}</span> : null}
        </Button>
      </div>
      {secondaryActions ? <div className="flex flex-wrap gap-2">{secondaryActions}</div> : null}
      <p aria-live="polite" className="sr-only">
        {announce || (cooldown === 0 && !locked ? t("codeChallenge.canResend") : "")}
      </p>
    </form>
  );
}
