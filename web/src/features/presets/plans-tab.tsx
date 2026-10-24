"use client";

import Link from "next/link";
import { useLocale, useTranslations } from "next-intl";
import { useState } from "react";
import { errorMessage } from "@/shared/api/error-message";
import type { components } from "@/shared/api/schema.gen";
import { formatMoney } from "@/shared/format";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { withLocale } from "@/shared/i18n/paths";
import { Label } from "@/shared/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import { Skeleton } from "@/shared/ui/skeleton";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/shared/ui/table";
import { usePresetVersions } from "./api/use-presets";
import type { FirmDetail, PresetRules, PresetVersion } from "./api/types";
import { ResetTime } from "./reset-time";
import { RuleCell } from "./rule-cell";

type Schemas = components["schemas"];
type Phase = "evaluation" | "funded" | "live";

function Filter({ id, label, value, onChange, options }: { id: string; label: string; value: string; onChange: (value: string) => void; options: { value: string; label: string }[] }) {
  return (
    <div className="grid gap-1.5">
      <Label htmlFor={id}>{label}</Label>
      <Select value={value} onValueChange={onChange}>
        <SelectTrigger id={id} className="min-w-44"><SelectValue /></SelectTrigger>
        <SelectContent>{options.map((o) => <SelectItem key={o.value} value={o.value}>{o.label}</SelectItem>)}</SelectContent>
      </Select>
    </div>
  );
}

/** The rules of the platform filter, or the single platform-less set. */
const rulesFor = (version: PresetVersion, platform: string): PresetRules | undefined =>
  (version.rule_sets.find((set) => (set.firm_platform ?? "") === platform) ?? version.rule_sets[0])?.rules;

