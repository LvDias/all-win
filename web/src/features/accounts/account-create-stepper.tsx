"use client";

import { useLocale, useTranslations } from "next-intl";
import { useRouter } from "next/navigation";
import { useEffect, useId, useMemo, useRef, useState } from "react";
import { Controller, useForm, useWatch } from "react-hook-form";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { FormStepper, type FormStep, type ReviewRow, type SubmitError } from "@/shared/components/form-stepper";
import { formatMoney, parseDecimalInput } from "@/shared/format";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { withLocale } from "@/shared/i18n/paths";
import { StepUpCancelled } from "@/shared/session/step-up";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Label } from "@/shared/ui/label";
import { RadioGroup, RadioGroupItem } from "@/shared/ui/radio-group";
import { Skeleton } from "@/shared/ui/skeleton";
import { BindingField, DataFields, FirmFields, useFieldError } from "./account-fields";
import { useConnections } from "./api/use-connections";
import { getNearestProfileSizeCandidates } from "./api/accounts-api";
import { useCreateAccount, useOptInRules } from "./api/use-accounts";
import type { AccountKind } from "./api/types";
import type { PresetSlot } from "./api/preset-slot";
import { OptInFormSection } from "./opt-in-form-section";
import { emptyAccountForm, formErrorKeys, fieldFromPointer, stepSchemas, toWriteBody, type AccountFormValues } from "./lib/account-form";
import { useStepUpRunner, type StepUpRunner } from "./lib/use-step-up";
import { PageHeader } from "@/shared/components/page-header";

/**
 * "Nova conta" as a step-by-step flow (S5 rev. 4 §4.0, F§10.1). Prop: Tipo → Firma e plano → Dados da conta →
 * Conta na plataforma → Ordens automatizadas → Revisar. Real: Tipo → Dados → Conta na plataforma → Revisar.
 */
