"use client";

import { useTranslations } from "next-intl";
import { useState } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Spinner } from "@/shared/ui/spinner";
import { useArchiveAccount, useArchiveImpact } from "./api/use-accounts";
import type { Account } from "./api/types";

/** Delete = archive. With an open position it is an info dialog with no confirm button (spec §11). */
export function DeleteDialog({ account, onClose, onDeleted }: { account: Account; onClose: () => void; onDeleted: () => void }) {
  const t = useTranslations();
  const impact = useArchiveImpact(account.id, true);
  const archive = useArchiveAccount();
  const [blocked, setBlocked] = useState(false);
  const [failure, setFailure] = useState<string | null>(null);

  const openPosition = blocked || account.has_open_position || impact.data?.has_open_position === true;

  if (openPosition) {
    return (
      <AlertDialog open onOpenChange={(open) => (open ? undefined : onClose())}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("accounts.delete.blockedTitle", { name: account.display_name })}</AlertDialogTitle>
            <AlertDialogDescription>{t("accounts.delete.blocked", { name: account.display_name })}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel autoFocus>{t("accounts.actions.understood")}</AlertDialogCancel>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    );
  }

  const routes = impact.data?.routes ?? [];
  const links = impact.data?.follower_links ?? [];

  function confirm() {
    setFailure(null);
    archive.mutate(account.id, {
      onSuccess: () => {
        toast.success(t("accounts.delete.done"));
        onDeleted();
      },
      onError: (error) => {
        if (error instanceof ApiError && error.code === "accounts.delete_open_position") setBlocked(true);
        else setFailure(errorMessage(t, error).message);
      },
    });
  }

  return (
    <AlertDialog open onOpenChange={(open) => (open ? undefined : onClose())}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{t("accounts.delete.title", { name: account.display_name })}</AlertDialogTitle>
          <AlertDialogDescription>{t("accounts.delete.body")}</AlertDialogDescription>
        </AlertDialogHeader>
        {impact.isPending ? <Spinner /> : null}
        {routes.length + links.length > 0 ? (
          <div className="space-y-1 text-sm">
            <p>{t("accounts.delete.removes", { routes: routes.length, links: links.length, both: routes.length > 0 && links.length > 0 ? "yes" : "no" })}</p>
            <ul className="list-disc pl-5 text-muted-foreground">
              {routes.map((r) => <li key={r.source_id}>{r.source_name}</li>)}
              {links.map((l) => <li key={l.account_id}>{l.display_name}</li>)}
            </ul>
          </div>
        ) : null}
        {failure ? <Alert tone="danger">{failure}</Alert> : null}
        <AlertDialogFooter>
          <AlertDialogCancel autoFocus disabled={archive.isPending}>{t("common.actions.cancel")}</AlertDialogCancel>
          <AlertDialogAction
            variant="destructive"
            disabled={archive.isPending || impact.isPending}
            onClick={(event) => {
              event.preventDefault();
              confirm();
            }}
          >
            {archive.isPending ? t("common.states.saving") : t("accounts.delete.confirm")}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
