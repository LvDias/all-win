"use client";

import { ChevronDown } from "lucide-react";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { useState } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { segment } from "@/shared/api/ids";
import type { CommissionSource, Trade } from "@/shared/api/types";
import { focusOnMount } from "@/shared/lib/focus";
import { formatDateTime, formatMoney, formatPrice } from "@/shared/format";
import { useLocalePath } from "@/shared/i18n/navigation";
import { useDisplay } from "@/shared/session/display";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Badge } from "@/shared/ui/badge";
import { Button } from "@/shared/ui/button";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/shared/ui/collapsible";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { Skeleton } from "@/shared/ui/skeleton";
import { Spinner } from "@/shared/ui/spinner";
import { useDeleteManualTrade, useTrade } from "./api";
import { DirectionLabel, ExcursionValue, OriginLabel, PnlText, formatDuration } from "./trade-parts";

type Props = { tradeId: string; fallback: Trade | undefined; onClose: () => void };

/** S21-02: one trade in a side panel (a full-screen sheet on narrow screens). Focus goes to the title; closing returns it to the row. */
export function TradeDetail({ tradeId, fallback, onClose }: Props) {
  const t = useTranslations("trades.detail");
  const root = useTranslations();
  const query = useTrade(tradeId);
  const trade = query.data ?? fallback;
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        {trade ? (
          <Body trade={trade} onClose={onClose} />
        ) : query.isError ? (
          <>
            <DialogHeader>
              <DialogTitle>{t("title")}</DialogTitle>
              <DialogDescription>{query.error instanceof ApiError && query.error.status === 404 ? root("errors.common.not_found") : errorMessage(root, query.error).message}</DialogDescription>
            </DialogHeader>
          </>
        ) : (
          <>
            <DialogHeader>
              <DialogTitle>{t("title")}</DialogTitle>
              <DialogDescription>{root("common.states.loading")}</DialogDescription>
            </DialogHeader>
            <Skeleton className="h-40 w-full" />
          </>
        )}
      </DialogContent>
    </Dialog>
  );
}

const commissionKey: Record<CommissionSource, string> = { platform: "platform", configured: "configured", mixed: "mixed", manual: "manual", none: "none" };

