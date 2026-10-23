"use client";

import { useQuery } from "@tanstack/react-query";
import { Landmark } from "lucide-react";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useMemo, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { FormStepper, type FormStep, type SubmitError } from "@/shared/components/form-stepper";
import { PageHeader } from "@/shared/components/page-header";
import { formatDateTime, zonedToInstant } from "@/shared/format";
import { useLocalePath } from "@/shared/i18n/navigation";
import { useDebounced } from "@/shared/lib/use-debounced";
import { useDisplay } from "@/shared/session/display";
import { Button } from "@/shared/ui/button";
import { EmptyState } from "@/shared/ui/empty-state";
import { ErrorState } from "@/shared/ui/error-state";
import { Label } from "@/shared/ui/label";
import { RadioGroup, RadioGroupItem } from "@/shared/ui/radio-group";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import { Skeleton } from "@/shared/ui/skeleton";
import { previewManualTrade, useCreateManualTrade, useInstruments, useTradeAccounts } from "./api";
import { emptyValues, excursionsSchema, pointerToField, stepFields, timesSchema, toRequest, tradeSchema, type ManualTradeValues } from "./manual-trade-model";
import { Calculated, DateTime, Field, TextField } from "./trade-fields";

const newKey = () => (typeof crypto !== "undefined" && "randomUUID" in crypto ? crypto.randomUUID() : `${Date.now()}-${Math.random().toString(16).slice(2)}`);

/** S21-03: record a trade the platform did not capture, step by step (F§10.1). */
export function ManualTradeFlow() {
  const t = useTranslations("trades.manual");
  const root = useTranslations();
  const path = useLocalePath();
  const accounts = useTradeAccounts();
  const instruments = useInstruments();
  // "Save and add another" restarts the flow keeping only the account (spec §4.2).
  const [round, setRound] = useState<{ n: number; accountId: string }>({ n: 0, accountId: "" });
  const header = <PageHeader title={t("title")} />;

  if (accounts.isPending || instruments.isPending)
    return (
      <div className="grid max-w-180 gap-4" aria-busy="true">
        {header}
        <Skeleton className="h-40 w-full" />
      </div>
    );
  if (accounts.isError || instruments.isError)
    return (
      <div className="grid max-w-180 gap-4">
        {header}
        <ErrorState title={t("loadError")} code={errorMessage(root, accounts.error ?? instruments.error).code} onRetry={() => {
          void accounts.refetch();
          void instruments.refetch();
        }} />
      </div>
    );
  if (accounts.data.accounts.length === 0)
    return (
      <div className="grid max-w-180 gap-4">
        {header}
        <EmptyState icon={Landmark} title={t("noAccounts")} description={t("noAccountsBody")}>
          <Button asChild variant="outline">
            <Link href={path("/trades")}>{t("backToList")}</Link>
          </Button>
        </EmptyState>
      </div>
    );
  const instrument = instruments.data.instruments[0];
  return (
    <div className="grid max-w-180 gap-4">
      {header}
      <Flow
        key={round.n}
        initialAccountId={round.accountId}
        restarted={round.n > 0}
        accounts={accounts.data.accounts.map((account) => ({ id: account.id, name: account.display_name }))}
        instrument={instrument ? { id: instrument.id, symbol: instrument.symbol, tickSize: instrument.tick_size } : undefined}
        onAnother={(accountId) => setRound((previous) => ({ n: previous.n + 1, accountId }))}
        onDone={() => undefined}
      />
    </div>
  );
}

type FlowProps = {
  initialAccountId: string;
  restarted: boolean;
  accounts: { id: string; name: string }[];
  instrument: { id: string; symbol: string; tickSize: string } | undefined;
  onAnother: (accountId: string) => void;
  onDone: () => void;
};

