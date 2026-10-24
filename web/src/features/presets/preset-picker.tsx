"use client";

import { useLocale, useTranslations } from "next-intl";
import { useId, useState } from "react";
import { errorMessage } from "@/shared/api/error-message";
import { ConflictPicker, otherValue } from "@/shared/components/conflict-picker";
import { StanceBadge } from "@/shared/components/stance-badge";
import { formatMoney } from "@/shared/format";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Badge } from "@/shared/ui/badge";
import { Button } from "@/shared/ui/button";
import { Combobox } from "@/shared/ui/combobox";
import { ErrorState } from "@/shared/ui/error-state";
import { Label } from "@/shared/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import { Skeleton } from "@/shared/ui/skeleton";
import type { RuleField } from "./api/account-preset";
import { useFirm, useFirms } from "./api/use-presets";
import type { FirmDetail } from "./api/types";
import { emptyPresetDraft, requiredFields, type PresetDraft } from "./lib/preset-selection";
import { useRuleValueText } from "./lib/rule-value";
import { emptyDraft, OtherValueEditor } from "./other-value";
import { RuleCell, type Provenance } from "./rule-cell";
import { StancesTab } from "./stances-tab";
import type { PresetSelection } from "./use-preset-selection";

type Level = "firm" | "plan" | "phase" | "size" | "platform";
const fieldsShown: RuleField[] = ["max_position_qty", "drawdown", "daily_loss_limit", "daily_profit_target", "trading_day_reset", "consistency_rule", "commission_per_contract_side", "evaluation_profit_target"];

function SelectField({ id, label, value, onChange, options, disabled, hint }: { id: string; label: string; value: string; onChange: (v: string) => void; options: { value: string; label: string }[]; disabled?: boolean; hint?: string }) {
  return (
    <div className="grid gap-1.5">
      <Label htmlFor={id}>{label}</Label>
      <Select value={value} onValueChange={onChange} disabled={disabled}>
        <SelectTrigger id={id} aria-disabled={disabled} aria-describedby={disabled && hint ? `${id}-hint` : undefined}><SelectValue /></SelectTrigger>
        <SelectContent>{options.map((o) => <SelectItem key={o.value} value={o.value}>{o.label}</SelectItem>)}</SelectContent>
      </Select>
      {disabled && hint ? <p id={`${id}-hint`} className="text-xs text-muted-foreground">{hint}</p> : null}
    </div>
  );
}

