"use client";

import { useQueryClient } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { errorMessage } from "@/shared/api/error-message";
import { useLocalePath, useAppLocale } from "@/shared/i18n/navigation";
import { loginPath } from "@/shared/session/redirects";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { signOutEverywhere } from "./api";

type Props = { open: boolean; onClose: () => void; sources: number; connections: number };

/** Ends all sessions. It does not revoke source URLs or NinjaTrader pairings, so the dialog says so and lists what is still active. */
export function SignOutEverywhereDialog({ open, onClose, sources, connections }: Props) {
  const t = useTranslations("security.signOutAll");
  const root = useTranslations();
  const path = useLocalePath();
  const locale = useAppLocale();
  const router = useRouter();
  const queryClient = useQueryClient();
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function confirm() {
    if (busy) return;
    setBusy(true);
    setError(null);
    try {
      await signOutEverywhere();
      queryClient.clear();
      router.replace(loginPath(locale, { reason: "sessions_ended" }));
    } catch (failure) {
      setError(errorMessage(root, failure).message);
      setBusy(false);
    }
  }

  return (
    <AlertDialog open={open} onOpenChange={(next) => !next && !busy && onClose()}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{t("title")}</AlertDialogTitle>
          <AlertDialogDescription>{t("body")}</AlertDialogDescription>
        </AlertDialogHeader>
        <div className="grid gap-2 text-sm">
          <p className="font-medium">{t("review")}</p>
          <p>{t("notRevoked")}</p>
          <ul className="grid gap-1">
            <li>
              <Link className="underline underline-offset-4" href={path("/connections/sources")}>
                {t("activeSources", { count: sources })}
              </Link>
            </li>
            <li>
              <Link className="underline underline-offset-4" href={path("/connections/ninjatrader")}>
                {t("pairedConnections", { count: connections })}
              </Link>
            </li>
          </ul>
        </div>
        {error ? <Alert tone="danger">{error}</Alert> : null}
        <AlertDialogFooter>
          <AlertDialogCancel disabled={busy}>{root("common.actions.cancel")}</AlertDialogCancel>
          <AlertDialogAction variant="destructive" disabled={busy} onClick={(event) => {
            event.preventDefault();
            void confirm();
          }}>
            {t("confirm")}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
