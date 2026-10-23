"use client";

import { useQuery } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useMemo, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { PageHeader } from "@/shared/components/page-header";
import { instantToZoned } from "@/shared/format";
import { useLocalePath } from "@/shared/i18n/navigation";
import { useUnsavedChanges } from "@/shared/i18n/locale-switch";
import { focusOnMount } from "@/shared/lib/focus";
import { useDebounced } from "@/shared/lib/use-debounced";
import { useDisplay } from "@/shared/session/display";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/shared/ui/card";
import { ErrorState } from "@/shared/ui/error-state";
import { RadioGroup, RadioGroupItem } from "@/shared/ui/radio-group";
import { Label } from "@/shared/ui/label";
import { Skeleton } from "@/shared/ui/skeleton";
import { Spinner } from "@/shared/ui/spinner";
import { previewManualTrade, useInstruments, useTrade, useUpdateManualTrade } from "./api";
import { excursionsSchema, pointerToField, timesSchema, toRequest, tradeSchema, type ManualTradeValues } from "./manual-trade-model";
import { Calculated, DateTime, TextField } from "./trade-fields";
import type { Trade } from "@/shared/api/types";

/** S21-04: edit a manual trade on one page with sections (F§10.1: an edit is not a stepper). Captured trades never get here. */
export function ManualTradeEdit({ tradeId }: { tradeId: string }) {
  const t = useTranslations("trades.edit");
  const root = useTranslations();
  const trade = useTrade(tradeId);
  const instruments = useInstruments();
  const path = useLocalePath();
  const header = <PageHeader title={t("title")} />;
  if (trade.isPending || instruments.isPending)
    return (
      <div className="grid max-w-180 gap-4" aria-busy="true">
        {header}
        <Skeleton className="h-40 w-full" />
      </div>
    );
  if (trade.isError || instruments.isError) {
    const error = trade.error ?? instruments.error;
    const notFound = error instanceof ApiError && error.status === 404;
    return (
      <div className="grid max-w-180 gap-4">
        {header}
        <ErrorState title={notFound ? root("errors.common.not_found") : t("loadError")} code={errorMessage(root, error).code} onRetry={notFound ? undefined : () => void trade.refetch()} />
      </div>
    );
  }
  if (!trade.data.editable || trade.data.row_version === undefined)
    return (
      <div className="grid max-w-180 gap-4">
        {header}
        <Alert tone="info">{t("notEditable")}</Alert>
        <Link href={path("/trades")} className="text-sm underline underline-offset-4">
          {t("back")}
        </Link>
      </div>
    );
  const instrument = instruments.data.instruments.find((item) => item.id === trade.data.instrument_id);
  return <Editor key={trade.data.row_version} trade={trade.data} rowVersion={trade.data.row_version} tickSize={instrument?.tick_size} />;
}

