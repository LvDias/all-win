"use client";

import { useTranslations } from "next-intl";
import { useState, type ReactNode } from "react";
import { useViewOnceLock } from "@/shared/i18n/locale-switch";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { SecretField } from "@/shared/ui/secret-field";

type Props = {
  title: string;
  description: string;
  /** The view-once link. It lives only in this component's props/state and is gone when the dialog closes. */
  url: string;
  label: string;
  /** Extra content under the link (expiry, warnings). */
  children?: ReactNode;
  /** When true, closing without copying asks for confirmation (no e-mail copy of the link exists). */
  guardClose: boolean;
  onClose: () => void;
};

/** Shows an invite or reset link once. The language switcher stays locked while it is open (S4 spec §3.6). */
export function OneTimeLinkDialog({ title, description, url, label, children, guardClose, onClose }: Props) {
  const t = useTranslations("admin.link");
  const [copied, setCopied] = useState(false);
  const [guard, setGuard] = useState(false);
  useViewOnceLock(true);

  function requestClose() {
    if (guardClose && !copied) setGuard(true);
    else onClose();
  }

  return (
    <>
      <Dialog open onOpenChange={(open) => !open && requestClose()}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{title}</DialogTitle>
            <DialogDescription>{description}</DialogDescription>
          </DialogHeader>
          {children}
          <SecretField value={url} maskedDisplay={t("masked")} label={label} onCopied={() => setCopied(true)} />
          <DialogFooter>
            <Button onClick={requestClose}>{t("done")}</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
      <AlertDialog open={guard} onOpenChange={setGuard}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("guardTitle")}</AlertDialogTitle>
            <AlertDialogDescription>{t("guardBody")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{t("guardBack")}</AlertDialogCancel>
            <AlertDialogAction onClick={onClose}>{t("guardClose")}</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
}