/** S16-02 tab "Plans and rules": one table of sizes for the chosen plan, phase and platform. Money is shown exactly as published. */
export function PlansTab({ firm }: { firm: FirmDetail }) {
  const t = useTranslations("presets.plans");
  const tc = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const [planId, setPlanId] = useState(firm.plans[0]?.plan_id ?? "");
  const plan = firm.plans.find((p) => p.plan_id === planId) ?? firm.plans[0];
  const [phaseKey, setPhaseKey] = useState<string>("");
  const phase = plan?.phases.find((p) => p.phase === phaseKey) ?? plan?.phases[0];
  const [platformKey, setPlatformKey] = useState("");
  const sizes = phase?.sizes ?? [];
  const versions = usePresetVersions(sizes.map((s) => s.preset_version_id));
  const platformSpecific = sizes.some((s) => s.platform_specific);
  const platform = platformSpecific ? (firm.platforms.find((p) => p.code === platformKey)?.code ?? firm.platforms[0]?.code ?? "") : "";
  const evaluation = phase?.phase === "evaluation";

  const money = (amount: string) => formatMoney(amount, locale);
  const drawdown = (v: Schemas["DrawdownValue"]) => `${money(v.amount)} · ${t(`drawdown.${v.type}`)}`;
  const dailyLoss = (v: Schemas["DailyLossValue"]) => (v.kind === "none" || v.amount === undefined ? t("dailyLoss.none") : `${money(v.amount)} · ${t(`dailyLoss.${v.kind}`)}`);
  const reset = (v: Schemas["TradingDayResetValue"]) => <ResetTime localTime={v.local_time} timeZone={v.time_zone} publishedText={v.published_text} />;

  if (!plan) return <p className="text-sm">{t("none")}</p>;
  return (
    <div className="space-y-4">
      <div className="flex flex-wrap gap-3">
        <Filter id="plan" label={t("filters.plan")} value={plan.plan_id} onChange={(v) => { setPlanId(v); setPhaseKey(""); }} options={firm.plans.map((p) => ({ value: p.plan_id, label: p.name }))} />
        <Filter id="phase" label={t("filters.phase")} value={phase?.phase ?? ""} onChange={setPhaseKey} options={plan.phases.map((p) => ({ value: p.phase, label: t(`phase.${p.phase as Phase}`) }))} />
        {platformSpecific ? <Filter id="platform" label={t("filters.platform")} value={platform} onChange={setPlatformKey} options={firm.platforms.map((p) => ({ value: p.code, label: p.label }))} /> : null}
      </div>
      <div className="overflow-x-auto">
        <Table>
          <caption className="sr-only">{t("caption", { plan: plan.name })}</caption>
          <TableHeader>
            <TableRow>
              <TableHead scope="col" className="sticky left-0 bg-background">{t("columns.size")}</TableHead>
              <TableHead scope="col">{t("columns.drawdown")}</TableHead>
              <TableHead scope="col">{t("columns.dailyLoss")}</TableHead>
              <TableHead scope="col">{t("columns.dailyTarget")}</TableHead>
              <TableHead scope="col">{t("columns.maxContracts")}</TableHead>
              <TableHead scope="col">{t("columns.reset")}</TableHead>
              <TableHead scope="col">{t("columns.consistency")}</TableHead>
              <TableHead scope="col">{t("columns.commission")}</TableHead>
              {evaluation ? <TableHead scope="col">{t("columns.evaluationTarget")}</TableHead> : null}
            </TableRow>
          </TableHeader>
          <TableBody>
            {sizes.map((size, index) => {
              const query = versions[index];
              const label = size.size_label ?? money(size.size_amount);
              if (!query || query.isPending)
                return <TableRow key={size.preset_version_id}><TableHead scope="row" className="sticky left-0 bg-background">{label}</TableHead><TableCell colSpan={evaluation ? 8 : 7}><Skeleton className="h-5 w-full" /></TableCell></TableRow>;
              if (query.isError)
                return <TableRow key={size.preset_version_id}><TableHead scope="row" className="sticky left-0 bg-background">{label}</TableHead><TableCell colSpan={evaluation ? 8 : 7} className="text-status-danger">{errorMessage(tc, query.error).message}</TableCell></TableRow>;
              const rules = rulesFor(query.data, platform);
              return (
                <TableRow key={size.preset_version_id}>
                  <TableHead scope="row" className="sticky left-0 bg-background">{label}</TableHead>
                  <TableCell><RuleCell rule={rules?.drawdown} format={drawdown} label={t("columns.drawdown")} /></TableCell>
                  <TableCell><RuleCell rule={rules?.daily_loss_limit} format={dailyLoss} label={t("columns.dailyLoss")} /></TableCell>
                  <TableCell><RuleCell rule={rules?.daily_profit_target} format={(v) => money(v.amount)} label={t("columns.dailyTarget")} /></TableCell>
                  <TableCell><RuleCell rule={rules?.max_position_qty} format={(v) => v.published_text ?? String(v.contracts)} label={t("columns.maxContracts")} /></TableCell>
                  <TableCell><RuleCell rule={rules?.trading_day_reset} format={reset} label={t("columns.reset")} /></TableCell>
                  <TableCell><RuleCell rule={rules?.consistency_rule} format={(v) => v.text} label={t("columns.consistency")} /></TableCell>
                  <TableCell>{rules?.commission_per_contract_side ? <RuleCell rule={rules.commission_per_contract_side} format={(v) => money(v.amount)} label={t("columns.commission")} /> : <span>{"—"}</span>}</TableCell>
                  {evaluation ? <TableCell><RuleCell rule={rules?.evaluation_profit_target} format={(v) => money(v.amount)} label={t("columns.evaluationTarget")} /></TableCell> : null}
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
      </div>
      {firm.excluded_plans.length > 0 ? (
        <p className="text-sm">
          {t("excluded", { list: firm.excluded_plans.join(", ") })}{" "}
          <Link className="underline underline-offset-4" href={withLocale(locale, "/accounts/new")}>{t("registerManually")}</Link>
        </p>
      ) : null}
      {/* Said only when it is true: every size loaded and none publishes a commission. */}
      {versions.length > 0 && versions.every((query) => query.data && !query.data.rule_sets.some((set) => set.rules.commission_per_contract_side)) ? <p className="text-xs text-muted-foreground">{t("commissionMissing")}</p> : null}
    </div>
  );
}
