"use client";

import { Ban, CircleCheck, Clock, Hourglass } from "lucide-react";
import { useTranslations } from "next-intl";
import { useState } from "react";
import { toast } from "sonner";
import { errorMessage } from "@/shared/api/error-message";
import type { Invitation } from "@/shared/api/types";
import { formatDateTime } from "@/shared/format";
import { useAppLocale } from "@/shared/i18n/navigation";
import { useMe } from "@/shared/session/me";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Badge, type Tone } from "@/shared/ui/badge";
import { Button } from "@/shared/ui/button";
import { Card } from "@/shared/ui/card";
import { ErrorState } from "@/shared/ui/error-state";
import { Skeleton } from "@/shared/ui/skeleton";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/shared/ui/table";
import { AdminGate, AdminNotFound } from "./admin-gate";
import { isForbidden, useInvitations, useRevokeInvitation } from "./api";
import { CreateInviteDialog } from "./create-invite-dialog";

const statusView: Record<Invitation["status"], { tone: Tone; icon: typeof Clock }> = {
  pending: { tone: "info", icon: Hourglass },
  accepted: { tone: "success", icon: CircleCheck },
  expired: { tone: "neutral", icon: Clock },
  revoked: { tone: "neutral", icon: Ban },
};

export function InvitesPage() {
  return (
    <AdminGate>
      <Invites />
    </AdminGate>
  );
}

function Invites() {
  const t = useTranslations();
  const locale = useAppLocale();
  const me = useMe();
  const invitations = useInvitations();
  const revoke = useRevokeInvitation();
  const [creating, setCreating] = useState(false);
  const [revoking, setRevoking] = useState<Invitation | null>(null);
  const zone = me.data?.time_zone ?? "UTC";
  const when = (value: string) => formatDateTime(value, locale, zone, { zone: true });

  if (invitations.isPending) {
    return (
      <div className="grid gap-3">
        <Skeleton className="h-10 w-full" />
        <Skeleton className="h-10 w-full" />
        <Skeleton className="h-10 w-full" />
      </div>
    );
  }
  if (invitations.isError) {
    if (isForbidden(invitations.error)) return <AdminNotFound />;
    const { message, code } = errorMessage(t, invitations.error);
    return <ErrorState title={message} code={code} onRetry={() => void invitations.refetch()} />;
  }

  async function confirmRevoke(invitation: Invitation) {
    setRevoking(null);
    try {
      await revoke.mutateAsync(invitation.id);
      toast.success(t("admin.invites.revokedToast"));
    } catch (failure) {
      toast.error(errorMessage(t, failure).message);
    }
  }

  const status = (invitation: Invitation) => {
    const view = statusView[invitation.status];
    return (
      <Badge tone={view.tone}>
        <view.icon aria-hidden="true" />
        {t(`admin.invites.status.${invitation.status}`)}
      </Badge>
    );
  };
  const revokeButton = (invitation: Invitation) =>
    invitation.status === "pending" ? (
      <Button variant="outline" size="sm" onClick={() => setRevoking(invitation)} aria-label={t("admin.invites.revokeFor", { email: invitation.email })}>
        {t("common.actions.revoke")}
      </Button>
    ) : null;

  return (
    <div className="grid gap-4">
      <div className="flex items-center justify-between gap-2">
        <h2 className="sr-only">{t("admin.invites.list")}</h2>
        <Button onClick={() => setCreating(true)}>{t("admin.invites.invite")}</Button>
      </div>

      {invitations.data.length === 0 ? (
        <Card className="grid gap-3 p-6">
          <p className="font-medium">{t("admin.invites.emptyTitle")}</p>
          <p className="text-sm text-muted-foreground">{t("admin.invites.emptyBody")}</p>
          <Button className="self-start" onClick={() => setCreating(true)}>
            {t("admin.invites.invite")}
          </Button>
        </Card>
      ) : (
        <>
          <div className="hidden md:block">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t("admin.invites.email")}</TableHead>
                  <TableHead>{t("admin.invites.statusLabel")}</TableHead>
                  <TableHead>{t("admin.invites.createdAt")}</TableHead>
                  <TableHead>{t("admin.invites.expiresLabel")}</TableHead>
                  <TableHead>
                    <span className="sr-only">{t("admin.invites.actions")}</span>
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {invitations.data.map((invitation) => (
                  <TableRow key={invitation.id}>
                    <TableCell className="font-medium">{invitation.email}</TableCell>
                    <TableCell>{status(invitation)}</TableCell>
                    <TableCell>{when(invitation.created_at)}</TableCell>
                    <TableCell>{when(invitation.expires_at)}</TableCell>
                    <TableCell className="text-right">{revokeButton(invitation)}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
          <ul className="grid gap-3 md:hidden">
            {invitations.data.map((invitation) => (
              <li key={invitation.id}>
                <Card className="grid gap-2 p-4">
                  <div className="flex items-start justify-between gap-2">
                    <p className="font-medium break-all">{invitation.email}</p>
                    {status(invitation)}
                  </div>
                  <dl className="grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-sm">
                    <dt className="text-muted-foreground">{t("admin.invites.createdAt")}</dt>
                    <dd>{when(invitation.created_at)}</dd>
                    <dt className="text-muted-foreground">{t("admin.invites.expiresLabel")}</dt>
                    <dd>{when(invitation.expires_at)}</dd>
                  </dl>
                  {revokeButton(invitation)}
                </Card>
              </li>
            ))}
          </ul>
        </>
      )}

      <CreateInviteDialog open={creating} onClose={() => setCreating(false)} />

      <AlertDialog open={revoking !== null} onOpenChange={(open) => !open && setRevoking(null)}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("admin.invites.revokeTitle", { email: revoking?.email ?? "" })}</AlertDialogTitle>
            <AlertDialogDescription>{t("admin.invites.revokeBody")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{t("common.actions.cancel")}</AlertDialogCancel>
            <AlertDialogAction onClick={() => revoking && void confirmRevoke(revoking)}>{t("common.actions.revoke")}</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