function Editor({ trade, rowVersion, tickSize }: { trade: Trade; rowVersion: number; tickSize: string | undefined }) {
  const t = useTranslations("trades.manual");
  const e = useTranslations("trades.edit");
  const root = useTranslations();
  const router = useRouter();
  const path = useLocalePath();
  const { locale, timeZone } = useDisplay();
  const update = useUpdateManualTrade(trade.trade_id);
  const entry = instantToZoned(trade.entry_time, timeZone);
  const exit = instantToZoned(trade.exit_time, timeZone);
  const initial: ManualTradeValues = {
    accountId: trade.account_id,
    direction: trade.direction,
    quantity: String(trade.quantity),
    entryDate: entry.date,
    entryTime: entry.time,
    entryPrice: trade.entry_price,
    exitDate: exit.date,
    exitTime: exit.time,
    exitPrice: trade.exit_price,
    // The stored commission is shown only when the user typed it; an estimated one stays empty so the Core recomputes it.
    commission: trade.commission_source === "manual" ? trade.commission : "",
    mae: trade.mae ?? "",
    mfe: trade.mfe ?? "",
  };
  const form = useForm<ManualTradeValues>({ defaultValues: initial });
  const values = form.watch();
  const [clock] = useState(() => ({ now: () => Date.now() }));
  const env = useMemo(() => ({ locale, timeZone, tick: tickSize, now: clock.now }), [locale, timeZone, tickSize, clock]);
  const m = (key: string) => t(key);
  const [failure, setFailure] = useState<{ message: string; reload: boolean } | null>(null);
  useUnsavedChanges(form.formState.isDirty);

  const request = toRequest(values, trade.instrument_id, env);
  const settled = useDebounced(request ? JSON.stringify(request) : "", 300);
  const preview = useQuery({
    queryKey: ["trades", "preview", settled],
    enabled: settled !== "",
    queryFn: ({ signal }) => {
      const body = JSON.parse(settled) as NonNullable<typeof request>;
      return previewManualTrade({ account_id: body.account_id, instrument_id: body.instrument_id, direction: body.direction, quantity: body.quantity, entry_time: body.entry_time, entry_price: body.entry_price, exit_time: body.exit_time, exit_price: body.exit_price, commission: body.commission }, signal);
    },
  });

  async function onSave() {
    setFailure(null);
    form.clearErrors();
    const current = form.getValues();
    let invalid = false;
    for (const [schema, fields] of [
      [tradeSchema(m), ["accountId", "direction", "quantity"]],
      [timesSchema(m, env), ["entryDate", "entryTime", "entryPrice", "exitDate", "exitTime", "exitPrice", "commission"]],
      [excursionsSchema(m, env), ["mae", "mfe"]],
    ] as const) {
      const result = schema.safeParse(Object.fromEntries(fields.map((field) => [field, current[field]])));
      if (result.success) continue;
      invalid = true;
      for (const issue of result.error.issues) {
        const name = issue.path.join(".") as keyof ManualTradeValues;
        if (!form.formState.errors[name]) form.setError(name, { type: "validate", message: issue.message });
      }
    }
    if (invalid) {
      const first = (Object.keys(form.formState.errors) as (keyof ManualTradeValues)[])[0];
      if (first) form.setFocus(first);
      return;
    }
    const body = toRequest(current, trade.instrument_id, env);
    if (!body) return setFailure({ message: t("errors.cannotSave"), reload: false });
    try {
      await update.mutateAsync({ ...body, row_version: rowVersion });
      toast.success(e("saved"));
      router.push(path("/trades"));
    } catch (error) {
      if (error instanceof ApiError && error.code === "common.version_conflict") return setFailure({ message: e("conflict"), reload: true });
      const field = error instanceof ApiError && error.pointer ? pointerToField[error.pointer] : undefined;
      if (field) {
        form.setError(field, { type: "server", message: errorMessage(root, error).message });
        form.setFocus(field);
      } else setFailure({ message: errorMessage(root, error).message, reload: false });
    }
  }

  return (
    <form
      onSubmit={(event) => {
        event.preventDefault();
        void onSave();
      }}
      noValidate
      className="grid max-w-180 gap-4"
    >
      <PageHeader title={e("title")} description={e("description", { account: trade.account_name })} />
      <Alert tone="info">{t("note")}</Alert>
      {failure ? (
        <Alert tone="danger" tabIndex={-1} ref={focusOnMount}>
          <div className="grid gap-2">
            <p>{failure.message}</p>
            {failure.reload ? (
              <Button type="button" size="sm" variant="outline" onClick={() => router.refresh()}>
                {e("reload")}
              </Button>
            ) : null}
          </div>
        </Alert>
      ) : null}

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("steps.trade")}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <p className="text-sm text-muted-foreground">
            {t("fields.account")}{": "}{trade.account_name}
            {" · "}
            {t("fields.instrument")}{": "}{trade.instrument_symbol}
          </p>
          <Controller
            control={form.control}
            name="direction"
            render={({ field, fieldState }) => (
              <fieldset className="grid gap-2">
                <legend className="mb-1 text-sm font-medium">{t("fields.direction")}</legend>
                <RadioGroup value={field.value} onValueChange={field.onChange}>
                  {(["long", "short"] as const).map((direction) => (
                    <Label key={direction} className="flex min-h-11 items-center gap-3 rounded-md border p-3">
                      <RadioGroupItem value={direction} />
                      {t(`direction.${direction}`)}
                    </Label>
                  ))}
                </RadioGroup>
                {fieldState.error ? <p className="text-sm text-status-danger">{fieldState.error.message}</p> : null}
              </fieldset>
            )}
          />
          <TextField form={form} name="quantity" id="me-quantity" label={t("fields.quantity")} inputMode="numeric" />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("steps.times")}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <DateTime form={form} idPrefix="me-entry" date="entryDate" time="entryTime" label={t("fields.entryAt", { zone: timeZone })} />
          <TextField form={form} name="entryPrice" id="me-entry-price" label={t("fields.entryPrice")} inputMode="decimal" />
          <DateTime form={form} idPrefix="me-exit" date="exitDate" time="exitTime" label={t("fields.exitAt", { zone: timeZone })} />
          <TextField form={form} name="exitPrice" id="me-exit-price" label={t("fields.exitPrice")} inputMode="decimal" />
          <TextField form={form} name="commission" id="me-commission" label={t("fields.commission")} inputMode="decimal" help={t("commissionHelp")} />
          <Calculated preview={preview.data} pending={preview.isFetching && settled !== ""} failed={preview.isError} ready={settled !== ""} />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("steps.excursions")}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <TextField form={form} name="mae" id="me-mae" label={t("fields.mae")} inputMode="decimal" />
          <TextField form={form} name="mfe" id="me-mfe" label={t("fields.mfe")} inputMode="decimal" />
        </CardContent>
      </Card>

      <div className="sticky bottom-0 flex flex-wrap justify-end gap-2 border-t bg-background p-3">
        <Button asChild variant="outline">
          <Link href={path("/trades")}>{root("common.actions.cancel")}</Link>
        </Button>
        <Button type="submit" disabled={update.isPending}>
          {update.isPending ? <Spinner /> : null}
          {e("save")}
        </Button>
      </div>
    </form>
  );
}
