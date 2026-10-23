"use client";

import { useLocale, useTranslations } from "next-intl";
import { useRouter } from "next/navigation";
import { useId, useMemo, useRef, useState } from "react";
import { Controller, useForm, useWatch } from "react-hook-form";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { FormStepper, type FormStep, type ReviewRow, type SubmitError } from "@/shared/components/form-stepper";
import { formatMoney } from "@/shared/format";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { withLocale } from "@/shared/i18n/paths";
import { StepUpCancelled } from "@/shared/session/step-up";
import { Alert } from "@/shared/ui/alert";
import { Label } from "@/shared/ui/label";
import { RadioGroup, RadioGroupItem } from "@/shared/ui/radio-group";
import { Skeleton } from "@/shared/ui/skeleton";
import { BindingField, DataFields, FirmFields, useFieldError } from "./account-fields";
import { useConnections } from "./api/use-connections";
import { useCreateAccount, useOptInRules } from "./api/use-accounts";
import type { AccountKind } from "./api/types";
import { OptInFormSection } from "./opt-in-form-section";
import { emptyAccountForm, formErrorKeys, fieldFromPointer, stepSchemas, toWriteBody, type AccountFormValues } from "./lib/account-form";
import { useStepUpRunner, type StepUpRunner } from "./lib/use-step-up";

/**
 * "Nova conta" as a step-by-step flow (S5 rev. 4 §4.0, F§10.1). Prop: Tipo → Firma e plano → Dados da conta →
 * Conta na plataforma → Ordens automatizadas → Revisar. Real: Tipo → Dados → Conta na plataforma → Revisar.
 */
export function AccountCreateStepper({ stepUp }: { stepUp?: StepUpRunner }) {
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
  const idempotencyKey = useRef(crypto.randomUUID());
  const [staleNotice, setStaleNotice] = useState(false);

  const kind: AccountKind | null = values.kind === "" ? null : values.kind;
  const firmFilled = values.firmName.trim() !== "" && values.planName.trim() !== "" && values.phase.trim() !== "";
  const rules = useOptInRules(null, null, kind === "prop" && firmFilled);
  const rule = rules.data?.automation;
  // The step stays while the rule is unknown; it leaves once the firm is known to allow automation (nothing to ask).
  const optInApplies = kind === "prop" && (rule === undefined || rule.firm_stance !== "allowed");
  const schemas = useMemo(() => stepSchemas(locale, { automationRuleId: rule?.id }), [locale, rule?.id]);

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
      fields: ["firmName", "planName", "phase"],
      when: (v) => v.kind === "prop",
      schema: schemas.firm,
      content: <FirmFields form={form} formId={formId} />,
      review: (v) => [
        { label: t("accounts.form.fields.firm"), value: v.firmName },
        { label: t("accounts.form.fields.plan"), value: v.planName },
        { label: t("accounts.form.fields.phase"), value: v.phase },
      ],
    },
    {
      id: "data",
      label: t("accounts.form.sections.data"),
      fields: ["displayName", "size", "maxPosition", "multiplier"],
      schema: schemas.data,
      content: <DataFields form={form} formId={formId} editing={false} withFirm={false} />,
      review: (v) => [
        { label: t("accounts.form.fields.name"), value: v.displayName },
        { label: t("accounts.form.fields.size"), value: v.size.trim() === "" ? null : formatMoney(v.size.replace(",", "."), locale) },
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
    const body = toWriteBody({ ...v, automationChoice: optInApplies ? v.automationChoice : "" }, { kind, locale, automationRuleVersionId: rule?.id });
    const send = (proof?: { challenge_id: string; code: string }) =>
      create.mutateAsync({ body: { ...body, kind, ...(proof ? { step_up: proof } : {}) }, idempotencyKey: idempotencyKey.current });
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
      <h1 className="text-xl font-semibold md:text-2xl">{t("accounts.form.titleNew")}</h1>
      <FormStepper form={form} steps={steps} onSubmit={onSubmit} submitLabel={t("accounts.form.save")} onCancel={() => router.back()} />
    </div>
  );
}
