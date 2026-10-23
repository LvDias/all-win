"use client";

import { Check, Target } from "lucide-react";
import { useTranslations } from "next-intl";
import { useState } from "react";
import type { AccountRisk } from "@/shared/api/types";
import { formatDate, formatDateTime, formatMoney } from "@/shared/format";
import { useDisplay } from "@/shared/session/display";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/shared/ui/card";
import { StatusBadge } from "@/shared/components/status-badge";
import { isZeroAmount } from "./amounts";
import { RestartEvaluationDialog } from "./restart-evaluation-dialog";

/** S13b-01: profit left to pass the evaluation. Success tone only: this is a target, never a limit, and nothing is closed or blocked. */
export function EvaluationCard({ accountId, risk }: { accountId: string; risk: AccountRisk }) {
  const t = useTranslations("risk.evaluation");
  const { locale, timeZone } = useDisplay();
  const [restarting, setRestarting] = useState(false);
  const evaluation = risk.evaluation;
  if (!evaluation) return null;
  const reached = evaluation.state === "target_reached";
  const percent = Math.min(100, evaluation.percent);
  const dayLabel = formatDate(`${evaluation.start}T00:00:00Z`, locale, "UTC");
  const commissionUnknown = risk.commission_source === "none" || risk.commission_source === "configured" || risk.commission_source === "mixed";

  return (
    <Card aria-labelledby="evaluation-title">
      <CardHeader className="flex-row flex-wrap items-center justify-between gap-2">
        <CardTitle id="evaluation-title" className="flex items-center gap-2 text-base">
          {reached ? <Check aria-hidden="true" className="size-4 text-status-success" /> : <Target aria-hidden="true" className="size-4" />}
          {t("title")}
        </CardTitle>
        {risk.stale ? <StatusBadge kind="stale" /> : null}
      </CardHeader>
      <CardContent className="grid gap-3">
        <div role="progressbar" aria-label={t("title")} aria-valuemin={0} aria-valuemax={100} aria-valuenow={percent} aria-valuetext={t("progress", { percent: evaluation.percent })} className="h-2.5 overflow-hidden rounded-full bg-muted">
          <div className="h-full rounded-full bg-status-success" style={{ width: `${percent}%` }} />
        </div>
        <p className="text-sm text-muted-foreground">{t("progress", { percent: evaluation.percent })}</p>
        <dl className="grid gap-2 text-sm sm:grid-cols-3">
          <div>
            <dt className="text-muted-foreground">{t("target")}</dt>
            <dd className="font-medium tabular-nums">{formatMoney(evaluation.profit_target, locale)}</dd>
          </div>
          <div>
            <dt className="text-muted-foreground">{t("achieved")}</dt>
            <dd className="font-medium tabular-nums">{formatMoney(evaluation.achieved, locale)}</dd>
          </div>
          <div>
            {reached && evaluation.reached_at ? (
              <>
                <dt className="text-muted-foreground">{t("reachedLabel")}</dt>
                <dd className="font-medium">{formatDateTime(evaluation.reached_at, locale, timeZone, { zone: true })}</dd>
              </>
            ) : (
              <>
                <dt className="text-muted-foreground">{t("remaining")}</dt>
                <dd className="font-medium tabular-nums">{formatMoney(evaluation.remaining, locale)}</dd>
              </>
            )}
          </div>
        </dl>
        <p className="text-xs text-muted-foreground">
          {!isZeroAmount(evaluation.earned_before)
            ? t("countSinceWithBefore", { start: dayLabel, earned: formatMoney(evaluation.earned_before, locale) })
            : t("countSince", { start: dayLabel })}
        </p>
        <p className="text-xs text-muted-foreground">
          {t("asOf")} <time dateTime={evaluation.as_of}>{formatDateTime(evaluation.as_of, locale, timeZone, { seconds: true, zone: true })}</time>
        </p>
        {commissionUnknown ? <Alert tone="info">{t("commissionUnknown")}</Alert> : null}
        {reached ? (
          <Alert tone="success" role="status">
            <Check aria-hidden="true" />
            <span>{t("reachedNotice")}</span>
          </Alert>
        ) : null}
        <div>
          <Button variant="outline" size="sm" onClick={() => setRestarting(true)}>
            {t("adjust")}
          </Button>
        </div>
      </CardContent>
      {restarting ? <RestartEvaluationDialog accountId={accountId} currentStart={evaluation.start} currentEarnedBefore={evaluation.earned_before} onClose={() => setRestarting(false)} /> : null}
    </Card>
  );
}
