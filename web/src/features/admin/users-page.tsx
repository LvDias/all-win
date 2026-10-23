"use client";

import { CircleCheck, MoreVertical, ShieldAlert } from "lucide-react";
import { useTranslations } from "next-intl";
import { useId, useState } from "react";
import { toast } from "sonner";
import { errorMessage } from "@/shared/api/error-message";
import type { AdminUser } from "@/shared/api/types";
import { formatDate, formatDateTime } from "@/shared/format";
import { useAppLocale } from "@/shared/i18n/navigation";
import { StepUpCancelled, useStepUp } from "@/shared/session/step-up";
import { useMe } from "@/shared/session/me";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Badge } from "@/shared/ui/badge";
import { Button } from "@/shared/ui/button";
import { Card } from "@/shared/ui/card";
import { Checkbox } from "@/shared/ui/checkbox";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/shared/ui/dropdown-menu";
import { ErrorState } from "@/shared/ui/error-state";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { Skeleton } from "@/shared/ui/skeleton";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/shared/ui/table";
import { AdminGate, AdminNotFound } from "./admin-gate";
import { isForbidden, useCreateResetLink, useResetTotp, useUsers, type ResetLink } from "./api";
import { OneTimeLinkDialog } from "./one-time-link-dialog";

type Action = { kind: "reset-link" | "totp-reset"; user: AdminUser };

export function UsersPage() {
  return (
    <AdminGate>
      <Users />
    </AdminGate>
  );
}

