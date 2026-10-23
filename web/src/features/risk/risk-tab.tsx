"use client";

import { useQueryClient } from "@tanstack/react-query";
import { ChevronDown, Gauge, Lock } from "lucide-react";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { useState } from "react";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import type { AccountRisk, RiskMeterData, RiskRuleKind, RiskState } from "@/shared/api/types";
import { RiskMeter } from "@/shared/components/risk-meter";
import { PageHeader } from "@/shared/components/page-header";
import { StatusBadge } from "@/shared/components/status-badge";
import { formatDateTime, formatDecimal, formatMoney } from "@/shared/format";
import { useLocalePath } from "@/shared/i18n/navigation";
import { useDisplay } from "@/shared/session/display";
import { useEvents } from "@/shared/session/events";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Card, CardContent } from "@/shared/ui/card";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/shared/ui/collapsible";
import { EmptyState } from "@/shared/ui/empty-state";
import { ErrorState } from "@/shared/ui/error-state";
import { Skeleton } from "@/shared/ui/skeleton";
import { riskQueryKey, useAccountRisk } from "./api";
import { EvaluationCard } from "./evaluation-card";

const knownStates: readonly RiskState[] = ["normal", "warning", "reached", "target_reached"];
const severity: Record<RiskState, number> = { normal: 0, target_reached: 1, warning: 2, reached: 3 };

/** S13-01: live risk state of one account. Every figure and state is the Core's; this component only formats and places them. */
export function RiskTab({ accountId }: { accountId: string }) {
  const t = useTranslations("risk");
  const root = useTranslations();
  const queryClient = useQueryClient();
  const path = useLocalePath();
  const risk = useAccountRisk(accountId);
  const [announcement, setAnnouncement] = useState("");

  // Only a change of state is announced (F§7.3), never a tick; the refetch brings the new figures in place.
  useEvents((event) => {
    if (event.type !== "risk.state_changed" || event.account_id !== accountId) return;
    void queryClient.invalidateQueries({ queryKey: riskQueryKey });
    // The state comes from the stream: an unknown value is ignored rather than shown as a raw key.
    if ((event.reason === "state" || event.reason === undefined) && knownStates.includes(event.state)) setAnnouncement(t(`announce.${event.state}`));
  });

  const editHref = path(`/accounts/${accountId}/risk/edit`);
  const header = (
    <PageHeader
      title={t("title")}
      actions={
        <Button asChild variant="outline">
          <Link href={editHref}>{t("editAction")}</Link>
        </Button>
      }
    />
  );

  if (risk.isPending)
    return (
      <div className="grid gap-4" aria-busy="true">
        {header}
        <Skeleton className="h-24 w-full" />
        <Skeleton className="h-16 w-full" />
        <Skeleton className="h-16 w-full" />
        <Skeleton className="h-16 w-full" />
      </div>
    );
  if (risk.isError) {
    const failure = errorMessage(root, risk.error);
    const notFound = risk.error instanceof ApiError && risk.error.status === 404;
    return (
      <div className="grid gap-4">
        {header}
        <ErrorState title={notFound ? root("errors.common.not_found") : t("loadError")} code={failure.code} onRetry={notFound ? undefined : () => void risk.refetch()} />
      </div>
    );
  }
  return (
    <div className="grid gap-4">
      {header}
      <p role="status" className="sr-only">
        {announcement}
      </p>
      <RiskBody accountId={accountId} risk={risk.data} editHref={editHref} />
    </div>
  );
}

