"use client";

import { Check, Copy, Download, ShieldAlert } from "lucide-react";
import { useTranslations } from "next-intl";
import { useId, useState } from "react";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Checkbox } from "@/shared/ui/checkbox";
import { Label } from "@/shared/ui/label";

export const recoveryFileName = "all-win-recovery-codes.txt";

/** One code per line plus the generation date, built on the client from the response. */
export function recoveryFileText(codes: readonly string[], generatedAt: Date): string {
  return `${[...codes].join("\n")}\n\n${generatedAt.toISOString()}\n`;
}

type Props = {
  codes: readonly string[];
  /** Reports whether the user confirmed saving the codes (drives the close guard). */
  onAcknowledgedChange: (acknowledged: boolean) => void;
  onDone: () => void;
  warning: string;
};

/** Recovery codes, shown once (Q22). They live only in the props of this view and are never persisted. */
export function RecoveryCodesView({ codes, onAcknowledgedChange, onDone, warning }: Props) {
  const t = useTranslations("security.recovery");
  const [acknowledged, setAcknowledged] = useState(false);
  const checkboxId = useId();
  const [copied, setCopied] = useState(false);

  async function copyAll() {
    try {
      await navigator.clipboard.writeText(codes.join("\n"));
      setCopied(true);
      window.setTimeout(() => setCopied(false), 2000);
    } catch {
      // Clipboard blocked: the codes are on screen and can be copied by hand or downloaded.
    }
  }

  function download() {
    const blob = new Blob([recoveryFileText(codes, new Date())], { type: "text/plain;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = recoveryFileName;
    link.click();
    URL.revokeObjectURL(url);
  }

  return (
    <div className="grid gap-4">
      <Alert tone="warning">
        <ShieldAlert aria-hidden="true" />
        <p>{warning}</p>
      </Alert>
      <ul className="grid grid-cols-2 gap-x-4 gap-y-2 rounded-md border bg-muted p-3 font-mono text-sm" aria-label={t("listLabel")}>
        {codes.map((code) => (
          <li key={code}>{code}</li>
        ))}
      </ul>
      <div className="flex flex-wrap gap-2">
        <Button type="button" variant="outline" size="sm" onClick={() => void copyAll()}>
          {copied ? <Check aria-hidden="true" /> : <Copy aria-hidden="true" />}
          {copied ? t("copied") : t("copyAll")}
        </Button>
        <Button type="button" variant="outline" size="sm" onClick={download}>
          <Download aria-hidden="true" />
          {t("download")}
        </Button>
      </div>
      <p role="status" className="sr-only">
        {copied ? t("copied") : ""}
      </p>
      <div className="flex items-start gap-3">
        <Checkbox
          id={checkboxId}
          checked={acknowledged}
          onCheckedChange={(checked) => {
            setAcknowledged(checked === true);
            onAcknowledgedChange(checked === true);
          }}
          className="mt-1"
        />
        <Label htmlFor={checkboxId} className="min-h-6">
          {t("saved")}
        </Label>
      </div>
      <Button type="button" disabled={!acknowledged} onClick={onDone}>
        {t("done")}
      </Button>
    </div>
  );
}
