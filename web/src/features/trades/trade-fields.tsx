"use client";

import { useTranslations } from "next-intl";
import { useId, type ReactNode } from "react";
import type { FieldPath, useForm } from "react-hook-form";
import type { ManualTradePreview } from "@/shared/api/types";
import { formatMoney } from "@/shared/format";
import { useDisplay } from "@/shared/session/display";
import { Alert } from "@/shared/ui/alert";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { Skeleton } from "@/shared/ui/skeleton";
import type { ManualTradeValues } from "./manual-trade-model";
import { PnlText, formatDuration } from "./trade-parts";

export type FormApi = ReturnType<typeof useForm<ManualTradeValues>>;

export function Calculated({ preview, pending, failed, ready }: { preview: ManualTradePreview | undefined; pending: boolean; failed: boolean; ready: boolean }) {
  const t = useTranslations("trades.manual.calculated");
  const { locale } = useDisplay();
  if (!ready) return <p className="text-sm text-muted-foreground">{t("waiting")}</p>;
  if (failed) return <Alert tone="info">{t("failed")}</Alert>;
  if (pending || !preview) return <Skeleton className="h-24 w-full" aria-label={t("loading")} />;
  return (
    <section aria-label={t("title")} className="grid gap-2 rounded-lg border bg-muted p-3 text-sm">
      <h3 className="font-semibold">{t("title")}</h3>
      <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1">
        <dt className="text-muted-foreground">{t("net")}</dt>
        <dd className="text-base font-semibold">
          <PnlText value={preview.pnl} locale={locale} />
        </dd>
        <dt className="text-muted-foreground">{t("gross")}</dt>
        <dd>
          <PnlText value={preview.gross_pnl} locale={locale} />
        </dd>
        <dt className="text-muted-foreground">{t("commission")}</dt>
        <dd>
          <span className="tabular-nums">{formatMoney(preview.commission, locale)}</span>{" · "}{t(`source.${preview.commission_source}`)}
        </dd>
        <dt className="text-muted-foreground">{t("duration")}</dt>
        <dd>{formatDuration(preview.duration_seconds)}</dd>
      </dl>
      {preview.commission_source === "none" ? <Alert tone="warning">{t("none")}</Alert> : null}
      {preview.commission_source === "configured" ? <p className="text-xs text-muted-foreground">{t("estimatedHelp")}</p> : null}
      <p className="text-xs text-muted-foreground">{t("help")}</p>
    </section>
  );
}

export function Field({ id, label, error, help, children }: { id: string; label: string; error?: string; help?: string; children: ReactNode }) {
  return (
    <div className="grid gap-2">
      <Label htmlFor={id}>{label}</Label>
      {children}
      {help ? <p className="text-xs text-muted-foreground">{help}</p> : null}
      {error ? (
        <p id={`${id}-error`} className="text-sm text-status-danger">
          {error}
        </p>
      ) : null}
    </div>
  );
}

export function TextField({ form, name, id, label, inputMode, help }: { form: FormApi; name: FieldPath<ManualTradeValues>; id: string; label: string; inputMode?: "numeric" | "decimal"; help?: string }) {
  const error = form.formState.errors[name]?.message;
  const { ref, ...register } = form.register(name);
  return (
    <Field id={id} label={label} error={typeof error === "string" ? error : undefined} help={help}>
      <Input id={id} ref={ref} {...register} inputMode={inputMode} autoComplete="off" aria-invalid={error ? true : undefined} aria-describedby={error ? `${id}-error` : undefined} />
    </Field>
  );
}

/** A date and a time, typed in the display zone (the zone is in the label). Both native inputs are fully keyboard operable. */
export function DateTime({ form, idPrefix, date, time, label }: { form: FormApi; idPrefix: string; date: "entryDate" | "exitDate"; time: "entryTime" | "exitTime"; label: string }) {
  const groupId = useId();
  const error = form.formState.errors[date]?.message ?? form.formState.errors[time]?.message;
  const dateRegister = form.register(date);
  const timeRegister = form.register(time);
  return (
    <fieldset className="grid gap-2" aria-describedby={error ? `${groupId}-error` : undefined}>
      <legend className="mb-1 text-sm font-medium">{label}</legend>
      <div className="flex flex-wrap gap-3">
        <Input id={`${idPrefix}-date`} type="date" aria-label={`${label} (date)`} {...dateRegister} aria-invalid={error ? true : undefined} className="w-auto" />
        <Input id={`${idPrefix}-time`} type="time" step={1} aria-label={`${label} (time)`} {...timeRegister} aria-invalid={error ? true : undefined} className="w-auto" />
      </div>
      {typeof error === "string" ? (
        <p id={`${groupId}-error`} className="text-sm text-status-danger">
          {error}
        </p>
      ) : null}
    </fieldset>
  );
}
