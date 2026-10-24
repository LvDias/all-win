"use client";

import { useLocale, useTranslations } from "next-intl";
import { useId, useState, type ReactNode } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { ConflictPicker, otherValue } from "@/shared/components/conflict-picker";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { StepUpCancelled, useStepUp } from "@/shared/session/step-up";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { RadioGroup, RadioGroupItem } from "@/shared/ui/radio-group";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import { Skeleton } from "@/shared/ui/skeleton";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/shared/ui/table";
import { useQueryClient } from "@tanstack/react-query";
import { accountKeys } from "@/shared/api/account-keys";
import { accountPresetKeys, useApplyPresetVersion, usePresetDiff, type ApplyPresetVersionRequest, type PresetDiff, type RuleField } from "./api/account-preset";
import { usePresetVersion } from "./api/use-presets";
import type { SourceRef } from "./api/types";
import { useRuleValueText } from "./lib/rule-value";
import { ruleSetFor } from "./lib/preset-selection";
import { emptyDraft, OtherValueEditor, parseOther, type OtherDraft } from "./other-value";

type Row = PresetDiff["rows"][number];
type Kind = "automation" | "copy";
type OptInDraft = { choice: "accepted" | "declined" | ""; ack: boolean };
type Pick = { selected: string; other: OtherDraft };
type Choice = "keep_mine" | "use_preset";

export type RenderOptIn = (props: {
  kind: Kind;
  /** The firm the preset belongs to, for the opt-in sentences. */
  firm: string;
  ruleVersionId: string;
  value: OptInDraft;
  onChange: (value: OptInDraft) => void;
  onRequiredChange: (required: boolean) => void;
  choiceError?: string;
  ackError?: string;
}) => ReactNode;

type Published = { value: unknown; sources: SourceRef[] }[];
const publishedOf = (rules: unknown, field: RuleField): Published => {
  const rule = (rules as Record<string, { published_values?: Published } | undefined> | undefined)?.[field];
  return rule?.published_values ?? [];
};

/**
 * "Mudanças da versão n" (S16-05): only the fields that differ. Every edited field and every field that is not confirmed
 * in the new version is answered by the user — nothing is preselected — then one apply, with step-up when an opt-in is accepted.
 */