function RiskBody({ accountId, risk, editHref }: { accountId: string; risk: AccountRisk; editHref: string }) {
  const t = useTranslations("risk");
  const { locale, timeZone } = useDisplay();
  const at = (value: string, seconds = false) => formatDateTime(value, locale, timeZone, { zone: true, seconds });
  const empty = risk.meters.length === 0 && risk.untracked_rules.length === 0;
  const dimmed = risk.stale;

  const banners = risk.meters.filter((meter) => meter.state === "warning" || meter.state === "reached").sort((a, b) => severity[b.state] - severity[a.state]);

  return (
    <>
      <Card>
        <CardContent className="grid gap-2 pt-4 text-sm">
          <p>{t("strip.day", { start: at(risk.trading_day.start), end: at(risk.trading_day.end) })}</p>
          <p>{t("strip.nextReset", { time: at(risk.trading_day.next_reset) })}</p>
          <p className="text-muted-foreground">
            {risk.trading_day.source === "cme_session_close"
              ? t("strip.sourceCme")
              : t("strip.sourceFirm", { time: risk.trading_day.firm_reset?.local_time ?? "", zone: risk.trading_day.firm_reset?.time_zone ?? "" })}
          </p>
          {risk.trading_day.playback_clock ? <p className="text-muted-foreground">{t("strip.playback", { time: at(risk.trading_day.playback_clock, true) })}</p> : null}
          <p className="flex flex-wrap items-center gap-2 text-muted-foreground">
            <span>
              {t("strip.asOf")} <time dateTime={risk.as_of}>{at(risk.as_of, true)}</time>
            </span>
            {risk.stale ? <StatusBadge kind="stale" /> : null}
          </p>
          {risk.stale && risk.stale_reason === "connection_down" ? <p className="text-status-warning">{t("strip.connectionDown")}</p> : null}
        </CardContent>
      </Card>

      {risk.blocked ? (
        <Alert tone={risk.blocked.cause === "daily_profit_target" ? "success" : "danger"} role="alert">
          <Lock aria-hidden="true" />
          <div className="grid gap-1">
            <p className="font-medium">{t("blocked.title", { time: at(risk.blocked.until), reason: t(`blocked.reason.${risk.blocked.cause}`) })}</p>
            <p>{t("blocked.noUnlock")}</p>
          </div>
        </Alert>
      ) : null}

      {banners.map((meter) => (
        <Alert key={meter.kind} tone={meter.state === "reached" ? "danger" : "warning"}>
          <div className="grid gap-1">
            <p className="flex flex-wrap items-center gap-2 font-medium">
              <StatusBadge kind={meter.state === "reached" ? "riskReached" : "riskWarning"} />
              <span>{label(t, meter)}</span>
            </p>
            <p>{valueText(t, meter, locale)}</p>
          </div>
        </Alert>
      ))}

      <div className="grid gap-1 text-sm text-muted-foreground">
        <p>{risk.pnl_basis === "net" ? t("basis.net") : t("basis.gross")}</p>
        <p>{t("basis.manual")}</p>
      </div>
      {risk.commission_source === "none" ? (
        <Alert tone="info">
          <div className="flex flex-wrap items-center gap-x-3 gap-y-1">
            <span>{t("commission.none")}</span>
            <Link href={`${editHref}#commission`} className="font-medium underline underline-offset-4">
              {t("commission.inform")}
            </Link>
          </div>
        </Alert>
      ) : null}
      {risk.pending_commission_change ? (
        <p className="text-sm text-muted-foreground">
          {risk.pending_commission_change.change === "commission_floor_removed" || !risk.pending_commission_change.new_commission_per_contract_side
            ? t("pending.commissionRemoved", { time: at(risk.pending_commission_change.effective_from) })
            : t("pending.commissionLowered", { value: formatMoney(risk.pending_commission_change.new_commission_per_contract_side, locale), time: at(risk.pending_commission_change.effective_from) })}
        </p>
      ) : null}
      {risk.limits_pending ? <Alert tone="info">{t("limitsPending")}</Alert> : null}

      <EvaluationCard accountId={accountId} risk={risk} />

      {empty ? (
        <EmptyState icon={Gauge} title={t("empty.title")} description={t("empty.body")}>
          <Button asChild>
            <Link href={editHref}>{t("empty.action")}</Link>
          </Button>
        </EmptyState>
      ) : (
        <section aria-labelledby="risk-limits" className="grid gap-4">
          <h2 id="risk-limits" className="text-base font-semibold">
            {t("limits")}
          </h2>
          {risk.meters.map((meter) => (
            <RiskMeter key={meter.kind} label={label(t, meter)} value={valueText(t, meter, locale)} percent={meter.percent} state={meter.state} detail={detail(t, meter, locale)} tone={meter.kind === "daily_profit_target" ? "progress" : "limit"} dimmed={dimmed} />
          ))}
          {risk.untracked_rules.map((kind) => (
            <p key={kind} className="text-sm text-muted-foreground">
              {t("untracked", { rule: t(`kind.${kind}`) })}
            </p>
          ))}
          {risk.unset_rules.map((kind) => (
            <div key={kind} className="flex flex-wrap items-center gap-x-3 gap-y-1 text-sm">
              <span className="text-muted-foreground">{t("unset", { rule: t(`kind.${kind}`) })}</span>
              <Link href={editHref} className="font-medium underline underline-offset-4">
                {t("define")}
              </Link>
            </div>
          ))}
          {risk.pending_changes.map((change) => (
            <p key={`${change.kind}-${change.effective_from}`} className="text-sm text-muted-foreground">
              {pendingText(t, change, locale, at)}
            </p>
          ))}
          {risk.pending_reset_change ? <p className="text-sm text-muted-foreground">{t("pending.reset", { value: risk.pending_reset_change.new_reset.local_time, zone: risk.pending_reset_change.new_reset.time_zone, time: at(risk.pending_reset_change.effective_from) })}</p> : null}
        </section>
      )}

      <Collapsible className="rounded-lg border">
        <CollapsibleTrigger className="group flex min-h-11 w-full items-center justify-between gap-2 px-3 text-left text-sm font-medium">
          {t("whatHappens.title")}
          <ChevronDown aria-hidden="true" className="size-4 transition-transform group-data-[state=open]:rotate-180" />
        </CollapsibleTrigger>
        <CollapsibleContent forceMount className="grid gap-2 px-3 pb-3 text-sm text-muted-foreground data-[state=closed]:hidden">
          <p>{t("whatHappens.refuse")}</p>
          <p>{t("whatHappens.enforcement")}</p>
          <p>{t("whatHappens.target")}</p>
          <p>{t("whatHappens.consistency")}</p>
        </CollapsibleContent>
      </Collapsible>
    </>
  );
}

