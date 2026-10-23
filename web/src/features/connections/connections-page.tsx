"use client";

import { useQueryClient } from "@tanstack/react-query";
import { Cable, ChevronDown, ChevronRight, Plus } from "lucide-react";
import { useTranslations } from "next-intl";
import { Fragment, useState } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import type { Connection, PairingCode } from "@/shared/api/types";
import { formatDateTime } from "@/shared/format";
import { useIsDesktop } from "@/shared/lib/use-media-query";
import { useDisplay } from "@/shared/session/display";
import { useEvents } from "@/shared/session/events";
import { StepUpCancelled, useStepUp } from "@/shared/session/step-up";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Button } from "@/shared/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/shared/ui/card";
import { EmptyState } from "@/shared/ui/empty-state";
import { ErrorState } from "@/shared/ui/error-state";
import { Label } from "@/shared/ui/label";
import { NameDialog } from "@/shared/ui/name-dialog";
import { Skeleton } from "@/shared/ui/skeleton";
import { Switch } from "@/shared/ui/switch";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/shared/ui/table";
import { connectionsQueryKey, useConnections, useCreateConnection, useNewPairingCode, useRenameConnection } from "./api";
import { ConnectionActions } from "./connection-actions";
import { ConnectionDetails, StatusBadge } from "./connection-parts";
import { PairingDialog } from "./pairing-dialog";
import { UnpairDialog } from "./unpair-dialog";

type Dialog =
  | { kind: "create" }
  | { kind: "rename"; id: string }
  | { kind: "move"; id: string }
  | { kind: "unpair"; id: string }
  | { kind: "pair"; connection: { id: string; name: string }; pairing: PairingCode; moving: boolean }
  | null;

