"use client";

import { useQueryClient } from "@tanstack/react-query";
import { Ban, Bot, Plus, Radio } from "lucide-react";
import { useTranslations } from "next-intl";
import { useState } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import type { Source, SourceWithUrl } from "@/shared/api/types";
import { formatDateTime, formatRelative } from "@/shared/format";
import { useIsDesktop } from "@/shared/lib/use-media-query";
import { useDisplay } from "@/shared/session/display";
import { StepUpCancelled, useStepUp } from "@/shared/session/step-up";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Badge } from "@/shared/ui/badge";
import { Button } from "@/shared/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/shared/ui/card";
import { EmptyState } from "@/shared/ui/empty-state";
import { ErrorState } from "@/shared/ui/error-state";
import { Label } from "@/shared/ui/label";
import { NameDialog } from "@/shared/ui/name-dialog";
import { Skeleton } from "@/shared/ui/skeleton";
import { Switch } from "@/shared/ui/switch";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/shared/ui/table";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/shared/ui/tooltip";
import { sourcesQueryKey, useCreateSource, useRenameSource, useRevokeSource, useRotateSource, useSources } from "./api";
import { SourceActions } from "./source-actions";
import { WebhookUrlDialog } from "./webhook-url-dialog";

type Dialog =
  | { kind: "create" }
  | { kind: "rename"; source: Source }
  | { kind: "rotate"; source: Source }
  | { kind: "revoke"; source: Source }
  | { kind: "reveal"; result: SourceWithUrl; mode: "created" | "rotated" }
  | null;

