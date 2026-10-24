"use client";

import { useQueryClient } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import { useEffect, useId, useState, type FormEvent } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import type { ChallengeDelivery, ChallengeIssued, ChannelChange } from "@/shared/api/types";
import { challengeDelivery, resendChallenge } from "@/shared/api/challenges";
import { CodeChallenge } from "@/shared/components/code-challenge";
import { formatClock, formatDateTime } from "@/shared/format";
import { useViewOnceLock } from "@/shared/i18n/locale-switch";
import { useCountdown } from "@/shared/lib/use-countdown";
import { useDisplay } from "@/shared/session/display";
import { securityQueryKey } from "@/shared/session/security";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { Spinner } from "@/shared/ui/spinner";
import { Stepper } from "@/shared/ui/stepper";
import { focusOnMount } from "@/shared/lib/focus";
import { startEmailChange, verifyChannelChangeCode } from "./api";

type Role = "current_value" | "new_value";
const emailPattern = /^\S+@\S+\.\S+$/;

export function ChangeEmailDialog({ open, onClose, currentEmail }: { open: boolean; onClose: () => void; currentEmail: string }) {
  return open ? <ChangeEmailFlow onClose={onClose} currentEmail={currentEmail} /> : null;
}

/** S3-05: one flow, two codes (current and new address). Nothing changes until both are confirmed. */
function ChangeEmailFlow({ onClose, currentEmail }: { onClose: () => void; currentEmail: string }) {
  const t = useTranslations();
  const queryClient = useQueryClient();
  const { locale, timeZone } = useDisplay();
  const emailId = useId();
  const [email, setEmail] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [change, setChange] = useState<ChannelChange | null>(null);
  // A resend replaces the code of its own challenge; the id may be rotated by the Core.
  const [issued, setIssued] = useState<Partial<Record<Role, ChallengeIssued>>>({});
  const [delivery, setDelivery] = useState<ChallengeDelivery | undefined>(undefined);
  const [closed, setClosed] = useState(false);
  useViewOnceLock(change !== null && change.status === "awaiting_codes");

  const steps = [
    { id: "new", label: t("security.email.stepNew") },
    { id: "current", label: t("security.email.stepCurrent") },
    { id: "confirm", label: t("security.email.stepNewCode") },
    { id: "done", label: t("security.email.stepDone") },
  ];
  const done = change !== null && (change.status === "completed" || change.status === "on_hold");
  const pendingCode = change?.codes.find((code) => !code.confirmed);
  const role: Role | null = pendingCode && (pendingCode.role === "current_value" || pendingCode.role === "new_value") ? pendingCode.role : null;
  const stepIndex = change === null ? 0 : done ? 3 : role === "new_value" ? 2 : 1;
  const expiresIn = useCountdown(change && !done ? Date.parse(change.expires_at) : null);

  const challengeId = role && change ? (issued[role]?.challenge_id ?? change.codes.find((code) => code.role === role)?.challenge_id) : undefined;
  useEffect(() => {
    if (!challengeId || role !== "new_value") return;
    let live = true;
    const poll = () => challengeDelivery(challengeId).then((state) => live && setDelivery(state), () => undefined);
    void poll();
    const timer = window.setInterval(() => void poll(), 5000);
    return () => {
      live = false;
      window.clearInterval(timer);
    };
  }, [challengeId, role]);

  async function start(event: FormEvent) {
    event.preventDefault();
    const value = email.trim();
    if (busy || !emailPattern.test(value)) return;
    setBusy(true);
    setError(null);
    try {
      setChange(await startEmailChange(value));
    } catch (failure) {
      if (failure instanceof ApiError && failure.code === "me.channel_change_pending") void queryClient.invalidateQueries({ queryKey: securityQueryKey });
      setError(errorMessage(t, failure).message);
    } finally {
      setBusy(false);
    }
  }

  function finish() {
    void queryClient.invalidateQueries({ queryKey: securityQueryKey });
    onClose();
  }

  function dismiss() {
    if (change && !done) toast.info(t("security.email.cancelled", { email: currentEmail }));
    finish();
  }

  const heading = steps[stepIndex]?.label ?? "";
  return (
    <Dialog open onOpenChange={(next) => !next && dismiss()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t("security.email.title")}</DialogTitle>
          <DialogDescription>{t("security.email.description")}</DialogDescription>
        </DialogHeader>
        <Stepper steps={steps} current={stepIndex} />
        {change && !done && !closed ? <p className="text-sm text-muted-foreground">{t("security.email.expiresIn", { time: formatClock(expiresIn) })}</p> : null}
        {closed ? (
          <>
            <Alert tone="warning">{t("errors.me.channel_change_closed")}</Alert>
            <DialogFooter>
              <Button onClick={finish}>{t("common.actions.close")}</Button>
            </DialogFooter>
          </>
        ) : change === null ? (
          <form onSubmit={(event) => void start(event)} className="grid gap-4">
            <p className="text-sm text-muted-foreground">{t("security.email.codesToBoth", { current: currentEmail })}</p>
            {error ? <Alert tone="danger">{error}</Alert> : null}
            <div className="grid gap-2">
              <Label htmlFor={emailId}>{t("security.email.newLabel")}</Label>
              <Input id={emailId} type="email" autoComplete="email" inputMode="email" autoCapitalize="none" spellCheck={false} autoFocus value={email} onChange={(event) => setEmail(event.target.value)} />
            </div>
            <DialogFooter>
              <Button type="button" variant="outline" onClick={dismiss}>
                {t("common.actions.cancel")}
              </Button>
              <Button type="submit" disabled={busy || !emailPattern.test(email.trim())}>
                {busy ? <Spinner /> : null}
                {t("security.email.sendCodes")}
              </Button>
            </DialogFooter>
          </form>
        ) : done ? (
          <div className="grid gap-4">
            <p role="status" tabIndex={-1} ref={focusOnMount} className="text-sm outline-none">
              {change.status === "on_hold" && change.hold_until
                ? t("security.email.onHold", { date: formatDateTime(change.hold_until, locale, timeZone, { zone: true }) })
                : t("security.email.completed", { email: change.masked_new_value })}
            </p>
            <DialogFooter>
              <Button onClick={finish}>{t("security.email.finish")}</Button>
            </DialogFooter>
          </div>
        ) : role && challengeId ? (
          <>
            <p className="sr-only" aria-live="polite">
              {heading}
            </p>
            <CodeChallenge
              key={challengeId}
              challenge={issued[role] ?? { challenge_id: challengeId, expires_at: change.expires_at, resend_after_s: 60 }}
              delivery={role === "new_value" ? delivery : undefined}
              description={role === "current_value" ? t("security.email.codeCurrent", { email: currentEmail }) : t("security.email.codeNew", { email: change.masked_new_value })}
              onVerify={async (code) => {
                try {
                  setChange(await verifyChannelChangeCode(change.id, { challenge_id: challengeId, code }));
                } catch (failure) {
                  if (failure instanceof ApiError && failure.code === "me.channel_change_closed") setClosed(true);
                  throw failure;
                }
              }}
              onResend={async () => {
                const next = await resendChallenge(challengeId);
                setIssued((current) => ({ ...current, [role]: next }));
                return next;
              }}
              onNewChallenge={async () => {
                // A closed code restarts the whole flow (the change is still open for its other code).
                setChange(null);
                setIssued({});
                throw new ApiError({ code: "me.channel_change_closed", status: 409 });
              }}
              secondaryActions={
                <Button type="button" variant="ghost" onClick={dismiss}>
                  {t("common.actions.cancel")}
                </Button>
              }
            />
          </>
        ) : null}
      </DialogContent>
    </Dialog>
  );
}
