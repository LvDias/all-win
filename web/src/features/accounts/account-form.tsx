"use client";

import { useLocale, useTranslations } from "next-intl";
import { useRouter } from "next/navigation";
import { useEffect, useId, useMemo, useState } from "react";
import { useForm, useWatch } from "react-hook-form";
import { toast } from "sonner";
import { formatMoney } from "@/shared/format";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { withLocale } from "@/shared/i18n/paths";
import { StepUpCancelled } from "@/shared/session/step-up";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Button } from "@/shared/ui/button";
import { Card } from "@/shared/ui/card";
import { ErrorState } from "@/shared/ui/error-state";
import { Skeleton } from "@/shared/ui/skeleton";
import { BindingField, DataFields, FirmFields } from "./account-fields";
import { useAccount, useOptInRules, useUpdateAccount } from "./api/use-accounts";
import type { Account } from "./api/types";
import type { PresetSlot } from "./api/preset-slot";
import { OptInFormSection } from "./opt-in-form-section";
import { accountFormSchema, bindingToValue, emptyAccountForm, fieldFromPointer, toUpdateBody, toWriteBody, type AccountFormValues } from "./lib/account-form";
import { needsStepUp } from "./lib/opt-in";
import { useStepUpRunner, type StepUpRunner } from "./lib/use-step-up";
import { PageHeader } from "@/shared/components/page-header";

/** Edit page: the same fields as "Nova conta", as sections on one page (F§10.1: edit is not a stepper). */
/** `preset`: the S16 picker, composed by the page (features do not import each other); without it the edit page keeps its current firm fields. */
export function AccountFormPage({ accountId, stepUp, preset }: { accountId: string; stepUp?: StepUpRunner; preset?: PresetSlot }) {
  const t = useTranslations();
  const existing = useAccount(accountId);
  if (existing.isPending)
    return (
      <div className="space-y-4" aria-busy="true" role="status">
        <span className="sr-only">{t("common.states.loading")}</span>
        <Skeleton className="h-11 w-full max-w-xl" />
        <Skeleton className="h-11 w-full max-w-xl" />
      </div>
    );
  if (existing.isError) {
    const { code } = errorMessage(t, existing.error);
    const notFound = existing.error instanceof ApiError && existing.error.code === "common.not_found";
    return <ErrorState title={notFound ? t("accounts.detail.notFound") : t("accounts.detail.loadError")} code={code} onRetry={() => void existing.refetch()} />;
  }
  return <AccountEditBody account={existing.data} stepUp={stepUp} preset={preset} />;
}

type FirmMode = "keep" | "preset" | "manual";

