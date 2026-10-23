"use client";

import { useLocale, useTranslations } from "next-intl";
import { useRouter } from "next/navigation";
import { useId, useMemo, useRef, useState, type ReactNode } from "react";
import { Controller, useForm, useWatch } from "react-hook-form";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { withLocale } from "@/shared/i18n/paths";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Button } from "@/shared/ui/button";
import { Card } from "@/shared/ui/card";
import { ErrorState } from "@/shared/ui/error-state";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { RadioGroup, RadioGroupItem } from "@/shared/ui/radio-group";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import { Skeleton } from "@/shared/ui/skeleton";
import { useConnections } from "./api/use-connections";
import { useAccount, useAccounts, useCreateAccount, useOptInRules, useUpdateAccount } from "./api/use-accounts";
import type { Account, AccountKind } from "./api/types";
import { OptInSection, type OptInValue } from "./opt-in-section";
import {
  accountFormSchema,
  bindingToValue,
  emptyAccountForm,
  fieldFromPointer,
  toUpdateBody,
  toWriteBody,
  type AccountFormValues,
} from "./lib/account-form";
import { needsStepUp, optInKindFromPointer } from "./lib/opt-in";
import { useStepUpRunner, type StepUpRunner } from "./lib/use-step-up";

const noBinding = "__none__";

function Field({ id, label, help, error, children }: { id: string; label: string; help?: string; error?: string; children: ReactNode }) {
  return (
    <div className="space-y-1.5">
      <Label htmlFor={id}>{label}</Label>
      {children}
      {help ? <p id={`${id}-help`} className="text-sm text-muted-foreground">{help}</p> : null}
      {error ? <p id={`${id}-error`} className="text-sm text-destructive">{error}</p> : null}
    </div>
  );
}

const describedBy = (id: string, help: boolean, error: boolean) => [help ? `${id}-help` : null, error ? `${id}-error` : null].filter(Boolean).join(" ") || undefined;

export function AccountFormPage({ accountId, stepUp }: { accountId?: string; stepUp?: StepUpRunner }) {
  const t = useTranslations();
  const existing = useAccount(accountId ?? "");
  if (accountId === undefined) return <AccountFormBody stepUp={stepUp} />;
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
  return <AccountFormBody account={existing.data} stepUp={stepUp} />;
}

