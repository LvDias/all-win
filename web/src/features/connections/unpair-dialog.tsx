"use client";

import { TriangleAlert } from "lucide-react";
import { useTranslations } from "next-intl";
import { useId, useState } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import type { Connection } from "@/shared/api/types";
import { StepUpCancelled, useStepUp } from "@/shared/session/step-up";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Button } from "@/shared/ui/button";
import { Checkbox } from "@/shared/ui/checkbox";
import { Label } from "@/shared/ui/label";
import { Spinner } from "@/shared/ui/spinner";
import { useUnpairConnection } from "./api";

type Props = {
  /** The freshest copy of the connection: a stale list can be missing open positions (409 refetches it). */
  connection: Connection;
  onRefresh: () => void;
  onClose: () => void;
};

/** Unpair (Q18): allowed after a confirmation that lists affected accounts; nothing is closed automatically. */
export function UnpairDialog({ connection, onRefresh, onClose }: Props) {
  const t = useTranslations();
  const stepUp = useStepUp();
  const unpair = useUnpairConnection();
  const [acknowledged, setAcknowledged] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const checkboxId = useId();

  const bound = (connection.visible_accounts ?? []).filter((account) => account.account_id).map((account) => account.platform_account);
  const openPositions = connection.open_position_accounts ?? [];
  const needsAck = openPositions.length > 0;

  async function confirm() {
    setError(null);
    try {
      await stepUp.run(() => unpair.mutateAsync({ id: connection.id, acknowledgeOpenPositions: acknowledged }));
      toast.success(t("connections.unpair.toast", { name: connection.name }));
      onClose();
    } catch (failure) {
      if (failure instanceof StepUpCancelled) return;
      onRefresh();
      if (failure instanceof ApiError && failure.code === "connections.unpaired") {
        toast.info(t("errors.connections.unpaired"));
        onClose();
      } else setError(errorMessage(t, failure).message);
    }
  }

  return (
    <AlertDialog open onOpenChange={(open) => !open && onClose()}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{t("connections.unpair.title", { name: connection.name })}</AlertDialogTitle>
          <AlertDialogDescription>{t("connections.unpair.body")}</AlertDialogDescription>
        </AlertDialogHeader>
        {bound.length > 0 ? <p className="text-sm">{t("connections.unpair.bound", { accounts: bound.join(", ") })}</p> : null}
        {needsAck ? (
          <Alert tone="danger">
            <TriangleAlert aria-hidden="true" />
            <p>{t("connections.unpair.openPositions", { accounts: openPositions.join(", ") })}</p>
          </Alert>
        ) : null}
        {needsAck ? (
          <div className="flex items-start gap-3">
            <Checkbox id={checkboxId} checked={acknowledged} onCheckedChange={(checked) => setAcknowledged(checked === true)} className="mt-1" />
            <Label htmlFor={checkboxId} className="min-h-6">
              {t("connections.unpair.acknowledge")}
            </Label>
          </div>
        ) : null}
        {error ? <Alert tone="danger">{error}</Alert> : null}
        <AlertDialogFooter>
          <Button variant="outline" onClick={onClose}>
            {t("common.actions.cancel")}
          </Button>
          <Button variant="destructive" disabled={unpair.isPending || (needsAck && !acknowledged)} onClick={() => void confirm()}>
            {unpair.isPending ? <Spinner /> : null}
            {t("connections.unpair.confirm")}
          </Button>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
