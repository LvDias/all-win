"use client";

import { useLocale, useTranslations } from "next-intl";
import Link from "next/link";
import type { ReactNode } from "react";
import { Controller, type UseFormReturn } from "react-hook-form";
import { formatMoney } from "@/shared/format";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { withLocale } from "@/shared/i18n/paths";
import { Alert } from "@/shared/ui/alert";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { RadioGroup, RadioGroupItem } from "@/shared/ui/radio-group";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import { useAccounts } from "./api/use-accounts";
import { useConnections } from "./api/use-connections";
import { isFormErrorKey, type AccountFormValues } from "./lib/account-form";

type Form = UseFormReturn<AccountFormValues>;
const noBinding = "__none__";

/** Translates a field error: our own keys live under `accounts.form.errors`, server messages are already text. */
export function useFieldError(form: Form) {
  const t = useTranslations("accounts.form.errors");
  return (field: keyof AccountFormValues) => {
    const message = form.formState.errors[field]?.message;
    if (typeof message !== "string") return undefined;
    return isFormErrorKey(message) ? t(message) : message;
  };
}

export function Field({ id, label, help, error, children }: { id: string; label: string; help?: string; error?: string; children: ReactNode }) {
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

function TextField({ form, name, label, formId, maxLength, inputMode, help, trailing }: { form: Form; name: keyof AccountFormValues & string; label: string; formId: string; maxLength?: number; inputMode?: "numeric" | "decimal"; help?: string; trailing?: ReactNode }) {
  const error = useFieldError(form)(name);
  const id = `${formId}-${name}`;
  return (
    <Field id={id} label={label} help={help} error={error}>
      <Input id={id} maxLength={maxLength} inputMode={inputMode} aria-invalid={error ? true : undefined} aria-describedby={describedBy(id, !!help, !!error)} {...form.register(name as "displayName")} />
      {trailing}
    </Field>
  );
}

/** Prop only: firm, plan and phase (free text until the S16 preset picker exists). */
export function FirmFields({ form, formId }: { form: Form; formId: string }) {
  const t = useTranslations("accounts.form.fields");
  return (
    <>
      <TextField form={form} formId={formId} name="firmName" label={t("firm")} maxLength={80} />
      <TextField form={form} formId={formId} name="planName" label={t("plan")} maxLength={80} />
      <TextField form={form} formId={formId} name="phase" label={t("phase")} maxLength={80} />
    </>
  );
}

/** The user picks which standard size an in-between account size copies its profile from (Q60). */
export function ProfileSizeField({ form, formId, candidates }: { form: Form; formId: string; candidates: string[] }) {
  const t = useTranslations("accounts.form.profileSize");
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const error = useFieldError(form)("profileSize");
  const id = `${formId}-profileSize`;
  return (
    <fieldset className="space-y-1.5">
      <legend className="text-sm font-medium">{t("label")}</legend>
      <p id={`${id}-help`} className="text-sm text-muted-foreground">{t("help")}</p>
      <Controller
        control={form.control}
        name="profileSize"
        render={({ field }) => (
          <RadioGroup aria-describedby={describedBy(id, true, !!error)} aria-invalid={error ? true : undefined} value={field.value} onValueChange={field.onChange}>
            {candidates.map((size) => (
              <Label key={size} htmlFor={`${id}-${size}`} className="flex min-h-11 cursor-pointer items-center gap-3 rounded-lg border p-3">
                <RadioGroupItem id={`${id}-${size}`} value={size} />
                {formatMoney(size, locale)}
              </Label>
            ))}
          </RadioGroup>
        )}
      />
      {error ? <p id={`${id}-error`} className="text-sm text-destructive">{error}</p> : null}
    </fieldset>
  );
}

/** `preset`: size comes from the chosen preset (read-only) and the maximum position is capped by it (S5 AC-3). */
export function DataFields({ form, formId, editing, withFirm, profileSizeCandidates, preset }: { form: Form; formId: string; editing: boolean; withFirm: boolean; profileSizeCandidates?: string[]; preset?: { sizeText: string; maxPositionCap: number | null } }) {
  const t = useTranslations("accounts.form.fields");
  return (
    <>
      <TextField form={form} formId={formId} name="displayName" label={t("name")} maxLength={80} />
      {withFirm ? <FirmFields form={form} formId={formId} /> : null}
      {preset ? (
        <div className="space-y-1.5">
          <p className="text-sm font-medium">{t("size")}</p>
          <p className="text-sm">{preset.sizeText}</p>
          <p className="text-sm text-muted-foreground">{t("sizeFromPreset")}</p>
        </div>
      ) : (
        <TextField form={form} formId={formId} name="size" label={t("size")} inputMode="decimal" />
      )}
      {profileSizeCandidates ? <ProfileSizeField form={form} formId={formId} candidates={profileSizeCandidates} /> : null}
      <TextField form={form} formId={formId} name="maxPosition" label={t("maxPosition")} help={preset?.maxPositionCap != null ? `${t("maxPositionHelp")} ${t("maxPositionPresetCap", { n: preset.maxPositionCap })}` : t("maxPositionHelp")} inputMode="numeric" trailing={editing ? <p className="text-sm text-muted-foreground">{t("maxPositionShrink")}</p> : undefined} />
      <TextField form={form} formId={formId} name="multiplier" label={t("multiplier")} help={t("multiplierHelp")} inputMode="numeric" />
    </>
  );
}

/** The platform-account select. Options already bound to another account are disabled with the reason. */
export function BindingField({ form, formId, accountId, locked }: { form: Form; formId: string; accountId?: string; locked?: boolean }) {
  const t = useTranslations("accounts");
  const connections = useConnections();
  const accounts = useAccounts();
  const error = useFieldError(form)("binding");
  const boundByOther = new Map<string, string>();
  for (const a of accounts.data ?? []) if (a.binding && a.id !== accountId) boundByOther.set(`${a.binding.connection_id}|${a.binding.platform_account}`, a.display_name);
  const paired = (connections.data ?? []).filter((c) => c.status !== "unpaired" && c.status !== "awaiting_pairing");
  const options = paired.flatMap((c) => (c.visible_accounts ?? []).map((v) => ({ value: `${c.id}|${v.platform_account}`, conn: c, v })));
  const id = `${formId}-binding`;
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  if (paired.length === 0)
    return (
      <Alert tone="info" className="flex-wrap items-center justify-between">
        <span>{t("form.binding.noConnections")}</span>
        <Link className="underline underline-offset-4" href={withLocale(locale, "/connections/ninjatrader")}>{t("form.binding.pair")}</Link>
      </Alert>
    );
  return (
    <Field id={id} label={t("form.binding.label")} error={error}>
      <Controller
        control={form.control}
        name="binding"
        render={({ field }) => (
          <Select value={field.value === "" ? noBinding : field.value} onValueChange={(v) => field.onChange(v === noBinding ? "" : v)} disabled={locked}>
            <SelectTrigger id={id} aria-invalid={error ? true : undefined}><SelectValue /></SelectTrigger>
            <SelectContent>
              <SelectItem value={noBinding}>{t("form.binding.none")}</SelectItem>
              {options.map(({ value, conn, v }) => {
                const other = boundByOther.get(value);
                // No environment flag reaches the web: live and unknown kinds stay blocked (spec §4.3); the Core also refuses them.
                const blocked = v.kind === "live" || v.kind === "unknown";
                return (
                  <SelectItem key={value} value={value} disabled={other !== undefined || blocked}>
                    {[`${v.platform_account} · ${conn.name} (${t(`platformKind.${v.kind}`)})`, other ? t("form.binding.taken", { name: other }) : null, blocked ? t("form.binding.liveBlocked") : null].filter(Boolean).join(" — ")}
                  </SelectItem>
                );
              })}
            </SelectContent>
          </Select>
        )}
      />
      {locked ? <p className="text-sm text-muted-foreground">{t("form.binding.openPosition")}</p> : null}
    </Field>
  );
}