export function SourcesPage() {
  const t = useTranslations();
  const queryClient = useQueryClient();
  const stepUp = useStepUp();
  const desktop = useIsDesktop();
  const { locale, timeZone } = useDisplay();
  const [includeRevoked, setIncludeRevoked] = useState(false);
  const [dialog, setDialog] = useState<Dialog>(null);
  const sources = useSources(includeRevoked);
  const create = useCreateSource();
  const rotate = useRotateSource();
  const rename = useRenameSource();
  const revoke = useRevokeSource();

  const refresh = () => void queryClient.invalidateQueries({ queryKey: sourcesQueryKey });
  const close = () => setDialog(null);

  /** A stale row: refetch and tell the user why the action did nothing (S3 spec §3). */
  function handleFailure(error: unknown) {
    if (error instanceof StepUpCancelled) return;
    refresh();
    if (error instanceof ApiError && error.code === "sources.revoked") toast.info(t("errors.sources.revoked"));
    else toast.error(errorMessage(t, error).message);
  }

  async function confirmRotate(source: Source) {
    close();
    try {
      const result = await stepUp.run({ action: "source_rotate", targetId: source.id, description: t("stepUp.actions.sourceRotate", { name: source.name }) }, (proof) => rotate.mutateAsync({ id: source.id, proof }));
      setDialog({ kind: "reveal", result, mode: "rotated" });
    } catch (error) {
      handleFailure(error);
    }
  }

  async function confirmRevoke(source: Source) {
    close();
    try {
      await revoke.mutateAsync(source.id);
      toast.success(t("sources.revoke.toast", { name: source.name }));
    } catch (error) {
      handleFailure(error);
    }
  }

  function closeReveal(result: SourceWithUrl, mode: "created" | "rotated") {
    close();
    refresh();
    toast.success(mode === "created" ? t("sources.created") : t("sources.rotated", { name: result.source.name }));
  }

  const rows = sources.data ?? [];
  const actions = (source: Source) => (
    <SourceActions source={source} onRename={() => setDialog({ kind: "rename", source })} onRotate={() => setDialog({ kind: "rotate", source })} onRevoke={() => setDialog({ kind: "revoke", source })} />
  );

  const statusBadge = (source: Source) =>
    source.status === "active" ? (
      <Badge tone="success">{t("sources.status.active")}</Badge>
    ) : (
      <Badge tone="neutral">
        <Ban aria-hidden="true" />
        {t("sources.status.revoked")}
        {source.revoked_at ? ` · ${formatDateTime(source.revoked_at, locale, timeZone)}` : ""}
      </Badge>
    );
  const typeCell = (source: Source) => (
    <span className="flex flex-wrap items-center gap-2">
      {t("sources.type.tradingview")}
      {source.is_automated ? (
        <Badge tone="info">
          <Bot aria-hidden="true" />
          {t("sources.automated")}
        </Badge>
      ) : null}
    </span>
  );
  const urlCell = (source: Source) => (
    <Tooltip>
      <TooltipTrigger asChild>
        <span tabIndex={0} aria-label={t("sources.urlAria", { keyId: source.key_id })} className="block max-w-72 truncate font-mono text-xs">
          {source.masked_url}
        </span>
      </TooltipTrigger>
      <TooltipContent className="font-mono break-all">{source.masked_url}</TooltipContent>
    </Tooltip>
  );
  const lastSignal = (source: Source) =>
    source.last_received_at ? (
      <time dateTime={source.last_received_at} title={formatDateTime(source.last_received_at, locale, timeZone)}>
        {formatRelative(source.last_received_at, locale)}
      </time>
    ) : (
      t("common.status.never")
    );

  return (
    <div className="grid gap-6">
      <div className="flex flex-wrap items-start justify-between gap-4">
        <div className="grid gap-1">
          <h1 className="text-xl font-semibold md:text-2xl">{t("sources.title")}</h1>
          <p className="max-w-xl text-sm text-muted-foreground">{t("sources.description")}</p>
        </div>
        <Button onClick={() => setDialog({ kind: "create" })}>
          <Plus aria-hidden="true" />
          {t("sources.new")}
        </Button>
      </div>

      <div className="flex items-center gap-3">
        <Switch id="show-revoked" checked={includeRevoked} onCheckedChange={setIncludeRevoked} />
        <Label htmlFor="show-revoked" className="min-h-6">
          {t("sources.showRevoked")}
        </Label>
      </div>

      {sources.isPending ? (
        <div className="grid gap-2" aria-busy="true">
          <p className="sr-only" role="status">
            {t("common.states.loading")}
          </p>
          {[0, 1, 2].map((row) => (
            <Skeleton key={row} className="h-14 w-full" />
          ))}
        </div>
      ) : sources.isError ? (
        <ErrorState title={errorMessage(t, sources.error).message} code={errorMessage(t, sources.error).code} onRetry={() => void sources.refetch()} />
      ) : rows.length === 0 ? (
        <EmptyState icon={Radio} title={t("sources.empty.title")} description={t("sources.empty.text")}>
          <Button onClick={() => setDialog({ kind: "create" })}>{t("sources.new")}</Button>
        </EmptyState>
      ) : desktop ? (
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t("sources.columns.name")}</TableHead>
              <TableHead>{t("sources.columns.type")}</TableHead>
              <TableHead>{t("sources.columns.url")}</TableHead>
              <TableHead>{t("sources.columns.status")}</TableHead>
              <TableHead>{t("sources.columns.created")}</TableHead>
              <TableHead>{t("sources.columns.lastSignal")}</TableHead>
              <TableHead>
                <span className="sr-only">{t("sources.columns.actions")}</span>
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {rows.map((source) => (
              <TableRow key={source.id}>
                <TableCell className="font-medium">{source.name}</TableCell>
                <TableCell>{typeCell(source)}</TableCell>
                <TableCell>{urlCell(source)}</TableCell>
                <TableCell>{statusBadge(source)}</TableCell>
                <TableCell>{formatDateTime(source.created_at, locale, timeZone)}</TableCell>
                <TableCell>{lastSignal(source)}</TableCell>
                <TableCell>{actions(source)}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      ) : (
        <ul className="grid gap-3">
          {rows.map((source) => (
            <li key={source.id}>
              <Card>
                <CardHeader className="flex-row items-start justify-between gap-2">
                  <CardTitle className="text-base break-all">{source.name}</CardTitle>
                  {statusBadge(source)}
                </CardHeader>
                <CardContent>
                  <dl className="grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-sm">
                    <dt className="text-muted-foreground">{t("sources.columns.type")}</dt>
                    <dd>{typeCell(source)}</dd>
                    <dt className="text-muted-foreground">{t("sources.columns.url")}</dt>
                    <dd className="min-w-0">{urlCell(source)}</dd>
                    <dt className="text-muted-foreground">{t("sources.columns.created")}</dt>
                    <dd>{formatDateTime(source.created_at, locale, timeZone)}</dd>
                    <dt className="text-muted-foreground">{t("sources.columns.lastSignal")}</dt>
                    <dd>{lastSignal(source)}</dd>
                  </dl>
                  <div className="flex justify-end">{actions(source)}</div>
                </CardContent>
              </Card>
            </li>
          ))}
        </ul>
      )}

      {dialog?.kind === "create" ? (
        <NameDialog
          label={t("sources.name.label")}
          placeholder={t("sources.name.placeholder")}
          unknownOutcome={t("sources.unknownOutcome")}
          title={t("sources.create.title")}
          description={t("sources.create.description")}
          submitLabel={t("common.actions.create")}
          onClose={close}
          onRefresh={refresh}
          onSubmit={async (name) => {
            const result = await stepUp.run({ action: "source_create", description: t("stepUp.actions.sourceCreate", { name }) }, (proof) => create.mutateAsync({ name, proof }));
            setDialog({ kind: "reveal", result, mode: "created" });
          }}
        />
      ) : null}
      {dialog?.kind === "rename" ? (
        <NameDialog
          label={t("sources.name.label")}
          placeholder={t("sources.name.placeholder")}
          unknownOutcome={t("sources.unknownOutcome")}
          title={t("sources.rename.title")}
          description={t("sources.rename.description")}
          submitLabel={t("common.actions.save")}
          initialName={dialog.source.name}
          onClose={close}
          onRefresh={refresh}
          onSubmit={async (name) => {
            try {
              await rename.mutateAsync({ id: dialog.source.id, name });
              close();
            } catch (error) {
              if (error instanceof ApiError && error.code === "sources.revoked") {
                close();
                handleFailure(error);
                return;
              }
              throw error;
            }
          }}
        />
      ) : null}
      {dialog?.kind === "reveal" ? <WebhookUrlDialog result={dialog.result} mode={dialog.mode} onClose={() => closeReveal(dialog.result, dialog.mode)} /> : null}

      <AlertDialog open={dialog?.kind === "rotate"} onOpenChange={(open) => !open && close()}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("sources.rotate.title", { name: dialog?.kind === "rotate" ? dialog.source.name : "" })}</AlertDialogTitle>
            <AlertDialogDescription>{t("sources.rotate.body")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{t("common.actions.cancel")}</AlertDialogCancel>
            <AlertDialogAction variant="destructive" onClick={() => dialog?.kind === "rotate" && void confirmRotate(dialog.source)}>
              {t("sources.rotate.confirm")}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
      <AlertDialog open={dialog?.kind === "revoke"} onOpenChange={(open) => !open && close()}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("sources.revoke.title", { name: dialog?.kind === "revoke" ? dialog.source.name : "" })}</AlertDialogTitle>
            <AlertDialogDescription>{t("sources.revoke.body")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{t("common.actions.cancel")}</AlertDialogCancel>
            <AlertDialogAction variant="destructive" onClick={() => dialog?.kind === "revoke" && void confirmRevoke(dialog.source)}>
              {t("sources.revoke.confirm")}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