function AccountFormBody({ account, stepUp }: { account?: Account; stepUp?: StepUpRunner }) {
  const t = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const router = useRouter();
  const editing = account !== undefined;
  const [kind, setKind] = useState<AccountKind | null>(account?.kind ?? null);
  const idKey = useRef(crypto.randomUUID());
  const interimStepUp = useStepUpRunner();
  const runStepUp = stepUp ?? interimStepUp;
  const [optIn, setOptIn] = useState<OptInValue & { ruleId: string | null }>({ choice: "", ack: false, ruleId: null });
  const [staleNotice, setStaleNotice] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [conflict, setConflict] = useState(false);
  const [confirmDiscard, setConfirmDiscard] = useState(false);
  const [submitted, setSubmitted] = useState(false);
  const formId = useId();

  const create = useCreateAccount();
  const update = useUpdateAccount(account?.id ?? "");
  const connections = useConnections();
  const accounts = useAccounts();

  const defaults = useMemo<AccountFormValues>(
    () =>
      account
        ? { ...emptyAccountForm, displayName: account.display_name, firmName: account.firm_name ?? "", planName: account.plan_name ?? "", phase: account.phase ?? "", size: account.size_amount, maxPosition: String(account.max_position_qty), multiplier: String(account.multiplier), binding: bindingToValue(account.binding) }
        : emptyAccountForm,
    [account],
  );

  const draftFilled = (v: AccountFormValues) => v.firmName.trim() !== "" && v.planName.trim() !== "" && v.phase.trim() !== "" && v.size.trim() !== "";
  const form = useForm<AccountFormValues>({ defaultValues: defaults });
  const values = useWatch({ control: form.control }) as AccountFormValues;
  const propFilled = kind === "prop" && draftFilled(values);
  const rules = useOptInRules(null, null, propFilled);
  const automationRule = rules.data?.automation;
  const differs = !editing || (automationRule !== undefined && automationRule.id !== account.opt_ins.automation.current_rule_version_id);
  const optInRequired = propFilled && automationRule !== undefined && automationRule.firm_stance !== "allowed" && differs;
  const showSection = propFilled && automationRule !== undefined && differs;

  const stale = optIn.ruleId !== null && automationRule !== undefined && optIn.ruleId !== automationRule.id;
  const effective: OptInValue = stale ? { choice: "", ack: false } : { choice: optIn.choice, ack: optIn.ack };
  const schema = useMemo(() => accountFormSchema(locale, { kind: kind ?? "real", optInRequired }), [locale, kind, optInRequired]);

  const [errors, setErrors] = useState<Partial<Record<keyof AccountFormValues, string>>>({});
  const errorText = (field: keyof AccountFormValues) => {
    const e = errors[field];
    if (!e) return undefined;
    return e.includes(".") ? e : t(`accounts.form.errors.${e}`);
  };

  function applyApiError(error: unknown) {
    if (error instanceof ApiError) {
      if (error.code === "accounts.rule_version_changed") {
        setStaleNotice(true);
        setOptIn({ choice: "", ack: false, ruleId: null });
        void rules.refetch();
        return;
      }
      if (error.code === "accounts.opt_in_not_applicable") {
        void rules.refetch();
        return;
      }
      if (error.code === "common.version_conflict") return setConflict(true);
      const field = fieldFromPointer(error.pointer);
      if (field && error.status !== 400) {
        setErrors({ [field]: errorMessage(t, error).message });
        if (optInKindFromPointer(error.pointer) === null) form.setFocus(field);
        return;
      }
    }
    setFormError(errorMessage(t, error).message);
  }

  async function submit(formValues: AccountFormValues) {
    setFormError(null);
    setConflict(false);
    const merged: AccountFormValues = {
      ...formValues,
      automationChoice: showSection ? effective.choice : "",
      automationAck: showSection ? effective.ack : false,
    };
    const parsed = schema.safeParse(merged);
    if (!parsed.success) {
      const next: typeof errors = {};
      for (const issue of parsed.error.issues) next[issue.path[0] as keyof AccountFormValues] ??= issue.message;
      setErrors(next);
      const first = Object.keys(next)[0] as keyof AccountFormValues | undefined;
      if (first && first !== "automationChoice" && first !== "automationAck") form.setFocus(first);
      return;
    }
    setErrors({});
    if (kind === null) return;
    const body = toWriteBody(merged, { kind, locale, automationRuleVersionId: automationRule?.id });
    const send = (step_up?: { challenge_id: string; code: string }) => {
      const withProof = { ...body, ...(step_up ? { step_up } : {}) };
      return editing
        ? update.mutateAsync(toUpdateBody(withProof, account.row_version))
        : create.mutateAsync({ body: { ...withProof, kind }, idempotencyKey: idKey.current });
    };
    try {
      const saved = needsStepUp(body.opt_ins)
        ? await runStepUp({ action: editing ? "account_opt_in_accept" : "account_create_opt_in_accept", target: account?.id, send })
        : await send();
      toast.success(t(editing ? "accounts.form.success.saved" : "accounts.form.success.created"));
      router.push(withLocale(locale, `/accounts/${saved.id}`));
    } catch (error) {
      applyApiError(error);
    }
  }

  const pending = create.isPending || update.isPending;
  const boundByOther = new Map<string, string>();
  for (const a of accounts.data ?? []) if (a.binding && a.id !== account?.id) boundByOther.set(`${a.binding.connection_id}|${a.binding.platform_account}`, a.display_name);
  const pairedConnections = (connections.data ?? []).filter((c) => c.status !== "unpaired" && c.status !== "awaiting_pairing");
  const bindingOptions = pairedConnections.flatMap((c) => (c.visible_accounts ?? []).map((v) => ({ value: `${c.id}|${v.platform_account}`, conn: c, v })));
  const openPositionLock = editing && account.has_open_position && account.binding !== undefined;
  const errorCount = Object.keys(errors).length;
  const firmFallback = t("accounts.optIn.automation.genericFirm");
  const nameId = `${formId}-name`;

  return (
    <form
      noValidate
      className="mx-auto max-w-[720px] space-y-6 pb-24"
      onSubmit={(event) => {
        setSubmitted(true);
        void form.handleSubmit(submit)(event);
      }}
    >
      <h1 className="text-xl font-semibold md:text-2xl">{t(editing ? "accounts.form.titleEdit" : "accounts.form.titleNew")}</h1>

      {submitted && errorCount >= 3 ? <Alert tone="danger">{t("accounts.form.errors.summary")}</Alert> : null}
      {conflict ? (
        <Alert tone="warning" className="items-center justify-between">
          <span>{t("accounts.form.versionConflict")}</span>
          <Button type="button" variant="outline" onClick={() => window.location.reload()}>{t("accounts.actions.reload")}</Button>
        </Alert>
      ) : null}

      <Card className="space-y-4 p-4">
        <h2 className="text-base font-semibold">{t("accounts.form.sections.type")}</h2>
        {editing ? (
          <p>{t("accounts.form.kind.fixed", { kind: t(`accounts.type.${account.kind}`) })}</p>
        ) : (
          <RadioGroup aria-label={t("accounts.form.sections.type")} value={kind ?? ""} onValueChange={(v) => setKind(v as AccountKind)}>
            {(["prop", "real"] as const).map((k) => (
              <Label key={k} htmlFor={`${formId}-kind-${k}`} className="flex min-h-11 cursor-pointer items-center gap-3 rounded-lg border p-3">
                <RadioGroupItem id={`${formId}-kind-${k}`} value={k} />
                {t(`accounts.form.kind.${k}`)}
              </Label>
            ))}
          </RadioGroup>
        )}
      </Card>

      {kind === null ? null : (
        <>
          <Card className="space-y-4 p-4">
            <h2 className="text-base font-semibold">{t("accounts.form.sections.data")}</h2>
            <Field id={nameId} label={t("accounts.form.fields.name")} error={errorText("displayName")}>
              <Input id={nameId} maxLength={80} aria-invalid={errors.displayName ? true : undefined} aria-describedby={describedBy(nameId, false, !!errors.displayName)} {...form.register("displayName")} />
            </Field>
            {kind === "prop" ? (
              <>
                {(["firmName", "planName", "phase"] as const).map((f) => (
                  <Field key={f} id={`${formId}-${f}`} label={t(`accounts.form.fields.${f === "firmName" ? "firm" : f === "planName" ? "plan" : "phase"}`)} error={errorText(f)}>
                    <Input id={`${formId}-${f}`} maxLength={80} aria-invalid={errors[f] ? true : undefined} aria-describedby={describedBy(`${formId}-${f}`, false, !!errors[f])} {...form.register(f)} />
                  </Field>
                ))}
              </>
            ) : null}
            <Field id={`${formId}-size`} label={t("accounts.form.fields.size")} error={errorText("size")}>
              <Input id={`${formId}-size`} inputMode="decimal" aria-invalid={errors.size ? true : undefined} aria-describedby={describedBy(`${formId}-size`, false, !!errors.size)} {...form.register("size")} />
            </Field>
            <Field id={`${formId}-max`} label={t("accounts.form.fields.maxPosition")} help={t("accounts.form.fields.maxPositionHelp")} error={errorText("maxPosition")}>
              <Input id={`${formId}-max`} inputMode="numeric" aria-invalid={errors.maxPosition ? true : undefined} aria-describedby={describedBy(`${formId}-max`, true, !!errors.maxPosition)} {...form.register("maxPosition")} />
              {editing ? <p className="text-sm text-muted-foreground">{t("accounts.form.fields.maxPositionShrink")}</p> : null}
            </Field>
            <Field id={`${formId}-mult`} label={t("accounts.form.fields.multiplier")} help={t("accounts.form.fields.multiplierHelp")} error={errorText("multiplier")}>
              <Input id={`${formId}-mult`} inputMode="numeric" aria-invalid={errors.multiplier ? true : undefined} aria-describedby={describedBy(`${formId}-mult`, true, !!errors.multiplier)} {...form.register("multiplier")} />
            </Field>
          </Card>

          <Card className="space-y-3 p-4">
            <h2 className="text-base font-semibold">{t("accounts.form.sections.platform")}</h2>
            {pairedConnections.length === 0 ? (
              <Alert tone="info">{t("accounts.form.binding.noConnections")}</Alert>
            ) : (
              <Field id={`${formId}-binding`} label={t("accounts.form.binding.label")} error={errorText("binding")}>
                <Controller
                  control={form.control}
                  name="binding"
                  render={({ field }) => (
                    <Select value={field.value === "" ? noBinding : field.value} onValueChange={(v) => field.onChange(v === noBinding ? "" : v)} disabled={openPositionLock}>
                      <SelectTrigger id={`${formId}-binding`} aria-invalid={errors.binding ? true : undefined}><SelectValue /></SelectTrigger>
                      <SelectContent>
                        <SelectItem value={noBinding}>{t("accounts.form.binding.none")}</SelectItem>
                        {bindingOptions.map(({ value, conn, v }) => {
                          const other = boundByOther.get(value);
                          return (
                            <SelectItem key={value} value={value} disabled={other !== undefined}>
                              {[`${v.platform_account} · ${conn.name} (${t(`accounts.platformKind.${v.kind}`)})`, other ? t("accounts.form.binding.taken", { name: other }) : null].filter(Boolean).join(" — ")}
                            </SelectItem>
                          );
                        })}
                      </SelectContent>
                    </Select>
                  )}
                />
                {openPositionLock ? <p className="text-sm text-muted-foreground">{t("accounts.form.binding.openPosition")}</p> : null}
              </Field>
            )}
          </Card>

          {kind === "prop" && propFilled && rules.isError ? <Alert tone="danger">{t("accounts.optIn.rulesError")}</Alert> : null}
          {showSection && automationRule ? (
            <Card className="p-4">
              <OptInSection
                kind="automation"
                rule={automationRule}
                value={effective}
                onChange={(v) => {
                  setStaleNotice(false);
                  setErrors((e) => ({ ...e, automationChoice: undefined, automationAck: undefined }));
                  setOptIn({ ...v, ruleId: automationRule.id });
                }}
                choiceError={errorText("automationChoice")}
                ackError={errorText("automationAck")}
                notice={staleNotice ? t("accounts.optIn.ruleStale") : stale ? t("accounts.optIn.ruleChanged") : undefined}
                firmFallback={firmFallback}
              />
            </Card>
          ) : null}

          {formError ? <Alert tone="danger">{formError}</Alert> : null}

          <div className="fixed inset-x-0 bottom-0 z-10 flex justify-end gap-2 border-t bg-background p-3">
            <Button type="button" variant="outline" onClick={() => (form.formState.isDirty ? setConfirmDiscard(true) : router.back())}>{t("accounts.form.cancel")}</Button>
            <Button type="submit" disabled={pending}>{pending ? t("common.states.saving") : t("accounts.form.save")}</Button>
          </div>
        </>
      )}

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
