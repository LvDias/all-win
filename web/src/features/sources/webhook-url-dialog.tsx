"use client";

import { useTranslations } from "next-intl";
import Link from "next/link";
import { useState } from "react";
import { useLocalePath } from "@/shared/i18n/navigation";
import { useViewOnceLock } from "@/shared/i18n/locale-switch";
import type { SourceWithUrl } from "@/shared/api/types";
import { accountsPath } from "@/shared/routes";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { SecretField } from "@/shared/ui/secret-field";

/** Masked view keeps only the host, path and `awk_<key_id>_` (F§8.1). The secret is not in the DOM while masked. */
export function maskedWebhookUrl(webhookUrl: string, keyId: string): string {
  const host = (() => {
    try {
      return new URL(webhookUrl).host;
    } catch {
      return "";
    }
  })();
  return `https://${host}/ingress/v1/tradingview/awk_${keyId}_••••••••`;
}

type Props = {
  /** The create or rotate answer. Held in component state only and dropped on close. */
  result: SourceWithUrl;
  mode: "created" | "rotated";
  onClose: () => void;
};

/** View-once step of create and rotate (S3-09): the URL is shown once; closing without copying asks first. */
export function WebhookUrlDialog({ result, mode, onClose }: Props) {
  const t = useTranslations("sources.reveal");
  const path = useLocalePath();
  const [copied, setCopied] = useState(false);
  const [guard, setGuard] = useState(false);
  useViewOnceLock(true);

  function requestClose() {
    if (copied) onClose();
    else setGuard(true);
  }

  return (
    <>
      <Dialog open onOpenChange={(next) => !next && requestClose()}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{mode === "created" ? t("titleCreated") : t("titleRotated")}</DialogTitle>
            <DialogDescription className="sr-only">{t("description")}</DialogDescription>
          </DialogHeader>
          <Alert tone="warning">{t("warning")}</Alert>
          <SecretField
            value={result.webhook_url}
            maskedDisplay={maskedWebhookUrl(result.webhook_url, result.source.key_id)}
            label={t("urlLabel")}
            onCopied={() => setCopied(true)}
          />
          <ol className="list-decimal space-y-1 pl-5 text-sm">
            <li>{t("step1")}</li>
            <li>{t("step2")}</li>
            <li>{t("step3")}</li>
          </ol>
          {result.source.is_automated ? (
            <Alert tone="info">
              <p>
                {t("automation")}
                {accountsPath ? (
                  <>
                    {" "}
                    <Link className="underline underline-offset-4" href={path(accountsPath)}>
                      {t("automationLink")}
                    </Link>
                  </>
                ) : null}
              </p>
            </Alert>
          ) : null}
          <Button onClick={requestClose}>{t("done")}</Button>
        </DialogContent>
      </Dialog>
      <AlertDialog open={guard} onOpenChange={setGuard}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("guardTitle")}</AlertDialogTitle>
            <AlertDialogDescription>{t("guard")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel autoFocus>{t("guardBack")}</AlertDialogCancel>
            <AlertDialogAction onClick={onClose}>{t("guardClose")}</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
}
