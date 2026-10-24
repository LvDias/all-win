"use client";

import { useQueryClient } from "@tanstack/react-query";
import { ArrowDown, ArrowUp, ChartNoAxesColumn, Info } from "lucide-react";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { useState, type ReactNode } from "react";
import { errorMessage } from "@/shared/api/error-message";
import type { Performance, PerformanceFigures, PerformanceGroup } from "@/shared/api/types";
import { PageHeader } from "@/shared/components/page-header";
import { StatusBadge } from "@/shared/components/status-badge";
import { formatCount, formatDateTime, formatDecimal, formatMoney, formatSignedMoney } from "@/shared/format";
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
import { MultiSelect } from "@/shared/ui/multi-select";
import { Popover, PopoverContent, PopoverTrigger } from "@/shared/ui/popover";
import { Skeleton } from "@/shared/ui/skeleton";
import { Table, TableBody, TableCell, TableFooter, TableHead, TableHeader, TableRow } from "@/shared/ui/table";
import { performanceQueryKey, useAccountsForFilter, useHasAnyTrade, usePerformance, type GroupBy, type PerformanceSort } from "./api";
import { usePerformanceFilters } from "./use-performance-filters";

const groupOptions: GroupBy[] = ["source_signal", "source", "signal_type"];
const periodOptions = ["today", "7d", "30d", "custom"] as const;

/** The duration in seconds the Core sent, as "12 min 30 s" / "2 h 05 min". No rounding or averaging here. */
function formatDuration(totalSeconds: number): string {
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;
  if (hours > 0) return `${hours} h ${String(minutes).padStart(2, "0")} min`;
  if (minutes > 0) return `${minutes} min ${String(seconds).padStart(2, "0")} s`;
  return `${seconds} s`;
}

