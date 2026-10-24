"use client";

import { useQueryClient } from "@tanstack/react-query";
import { CircleCheck, Copy, Check } from "lucide-react";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { useEffect, useRef, useState } from "react";
import { errorMessage } from "@/shared/api/error-message";
import type { PairingCode } from "@/shared/api/types";
import { formatClock } from "@/shared/format";
import { useCountdown } from "@/shared/lib/use-countdown";
import { useLocalePath } from "@/shared/i18n/navigation";
import { accountsPath } from "@/shared/routes";
import { useEvents } from "@/shared/session/events";
import { StepUpCancelled, useStepUp } from "@/shared/session/step-up";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { Spinner } from "@/shared/ui/spinner";
import { connectionsQueryKey, useConnections, useNewPairingCode } from "./api";

type Phase = { kind: "waiting" } | { kind: "paired" } | { kind: "superseded" } | { kind: "locked" };

type Props = {
  connection: { id: string; name: string };
  pairing: PairingCode;
  /** True when the code moves a paired connection to another machine. */
  moving: boolean;
  onClose: () => void;
};

/** Visible, short-lived, single-use code: shown in clear text, never masked (S3-10). Closing needs no guard. */
export function PairingDialog({ connection, pairing: initial, moving, onClose }: Props) {
  const t = useTranslations();
  const stepUp = useStepUp();
  const newCode = useNewPairingCode();
  const queryClient = useQueryClient();
  const connections = useConnections(false);
  const path = useLocalePath();
  const [pairing, setPairing] = useState<PairingCode | null>(initial);
  const [phase, setPhase] = useState<Phase>({ kind: "waiting" });
  const [error, setError] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);
  const heading = useRef<HTMLHeadingElement>(null);

  const remaining = useCountdown(pairing ? Date.parse(pairing.expires_at) : null);
  const expired = phase.kind === "waiting" && pairing !== null && remaining === 0;

  useEvents((event) => {
    if (event.type === "connection.paired" && event.connection_id === connection.id) setPhase({ kind: "paired" });
    // The accounts arrive after the plugin's hello, with the `connected` status, so this dialog refetches the list itself.
    if ((event.type === "connection.paired" || event.type === "connection.status_changed") && event.connection_id === connection.id) void queryClient.invalidateQueries({ queryKey: connectionsQueryKey });
    if (event.type === "connection.pairing_code_invalidated" && event.connection_id === connection.id) {
      setPairing(null);
      setPhase({ kind: event.reason === "superseded" ? "superseded" : "locked" });
    }
  });

  useEffect(() => {
    if (phase.kind === "paired") heading.current?.focus();
  }, [phase.kind]);

  async function generate() {
    setError(null);
    try {
      const next = await stepUp.run({ action: "pairing_code_new", targetId: connection.id, description: t("stepUp.actions.pairingCode", { name: connection.name }) }, (proof) => newCode.mutateAsync({ id: connection.id, proof }));
      setPairing(next);
      setPhase({ kind: "waiting" });
    } catch (failure) {
      if (!(failure instanceof StepUpCancelled)) setError(errorMessage(t, failure).message);
    }
  }

  async function copy() {
    if (!pairing) return;
    try {
      await navigator.clipboard.writeText(pairing.pairing_code);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 2000);
    } catch {
      // The code is on screen; the user can select it by hand.
    }
  }

  // `visible_accounts` of `connection.paired` is not used: it is empty by design (S3 pairing step 2). The count comes from the list.
  const row = connections.data?.find((item) => item.id === connection.id);
  const searching = phase.kind === "paired" && !moving && row?.status !== "connected";
  const pairedAccounts = row?.visible_accounts ?? [];

  const announce = expired ? t("connections.pairing.expired") : phase.kind === "waiting" && remaining > 0 && remaining <= 60 ? t("connections.pairing.oneMinute") : "";

  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle ref={heading} tabIndex={-1}>
            {phase.kind === "paired" ? t(searching ? "connections.pairing.pairedName" : "connections.pairing.connectedName", { name: connection.name }) : moving ? t("connections.pairing.moveTitle") : t("connections.pairing.title", { name: connection.name })}
          </DialogTitle>
          <DialogDescription>{phase.kind === "paired" ? t("connections.pairing.pairedDescription") : t("connections.pairing.share")}</DialogDescription>
        </DialogHeader>

        {phase.kind === "paired" ? (
          <div className="grid gap-4">
            {/* One persistent live region: only its content changes, so the final count is announced. */}
            <Alert tone={searching ? "info" : "success"}>
              {searching ? <Spinner /> : <CircleCheck aria-hidden="true" />}
              <p>
                {searching
                  ? t("connections.pairing.searchingAccounts")
                  : moving
                    ? t("connections.pairing.moved", { name: connection.name })
                    : t("connections.pairing.accountsFound", { count: pairedAccounts.length })}
                {!searching && !moving && pairedAccounts.length > 0 ? ` ${pairedAccounts.map((account) => account.platform_account).join(", ")}` : ""}
              </p>
            </Alert>
            {!searching && !moving && pairedAccounts.length > 0 && accountsPath ? (
              <Button asChild>
                <Link href={path(accountsPath)}>{t("connections.pairing.bindAccounts")}</Link>
              </Button>
            ) : null}
            <Button variant="outline" onClick={onClose}>{t("common.actions.done")}</Button>
          </div>
        ) : (
          <div className="grid gap-4">
            {error ? <Alert tone="danger">{error}</Alert> : null}
            {phase.kind === "superseded" ? <Alert tone="info">{t("connections.pairing.superseded")}</Alert> : null}
            {phase.kind === "locked" ? <Alert tone="warning">{t("connections.pairing.locked")}</Alert> : null}
            <div className="flex flex-wrap items-center gap-3">
              <p aria-label={t("connections.pairing.codeLabel")} className="font-mono text-3xl tracking-widest" translate="no">
                {pairing ? pairing.pairing_code : "—"}
              </p>
              {pairing && !expired ? (
                <Button type="button" variant="outline" size="sm" onClick={() => void copy()}>
                  {copied ? <Check aria-hidden="true" /> : <Copy aria-hidden="true" />}
                  {copied ? t("secret.copied") : t("common.actions.copy")}
                </Button>
              ) : null}
            </div>
            {pairing && !expired ? (
              <>
                <p className="text-sm" aria-live="off">
                  {t("connections.pairing.expiresIn", { time: formatClock(remaining) })}
                </p>
                <p className="text-sm text-muted-foreground">{t("connections.pairing.dontShare")}</p>
                <p className="text-sm">{t("connections.pairing.instructions")}</p>
                <p className="flex items-center gap-2 text-sm text-muted-foreground">
                  <Spinner />
                  {t("connections.pairing.waiting")}
                </p>
              </>
            ) : null}
            {expired ? <Alert tone="warning">{t("connections.pairing.expired")}</Alert> : null}
            {expired || phase.kind === "superseded" || phase.kind === "locked" ? (
              <Button onClick={() => void generate()} disabled={newCode.isPending}>
                {newCode.isPending ? <Spinner /> : null}
                {t("connections.pairing.newCode")}
              </Button>
            ) : null}
            <p role="status" className="sr-only">
              {announce}
            </p>
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
}