export function AccountCreateStepper({ stepUp, preset }: { stepUp?: StepUpRunner; preset?: PresetSlot }) {
  const t = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const router = useRouter();
  const formId = useId();
  const form = useForm<AccountFormValues>({ defaultValues: emptyAccountForm });
  const values = useWatch({ control: form.control }) as AccountFormValues;
  const fieldError = useFieldError(form);
  const create = useCreateAccount();
  const connections = useConnections();
  const interim = useStepUpRunner();
  const runStepUp = stepUp ?? interim;
  // One key per distinct body: the Core answers 409 when a key is reused with other data. A step-up retry keeps the body, so it keeps the key.
  const idempotency = useRef<{ key: string; body: string } | null>(null);
  const keyFor = (body: object) => {
    const fingerprint = JSON.stringify(body);
    if (idempotency.current?.body !== fingerprint) idempotency.current = { key: crypto.randomUUID(), body: fingerprint };
    return idempotency.current.key;
  };
  const [staleNotice, setStaleNotice] = useState(false);
  // Set when the Core says the size is exactly between two standard sizes; only valid for the size it was asked about.
  const [tie, setTie] = useState<{ size: string; candidates: string[] } | null>(null);
  const profileSizeCandidates = tie && tie.size === values.size ? tie.candidates : undefined;

  const kind: AccountKind | null = values.kind === "" ? null : values.kind;
  const firmFilled = values.firmName.trim() !== "" && values.planName.trim() !== "" && values.phase.trim() !== "";
  // With a preset slot the firm step is the picker; "Cadastrar sem preset" (Q40) falls back to the free-text firm, plan and phase.
  const [manual, setManual] = useState(false);
  const withPreset = preset !== undefined && kind === "prop" && !manual;
  const outcome = withPreset ? (preset.outcome ?? null) : null;
  const presetReady = outcome?.valid === true;
  const presetId = outcome?.presetVersionId ?? "";
  const presetSize = outcome?.sizeAmount;
  const presetDefault = outcome?.maxPositionDefault;
  // The picker lives outside the form; the form learns only what the steps validate: a resolved preset, its size, a default cap.
  useEffect(() => {
    form.setValue("presetVersionId", withPreset && presetReady ? presetId : "");
    if (withPreset && presetSize !== undefined) form.setValue("size", presetSize);
  }, [form, withPreset, presetReady, presetId, presetSize]);
  useEffect(() => {
    if (withPreset && presetDefault != null) form.setValue("maxPosition", String(presetDefault));
  }, [form, withPreset, presetId, presetDefault]);
  const rules = useOptInRules(outcome?.presetVersionId ?? null, outcome?.firmPlatform ?? null, kind === "prop" && (withPreset ? presetReady : firmFilled));
  const rule = rules.data?.automation;
  // The step stays while the rule is unknown; it leaves once the firm is known to allow automation (nothing to ask).
  const optInApplies = kind === "prop" && (rule === undefined || rule.firm_stance !== "allowed");
  const maxPositionCap = outcome?.maxPositionCap ?? null;
  const schemas = useMemo(() => stepSchemas(locale, { automationRuleId: rule?.id, maxPositionCap }), [locale, rule?.id, maxPositionCap]);

  const moneyOrNull = (input: string) => {
    const parsed = parseDecimalInput(input, locale, { scale: 2 });
    return parsed.ok ? formatMoney(parsed.value, locale) : null;
  };

  const choiceRow = (v: AccountFormValues): ReviewRow => ({
    label: t("accounts.optIn.automation.legend"),
    value: v.automationChoice === "" ? null : t(v.automationChoice === "accepted" ? "accounts.state.acceptsAutomation" : "accounts.state.noAutomation"),
  });

  const steps: FormStep<AccountFormValues>[] = [
    {
      id: "type",
      label: t("accounts.form.sections.type"),
      fields: ["kind"],
      schema: schemas.type,
      content: (
        <Controller
          control={form.control}
          name="kind"
          render={({ field }) => (
            <div className="space-y-1.5">
              <RadioGroup aria-label={t("accounts.form.sections.type")} value={field.value} onValueChange={field.onChange} aria-invalid={fieldError("kind") ? true : undefined}>
                {(["prop", "real"] as const).map((k) => (
                  <Label key={k} htmlFor={`${formId}-kind-${k}`} className="flex min-h-11 cursor-pointer items-center gap-3 rounded-lg border p-3">
                    <RadioGroupItem id={`${formId}-kind-${k}`} value={k} />
                    {t(`accounts.form.kind.${k}`)}
                  </Label>
                ))}
              </RadioGroup>
              {fieldError("kind") ? <p className="text-sm text-destructive">{fieldError("kind")}</p> : null}
            </div>
          )}
        />
      ),
      review: (v) => [{ label: t("accounts.list.columns.type"), value: v.kind === "" ? null : t(`accounts.form.kind.${v.kind}`) }],
    },
    {
      id: "firm",
      label: t("accounts.form.steps.firm"),
      fields: withPreset ? ["presetVersionId"] : ["firmName", "planName", "phase"],
      when: (v) => v.kind === "prop",
      schema: withPreset ? schemas.preset : schemas.firm,
      content: withPreset ? (
        preset.render({ switchToManual: () => setManual(true), error: fieldError("presetVersionId") })
      ) : (
        <>
          {preset ? <Button type="button" variant="link" className="h-auto justify-start px-0" onClick={() => setManual(false)}>{t("accounts.form.preset.choose")}</Button> : null}
          <FirmFields form={form} formId={formId} />
        </>
      ),
      review: (v) =>
        withPreset
          ? [{ label: t("accounts.form.steps.firm"), value: outcome?.label ?? null }]
          : [
              { label: t("accounts.form.fields.firm"), value: v.firmName },
              { label: t("accounts.form.fields.plan"), value: v.planName },
              { label: t("accounts.form.fields.phase"), value: v.phase },
            ],
    },
    {
      id: "data",
      label: t("accounts.form.sections.data"),
      fields: ["displayName", "size", "profileSize", "maxPosition", "multiplier"],
      schema: schemas.data,
      content: <DataFields form={form} formId={formId} editing={false} withFirm={false} profileSizeCandidates={withPreset ? undefined : profileSizeCandidates} preset={withPreset && outcome ? { sizeText: moneyOrNull(outcome.sizeAmount) ?? outcome.sizeAmount, maxPositionCap: outcome.maxPositionCap } : undefined} />,
      review: (v) => [
        { label: t("accounts.form.fields.name"), value: v.displayName },
        { label: t("accounts.form.fields.size"), value: moneyOrNull(v.size) },
        ...(profileSizeCandidates && v.profileSize !== "" ? [{ label: t("accounts.form.profileSize.label"), value: formatMoney(v.profileSize, locale) }] : []),
        { label: t("accounts.form.fields.maxPosition"), value: v.maxPosition },
        { label: t("accounts.form.fields.multiplier"), value: `${v.multiplier.trim() === "" ? 1 : v.multiplier}×` },
      ],
    },
    {
      id: "platform",
      label: t("accounts.form.sections.platform"),
      optional: true,
      fields: ["binding"],
      schema: schemas.platform,
      content: <BindingField form={form} formId={formId} />,
      review: (v) => {
        const [connectionId, ...rest] = v.binding.split("|");
        const platformAccount = rest.join("|");
        const conn = connections.data?.find((c) => c.id === connectionId);
        return [{ label: t("accounts.form.binding.label"), value: v.binding === "" ? t("accounts.form.binding.none") : `${platformAccount}${conn ? ` · ${conn.name}` : ""}` }];
      },
    },
    {
      id: "optIn",
      label: t("accounts.form.steps.optIn"),
      fields: ["automationChoice", "automationAck", "automationRuleId"],
      when: () => optInApplies,
      schema: schemas.optIn,
      content: rules.isError ? (
        <Alert tone="danger">{t("accounts.optIn.rulesError")}</Alert>
      ) : rule ? (
        <OptInFormSection form={form} rule={rule} staleNotice={staleNotice} />
      ) : (
        <Skeleton className="h-40 w-full" />
      ),
      review: (v) => [choiceRow(v)],
    },
  ];

  async function onSubmit(v: AccountFormValues): Promise<SubmitError[] | void> {
    if (kind === null) return;
    const withChoice = optInApplies && v.automationChoice !== "";
    const body = toWriteBody({ ...v, automationChoice: optInApplies ? v.automationChoice : "" }, { kind, locale, automationRuleVersionId: rule?.id, ...(withPreset && outcome ? { preset: outcome } : {}) });
    const payload = { ...body, kind, ...(!withPreset && profileSizeCandidates && v.profileSize !== "" ? { profile_size: v.profileSize } : {}) };
    const idempotencyKey = keyFor(payload);
    const send = (proof?: { challenge_id: string; code: string }) =>
      create.mutateAsync({ body: { ...payload, ...(proof ? { step_up: proof } : {}) }, idempotencyKey });
    try {
      const saved =
        withChoice && v.automationChoice === "accepted"
          ? await runStepUp({ action: "account_create_opt_in_accept", description: t("accounts.form.stepUpDescription", { name: v.displayName.trim() }), send })
          : await send();
      toast.success(t("accounts.form.success.created"));
      router.push(withLocale(locale, `/accounts/${saved.id}`));
    } catch (error) {
      if (error instanceof StepUpCancelled) return [];
      if (error instanceof ApiError && error.code === "accounts.rule_version_changed") {
        setStaleNotice(true);
        form.setValue("automationChoice", "");
        form.setValue("automationAck", false);
        form.setValue("automationRuleId", "");
        void rules.refetch();
        return [{ pointer: "/automationChoice", message: formErrorKeys.optInRequired }];
      }
      if (error instanceof ApiError && (error.code === "accounts.profile_size_required" || error.code === "accounts.profile_size_invalid")) {
        form.setValue("profileSize", "");
        try {
          const candidates = await getNearestProfileSizeCandidates(body.size_amount ?? v.size);
          setTie({ size: v.size, candidates });
        } catch {
          return [{ message: t("common.errors.failed") }];
        }
        return [{ pointer: "/profileSize", message: errorMessage(t, error).message }];
      }
      if (error instanceof ApiError && error.code === "accounts.opt_in_not_applicable") {
        void rules.refetch();
        return [];
      }
      const field = error instanceof ApiError && error.status !== 400 ? fieldFromPointer(error.pointer) : null;
      return [{ ...(field ? { pointer: `/${field}` } : {}), message: errorMessage(t, error).message }];
    }
  }

  return (
    <div className="mx-auto max-w-[720px] space-y-6">
      <PageHeader title={t("accounts.form.titleNew")} />
      <FormStepper form={form} steps={steps} onSubmit={onSubmit} submitLabel={t("accounts.form.save")} onCancel={() => router.back()} />
    </div>
  );
}