/** `accountPlatform`: the account's firm platform ("" if none). With per-platform rules it decides which rule set the choices are offered from. */
export function VersionDiffDialog({ accountId, firm, targetId, rowVersion, accountPlatform, renderOptIn, onClose }: { accountId: string; firm: string; targetId: string; rowVersion: number; accountPlatform: string; renderOptIn: RenderOptIn; onClose: () => void }) {
  const t = useTranslations("presets.diff");
  const tf = useTranslations("presets.account.fields");
  const tc = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const base = useId();
  const stepUp = useStepUp();
  const valueText = useRuleValueText();
  const diff = usePresetDiff(accountId, targetId);
  const target = usePresetVersion(targetId, { keepPrevious: true });
  const apply = useApplyPresetVersion(accountId);
  const queryClient = useQueryClient();

  const [choices, setChoices] = useState<Partial<Record<RuleField, Choice>>>({});
  const [picks, setPicks] = useState<Partial<Record<RuleField, Pick>>>({});
  const [platform, setPlatform] = useState(accountPlatform);
  const [capInput, setCapInput] = useState("");
  const [optIns, setOptIns] = useState<Record<Kind, OptInDraft>>({ automation: { choice: "", ack: false }, copy: { choice: "", ack: false } });
  const [optInRequired, setOptInRequired] = useState<Record<Kind, boolean>>({ automation: true, copy: true });
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [notice, setNotice] = useState<string | null>(null);
  const [failure, setFailure] = useState<string | null>(null);

  const data = diff.data;
  // Moving to a newer target (after a conflict) shows the old comparison until the new one is in; it cannot be applied meanwhile.
  const settling = diff.isPlaceholderData || target.isPlaceholderData;
  // Never another platform's rules: a known platform picks its own set (or none); without one only a single or platform-less set applies.
  const rules = target.data ? ruleSetFor(target.data, platform)?.rules : undefined;
  // The answers are keyed by position in the platform's published values: another platform means another list.
  const changePlatform = (next: string) => {
    setPlatform(next);
    setPicks({});
  };
  const pick = (field: RuleField): Pick => picks[field] ?? { selected: "", other: emptyDraft };
  const setPick = (field: RuleField, next: Partial<Pick>) => setPicks((all) => ({ ...all, [field]: { ...pick(field), ...next } }));
  const kinds = (["automation", "copy"] as const).filter((kind) => data?.opt_in_changes[kind]);

  function reload(message: string) {
    setNotice(message);
    setChoices({});
    setPicks({});
    setCapInput("");
    setOptIns({ automation: { choice: "", ack: false }, copy: { choice: "", ack: false } });
    // The panel's state holds the newest target and row_version the dialog is opened with: refresh it (and the account), then
    // the dialog re-reads the diff for what the Core now says. Reading the old target again would answer 409 on every retry.
    void queryClient.invalidateQueries({ queryKey: accountPresetKeys.state(accountId) });
    void queryClient.invalidateQueries({ queryKey: accountKeys.detail(accountId) });
    void diff.refetch();
  }

  async function onApply() {
    if (!data || !target.data) return;
    const found: Record<string, string> = {};
    const body: ApplyPresetVersionRequest = { target_preset_version_id: targetId, row_version: rowVersion };
    const bodyChoices: NonNullable<ApplyPresetVersionRequest["choices"]> = {};
    const ruleValues: Record<string, unknown> = {};
    for (const row of data.rows) {
      const field = row.field;
      if (row.cap_conflict) {
        const parsed = parseOther("max_position_qty", { a: capInput, b: "" }, locale);
        const newCap = (row.new_preset_value as { contracts?: number } | undefined)?.contracts;
        if (!parsed.ok) found[field] = t(`errors.${parsed.reason}`);
        else if (newCap !== undefined && Number(parsed.value.contracts) > newCap) found[field] = t("errors.aboveCap", { n: newCap });
        else body.max_position_qty = Number(parsed.value.contracts);
        continue;
      }
      if (!row.requires_choice) continue;
      if (row.edited) {
        const choice = choices[field];
        if (choice === undefined) found[field] = t("errors.choiceRequired");
        else bodyChoices[field] = choice;
        continue;
      }
      const current = pick(field);
      const options = publishedOf(rules, field);
      if (current.selected === "") found[field] = t("errors.unresolved");
      else if (current.selected === otherValue) {
        const parsed = parseOther(field, current.other, locale);
        if (!parsed.ok) found[field] = t(`errors.${parsed.reason}`);
        else if (field === "max_position_qty") body.max_position_qty = Number(parsed.value.contracts);
        else ruleValues[field] = parsed.value;
      } else {
        const chosen = options[Number(current.selected)]?.value;
        if (field === "max_position_qty") body.max_position_qty = Number((chosen as { contracts: number }).contracts);
        else ruleValues[field] = chosen;
      }
    }
    if (data.firm_platform_required) {
      if (platform === "") found.platform = t("errors.platformRequired");
      else body.firm_platform = platform;
    }
    const optInBody: NonNullable<ApplyPresetVersionRequest["opt_ins"]> = {};
    for (const kind of kinds) {
      const ruleVersionId = data.opt_in_changes[kind];
      if (!ruleVersionId || !optInRequired[kind]) continue;
      const draft = optIns[kind];
      if (draft.choice === "") found[`opt_in.${kind}`] = tc(`accounts.optIn.${kind}.required`);
      else if (draft.choice === "accepted" && !draft.ack) found[`ack.${kind}`] = tc("accounts.form.errors.ackRequired");
      else optInBody[kind] = { choice: draft.choice, rule_version_id: ruleVersionId };
    }
    setErrors(found);
    setFailure(null);
    if (Object.keys(found).length > 0) return;
    if (Object.keys(bodyChoices).length > 0) body.choices = bodyChoices;
    if (Object.keys(ruleValues).length > 0) body.rule_values = ruleValues as ApplyPresetVersionRequest["rule_values"];
    if (Object.keys(optInBody).length > 0) body.opt_ins = optInBody;
    const send = (proof?: { challenge_id: string; code: string }) => apply.mutateAsync(proof ? { ...body, step_up: proof } : body);
    try {
      // Every apply goes through the runner (as the S5 edit does): it asks for a code only if the Core says so, never on our guess.
      await stepUp.run({ action: "account_opt_in_accept", targetId: accountId, description: t("stepUp") }, send);
      toast.success(t("applied", { n: target.data.release.version_number }));
      onClose();
    } catch (error) {
      if (error instanceof StepUpCancelled) return;
      if (error instanceof ApiError && error.code === "presets.version_superseded") return reload(t("superseded"));
      if (error instanceof ApiError && error.code === "accounts.rule_version_changed") return reload(t("ruleChanged"));
      if (error instanceof ApiError && error.code === "common.version_conflict") return reload(t("conflict"));
      // A 422 that names a field is shown at that field (the same places the checks above use).
      const pointer = error instanceof ApiError && error.status === 422 ? error.pointer : undefined;
      const field = pointer?.match(/^\/(?:choices|rule_values)\/([a-z_]+)$/)?.[1] ?? (pointer === "/max_position_qty" ? "max_position_qty" : undefined);
      if (field) return setErrors({ [field]: errorMessage(tc, error).message });
      setFailure(errorMessage(tc, error).message);
    }
  }

  const valueCell = (row: Row, key: "account_value" | "current_preset_value" | "new_preset_value") => {
    const value = row[key];
    return value === undefined ? t("none") : (valueText(row.field, value) ?? t("none"));
  };

  return (
    <Dialog open onOpenChange={(open) => { if (!open) onClose(); }}>
      <DialogContent className="max-h-[90vh] max-w-3xl overflow-y-auto">
        <DialogHeader>
          <DialogTitle>{t("title", { n: target.data?.release.version_number ?? "" })}</DialogTitle>
          <DialogDescription>{t("description", { firm })}</DialogDescription>
        </DialogHeader>
        {notice ? <Alert tone="warning" role="status">{notice}</Alert> : null}
        {diff.isPending || target.isPending ? <Skeleton className="h-40 w-full" /> : null}
        {diff.isError ? <Alert tone="danger">{errorMessage(tc, diff.error).message}</Alert> : null}
        {target.isError ? <Alert tone="danger">{errorMessage(tc, target.error).message}</Alert> : null}
        {data && target.data ? (
          <div className={settling ? "space-y-5 opacity-60" : "space-y-5"} aria-busy={settling || undefined}>
            {data.rows.length === 0 ? <p className="text-sm">{t("noRows")}</p> : (
              <div className="overflow-x-auto">
                <Table>
                  <caption className="sr-only">{t("caption")}</caption>
                  <TableHeader>
                    <TableRow>
                      <TableHead scope="col">{t("columns.field")}</TableHead>
                      <TableHead scope="col">{t("columns.account")}</TableHead>
                      <TableHead scope="col">{t("columns.current")}</TableHead>
                      <TableHead scope="col">{t("columns.new")}</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {data.rows.map((row) => (
                      <TableRow key={row.field}>
                        <TableHead scope="row">{tf(row.field)}{row.edited ? <span className="block text-xs font-normal text-muted-foreground">{t("youEdited")}</span> : null}</TableHead>
                        <TableCell>{valueCell(row, "account_value")}</TableCell>
                        <TableCell>{valueCell(row, "current_preset_value")}</TableCell>
                        <TableCell>{row.requires_choice && !row.edited ? t("notConfirmed") : valueCell(row, "new_preset_value")}{row.cap_conflict ? <span className="block text-xs text-status-warning">{t("capConflict", { n: (row.new_preset_value as { contracts?: number } | undefined)?.contracts ?? "" })}</span> : null}</TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </div>
            )}

            {/* TODO(6.2.0): getAccountPresetDiff gets an optional `firm_platform` query. Today an account without a platform (or with one the target does
                not offer) gets rows=[] and firm_platform_required=true, so the rows of the chosen platform are not visible before the apply. Until then the
                flow stays: choose the platform, then apply. With the query, re-read the diff for the chosen platform here and show its rows first. */}
            {data.firm_platform_required ? (
              <div className="grid gap-1.5">
                <Label htmlFor={`${base}-platform`}>{t("platform")}</Label>
                <Select value={platform} onValueChange={changePlatform}>
                  <SelectTrigger id={`${base}-platform`} aria-invalid={errors.platform ? true : undefined}><SelectValue placeholder={t("choose")} /></SelectTrigger>
                  <SelectContent>{(data.firm_platforms ?? []).map((code) => <SelectItem key={code} value={code}>{code}</SelectItem>)}</SelectContent>
                </Select>
                {errors.platform ? <p className="text-sm text-status-danger">{errors.platform}</p> : null}
              </div>
            ) : null}

            {data.rows.map((row) => {
              const field = row.field;
              const label = tf(field);
              if (row.cap_conflict)
                return (
                  <div key={field} className="grid gap-1.5">
                    <Label htmlFor={`${base}-${field}`}>{t("lowerCap", { field: label })}</Label>
                    <Input id={`${base}-${field}`} inputMode="numeric" value={capInput} aria-invalid={errors[field] ? true : undefined} onChange={(event) => setCapInput(event.target.value)} />
                    {errors[field] ? <p className="text-sm text-status-danger">{errors[field]}</p> : null}
                  </div>
                );
              if (!row.requires_choice) return null;
              if (row.edited)
                return (
                  <fieldset key={field} className="space-y-2">
                    <legend className="text-sm font-medium">{t("keepOrUse", { field: label })}</legend>
                    <RadioGroup aria-label={t("keepOrUse", { field: label })} value={choices[field] ?? ""} onValueChange={(value) => setChoices((all) => ({ ...all, [field]: value as Choice }))} aria-invalid={errors[field] ? true : undefined}>
                      {(["keep_mine", "use_preset"] as const).map((option) => (
                        <Label key={option} htmlFor={`${base}-${field}-${option}`} className="flex min-h-11 cursor-pointer items-center gap-3 rounded-lg border p-3">
                          <RadioGroupItem id={`${base}-${field}-${option}`} value={option} />
                          {t(`choice.${option}`)}
                        </Label>
                      ))}
                    </RadioGroup>
                    {errors[field] ? <p role="alert" className="text-sm text-status-danger">{errors[field]}</p> : null}
                  </fieldset>
                );
              const options = publishedOf(rules, field).map((published, index) => ({ id: String(index), label: valueText(field, published.value) ?? t("none"), sources: published.sources }));
              return (
                <ConflictPicker
                  key={field}
                  legend={label}
                  options={options}
                  value={pick(field).selected}
                  onValueChange={(selected) => setPick(field, { selected })}
                  otherInput={<OtherValueEditor field={field} id={`${base}-${field}-other`} draft={pick(field).other} onChange={(other) => setPick(field, { other })} invalid={Boolean(errors[field])} />}
                  error={errors[field]}
                />
              );
            })}

            {kinds.map((kind) => {
              const ruleVersionId = data.opt_in_changes[kind];
              return ruleVersionId ? (
                <div key={kind}>
                  {renderOptIn({
                    kind,
                    firm,
                    ruleVersionId,
                    value: optIns[kind],
                    onChange: (value) => setOptIns((all) => ({ ...all, [kind]: value })),
                    onRequiredChange: (required) => setOptInRequired((all) => (all[kind] === required ? all : { ...all, [kind]: required })),
                    choiceError: errors[`opt_in.${kind}`],
                    ackError: errors[`ack.${kind}`],
                  })}
                </div>
              ) : null;
            })}
            {failure ? <Alert tone="danger" role="alert">{failure}</Alert> : null}
          </div>
        ) : null}
        <DialogFooter>
          <Button type="button" variant="outline" onClick={onClose}>{t("keepCurrent")}</Button>
          <Button type="button" disabled={!data || !target.data || settling || apply.isPending} onClick={() => void onApply()}>{target.data ? t("apply", { n: target.data.release.version_number }) : t("apply", { n: "" })}</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