/** S16-03: Firma → Plano → Fase → Tamanho → Plataforma, then the rules this preset will fill into the account. */
export function PresetPicker({ selection, onManual, error }: { selection: PresetSelection; onManual: () => void; error?: string }) {
  const t = useTranslations("presets");
  const tp = useTranslations("presets.picker");
  const tc = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const base = useId();
  const valueText = useRuleValueText();
  const { draft, setDraft, version, outcome } = selection;
  const firms = useFirms();
  const firm = useFirm(draft.firmId);
  const [pending, setPending] = useState<{ level: Level; value: string } | null>(null);

  const edited = Object.values(draft.resolutions).filter((r) => r && r.selected !== "").length;
  const detail: FirmDetail | undefined = draft.firmId === "" ? undefined : firm.data;
  const plan = detail?.plans.find((p) => p.plan_id === draft.planId);
  const phase = plan?.phases.find((p) => p.phase === draft.phase);
  const size = phase?.sizes.find((s) => s.preset_version_id === draft.presetVersionId);
  const platformSpecific = size?.platform_specific ?? false;
  const sizeLabel = (s: { size_label?: string; size_amount: string }) => s.size_label ?? formatMoney(s.size_amount, locale);

  function apply(level: Level, value: string) {
    // Each level clears the ones after it; the preset version and every answer about its rules go with them.
    const next: PresetDraft = { ...emptyPresetDraft, firmId: draft.firmId, planId: draft.planId, phase: draft.phase, presetVersionId: draft.presetVersionId, firmPlatform: draft.firmPlatform };
    if (level === "firm") Object.assign(next, { firmId: value, planId: "", phase: "", presetVersionId: "", firmPlatform: "" });
    if (level === "plan") Object.assign(next, { planId: value, phase: "", presetVersionId: "", firmPlatform: "" });
    if (level === "phase") Object.assign(next, { phase: value, presetVersionId: "", firmPlatform: "" });
    if (level === "size") Object.assign(next, { presetVersionId: value, firmPlatform: "" });
    // The answers belong to one rule set: a new platform can bring another one, so they start over.
    if (level === "platform") Object.assign(next, { firmPlatform: value });
    setDraft(next);
  }
  function change(level: Level, value: string) {
    if (edited > 0) setPending({ level, value });
    else apply(level, value);
  }

  const setResolution = (field: RuleField, patch: Partial<NonNullable<PresetDraft["resolutions"][RuleField]>>) =>
    setDraft({ ...draft, resolutions: { ...draft.resolutions, [field]: { selected: "", other: emptyDraft, ...draft.resolutions[field], ...patch } } });

  if (firms.isPending) return <Skeleton className="h-40 w-full" />;
  if (firms.isError) return <ErrorState title={errorMessage(tc, firms.error).message} code={errorMessage(tc, firms.error).code} onRetry={() => void firms.refetch()} />;

  const rules = version.data?.rule_sets.find((set) => (set.firm_platform ?? "") === draft.firmPlatform)?.rules ?? version.data?.rule_sets[0]?.rules;
  const anyUnconfirmed = rules ? fieldsShown.some((f) => (rules[f as keyof typeof rules] as Provenance<unknown> | undefined)?.status === "not_confirmed") : false;
  const manualOnly = detail?.summary.manual_only;

  return (
    <div className="space-y-4">
      <div className="grid gap-1.5">
        <Label htmlFor={`${base}-firm`}>{tp("firm")}</Label>
        <Combobox
          id={`${base}-firm`}
          label={tp("firm")}
          value={draft.firmId === "" ? null : draft.firmId}
          onChange={(v) => change("firm", v)}
          options={firms.data.firms.map((f) => ({ value: f.id, label: `${f.name}${f.manual_only ? ` · ${t("manualOnly")}` : ""}${f.not_confirmed_count > 0 ? ` · ${t("catalog.notConfirmed", { count: f.not_confirmed_count })}` : ""}`, keywords: f.plan_names.join(" ") }))}
        />
      </div>
      <div className="rounded-lg border p-3 text-sm">
        <p>{tp("notFound")}</p>
        <Button type="button" variant="link" className="h-auto px-0" onClick={onManual}>{tp("manual")}</Button>
      </div>

      {draft.firmId !== "" && firm.isPending ? <Skeleton className="h-24 w-full" /> : null}
      {draft.firmId !== "" && firm.isError ? <ErrorState title={errorMessage(tc, firm.error).message} code={errorMessage(tc, firm.error).code} onRetry={() => void firm.refetch()} /> : null}
      {manualOnly ? <Alert tone="info"><p className="font-medium">{t("manualOnly")}</p><p>{manualOnly.reason.text}</p></Alert> : null}

      {detail ? (
        <div className="grid gap-3 sm:grid-cols-2">
          <SelectField id={`${base}-plan`} label={tp("plan")} value={draft.planId} onChange={(v) => change("plan", v)} options={detail.plans.map((p) => ({ value: p.plan_id, label: p.name }))} />
          <SelectField id={`${base}-phase`} label={tp("phase")} value={draft.phase} onChange={(v) => change("phase", v)} disabled={!plan} hint={tp("afterPlan")} options={(plan?.phases ?? []).map((p) => ({ value: p.phase, label: t(`plans.phase.${p.phase}`) }))} />
          <SelectField id={`${base}-size`} label={tp("size")} value={size?.preset_version_id ?? ""} onChange={(v) => change("size", v)} disabled={!phase} hint={tp("afterPhase")} options={(phase?.sizes ?? []).map((s) => ({ value: s.preset_version_id, label: sizeLabel(s) }))} />
          {platformSpecific ? <SelectField id={`${base}-platform`} label={tp("platform")} value={draft.firmPlatform} onChange={(v) => change("platform", v)} options={detail.platforms.map((p) => ({ value: p.code, label: p.label }))} /> : null}
        </div>
      ) : null}

      {error ? <p className="text-sm text-status-danger">{error}</p> : null}

      {version.isPending && draft.presetVersionId !== "" ? <Skeleton className="h-40 w-full" /> : null}
      {version.isError ? <ErrorState title={errorMessage(tc, version.error).message} code={errorMessage(tc, version.error).code} onRetry={() => void version.refetch()} /> : null}

      {version.data && rules && outcome && (!platformSpecific || draft.firmPlatform !== "") ? (
        <section className="space-y-4 rounded-lg border p-4" aria-labelledby={`${base}-preview`}>
          <h3 id={`${base}-preview`} className="text-base font-semibold">{tp("previewTitle")}</h3>
          <p className="text-sm text-muted-foreground">{tp("previewHeader", { label: outcome.label, n: version.data.release.version_number, date: version.data.release.verified_on })}</p>
          {detail ? (
            <div className="space-y-2">
              <div className="flex flex-wrap gap-2 text-sm">
                <span className="flex items-center gap-1">{t("catalog.columns.automation")} <StanceBadge subject="automation" value={detail.summary.stances.automation} /></span>
                <span className="flex items-center gap-1">{t("catalog.columns.copy")} <StanceBadge subject="copy" value={detail.summary.stances.copy} /></span>
              </div>
              <details className="text-sm">
                <summary className="min-h-11 cursor-pointer">{tp("officialRules")}</summary>
                <div className="pt-2"><StancesTab firm={detail} /></div>
              </details>
            </div>
          ) : null}
          {anyUnconfirmed ? <Alert tone="info">{tp("unconfirmedNotice")}</Alert> : null}
          <div className="space-y-4">
            {fieldsShown.map((field) => {
              const rule = rules[field as keyof typeof rules] as Provenance<unknown> | undefined;
              if (!rule) return null;
              const label = t(`account.fields.${field}`);
              const required = requiredFields.includes(field);
              const resolution = draft.resolutions[field];
              const problem = outcome.problems[field];
              const problemText = problem === undefined ? undefined : problem === "unresolved" ? tp("resolveToSave") : t(`diff.errors.${problem}`);
              if (rule.status === "not_confirmed") {
                const options = (rule.published_values ?? []).map((p, index) => ({ id: String(index), label: valueText(field, p.value) ?? t("diff.none"), sources: p.sources }));
                return (
                  <div key={field} className="space-y-1">
                    <Badge tone="warning">{t("rule.notConfirmed")}</Badge>
                    <ConflictPicker
                      legend={required ? `${label} *` : label}
                      options={options}
                      value={resolution?.selected ?? ""}
                      onValueChange={(selected) => setResolution(field, { selected })}
                      otherInput={<OtherValueEditor field={field} id={`${base}-${field}`} draft={resolution?.other ?? emptyDraft} onChange={(other) => setResolution(field, { other })} invalid={problem !== undefined} />}
                      error={problemText}
                    />
                  </div>
                );
              }
              const editing = resolution?.selected === otherValue;
              return (
                <div key={field} className="space-y-2">
                  <div className="flex flex-wrap items-center gap-2 text-sm">
                    <span className="min-w-44 text-muted-foreground">{label}</span>
                    <RuleCell rule={rule} format={(v) => valueText(field, v) ?? t("diff.none")} label={label} />
                    {editing ? <Badge tone="info">{t("account.edited")}</Badge> : null}
                    {field !== "max_position_qty" ? (
                      <Button type="button" variant="ghost" size="sm" onClick={() => setResolution(field, editing ? { selected: "", other: emptyDraft } : { selected: otherValue })}>
                        {editing ? tp("usePresetValue") : tp("edit")}
                      </Button>
                    ) : null}
                  </div>
                  {editing ? (
                    <div>
                      <OtherValueEditor field={field} id={`${base}-${field}`} draft={resolution?.other ?? emptyDraft} onChange={(other) => setResolution(field, { other })} invalid={problem !== undefined} />
                      {problemText ? <p className="text-sm text-status-danger">{problemText}</p> : null}
                    </div>
                  ) : null}
                </div>
              );
            })}
          </div>
        </section>
      ) : null}

      <AlertDialog open={pending !== null} onOpenChange={(open) => { if (!open) setPending(null); }}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{tp("replaceTitle")}</AlertDialogTitle>
            <AlertDialogDescription>{tp("replaceText", { count: edited })}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel autoFocus>{tc("common.actions.cancel")}</AlertDialogCancel>
            <AlertDialogAction onClick={() => { if (pending) apply(pending.level, pending.value); setPending(null); }}>{tp("continue")}</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