function Users() {
  const t = useTranslations();
  const locale = useAppLocale();
  const me = useMe();
  const users = useUsers();
  const stepUp = useStepUp();
  const createLink = useCreateResetLink();
  const resetTotp = useResetTotp();
  const [query, setQuery] = useState("");
  const [action, setAction] = useState<Action | null>(null);
  const [identityConfirmed, setIdentityConfirmed] = useState(false);
  const [link, setLink] = useState<ResetLink | null>(null);
  const searchId = useId();
  const identityId = useId();
  const zone = me.data?.time_zone ?? "UTC";

  if (users.isPending) {
    return (
      <div className="grid gap-3">
        <Skeleton className="h-10 w-full" />
        <Skeleton className="h-10 w-full" />
      </div>
    );
  }
  if (users.isError) {
    if (isForbidden(users.error)) return <AdminNotFound />;
    const { message, code } = errorMessage(t, users.error);
    return <ErrorState title={message} code={code} onRetry={() => void users.refetch()} />;
  }

  const needle = query.trim().toLowerCase();
  const visible = users.data.filter((user) => user.email.toLowerCase().includes(needle));

  function closeAction() {
    setAction(null);
    setIdentityConfirmed(false);
  }

  async function confirmAction(current: Action) {
    closeAction();
    try {
      if (current.kind === "reset-link") {
        setLink(await stepUp.run(() => createLink.mutateAsync(current.user.id)));
      } else {
        await stepUp.run(() => resetTotp.mutateAsync(current.user.id));
        toast.success(t("admin.users.totpResetToast", { email: current.user.email }));
      }
    } catch (failure) {
      if (failure instanceof StepUpCancelled) return;
      toast.error(errorMessage(t, failure).message);
    }
  }

  const totp = (user: AdminUser) =>
    user.totp_enabled ? (
      <Badge tone="success">
        <CircleCheck aria-hidden="true" />
        {t("admin.users.totpOn")}
      </Badge>
    ) : (
      <Badge tone="warning">
        <ShieldAlert aria-hidden="true" />
        {t("admin.users.totpOff")}
      </Badge>
    );

  // The admin manages their own security on the Security page, so their row has no actions.
  const actions = (user: AdminUser) =>
    user.id === me.data?.id ? null : (
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" size="icon" aria-label={t("admin.users.actionsFor", { email: user.email })}>
            <MoreVertical aria-hidden="true" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem onSelect={() => setAction({ kind: "reset-link", user })}>{t("admin.users.resetLink")}</DropdownMenuItem>
          <DropdownMenuItem onSelect={() => setAction({ kind: "totp-reset", user })}>{t("admin.users.resetTotp")}</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    );

  return (
    <div className="grid gap-4">
      <div className="grid max-w-sm gap-2">
        <Label htmlFor={searchId}>{t("admin.users.search")}</Label>
        <Input id={searchId} type="search" value={query} onChange={(event) => setQuery(event.target.value)} autoComplete="off" />
      </div>

      {visible.length === 0 ? (
        <p className="text-sm text-muted-foreground">{t("admin.users.noResults")}</p>
      ) : (
        <>
          <div className="hidden md:block">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t("admin.users.email")}</TableHead>
                  <TableHead>{t("admin.users.createdAt")}</TableHead>
                  <TableHead>{t("admin.users.totp")}</TableHead>
                  <TableHead>
                    <span className="sr-only">{t("admin.users.actions")}</span>
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {visible.map((user) => (
                  <TableRow key={user.id}>
                    <TableCell className="font-medium">{user.email}</TableCell>
                    <TableCell>{formatDate(user.created_at, locale, zone)}</TableCell>
                    <TableCell>{totp(user)}</TableCell>
                    <TableCell className="text-right">{actions(user)}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
          <ul className="grid gap-3 md:hidden">
            {visible.map((user) => (
              <li key={user.id}>
                <Card className="flex items-start justify-between gap-2 p-4">
                  <div className="grid gap-2">
                    <p className="font-medium break-all">{user.email}</p>
                    <p className="text-sm text-muted-foreground">{formatDate(user.created_at, locale, zone)}</p>
                    {totp(user)}
                  </div>
                  {actions(user)}
                </Card>
              </li>
            ))}
          </ul>
        </>
      )}

      <AlertDialog open={action?.kind === "reset-link"} onOpenChange={(open) => !open && closeAction()}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("admin.users.resetLinkTitle", { email: action?.user.email ?? "" })}</AlertDialogTitle>
            <AlertDialogDescription>{t("admin.users.resetLinkBody")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{t("common.actions.cancel")}</AlertDialogCancel>
            <AlertDialogAction onClick={() => action && void confirmAction(action)}>{t("admin.users.resetLinkConfirm")}</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <AlertDialog open={action?.kind === "totp-reset"} onOpenChange={(open) => !open && closeAction()}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("admin.users.resetTotpTitle", { email: action?.user.email ?? "" })}</AlertDialogTitle>
            <AlertDialogDescription>{t("admin.users.resetTotpBody")}</AlertDialogDescription>
          </AlertDialogHeader>
          <div className="flex items-start gap-3">
            <Checkbox id={identityId} checked={identityConfirmed} onCheckedChange={(checked) => setIdentityConfirmed(checked === true)} className="mt-1" />
            <Label htmlFor={identityId} className="min-h-6">
              {t("admin.users.identityConfirmed", { email: action?.user.email ?? "" })}
            </Label>
          </div>
          <AlertDialogFooter>
            <AlertDialogCancel>{t("common.actions.cancel")}</AlertDialogCancel>
            <AlertDialogAction
              disabled={!identityConfirmed}
              className="bg-destructive text-white hover:bg-destructive/90"
              onClick={() => action && void confirmAction(action)}
            >
              {t("admin.users.resetTotpConfirm")}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      {link ? (
        <OneTimeLinkDialog
          title={t("admin.users.linkTitle")}
          description={t("admin.users.linkBody")}
          url={link.reset_url}
          label={t("admin.users.linkLabel")}
          guardClose
          onClose={() => setLink(null)}
        >
          <p className="text-sm text-muted-foreground">{t("admin.users.linkExpires", { date: formatDateTime(link.expires_at, locale, zone, { zone: true }) })}</p>
        </OneTimeLinkDialog>
      ) : null}
    </div>
  );
}