export function ConnectionsPage() {
  const t = useTranslations();
  const queryClient = useQueryClient();
  const stepUp = useStepUp();
  const desktop = useIsDesktop();
  const { locale, timeZone } = useDisplay();
  const [includeUnpaired, setIncludeUnpaired] = useState(false);
  const [dialog, setDialog] = useState<Dialog>(null);
  const [expanded, setExpanded] = useState<ReadonlySet<string>>(new Set());
  const connections = useConnections(includeUnpaired);
  const create = useCreateConnection();
  const newCode = useNewPairingCode();
  const rename = useRenameConnection();

  const refresh = () => void queryClient.invalidateQueries({ queryKey: connectionsQueryKey });
  const close = () => {
    setDialog(null);
    refresh();
  };
  // Events are not replayed, so a live change just refetches the list (OpenAPI `eventStream`).
  useEvents((event) => {
    if (event.type === "connection.status_changed" || event.type === "connection.paired") refresh();
  });

  const rows = connections.data ?? [];
  const find = (id: string): Connection | undefined => rows.find((row) => row.id === id);
  const toggle = (id: string) =>
    setExpanded((current) => {
      const next = new Set(current);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });

  /** Issues a new code (also the re-pairing flow); a stale unpaired row only refreshes the list. */
  async function issueCode(connection: Connection, moving: boolean) {
    setDialog(null);
    try {
      const pairing = await stepUp.run({ action: "pairing_code_new", targetId: connection.id, description: t("stepUp.actions.pairingCode", { name: connection.name }) }, (proof) => newCode.mutateAsync({ id: connection.id, proof }));
      setDialog({ kind: "pair", connection: { id: connection.id, name: connection.name }, pairing, moving });
    } catch (error) {
      if (error instanceof StepUpCancelled) return;
      refresh();
      if (error instanceof ApiError && error.code === "connections.unpaired") toast.info(t("errors.connections.unpaired"));
      else toast.error(errorMessage(t, error).message);
    }
  }

  const target = dialog && "id" in dialog ? find(dialog.id) : undefined;
  const actions = (connection: Connection) => (
    <ConnectionActions
      connection={connection}
      onRename={() => setDialog({ kind: "rename", id: connection.id })}
      onNewCode={() => void issueCode(connection, false)}
      onMove={() => setDialog({ kind: "move", id: connection.id })}
      onUnpair={() => setDialog({ kind: "unpair", id: connection.id })}
    />
  );
  const nameCell = (connection: Connection) => (
    <span className="grid">
      <span className="font-medium break-all">{connection.name}</span>
      {connection.instance_name && connection.instance_name !== connection.name ? <span className="text-xs text-muted-foreground">{connection.instance_name}</span> : null}
    </span>
  );
  const plugin = (connection: Connection) => (connection.adapter_version ? connection.adapter_version : "—");
  const lastSeen = (connection: Connection) => (connection.last_seen_at ? formatDateTime(connection.last_seen_at, locale, timeZone) : t("common.status.never"));
  const accountCount = (connection: Connection) => connection.visible_accounts?.length ?? 0;

  return (
    <div className="grid gap-6">
      <div className="flex flex-wrap items-start justify-between gap-4">
        <h1 className="text-xl font-semibold md:text-2xl">{t("connections.title")}</h1>
        <Button onClick={() => setDialog({ kind: "create" })}>
          <Plus aria-hidden="true" />
          {t("connections.new")}
        </Button>
      </div>
      <div className="flex items-center gap-3">
        <Switch id="show-unpaired" checked={includeUnpaired} onCheckedChange={setIncludeUnpaired} />
        <Label htmlFor="show-unpaired" className="min-h-6">
          {t("connections.showUnpaired")}
        </Label>
      </div>

      {connections.isPending ? (
        <div className="grid gap-2" aria-busy="true">
          <p className="sr-only" role="status">
            {t("common.states.loading")}
          </p>
          {[0, 1, 2].map((row) => (
            <Skeleton key={row} className="h-14 w-full" />
          ))}
        </div>
      ) : connections.isError ? (
        <ErrorState title={errorMessage(t, connections.error).message} code={errorMessage(t, connections.error).code} onRetry={() => void connections.refetch()} />
      ) : rows.length === 0 ? (
        <EmptyState
          icon={Cable}
          title={t("connections.empty.title")}
          description={
            <ol className="list-decimal space-y-1 pl-5 text-left">
              <li>{t("connections.empty.step1")}</li>
              <li>{t("connections.empty.step2")}</li>
              <li>{t("connections.empty.step3")}</li>
            </ol>
          }
        >
          <Button onClick={() => setDialog({ kind: "create" })}>{t("connections.new")}</Button>
        </EmptyState>
      ) : desktop ? (
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t("connections.columns.name")}</TableHead>
              <TableHead>{t("connections.columns.status")}</TableHead>
              <TableHead>{t("connections.columns.accounts")}</TableHead>
              <TableHead>{t("connections.columns.plugin")}</TableHead>
              <TableHead>{t("connections.columns.lastSeen")}</TableHead>
              <TableHead>
                <span className="sr-only">{t("connections.columns.actions")}</span>
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {rows.map((connection) => (
              <Fragment key={connection.id}>
                <TableRow>
                  <TableCell>{nameCell(connection)}</TableCell>
                  <TableCell>
                    <StatusBadge connection={connection} />
                  </TableCell>
                  <TableCell>
                    <Button variant="ghost" size="sm" aria-expanded={expanded.has(connection.id)} onClick={() => toggle(connection.id)}>
                      {expanded.has(connection.id) ? <ChevronDown aria-hidden="true" /> : <ChevronRight aria-hidden="true" />}
                      {t("connections.accountCount", { count: accountCount(connection) })}
                    </Button>
                  </TableCell>
                  <TableCell>{plugin(connection)}</TableCell>
                  <TableCell>{lastSeen(connection)}</TableCell>
                  <TableCell>{actions(connection)}</TableCell>
                </TableRow>
                {expanded.has(connection.id) ? (
                  <TableRow>
                    <TableCell colSpan={6}>
                      <ConnectionDetails connection={connection} />
                    </TableCell>
                  </TableRow>
                ) : null}
              </Fragment>
            ))}
          </TableBody>
        </Table>
      ) : (
        <ul className="grid gap-3">
          {rows.map((connection) => (
            <li key={connection.id}>
              <Card>
                <CardHeader className="flex-row items-start justify-between gap-2">
                  <CardTitle className="text-base">{nameCell(connection)}</CardTitle>
                  {actions(connection)}
                </CardHeader>
                <CardContent>
                  <StatusBadge connection={connection} />
                  <dl className="grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-sm">
                    <dt className="text-muted-foreground">{t("connections.columns.plugin")}</dt>
                    <dd>{plugin(connection)}</dd>
                    <dt className="text-muted-foreground">{t("connections.columns.lastSeen")}</dt>
                    <dd>{lastSeen(connection)}</dd>
                  </dl>
                  <Button variant="outline" size="sm" className="self-start" aria-expanded={expanded.has(connection.id)} onClick={() => toggle(connection.id)}>
                    {t("connections.accountCount", { count: accountCount(connection) })}
                  </Button>
                  {expanded.has(connection.id) ? <ConnectionDetails connection={connection} /> : null}
                </CardContent>
              </Card>
            </li>
          ))}
        </ul>
      )}

      {dialog?.kind === "create" ? (
        <NameDialog
          title={t("connections.create.title")}
          description={t("connections.create.description")}
          label={t("connections.name.label")}
          placeholder={t("connections.name.placeholder")}
          unknownOutcome={t("connections.unknownOutcome")}
          submitLabel={t("common.actions.create")}
          onClose={close}
          onRefresh={refresh}
          onSubmit={async (name) => {
            const created = await stepUp.run({ action: "connection_create", description: t("stepUp.actions.connectionCreate", { name }) }, (proof) => create.mutateAsync({ name, proof }));
            setDialog({ kind: "pair", connection: { id: created.connection.id, name: created.connection.name }, pairing: created.pairing, moving: false });
          }}
        />
      ) : null}
      {dialog?.kind === "rename" && target ? (
        <NameDialog
          title={t("connections.rename.title")}
          description={t("connections.rename.description")}
          label={t("connections.name.label")}
          placeholder={t("connections.name.placeholder")}
          unknownOutcome={t("connections.unknownOutcome")}
          submitLabel={t("common.actions.save")}
          initialName={target.name}
          onClose={close}
          onRefresh={refresh}
          onSubmit={async (name) => {
            try {
              await rename.mutateAsync({ id: target.id, name });
              close();
            } catch (error) {
              if (error instanceof ApiError && error.code === "connections.unpaired") {
                close();
                toast.info(t("errors.connections.unpaired"));
                return;
              }
              throw error;
            }
          }}
        />
      ) : null}
      {dialog?.kind === "pair" ? <PairingDialog connection={dialog.connection} pairing={dialog.pairing} moving={dialog.moving} onClose={close} /> : null}
      {dialog?.kind === "unpair" && target ? <UnpairDialog connection={target} onRefresh={refresh} onClose={close} /> : null}

      <AlertDialog open={dialog?.kind === "move" && target !== undefined} onOpenChange={(open) => !open && setDialog(null)}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("connections.move.title", { name: target?.name ?? "" })}</AlertDialogTitle>
            <AlertDialogDescription>{t("connections.move.body")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{t("common.actions.cancel")}</AlertDialogCancel>
            <AlertDialogAction onClick={() => target && void issueCode(target, true)}>{t("connections.move.confirm")}</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