type Translate = ReturnType<typeof useTranslations<"risk">>;

function label(t: Translate, meter: RiskMeterData): string {
  return meter.kind === "drawdown" && meter.drawdown_type ? t("meter.drawdown", { type: t(`drawdownType.${meter.drawdown_type}`) }) : t(`kind.${meter.kind}`);
}

function valueText(t: Translate, meter: RiskMeterData, locale: Parameters<typeof formatMoney>[1]): string {
  if (meter.kind === "consistency") return t("meter.consistency", { used: formatDecimal(meter.used, locale, 0), limit: formatDecimal(meter.limit, locale, 0) });
  if (meter.kind === "daily_profit_target") return t("meter.progress", { used: formatMoney(meter.used, locale), limit: formatMoney(meter.limit, locale) });
  return t("meter.usedOf", { used: formatMoney(meter.used, locale), limit: formatMoney(meter.limit, locale) });
}

function detail(t: Translate, meter: RiskMeterData, locale: Parameters<typeof formatMoney>[1]): string | undefined {
  return meter.floor ? t("meter.floor", { value: formatMoney(meter.floor, locale) }) : undefined;
}

function pendingText(t: Translate, change: AccountRisk["pending_changes"][number], locale: Parameters<typeof formatMoney>[1], at: (value: string) => string): string {
  const time = at(change.effective_from);
  const kind: RiskRuleKind = change.kind;
  if (change.change === "rule_removed") return t("pending.removed", { rule: t(`kind.${kind}`), time });
  if (change.change === "profit_target_to_notify") return t("pending.toNotify", { time });
  if (change.change === "drawdown_type_changed" && change.new_drawdown) return t("pending.drawdownType", { type: t(`drawdownType.${change.new_drawdown.type}`), time });
  if (change.new_limit !== undefined) return t("pending.limit", { rule: t(`kind.${kind}`), value: kind === "consistency" ? formatDecimal(change.new_limit, locale, 0) : formatMoney(change.new_limit, locale), time });
  return t("pending.generic", { rule: t(`kind.${kind}`), time });
}