function AccountEditBody({ account, stepUp, preset }: { account: Account; stepUp?: StepUpRunner; preset?: PresetSlot }) {
  const t = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const router = useRouter();
  const formId = useId();
  const interim = useStepUpRunner();
  const runStepUp = stepUp ?? interim;
  const update = useUpdateAccount(account.id);
  const [staleNotice, setStaleNotice] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [conflict, setConflict] = useState(false);
  const [confirmDiscard, setConfirmDiscard] = useState(false);
  const [submitted, setSubmitted] = useState(false);

  const defaults = useMemo<AccountFormValues>(
    () => ({
      ...emptyAccountForm,
      kind: account.kind,
      displayName: account.display_name,
      firmName: account.firm_name ?? "",
      planName: account.plan_name ?? "",
      phase: account.phase ?? "",
      size: account.size_amount,
      maxPosition: String(account.max_position_qty),
      multiplier: String(account.multiplier),
      binding: bindingToValue(account.binding),
    }),
    [account],
  );
  const form = useForm<AccountFormValues>({ defaultValues: defaults });
  const values = useWatch({ control: form.control }) as AccountFormValues;

  const prop = account.kind === "prop";
  // Firm, plan, phase and size: keep the account's preset, pick (or attach) another one, or go back to free text (Q40).
  const hadPreset = account.preset_version_id !== undefined;
  const [mode, setMode] = useState<FirmMode>(hadPreset ? "keep" : "manual");
  const outcome = mode === "preset" ? (preset?.outcome ?? null) : null;
  const presetReady = outcome?.valid === true;
  const [presetError, setPresetError] = useState<string | undefined>();
  const sameSelection = outcome !== null && outcome.presetVersionId === account.preset_version_id && (outcome.firmPlatform ?? "") === (account.firm_platform ?? "");
  // The rules (and so the opt-in question) belong to the preset the account will have after this save, not to the generic text.
  const rulesPreset = mode === "keep" ? { id: account.preset_version_id ?? null, platform: account.firm_platform ?? null } : mode === "preset" && presetReady ? { id: outcome.presetVersionId, platform: outcome.firmPlatform } : { id: null, platform: null };
  const rules = useOptInRules(rulesPreset.id, rulesPreset.platform, prop && (mode !== "preset" || presetReady));
  const rule = rules.data?.automation;
  // Only a kind whose rule version this edit changes reopens a decision (contract 3.0.0); otherwise no section, no opt_ins.
  const differs = rule !== undefined && rule.id !== account.opt_ins.automation.current_rule_version_id;
  const showSection = prop && differs;
  const optInRequired = showSection && rule.firm_stance !== "allowed";
  const stale = values.automationRuleId !== "" && rule !== undefined && values.automationRuleId !== rule.id;
  const openPositionLock = account.has_open_position && account.binding !== undefined;
  const outcomeSize = outcome?.sizeAmount;
  useEffect(() => {
    if (outcomeSize !== undefined) form.setValue("size", outcomeSize);
  }, [form, outcomeSize]);

  async function submit(formValues: AccountFormValues) {
    setFormError(null);
    setConflict(false);
    const choice = showSection && !stale ? formValues.automationChoice : "";
    const merged: AccountFormValues = { ...formValues, automationChoice: choice, automationAck: choice === "" ? false : formValues.automationAck };
    const parsed = accountFormSchema(locale, { kind: account.kind, optInRequired, maxPositionCap: presetReady ? outcome.maxPositionCap : null }).safeParse(merged);
    form.clearErrors();
    setPresetError(undefined);
    if (mode === "preset" && !presetReady) {
      setPresetError(t("accounts.form.errors.presetRequired"));
      return;
    }
    if (!parsed.success) {
      let first: keyof AccountFormValues | null = null;
      for (const issue of parsed.error.issues) {
        const field = issue.path[0] as keyof AccountFormValues;
        if (form.formState.errors[field]) continue;
        form.setError(field, { type: "validate", message: issue.message });
        first ??= field;
      }
      if (first && first !== "automationChoice" && first !== "automationAck") form.setFocus(first);
      return;
    }
    // A preset account stays on its preset: only the selection is sent, never the manual fields (that would switch it to manual
    // mode and drop the preset link and each value's origin). `rule_values` is omitted: the stored values are kept (contract).
    // A new selection sends its own rule_values; an unchanged one must not (422 accounts.rule_values_without_selection_change).
    const sentPreset =
      mode === "keep" && account.preset_version_id
        ? { presetVersionId: account.preset_version_id, firmPlatform: account.firm_platform ?? null, ruleValues: {} }
        : mode === "preset" && outcome
          ? { presetVersionId: outcome.presetVersionId, firmPlatform: outcome.firmPlatform, ruleValues: sameSelection ? {} : outcome.ruleValues }
          : undefined;
    const body = toWriteBody(merged, { kind: account.kind, locale, automationRuleVersionId: rule?.id, ...(sentPreset ? { preset: sentPreset } : {}) });
    const send = (proof?: { challenge_id: string; code: string }) => update.mutateAsync(toUpdateBody({ ...body, ...(proof ? { step_up: proof } : {}) }, account.row_version));
    try {
      // Always through the runner: it sends without proof first and only opens the dialog when the Core asks, which also covers
      // a binding change on an account that receives automation (account_binding_change, Q35).
      const accepting = needsStepUp(body.opt_ins);
      await runStepUp({
        action: accepting ? "account_opt_in_accept" : "account_binding_change",
        target: account.id,
        description: t(accepting ? "accounts.form.stepUpDescriptionEdit" : "accounts.form.stepUpDescriptionBinding"),
        send,
      });
      toast.success(t("accounts.form.success.saved"));
      router.push(withLocale(locale, `/accounts/${account.id}`));
    } catch (error) {
      if (error instanceof StepUpCancelled) return;
      if (error instanceof ApiError) {
        if (error.code === "accounts.rule_version_changed") {
          setStaleNotice(true);
          form.setValue("automationChoice", "");
          form.setValue("automationAck", false);
          form.setValue("automationRuleId", "");
          void rules.refetch();
          return;
        }
        if (error.code === "accounts.opt_in_not_applicable") return void rules.refetch();
        if (error.code === "common.version_conflict") return setConflict(true);
        const field = error.status === 400 ? null : fieldFromPointer(error.pointer);
        if (field === "presetVersionId") return setPresetError(errorMessage(t, error).message);
        // The edit contract has no profile_size (only create does), so this field is not on the page: if the Core ever answers about it, say so in the banner instead of failing silently.
        if (field === "profileSize") return setFormError(errorMessage(t, error).message);
        if (field) {
          form.setError(field, { type: "server", message: errorMessage(t, error).message });
          if (field !== "automationChoice") form.setFocus(field);
          return;
        }
      }
      setFormError(errorMessage(t, error).message);
    }
  }

  const errorCount = Object.keys(form.formState.errors).length;

  return (
    <form
      noValidate
      className="mx-auto max-w-[720px] space-y-6 pb-24"
      onSubmit={(event) => {
        setSubmitted(true);
        void form.handleSubmit(submit)(event);
      }}
    >
      <PageHeader title={t("accounts.form.titleEdit")} />
      {submitted && errorCount >= 3 ? <Alert tone="danger">{t("accounts.form.errors.summary")}</Alert> : null}
      {conflict ? (
        <Alert tone="warning" className="items-center justify-between">
          <span>{t("accounts.form.versionConflict")}</span>
          <Button type="button" variant="outline" onClick={() => window.location.reload()}>{t("accounts.actions.reload")}</Button>
        </Alert>
      ) : null}

      <Card className="space-y-4 p-4">
        <h2 className="text-base font-semibold">{t("accounts.form.sections.type")}</h2>
        <p>{t("accounts.form.kind.fixed", { kind: t(`accounts.type.${account.kind}`) })}</p>
      </Card>

      {prop && preset ? (
        <Card className="space-y-3 p-4">
          <h2 className="text-base font-semibold">{t("accounts.form.steps.firm")}</h2>
          {mode === "keep" ? (
            <>
              <p className="text-sm">{t("accounts.form.preset.current", { label: `${account.firm_name ?? ""} · ${account.plan_name ?? ""} · ${account.phase ?? ""}` })}</p>
              <div className="flex flex-wrap gap-2">
                <Button type="button" variant="outline" onClick={() => setMode("preset")}>{t("accounts.form.preset.change")}</Button>
                <Button type="button" variant="ghost" onClick={() => setMode("manual")}>{t("accounts.form.preset.manual")}</Button>
              </div>
            </>
          ) : mode === "preset" ? (
            <>
              {!hadPreset ? <Alert tone="warning">{t("accounts.form.preset.attachNotice")}</Alert> : null}
              {preset.render({ switchToManual: () => setMode("manual"), error: presetError })}
              <Button type="button" variant="ghost" onClick={() => setMode(hadPreset ? "keep" : "manual")}>{t("accounts.form.preset.cancelChange")}</Button>
            </>
          ) : (
            <>
              <FirmFields form={form} formId={formId} />
              <div className="flex flex-wrap gap-2">
                <Button type="button" variant="outline" onClick={() => setMode("preset")}>{t("accounts.form.preset.choose")}</Button>
                {hadPreset ? <Button type="button" variant="ghost" onClick={() => setMode("keep")}>{t("accounts.form.preset.backToCurrent")}</Button> : null}
              </div>
            </>
          )}
        </Card>
      ) : null}

      <Card className="space-y-4 p-4">
        <h2 className="text-base font-semibold">{t("accounts.form.sections.data")}</h2>
        <DataFields
          form={form}
          formId={formId}
          editing
          // Firm, plan and phase are edited once: in the "Firma e plano" card when the page gives the picker, here only otherwise.
          withFirm={prop && mode === "manual" && !preset}
          preset={mode === "keep" ? { sizeText: formatMoney(account.size_amount, locale), maxPositionCap: null } : outcome ? { sizeText: formatMoney(outcome.sizeAmount, locale), maxPositionCap: outcome.maxPositionCap } : undefined}
        />
      </Card>

      <Card className="space-y-3 p-4">
        <h2 className="text-base font-semibold">{t("accounts.form.sections.platform")}</h2>
        <BindingField form={form} formId={formId} accountId={account.id} locked={openPositionLock} />
      </Card>

      {prop && rules.isError ? <Alert tone="danger">{t("accounts.optIn.rulesError")}</Alert> : null}
      {showSection && rule ? (
        <Card className="p-4">
          <OptInFormSection form={form} rule={rule} staleNotice={staleNotice} />
        </Card>
      ) : null}

      {formError ? <Alert tone="danger">{formError}</Alert> : null}

      <div className="fixed inset-x-0 bottom-0 z-10 flex justify-end gap-2 border-t bg-background p-3">
        <Button type="button" variant="outline" onClick={() => (form.formState.isDirty ? setConfirmDiscard(true) : router.back())}>{t("accounts.form.cancel")}</Button>
        <Button type="submit" disabled={update.isPending}>{update.isPending ? t("common.states.saving") : t("accounts.form.save")}</Button>
      </div>

      <AlertDialog open={confirmDiscard} onOpenChange={setConfirmDiscard}>
        <AlertDialogContent aria-describedby={undefined}>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("accounts.form.discard.title")}</AlertDialogTitle>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel autoFocus>{t("accounts.form.discard.keep")}</AlertDialogCancel>
            <AlertDialogAction variant="destructive" onClick={() => router.back()}>{t("accounts.form.discard.confirm")}</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </form>
  );
}
