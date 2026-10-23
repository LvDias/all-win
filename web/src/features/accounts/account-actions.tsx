"use client";

import { MoreHorizontal } from "lucide-react";
import { useLocale, useTranslations } from "next-intl";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { withLocale } from "@/shared/i18n/paths";
import { isLocale, defaultLocale } from "@/shared/i18n/config";
import { Button } from "@/shared/ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger } from "@/shared/ui/dropdown-menu";
import type { Account } from "./api/types";
import { DeleteDialog } from "./delete-dialog";
import { LifecycleDialog, useLifecycleQuickAction, type LifecycleAction } from "./lifecycle-dialog";

/** Row/detail kebab: view, edit, lifecycle, delete. Dialogs live here so the menu can close first. */
export function AccountActions({ account, onDeleted, showView = true }: { account: Account; onDeleted?: () => void; showView?: boolean }) {
  const t = useTranslations("accounts");
  const router = useRouter();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const [dialog, setDialog] = useState<{ type: "lifecycle"; action: LifecycleAction } | { type: "delete" } | null>(null);
  const quick = useLifecycleQuickAction();
  const go = (rest: string) => router.push(withLocale(locale, rest));
  const state = account.lifecycle_state;

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" size="icon" aria-label={`${t("list.columns.actions")}: ${account.display_name}`} disabled={quick.pending}>
            <MoreHorizontal aria-hidden />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          {showView ? <DropdownMenuItem onClick={() => go(`/accounts/${account.id}`)}>{t("actions.view")}</DropdownMenuItem> : null}
          <DropdownMenuItem onClick={() => go(`/accounts/${account.id}/edit`)}>{t("actions.edit")}</DropdownMenuItem>
          {state === "active" ? <DropdownMenuItem onClick={() => setDialog({ type: "lifecycle", action: "pause" })}>{t("actions.pause")}</DropdownMenuItem> : null}
          {state === "paused" ? <DropdownMenuItem onClick={() => quick.run(account)}>{t("actions.resume")}</DropdownMenuItem> : null}
          {state !== "disabled" ? <DropdownMenuItem onClick={() => setDialog({ type: "lifecycle", action: "disable" })}>{t("actions.disable")}</DropdownMenuItem> : null}
          {state === "disabled" ? <DropdownMenuItem onClick={() => quick.run(account)}>{t("actions.enable")}</DropdownMenuItem> : null}
          <DropdownMenuSeparator />
          <DropdownMenuItem className="text-destructive" onClick={() => setDialog({ type: "delete" })}>{t("actions.delete")}</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      {dialog?.type === "lifecycle" ? <LifecycleDialog account={account} action={dialog.action} onClose={() => setDialog(null)} /> : null}
      {dialog?.type === "delete" ? <DeleteDialog account={account} onClose={() => setDialog(null)} onDeleted={() => { setDialog(null); onDeleted?.(); }} /> : null}
    </>
  );
}
