"use client";

import { useQueryClient } from "@tanstack/react-query";
import { ArrowDown, ArrowUp, ListChecks } from "lucide-react";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { useState } from "react";
import { errorMessage } from "@/shared/api/error-message";
import type { AuditSignalType } from "@/shared/api/types";
import { PageHeader } from "@/shared/components/page-header";
import { StatusBadge } from "@/shared/components/status-badge";
import { formatCount, formatMoney, formatSignedMoney } from "@/shared/format";
import { useLocalePath } from "@/shared/i18n/navigation";
import { useIsDesktop } from "@/shared/lib/use-media-query";
import { cn } from "@/shared/lib/utils";
import { useDisplay } from "@/shared/session/display";
import { useEvents } from "@/shared/session/events";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Card, CardContent } from "@/shared/ui/card";
import { EmptyState } from "@/shared/ui/empty-state";
import { ErrorState } from "@/shared/ui/error-state";
import { ArchivedSwitch } from "@/shared/components/archived-switch";
import { MultiSelect } from "@/shared/ui/multi-select";
import { Skeleton } from "@/shared/ui/skeleton";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/shared/ui/table";
import { tradesQueryKey, useHasAnyTrade, useTradeAccounts, useTradeSources, useTrades, useTradesSummary, type TradeSort } from "./api";
import { TradeDetail } from "./trade-detail";
import { DirectionLabel, Excursions, OriginLabel, PnlText, TimePrice, formatDuration } from "./trade-parts";
import { useTradeFilters, type Period } from "./use-trade-filters";

const periods: Period[] = ["today", "7d", "30d", "custom"];
const signals: AuditSignalType[] = ["trigger", "trade_plan", "nq_trigger"];

