"use client";

import { Check, Copy, Eye, EyeOff } from "lucide-react";
import { useTranslations } from "next-intl";
import { useCallback, useEffect, useRef, useState } from "react";
import { Button } from "@/shared/ui/button";

type SecretFieldProps = {
  /** The secret. It is rendered only while revealed; the masked view uses `maskedDisplay`. */
  value: string;
  /** What the masked view shows: only non-secret parts plus bullets (F§8.1). */
  maskedDisplay: string;
  /** Accessible name of the field. */
  label: string;
  /** Optional readable form of the revealed secret, e.g. groups of four. */
  revealedDisplay?: string;
  /** Called once a copy succeeded (callers use it to drop the close guard). */
  onCopied?: () => void;
};

const remaskAfterMs = 60_000;
const copiedForMs = 2_000;

/**
 * View-once secret (F§8.1). Masked by default with the secret absent from the DOM; reveal and copy are explicit.
 * It re-masks after 60 s, when the tab is hidden, and on unmount. The value is never stored outside component state.
 */
export function SecretField({ value, maskedDisplay, label, revealedDisplay, onCopied }: SecretFieldProps) {
  const t = useTranslations("common.secret");
  const [revealed, setRevealed] = useState(false);
  const [copied, setCopied] = useState(false);
  const [copyFailed, setCopyFailed] = useState(false);
  const valueRef = useRef<HTMLParagraphElement>(null);

  useEffect(() => {
    if (!revealed) return;
    const timer = window.setTimeout(() => setRevealed(false), remaskAfterMs);
    const onVisibility = () => {
      if (document.visibilityState === "hidden") setRevealed(false);
    };
    document.addEventListener("visibilitychange", onVisibility);
    return () => {
      window.clearTimeout(timer);
      document.removeEventListener("visibilitychange", onVisibility);
    };
  }, [revealed]);

  useEffect(() => {
    if (!copied) return;
    const timer = window.setTimeout(() => setCopied(false), copiedForMs);
    return () => window.clearTimeout(timer);
  }, [copied]);

  useEffect(() => {
    if (!copyFailed || !revealed || !valueRef.current) return;
    const range = document.createRange();
    range.selectNodeContents(valueRef.current);
    const selection = window.getSelection();
    selection?.removeAllRanges();
    selection?.addRange(range);
  }, [copyFailed, revealed]);

  const copy = useCallback(async () => {
    try {
      await navigator.clipboard.writeText(value);
      setCopyFailed(false);
      setCopied(true);
      onCopied?.();
    } catch {
      // Clipboard blocked: show the value selected so the user can copy it by hand.
      setCopyFailed(true);
      setRevealed(true);
    }
  }, [value, onCopied]);

  return (
    <div className="flex flex-col gap-2">
      <p ref={valueRef} aria-label={revealed ? label : `${label}: ${t("hiddenSuffix")}`} className="rounded-md border bg-muted px-3 py-2 font-mono text-sm break-all select-all" spellCheck={false}>
        {revealed ? (revealedDisplay ?? value) : maskedDisplay}
      </p>
      <div className="flex flex-wrap gap-2">
        <Button type="button" variant="outline" size="sm" aria-pressed={revealed} onClick={() => setRevealed((current) => !current)}>
          {revealed ? <EyeOff aria-hidden="true" /> : <Eye aria-hidden="true" />}
          {revealed ? t("hide") : t("reveal")}
        </Button>
        <Button type="button" variant="outline" size="sm" onClick={() => void copy()}>
          {copied ? <Check aria-hidden="true" /> : <Copy aria-hidden="true" />}
          {copied ? t("copied") : t("copy")}
        </Button>
      </div>
      <p role="status" className="sr-only">
        {copied ? t("copied") : ""}
      </p>
      {copyFailed ? <p className="text-sm text-status-warning">{t("copyFailed")}</p> : null}
    </div>
  );
}