/** S22-01: totals and per-trade averages for a period and accounts, split by source and signal type. Every figure is the Core's. */
export function PerformancePage() {
  const t = useTranslations("performance");
  const root = useTranslations();
  const queryClient = useQueryClient();
  const path = useLocalePath();
  const desktop = useIsDesktop();
  const { locale, timeZone } = useDisplay();
  const { ui, query, groupBy, sort, order, update, clear, activeCount, tradesQuery } = usePerformanceFilters();
  const performance = usePerformance(query);
  const accounts = useAccountsForFilter();
  const empty = performance.data?.totals.trade_count === 0;
  const anyTrade = useHasAnyTrade(Boolean(empty));
  const [freshSince, setFreshSince] = useState<string | null>(null);

  // New trades never change the figures under the user's eyes (they keep reconciling with the list): a banner offers to refresh.
  useEvents((event) => {
    if (event.type === "trade.captured") setFreshSince((previous) => previous ?? event.at);
  });
  const refresh = () => {
    setFreshSince(null);
    void queryClient.invalidateQueries({ queryKey: performanceQueryKey });
  };

  const sortBy = (column: PerformanceSort) => update(sort === column ? { order: order === "desc" ? "asc" : "desc" } : { sort: column, order: "desc" });
  const accountOptions = (accounts.data?.accounts ?? []).map((account) => ({ value: account.id, label: account.display_name }));
  const dimmed = performance.isPlaceholderData;

  const header = <PageHeader title={t("title")} />;

  const filterBar = (
    <section aria-label={t("filters.label")} className="grid gap-3">
      <div role="group" aria-label={t("filters.period")} className="flex flex-wrap gap-2">
        {periodOptions.map((period) => (
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
        <MultiSelect label={t("filters.accounts")} options={accountOptions} selected={ui.accounts} onChange={(value) => update({ accounts: value })} />
        {activeCount > 0 ? (
          <Button type="button" variant="ghost" onClick={clear}>
            {t("filters.clear")}
          </Button>
        ) : null}
      </div>
      <p role="status" className="text-sm text-muted-foreground">
        {performance.data ? t("filters.updated", { count: performance.data.totals.trade_count }) : null}
      </p>
    </section>
  );

  let body;
  if (performance.isPending) {
    body = (
      <div className="grid gap-3" aria-busy="true">
        <div className="grid grid-cols-2 gap-3 lg:grid-cols-4">
          {[0, 1, 2, 3, 4, 5, 6].map((card) => (
            <Skeleton key={card} className="h-20 w-full" />
          ))}
        </div>
        {[0, 1, 2, 3].map((row) => (
          <Skeleton key={row} className="h-12 w-full" />
        ))}
      </div>
    );
  } else if (performance.isError) {
    body = <ErrorState title={t("loadError")} code={errorMessage(root, performance.error).code} onRetry={() => void performance.refetch()} />;
  } else if (performance.data.totals.trade_count === 0) {
    const first = anyTrade.data !== undefined && anyTrade.data.count === 0;
    body = first ? (
      <EmptyState icon={ChartNoAxesColumn} title={t("empty.title")} description={t("empty.body")}>
        <div className="flex flex-wrap justify-center gap-2">
          <Button asChild variant="outline">
            <Link href={path("/connections/ninjatrader")}>{t("empty.connections")}</Link>
          </Button>
          <Button asChild>
            <Link href={path("/trades/new")}>{t("empty.add")}</Link>
          </Button>
        </div>
      </EmptyState>
    ) : (
      // AC-5: the summary keeps its figures (0 trades, US$ 0,00, averages and rates as a dash), the sentence says why, and the table is replaced.
      <div className={cn("grid gap-6", dimmed && "opacity-60")} aria-busy={dimmed}>
        <Figures totals={performance.data.totals} />
        <p className="text-sm font-medium">{t("empty.noTrades")}</p>
        <EmptyState icon={ChartNoAxesColumn} title={t("empty.filteredTitle")} description={t("empty.filteredBody")}>
          <Button variant="outline" onClick={clear}>
            {t("filters.clear")}
          </Button>
        </EmptyState>
      </div>
    );
  } else {
    body = (
      <div className={cn("grid gap-6", dimmed && "opacity-60")} aria-busy={dimmed}>
        <Figures totals={performance.data.totals} />
        <Breakdown data={performance.data} desktop={desktop} sort={sort} order={order} onSort={sortBy} tradesQuery={tradesQuery} groupBy={groupBy} />
        <Button asChild variant="link" className="justify-self-start">
          <Link href={path(`/trades${tradesQuery.size > 0 ? `?${tradesQuery.toString()}` : ""}`)}>{t("seeTrades")}</Link>
        </Button>
      </div>
    );
  }

  return (
    <div className="grid gap-4">
      {header}
      {performance.data ? (
        <p className="flex flex-wrap items-center gap-2 text-sm text-muted-foreground">
          <span>
            {t("asOf")} <time dateTime={performance.data.as_of}>{formatDateTime(performance.data.as_of, locale, timeZone, { seconds: true, zone: true })}</time>
          </span>
          {performance.isError ? <StatusBadge kind="stale" /> : null}
        </p>
      ) : null}
      {filterBar}
      {freshSince ? (
        <div role="status" className="flex flex-wrap items-center gap-3 rounded-lg border border-status-info/30 bg-status-info-soft p-3 text-sm text-status-info">
          <span>{t("fresh.banner", { time: formatDateTime(freshSince, locale, timeZone, { zone: true }) })}</span>
          <Button size="sm" variant="outline" onClick={refresh}>
            {t("fresh.refresh")}
          </Button>
        </div>
      ) : null}
      <div role="group" aria-label={t("groupBy.label")} className="flex flex-wrap items-center gap-2">
        <span className="text-sm font-medium">{t("groupBy.label")}</span>
        {groupOptions.map((option) => (
          <Button key={option} type="button" size="sm" variant={groupBy === option ? "default" : "outline"} aria-pressed={groupBy === option} onClick={() => update({ groupBy: option })}>
            {t(`groupBy.${option}`)}
          </Button>
        ))}
      </div>
      {body}
    </div>
  );
}

function Definition({ figure, text }: { figure: string; text: string }) {
  const t = useTranslations("performance");
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button type="button" variant="ghost" size="icon" className="size-8 min-h-0" aria-label={t("howCalculated", { figure })}>
          <Info aria-hidden="true" className="size-4" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-72 p-3 text-sm">{text}</PopoverContent>
    </Popover>
  );
}

function PnlText({ value, className }: { value: string; className?: string }) {
  const { locale } = useDisplay();
  const negative = value.startsWith("-");
  const zero = /^-?0+(\.0+)?$/.test(value);
  return <span className={cn("tabular-nums", !zero && (negative ? "text-status-danger" : "text-status-success"), className)}>{formatSignedMoney(value, locale)}</span>;
}

/** The seven figures. `—`, never 0, when the Core sends no value. */
function Figures({ totals }: { totals: PerformanceFigures }) {
  const t = useTranslations("performance.figures");
  const d = useTranslations("performance.definitions");
  const { locale } = useDisplay();
  const dash = "—";
  const cards: { key: string; label: string; definition: string; value: ReactNode; note?: ReactNode }[] = [
    { key: "trades", label: t("trades"), definition: d("trades"), value: formatCount(String(totals.trade_count), locale) },
    { key: "total", label: t("totalPnl"), definition: d("totalPnl"), value: <PnlText value={totals.total_pnl} className="text-lg font-semibold" />, note: t("secondary", { gross: formatSignedMoney(totals.total_gross_pnl, locale), commission: formatMoney(totals.total_commission, locale) }) },
    { key: "avg", label: t("avgPnl"), definition: d("avgPnl"), value: totals.avg_pnl !== undefined ? <PnlText value={totals.avg_pnl} className="text-lg font-semibold" /> : dash },
    {
      key: "win",
      label: t("winRate"),
      definition: d("winRate"),
      value: totals.win_rate !== undefined ? t("percent", { value: formatDecimal(totals.win_rate, locale, 1) }) : dash,
      note: totals.trade_count > 0 ? t("counts", { wins: totals.win_count, losses: totals.loss_count, zero: totals.breakeven_count }) : undefined,
    },
    { key: "time", label: t("avgDuration"), definition: d("avgDuration"), value: totals.avg_duration_seconds !== undefined ? formatDuration(totals.avg_duration_seconds) : dash },
    {
      key: "mae",
      label: t("avgMae"),
      definition: d("excursion"),
      value: totals.avg_mae !== undefined ? formatMoney(totals.avg_mae, locale) : dash,
      note: totals.mae_sample_count < totals.trade_count ? t("basis", { k: totals.mae_sample_count, n: totals.trade_count }) : undefined,
    },
    {
      key: "mfe",
      label: t("avgMfe"),
      definition: d("excursion"),
      value: totals.avg_mfe !== undefined ? formatMoney(totals.avg_mfe, locale) : dash,
      note: totals.mfe_sample_count < totals.trade_count ? t("basis", { k: totals.mfe_sample_count, n: totals.trade_count }) : undefined,
    },
  ];
  return (
    <section aria-label={t("label")} className="grid gap-3">
      <dl className="grid grid-cols-2 gap-3 lg:grid-cols-4">
        {cards.map((card) => (
          // dt and dd sit one level (this div) under the dl, as HTML and axe require for a description list.
          <div key={card.key} className="grid gap-1 rounded-lg border bg-card p-4 text-card-foreground shadow-xs">
            <dt className="flex items-center justify-between gap-1 text-sm text-muted-foreground">
              {card.label}
              <Definition figure={card.label} text={card.definition} />
            </dt>
            <dd className="grid gap-1">
              <span className="text-lg font-semibold tabular-nums">{card.value}</span>
              {card.note ? <span className="text-xs text-muted-foreground">{card.note}</span> : null}
            </dd>
          </div>
        ))}
      </dl>
      {(totals.uncommissioned_count ?? 0) > 0 ? <Alert tone="warning">{t("uncommissioned", { count: totals.uncommissioned_count ?? 0 })}</Alert> : null}
      {(totals.uncommissioned_count ?? 0) > 0 ? <p className="flex items-center gap-1 text-xs text-muted-foreground"><Info aria-hidden="true" className="size-3.5" />{t("commissionsNotice")}</p> : null}
    </section>
  );
}

function groupName(group: PerformanceGroup, groupBy: GroupBy, t: ReturnType<typeof useTranslations<"performance.group">>): string {
  if (groupBy === "signal_type") return group.signal_type ? t(`signal.${group.signal_type}`) : t(group.origin === "manual" ? "manual" : "platform");
  const base = group.origin === "manual" ? t("manual") : group.origin === "platform" ? t("platform") : group.origin === "master" ? t("master", { account: group.master_account_name ?? "" }) : (group.source_name ?? "");
  const revoked = group.source_revoked ? ` ${t("revoked")}` : "";
  return groupBy === "source_signal" && group.signal_type ? `${base}${revoked}${t("separator")}${t(`signal.${group.signal_type}`)}` : `${base}${revoked}`;
}

function seeHref(group: PerformanceGroup, groupBy: GroupBy, base: URLSearchParams): string {
  const p = new URLSearchParams(base);
  if (groupBy !== "signal_type") {
    if (group.origin === "manual") p.append("source", "manual");
    else if (group.origin === "platform") p.append("source", "platform");
    else if (group.source_id) p.append("source", group.source_id);
  }
  if (groupBy !== "source" && group.signal_type) p.append("signal", group.signal_type);
  return `/trades?${p.toString()}`;
}

function Breakdown({ data, desktop, sort, order, onSort, tradesQuery, groupBy }: { data: Performance; desktop: boolean; sort: PerformanceSort; order: "asc" | "desc"; onSort: (column: PerformanceSort) => void; tradesQuery: URLSearchParams; groupBy: GroupBy }) {
  const t = useTranslations("performance");
  const g = useTranslations("performance.group");
  const { locale } = useDisplay();
  const path = useLocalePath();
  const ariaSort = (column: PerformanceSort) => (sort === column ? (order === "asc" ? "ascending" : "descending") : "none");
  const head = (column: PerformanceSort, label: string) => (
    <TableHead aria-sort={ariaSort(column)}>
      <button type="button" onClick={() => onSort(column)} className="inline-flex min-h-11 items-center gap-1 font-medium">
        {label}
        {sort === column ? order === "asc" ? <ArrowUp aria-hidden="true" className="size-3.5" /> : <ArrowDown aria-hidden="true" className="size-3.5" /> : null}
      </button>
    </TableHead>
  );
  const f = (value: string | undefined, format: (v: string) => string) => (value === undefined ? "—" : format(value));
  const dash = "—";

  if (!desktop) {
    const card = (name: string, figures: PerformanceFigures, key: string, href?: string, ratio?: string) => (
      <li key={key}>
        <Card>
          <CardContent className="grid gap-2 pt-4 text-sm">
            <div className="flex items-start justify-between gap-3">
              <p className="font-medium">{name}</p>
              <PnlText value={figures.total_pnl} className="font-semibold" />
            </div>
            {ratio ? <Bar ratio={ratio} negative={figures.total_pnl.startsWith("-")} /> : null}
            <dl className="grid grid-cols-2 gap-x-3 gap-y-1">
              <dt className="text-muted-foreground">{t("figures.trades")}</dt>
              <dd>
                {formatCount(String(figures.trade_count), locale)}
                {href ? (
                  <>
                    {" "}
                    <Link href={path(href)} className="underline underline-offset-4">
                      {t("table.see")}
                    </Link>
                  </>
                ) : null}
              </dd>
              <dt className="text-muted-foreground">{t("figures.avgPnl")}</dt>
              <dd>{figures.avg_pnl !== undefined ? <PnlText value={figures.avg_pnl} /> : dash}</dd>
              <dt className="text-muted-foreground">{t("figures.winRate")}</dt>
              <dd>{figures.win_rate !== undefined ? t("figures.percent", { value: formatDecimal(figures.win_rate, locale, 1) }) : dash}</dd>
              <dt className="text-muted-foreground">{t("figures.avgDuration")}</dt>
              <dd>{figures.avg_duration_seconds !== undefined ? formatDuration(figures.avg_duration_seconds) : dash}</dd>
              <dt className="text-muted-foreground">{t("table.mae")}</dt>
              <dd>{f(figures.avg_mae, (v) => formatMoney(v, locale))}</dd>
              <dt className="text-muted-foreground">{t("table.mfe")}</dt>
              <dd>{f(figures.avg_mfe, (v) => formatMoney(v, locale))}</dd>
            </dl>
          </CardContent>
        </Card>
      </li>
    );
    return (
      <ul className="grid gap-3">
        {card(t("table.total"), data.totals, "total")}
        {data.groups.map((group, index) => card(groupName(group, groupBy, g), group.figures, String(index), seeHref(group, groupBy, tradesQuery), group.bar_ratio))}
      </ul>
    );
  }

  return (
    <Table>
      <caption className="pb-2 text-left text-sm text-muted-foreground">{t("table.caption", { count: data.groups.length, column: t(`table.sortColumns.${sort}`), direction: t(`table.directions.${order}`) })}</caption>
      <TableHeader>
        <TableRow>
          <TableHead>{t("table.group")}</TableHead>
          {head("trade_count", t("figures.trades"))}
          {head("total_pnl", t("figures.totalPnl"))}
          {head("avg_pnl", t("figures.avgPnl"))}
          {head("win_rate", t("figures.winRate"))}
          <TableHead>{t("figures.avgDuration")}</TableHead>
          <TableHead>{t("table.mae")}</TableHead>
          <TableHead>{t("table.mfe")}</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {data.groups.map((group, index) => (
          <TableRow key={index}>
            <TableCell className="font-medium">{groupName(group, groupBy, g)}</TableCell>
            <TableCell className="whitespace-nowrap">
              {formatCount(String(group.figures.trade_count), locale)}{" "}
              <Link href={path(seeHref(group, groupBy, tradesQuery))} className="underline underline-offset-4">
                {t("table.see")}
              </Link>
            </TableCell>
            <TableCell>
              <PnlText value={group.figures.total_pnl} />
              <Bar ratio={group.bar_ratio} negative={group.figures.total_pnl.startsWith("-")} />
            </TableCell>
            <TableCell>{group.figures.avg_pnl !== undefined ? <PnlText value={group.figures.avg_pnl} /> : dash}</TableCell>
            <TableCell>{group.figures.win_rate !== undefined ? t("figures.percent", { value: formatDecimal(group.figures.win_rate, locale, 1) }) : dash}</TableCell>
            <TableCell className="whitespace-nowrap">{group.figures.avg_duration_seconds !== undefined ? formatDuration(group.figures.avg_duration_seconds) : dash}</TableCell>
            <TableCell title={group.figures.mae_sample_count < group.figures.trade_count ? t("figures.basis", { k: group.figures.mae_sample_count, n: group.figures.trade_count }) : undefined}>{f(group.figures.avg_mae, (v) => formatMoney(v, locale))}</TableCell>
            <TableCell title={group.figures.mfe_sample_count < group.figures.trade_count ? t("figures.basis", { k: group.figures.mfe_sample_count, n: group.figures.trade_count }) : undefined}>{f(group.figures.avg_mfe, (v) => formatMoney(v, locale))}</TableCell>
          </TableRow>
        ))}
      </TableBody>
      <TableFooter>
        <TableRow>
          <TableHead scope="row">{t("table.total")}</TableHead>
          <TableCell>{formatCount(String(data.totals.trade_count), locale)}</TableCell>
          <TableCell>
            <PnlText value={data.totals.total_pnl} />
          </TableCell>
          <TableCell>{data.totals.avg_pnl !== undefined ? <PnlText value={data.totals.avg_pnl} /> : dash}</TableCell>
          <TableCell>{data.totals.win_rate !== undefined ? t("figures.percent", { value: formatDecimal(data.totals.win_rate, locale, 1) }) : dash}</TableCell>
          <TableCell className="whitespace-nowrap">{data.totals.avg_duration_seconds !== undefined ? formatDuration(data.totals.avg_duration_seconds) : dash}</TableCell>
          <TableCell>{f(data.totals.avg_mae, (v) => formatMoney(v, locale))}</TableCell>
          <TableCell>{f(data.totals.avg_mfe, (v) => formatMoney(v, locale))}</TableCell>
        </TableRow>
      </TableFooter>
    </Table>
  );
}

/** The proportion bar. Its length is the Core's `bar_ratio` (CSS does the multiplication), so the browser computes nothing; it is decorative. */
function Bar({ ratio, negative }: { ratio: string; negative: boolean }) {
  return (
    <div aria-hidden="true" className="mt-1 h-1.5 w-full max-w-40 rounded-full bg-muted">
      <div className={cn("h-full rounded-full", negative ? "bg-status-danger" : "bg-status-success")} style={{ width: `calc(${ratio} * 100%)` }} />
    </div>
  );
}