/** S21-01: completed trades with filters in the URL, totals of the whole filtered set (Core figures) and paging by cursor. */
export function TradesPage() {
  const t = useTranslations("trades");
  const root = useTranslations();
  const queryClient = useQueryClient();
  const path = useLocalePath();
  const desktop = useIsDesktop();
  const { locale, timeZone } = useDisplay();
  const { ui, filters, query, sort, order, update, clear, activeCount } = useTradeFilters();
  const trades = useTrades(query);
  const summary = useTradesSummary(filters);
  const accounts = useTradeAccounts();
  const sources = useTradeSources();
  const [selected, setSelected] = useState<string | null>(null);
  const [fresh, setFresh] = useState(0);

  // A captured trade never inserts itself under the cursor: it only raises the "n new trades" banner (spec §3.5).
  useEvents((event) => {
    if (event.type === "trade.captured") setFresh((count) => count + 1);
  });

  const rows = trades.data?.pages.flatMap((page) => page.trades) ?? [];
  const unfilteredEmpty = trades.isSuccess && rows.length === 0 && activeCount === 0;
  const anyTrade = useHasAnyTrade(unfilteredEmpty);
  const showFresh = () => {
    setFresh(0);
    void queryClient.invalidateQueries({ queryKey: tradesQueryKey });
  };
  const sortBy = (column: TradeSort) => update(sort === column ? { order: order === "desc" ? "asc" : "desc" } : { sort: column, order: "desc" });

  const accountOptions = [...(accounts.data?.accounts ?? [])].map((account) => ({ value: account.id, label: account.display_name }));
  const sourceOptions = [
    ...(sources.data ?? []).map((source) => ({ value: source.id, label: source.name })),
    { value: "manual", label: t("origin.manual") },
    { value: "platform", label: t("origin.platform") },
  ];
  const signalOptions = signals.map((type) => ({ value: type, label: t(`origin.signal.${type}`) }));

  const header = (
    <PageHeader
      title={t("title")}
      actions={
        <Button asChild>
          <Link href={path("/trades/new")}>{t("add")}</Link>
        </Button>
      }
    />
  );

  const filterBar = (
    <section aria-label={t("filters.label")} className="grid gap-3">
      <div role="group" aria-label={t("filters.period")} className="flex flex-wrap gap-2">
        {periods.map((period) => (
          <Button key={period} type="button" size="sm" variant={ui.period === period ? "default" : "outline"} aria-pressed={ui.period === period} onClick={() => update({ period })}>
            {t(`filters.periods.${period}`)}
          </Button>
        ))}
      </div>
      {ui.period === "custom" ? (
        <div className="flex flex-wrap items-end gap-3">
          <label className="grid gap-1 text-sm">
            {t("filters.from", { zone: timeZone })}
            <input type="date" value={ui.fromDate} onChange={(event) => update({ fromDate: event.target.value })} className="min-h-11 rounded-md border border-input bg-background px-3" />
          </label>
          <label className="grid gap-1 text-sm">
            {t("filters.to", { zone: timeZone })}
            <input type="date" value={ui.toDate} onChange={(event) => update({ toDate: event.target.value })} className="min-h-11 rounded-md border border-input bg-background px-3" />
          </label>
        </div>
      ) : null}
      <div className="flex flex-wrap gap-2">
        <MultiSelect label={t("filters.account")} options={accountOptions} selected={ui.accounts} onChange={(value) => update({ accounts: value })} />
        <MultiSelect label={t("filters.source")} options={sourceOptions} selected={ui.sources} onChange={(value) => update({ sources: value })} />
        <MultiSelect label={t("filters.signal")} options={signalOptions} selected={ui.signals} onChange={(value) => update({ signals: value as AuditSignalType[] })} />
        <ArchivedSwitch checked={ui.archived} onChange={(archived) => update({ archived })} />
        {activeCount > 0 ? (
          <Button type="button" variant="ghost" onClick={clear}>
            {t("filters.clear")}
          </Button>
        ) : null}
      </div>
      <p role="status" className="text-sm text-muted-foreground">
        {activeCount > 0 ? t("filters.active", { count: activeCount }) : t("filters.none")}
      </p>
    </section>
  );

  const totals = (
    <Card aria-label={t("totals.label")}>
      <CardContent className="grid gap-2 pt-4">
        {summary.isPending ? (
          <Skeleton className="h-14 w-full" />
        ) : summary.isError ? (
          <div className="flex flex-wrap items-center gap-3">
            <span aria-label={t("totals.unavailable")}>{"—"}</span>
            <Button size="sm" variant="outline" onClick={() => void summary.refetch()}>
              {root("common.actions.retry")}
            </Button>
          </div>
        ) : (
          <>
            <div className="flex flex-wrap items-baseline gap-x-6 gap-y-1">
              <p className="text-sm text-muted-foreground">{t("totals.count", { count: summary.data.count })}</p>
              <p className="text-lg font-semibold">
                <span className="mr-2 text-sm font-normal text-muted-foreground">{t("totals.net")}</span>
                <PnlText value={summary.data.total_pnl} locale={locale} />
              </p>
              <p className="text-sm text-muted-foreground">{t("totals.secondary", { gross: formatSignedMoney(summary.data.total_gross_pnl, locale), commission: formatMoney(summary.data.total_commission, locale) })}</p>
            </div>
            {(summary.data.uncommissioned_count ?? 0) > 0 ? <Alert tone="warning">{t("totals.uncommissioned", { count: summary.data.uncommissioned_count ?? 0 })}</Alert> : null}
          </>
        )}
      </CardContent>
    </Card>
  );

  let body;
  if (trades.isPending) {
    body = (
      <div className="grid gap-2" aria-busy="true">
        {[0, 1, 2, 3, 4].map((row) => (
          <Skeleton key={row} className="h-12 w-full" />
        ))}
      </div>
    );
  } else if (trades.isError) {
    const failure = errorMessage(root, trades.error);
    body = <ErrorState title={t("loadError")} code={failure.code} onRetry={() => void trades.refetch()} />;
  } else if (unfilteredEmpty && anyTrade.isPending) {
    body = <Skeleton className="h-40 w-full" aria-busy="true" />;
  } else if (unfilteredEmpty && (anyTrade.data?.count ?? 0) > 0) {
    body = (
      <EmptyState icon={ListChecks} title={t("empty.archivedTitle")} description={t("empty.archivedBody")}>
        <Button variant="outline" onClick={() => update({ archived: true })}>
          {root("common.includeArchived")}
        </Button>
      </EmptyState>
    );
  } else if (unfilteredEmpty) {
    body = (
      <EmptyState icon={ListChecks} title={t("empty.title")} description={t("empty.body")}>
        <div className="flex flex-wrap justify-center gap-2">
          <Button asChild>
            <Link href={path("/trades/new")}>{t("add")}</Link>
          </Button>
          <Button asChild variant="outline">
            <Link href={path("/connections/ninjatrader")}>{t("empty.connections")}</Link>
          </Button>
        </div>
      </EmptyState>
    );
  } else if (rows.length === 0) {
    body = (
      <EmptyState icon={ListChecks} title={t("empty.filteredTitle")} description={t("empty.filteredBody")}>
        <Button variant="outline" onClick={clear}>
          {t("filters.clear")}
        </Button>
      </EmptyState>
    );
  } else {
    const ariaSort = (column: TradeSort) => (sort === column ? (order === "asc" ? "ascending" : "descending") : "none");
    const sortHead = (column: TradeSort, label: string, align = "") => (
      <TableHead aria-sort={ariaSort(column)} className={align}>
        <button type="button" onClick={() => sortBy(column)} className="inline-flex min-h-11 items-center gap-1 font-medium">
          {label}
          {sort === column ? order === "asc" ? <ArrowUp aria-hidden="true" className="size-3.5" /> : <ArrowDown aria-hidden="true" className="size-3.5" /> : null}
        </button>
      </TableHead>
    );
    body = (
      <div className="grid gap-3">
        {desktop ? (
          <Table>
            <caption className="pb-2 text-left text-sm text-muted-foreground">{t("table.caption", { shown: rows.length, total: summary.data?.count ?? rows.length })}</caption>
            <TableHeader>
              <TableRow>
                <TableHead>{t("table.account")}</TableHead>
                <TableHead>{t("table.direction")}</TableHead>
                <TableHead>{t("table.quantity")}</TableHead>
                <TableHead>{t("table.entry")}</TableHead>
                {sortHead("exit_time", t("table.exit"))}
                {sortHead("duration", t("table.duration"))}
                {sortHead("pnl", t("table.pnl"), "text-right")}
                <TableHead>{t("table.excursions")}</TableHead>
                <TableHead>{t("table.origin")}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {rows.map((trade) => (
                <TableRow key={trade.trade_id}>
                  <TableCell>
                    <button type="button" onClick={() => setSelected(trade.trade_id)} className="min-h-11 text-left font-medium underline-offset-4 hover:underline">
                      {trade.account_name}
                      {trade.account_deleted ? ` ${t("deleted")}` : ""}
                    </button>
                  </TableCell>
                  <TableCell>
                    <DirectionLabel direction={trade.direction} />
                  </TableCell>
                  <TableCell>{t("quantity", { count: trade.quantity })}</TableCell>
                  <TableCell>
                    <TimePrice time={trade.entry_time} price={trade.entry_price} />
                  </TableCell>
                  <TableCell>
                    <TimePrice time={trade.exit_time} price={trade.exit_price} />
                  </TableCell>
                  <TableCell className="whitespace-nowrap">{formatDuration(trade.duration_seconds)}</TableCell>
                  <TableCell className="text-right">
                    <PnlText value={trade.pnl} locale={locale} />
                  </TableCell>
                  <TableCell>
                    <Excursions trade={trade} />
                  </TableCell>
                  <TableCell>
                    <OriginLabel trade={trade} />
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        ) : (
          <ul className="grid gap-3">
            {rows.map((trade) => (
              <li key={trade.trade_id}>
                <Card>
                  <CardContent className="grid gap-2 pt-4 text-sm">
                    <div className="flex items-start justify-between gap-3">
                      <button type="button" onClick={() => setSelected(trade.trade_id)} className="min-h-11 text-left font-medium">
                        {trade.account_name}{" · "}<DirectionLabel direction={trade.direction} />
                      </button>
                      <PnlText value={trade.pnl} locale={locale} className="font-semibold" />
                    </div>
                    <dl className="grid grid-cols-2 gap-x-3 gap-y-1">
                      <dt className="text-muted-foreground">{t("table.quantity")}</dt>
                      <dd>{t("quantity", { count: trade.quantity })}</dd>
                      <dt className="text-muted-foreground">{t("table.entry")}</dt>
                      <dd>
                        <TimePrice time={trade.entry_time} price={trade.entry_price} />
                      </dd>
                      <dt className="text-muted-foreground">{t("table.exit")}</dt>
                      <dd>
                        <TimePrice time={trade.exit_time} price={trade.exit_price} />
                      </dd>
                      <dt className="text-muted-foreground">{t("table.duration")}</dt>
                      <dd>{formatDuration(trade.duration_seconds)}</dd>
                      <dt className="text-muted-foreground">{t("table.origin")}</dt>
                      <dd>
                        <OriginLabel trade={trade} />
                      </dd>
                    </dl>
                  </CardContent>
                </Card>
              </li>
            ))}
          </ul>
        )}
        <div className="flex flex-wrap items-center justify-between gap-2">
          <p className="text-sm text-muted-foreground">{t("table.count", { shown: formatCount(String(rows.length), locale), total: formatCount(String(summary.data?.count ?? rows.length), locale) })}</p>
          {trades.hasNextPage ? (
            <Button variant="outline" disabled={trades.isFetchingNextPage} onClick={() => void trades.fetchNextPage()}>
              {t("table.loadMore")}
            </Button>
          ) : null}
        </div>
      </div>
    );
  }

  return (
    <div className="grid gap-4">
      {header}
      {filterBar}
      {totals}
      {fresh > 0 ? (
        <div role="status" className={cn("flex flex-wrap items-center gap-3 rounded-lg border border-status-info/30 bg-status-info-soft p-3 text-sm text-status-info")}>
          <span>{t("fresh.banner", { count: fresh })}</span>
          <Button size="sm" variant="outline" onClick={showFresh}>
            {t("fresh.show")}
          </Button>
        </div>
      ) : null}
      {summary.isSuccess && trades.isError ? <StatusBadge kind="stale" /> : null}
      {body}
      {selected ? <TradeDetail tradeId={selected} fallback={rows.find((row) => row.trade_id === selected)} onClose={() => setSelected(null)} /> : null}
    </div>
  );
}