function Body({ trade, onClose }: { trade: Trade; onClose: () => void }) {
  const t = useTranslations("trades.detail");
  const origin = useTranslations("trades");
  const { locale, timeZone } = useDisplay();
  const path = useLocalePath();
  const [deleting, setDeleting] = useState(false);
  const at = (value: string) => formatDateTime(value, locale, timeZone, { seconds: true, zone: true });

  return (
    <div className="grid gap-4">
      <DialogHeader>
        <DialogTitle tabIndex={-1} ref={focusOnMount}>
          {t("heading", { account: trade.account_name })}
        </DialogTitle>
        <DialogDescription>
          <DirectionLabel direction={trade.direction} /> {origin("quantity", { count: trade.quantity })}{" · "}{trade.instrument_symbol}
        </DialogDescription>
      </DialogHeader>

      <section aria-labelledby="trade-summary" className="grid gap-2">
        <h3 id="trade-summary" className="text-sm font-semibold">
          {t("summary")}
        </h3>
        <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-sm">
          <dt className="text-muted-foreground">{t("netPnl")}</dt>
          <dd className="text-base font-semibold">
            <PnlText value={trade.pnl} locale={locale} />
          </dd>
          <dt className="text-muted-foreground">{t("grossPnl")}</dt>
          <dd>
            <PnlText value={trade.gross_pnl} locale={locale} />
          </dd>
          <dt className="text-muted-foreground">{t("commission")}</dt>
          <dd className="flex flex-wrap items-center gap-2">
            <span className="tabular-nums">{formatMoney(trade.commission, locale)}</span>
            <span className="text-muted-foreground">{t(`commissionSource.${commissionKey[trade.commission_source]}`)}</span>
            {trade.commission_source === "none" ? <Badge tone="warning">{t("commissionMissing")}</Badge> : null}
          </dd>
          <dt className="text-muted-foreground">{origin("table.entry")}</dt>
          <dd>
            <time dateTime={trade.entry_time}>{at(trade.entry_time)}</time>{" · "}<span className="tabular-nums">{formatPrice(trade.entry_price, locale, 2)}</span>
          </dd>
          <dt className="text-muted-foreground">{origin("table.exit")}</dt>
          <dd>
            <time dateTime={trade.exit_time}>{at(trade.exit_time)}</time>{" · "}<span className="tabular-nums">{formatPrice(trade.exit_price, locale, 2)}</span>
          </dd>
          <dt className="text-muted-foreground">{origin("table.duration")}</dt>
          <dd>{formatDuration(trade.duration_seconds)}</dd>
          <dt className="text-muted-foreground">{origin("table.mae")}</dt>
          <dd>
            <ExcursionValue value={trade.mae} />
          </dd>
          <dt className="text-muted-foreground">{origin("table.mfe")}</dt>
          <dd>
            <ExcursionValue value={trade.mfe} />
          </dd>
        </dl>
      </section>

      <section aria-labelledby="trade-origin" className="grid gap-2">
        <h3 id="trade-origin" className="text-sm font-semibold">
          {t("origin")}
        </h3>
        <p className="text-sm">
          <OriginLabel trade={trade} />
        </p>
        {trade.origin === "manual" ? <p className="text-sm text-muted-foreground">{t("registeredByYou", { time: at(trade.created_at) })}</p> : null}
        {trade.reconcile_mismatch ? <Alert tone="warning">{t("reconcileMismatch")}</Alert> : null}
      </section>

      <section aria-labelledby="trade-how" className="grid gap-2">
        <h3 id="trade-how" className="text-sm font-semibold">
          {t("how")}
        </h3>
        <p className="text-sm text-muted-foreground">{trade.origin === "manual" ? t("howManual") : t("howCaptured")}</p>
      </section>

      <Collapsible className="rounded-lg border">
        <CollapsibleTrigger className="group flex min-h-11 w-full items-center justify-between gap-2 px-3 text-left text-sm font-medium">
          {t("technical")}
          <ChevronDown aria-hidden="true" className="size-4 transition-transform group-data-[state=open]:rotate-180" />
        </CollapsibleTrigger>
        <CollapsibleContent forceMount className="grid gap-2 px-3 pb-3 data-[state=closed]:hidden">
          <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 font-mono text-xs">
            <dt>{t("ids.trade")}</dt>
            <dd className="break-all">{trade.trade_id}</dd>
            <dt>{t("ids.account")}</dt>
            <dd className="break-all">{trade.account_id}</dd>
            {trade.instruction_id ? (
              <>
                <dt>{t("ids.instruction")}</dt>
                <dd className="break-all">{trade.instruction_id}</dd>
              </>
            ) : null}
            {trade.command_id ? (
              <>
                <dt>{t("ids.command")}</dt>
                <dd className="break-all">{trade.command_id}</dd>
              </>
            ) : null}
            {trade.captured_at ? (
              <>
                <dt>{t("ids.capturedAt")}</dt>
                <dd>{at(trade.captured_at)}</dd>
              </>
            ) : null}
          </dl>
          <div>
            <Button type="button" size="sm" variant="outline" onClick={() => void navigator.clipboard?.writeText(trade.trade_id)}>
              {t("copyId")}
            </Button>
          </div>
        </CollapsibleContent>
      </Collapsible>

      {trade.editable ? (
        <div className="flex flex-wrap gap-2">
          <Button asChild variant="outline">
            <Link href={path(`/trades/${segment(trade.trade_id)}/edit`)}>{t("edit")}</Link>
          </Button>
          <Button type="button" variant="destructive" onClick={() => setDeleting(true)}>
            {t("delete")}
          </Button>
        </div>
      ) : (
        <p className="text-sm text-muted-foreground">{t("notEditable")}</p>
      )}
      {deleting ? <DeleteDialog trade={trade} onClose={() => setDeleting(false)} onDeleted={onClose} /> : null}
    </div>
  );
}

function DeleteDialog({ trade, onClose, onDeleted }: { trade: Trade; onClose: () => void; onDeleted: () => void }) {
  const t = useTranslations("trades.delete");
  const root = useTranslations();
  const remove = useDeleteManualTrade();
  const [error, setError] = useState<string | null>(null);
  return (
    <AlertDialog open onOpenChange={(open) => !open && !remove.isPending && onClose()}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{t("title")}</AlertDialogTitle>
          <AlertDialogDescription>{t("body")}</AlertDialogDescription>
        </AlertDialogHeader>
        {error ? (
          <Alert tone="danger" tabIndex={-1} ref={focusOnMount}>
            {error}
          </Alert>
        ) : null}
        <AlertDialogFooter>
          <AlertDialogCancel autoFocus>{root("common.actions.cancel")}</AlertDialogCancel>
          <AlertDialogAction
            variant="destructive"
            disabled={remove.isPending || trade.row_version === undefined}
            onClick={(event) => {
              event.preventDefault();
              if (trade.row_version === undefined) return;
              remove.mutate(
                { id: trade.trade_id, rowVersion: trade.row_version },
                {
                  onSuccess: () => {
                    toast.success(t("done"));
                    onDeleted();
                  },
                  onError: (failure) => setError(failure instanceof ApiError && failure.code === "common.version_conflict" ? t("conflict") : errorMessage(root, failure).message),
                },
              );
            }}
          >
            {remove.isPending ? <Spinner /> : null}
            {t("confirm")}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