function Flow({ initialAccountId, restarted, accounts, instrument, onAnother }: FlowProps) {
  const t = useTranslations("trades.manual");
  const root = useTranslations();
  const router = useRouter();
  const path = useLocalePath();
  const { locale, timeZone } = useDisplay();
  const create = useCreateManualTrade();
  // One key per form instance: a retry or a double click never creates two trades.
  const [key] = useState(newKey);
  const form = useForm<ManualTradeValues>({ defaultValues: emptyValues(initialAccountId), mode: "onSubmit" });
  const values = form.watch();
  const [clock] = useState(() => ({ now: () => Date.now() }));
  const env = useMemo(() => ({ locale, timeZone, tick: instrument?.tickSize, now: clock.now }), [locale, timeZone, instrument?.tickSize, clock]);
  const m = (k: string) => t(k);

  const request = toRequest(values, instrument?.id, env);
  const settled = useDebounced(request ? JSON.stringify(request) : "", 300);
  const preview = useQuery({
    queryKey: ["trades", "preview", settled],
    enabled: settled !== "",
    queryFn: ({ signal }) => {
      const body = JSON.parse(settled) as NonNullable<typeof request>;
      return previewManualTrade({ account_id: body.account_id, instrument_id: body.instrument_id, direction: body.direction, quantity: body.quantity, entry_time: body.entry_time, entry_price: body.entry_price, exit_time: body.exit_time, exit_price: body.exit_price, commission: body.commission }, signal);
    },
  });

  async function save(toast_: string, after: (accountId: string) => void) {
    const body = toRequest(form.getValues(), instrument?.id, env);
    if (!body) return [{ message: t("errors.cannotSave") }] satisfies SubmitError[];
    try {
      await create.mutateAsync({ body, key });
      toast.success(toast_);
      after(body.account_id);
      return undefined;
    } catch (error) {
      if (error instanceof ApiError && error.pointer) {
        const field = pointerToField[error.pointer];
        if (field) return [{ pointer: `/${field}`, message: errorMessage(root, error).message }] satisfies SubmitError[];
      }
      throw error;
    }
  }

  const accountName = accounts.find((account) => account.id === values.accountId)?.name ?? "";
  const calc = <Calculated preview={preview.data} pending={preview.isFetching && settled !== ""} failed={preview.isError} ready={settled !== ""} />;

  const steps: FormStep<ManualTradeValues>[] = [
    {
      id: "trade",
      label: t("steps.trade"),
      fields: [...stepFields.trade],
      schema: tradeSchema(m),
      content: (
        <>
          <Controller
            control={form.control}
            name="accountId"
            render={({ field, fieldState }) => (
              <Field id="mt-account" label={t("fields.account")} error={fieldState.error?.message}>
                <Select value={field.value} onValueChange={field.onChange}>
                  <SelectTrigger id="mt-account" ref={field.ref} aria-invalid={fieldState.error ? true : undefined} aria-describedby={fieldState.error ? "mt-account-error" : undefined}>
                    <SelectValue placeholder={t("choose")} />
                  </SelectTrigger>
                  <SelectContent>
                    {accounts.map((account) => (
                      <SelectItem key={account.id} value={account.id}>
                        {account.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </Field>
            )}
          />
          <Field id="mt-instrument" label={t("fields.instrument")}>
            <Select value={instrument?.id ?? ""} onValueChange={() => undefined}>
              <SelectTrigger id="mt-instrument">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>{instrument ? <SelectItem value={instrument.id}>{instrument.symbol}</SelectItem> : null}</SelectContent>
            </Select>
          </Field>
          <Controller
            control={form.control}
            name="direction"
            render={({ field, fieldState }) => (
              <fieldset className="grid gap-2">
                <legend className="mb-1 text-sm font-medium">{t("fields.direction")}</legend>
                <RadioGroup value={field.value} onValueChange={field.onChange} aria-invalid={fieldState.error ? true : undefined}>
                  {(["long", "short"] as const).map((direction) => (
                    <Label key={direction} className="flex min-h-11 items-center gap-3 rounded-md border p-3">
                      <RadioGroupItem value={direction} ref={direction === "long" ? field.ref : undefined} />
                      {t(`direction.${direction}`)}
                    </Label>
                  ))}
                </RadioGroup>
                {fieldState.error ? <p className="text-sm text-status-danger">{fieldState.error.message}</p> : null}
              </fieldset>
            )}
          />
          <TextField form={form} name="quantity" id="mt-quantity" label={t("fields.quantity")} inputMode="numeric" />
        </>
      ),
      review: (v) => [
        { label: t("fields.account"), value: accounts.find((account) => account.id === v.accountId)?.name },
        { label: t("fields.instrument"), value: instrument?.symbol },
        { label: t("fields.direction"), value: v.direction ? t(`direction.${v.direction}`) : "" },
        { label: t("fields.quantity"), value: v.quantity },
      ],
    },
    {
      id: "times",
      label: t("steps.times"),
      fields: [...stepFields.times],
      schema: timesSchema(m, env),
      content: (
        <>
          <DateTime form={form} idPrefix="mt-entry" date="entryDate" time="entryTime" label={t("fields.entryAt", { zone: timeZone })} />
          <TextField form={form} name="entryPrice" id="mt-entry-price" label={t("fields.entryPrice")} inputMode="decimal" help={instrument ? t("tickHelp", { tick: instrument.tickSize }) : undefined} />
          <DateTime form={form} idPrefix="mt-exit" date="exitDate" time="exitTime" label={t("fields.exitAt", { zone: timeZone })} />
          <TextField form={form} name="exitPrice" id="mt-exit-price" label={t("fields.exitPrice")} inputMode="decimal" />
          <TextField form={form} name="commission" id="mt-commission" label={t("fields.commission")} inputMode="decimal" help={t("commissionHelp")} />
          {calc}
        </>
      ),
      review: (v) => [
        { label: t("fields.entryAt", { zone: timeZone }), value: when(v.entryDate, v.entryTime) },
        { label: t("fields.entryPrice"), value: v.entryPrice },
        { label: t("fields.exitAt", { zone: timeZone }), value: when(v.exitDate, v.exitTime) },
        { label: t("fields.exitPrice"), value: v.exitPrice },
        { label: t("fields.commission"), value: v.commission || t("commissionEstimated") },
      ],
    },
    {
      id: "excursions",
      label: t("steps.excursions"),
      optional: true,
      fields: [...stepFields.excursions],
      schema: excursionsSchema(m, env),
      content: (
        <>
          <TextField form={form} name="mae" id="mt-mae" label={t("fields.mae")} inputMode="decimal" />
          <TextField form={form} name="mfe" id="mt-mfe" label={t("fields.mfe")} inputMode="decimal" />
        </>
      ),
      review: (v) => [
        { label: t("fields.mae"), value: v.mae || t("notProvided") },
        { label: t("fields.mfe"), value: v.mfe || t("notProvided") },
      ],
    },
  ];

  function when(date: string, time: string): string {
    const instant = zonedToInstant(date, time, timeZone);
    return instant ? formatDateTime(instant, locale, timeZone, { seconds: true, zone: true }) : "";
  }

  return (
    <FormStepper
      form={form}
      steps={steps}
      submitLabel={t("save")}
      onCancel={() => router.push(path("/trades"))}
      onSubmit={() => save(t("saved"), () => router.push(path("/trades")))}
      secondarySubmit={{ label: t("saveAnother"), onSubmit: () => save(t("savedAnother"), (accountId) => onAnother(accountId)) }}
      focusOnMount={restarted}
      reviewExtra={
        <div className="grid gap-3">
          <p className="text-sm text-muted-foreground">{t("originManual", { account: accountName })}</p>
          {calc}
        </div>
      }
    />
  );
}
