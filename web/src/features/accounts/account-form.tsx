"use client";

import { useLocale, useTranslations } from "next-intl";
import { useRouter } from "next/navigation";
import { useId, useMemo, useState } from "react";
import { useForm, useWatch } from "react-hook-form";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { withLocale } from "@/shared/i18n/paths";
import { StepUpCancelled } from "@/shared/session/step-up";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Button } from "@/shared/ui/button";
import { Card } from "@/shared/ui/card";
import { ErrorState } from "@/shared/ui/error-state";
import { Skeleton } from "@/shared/ui/skeleton";
import { BindingField, DataFields } from "./account-fields";
import { useAccount, useOptInRules, useUpdateAccount } from "./api/use-accounts";
import type { Account } from "./api/types";
import { OptInFormSection } from "./opt-in-form-section";
import { accountFormSchema, bindingToValue, emptyAccountForm, fieldFromPointer, toUpdateBody, toWriteBody, type AccountFormValues } from "./lib/account-form";
import { needsStepUp } from "./lib/opt-in";
import { useStepUpRunner, type StepUpRunner } from "./lib/use-step-up";
import { PageHeader } from "@/shared/components/page-header";

/** Edit page: the same fields as "Nova conta", as sections on one page (F§10.1: edit is not a stepper). */
export function AccountFormPage({ accountId, stepUp }: { accountId: string; stepUp?: StepUpRunner }) {
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
  return <AccountEditBody account={existing.data} stepUp={stepUp} />;
}

function AccountEditBody({ account, stepUp }: { account: Account; stepUp?: StepUpRunner }) {
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
  const rules = useOptInRules(null, null, prop);
  const rule = rules.data?.automation;
  // Only a kind whose rule version this edit changes reopens a decision (contract 3.0.0); otherwise no section, no opt_ins.
  const differs = rule !== undefined && rule.id !== account.opt_ins.automation.current_rule_version_id;
  const showSection = prop && differs;
  const optInRequired = showSection && rule.firm_stance !== "allowed";
  const stale = values.automationRuleId !== "" && rule !== undefined && values.automationRuleId !== rule.id;
  const openPositionLock = account.has_open_position && account.binding !== undefined;

  async function submit(formValues: AccountFormValues) {
    setFormError(null);
    setConflict(false);
    const choice = showSection && !stale ? formValues.automationChoice : "";
    const merged: AccountFormValues = { ...formValues, automationChoice: choice, automationAck: choice === "" ? false : formValues.automationAck };
    const parsed = accountFormSchema(locale, { kind: account.kind, optInRequired }).safeParse(merged);
    form.clearErrors();
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
    const body = toWriteBody(merged, { kind: account.kind, locale, automationRuleVersionId: rule?.id });
    const send = (proof?: { challenge_id: string; code: string }) => update.mutateAsync(toUpdateBody({ ...body, ...(proof ? { step_up: proof } : {}) }, account.row_version));
    try {
      if (needsStepUp(body.opt_ins)) await runStepUp({ action: "account_opt_in_accept", target: account.id, description: t("accounts.form.stepUpDescriptionEdit"), send });
      else await send();
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

      <Card className="space-y-4 p-4">
        <h2 className="text-base font-semibold">{t("accounts.form.sections.data")}</h2>
        <DataFields form={form} formId={formId} editing withFirm={prop} />
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
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("accounts.form.discard.title")}</AlertDialogTitle>
            <AlertDialogDescription>{t("accounts.form.discard.title")}</AlertDialogDescription>
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
