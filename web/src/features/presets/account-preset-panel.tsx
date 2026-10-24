"use client";

import { PencilLine } from "lucide-react";
import { useLocale, useTranslations } from "next-intl";
import { useState, type ComponentProps } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { Alert } from "@/shared/ui/alert";
import { Badge } from "@/shared/ui/badge";
import { Button } from "@/shared/ui/button";
import { ErrorState } from "@/shared/ui/error-state";
import { Card, CardContent, CardHeader, CardTitle } from "@/shared/ui/card";
import { Popover, PopoverContent, PopoverTrigger } from "@/shared/ui/popover";
import { useAccountPreset, useRestorePresetValue, type AccountPresetState } from "./api/account-preset";
import { usePresetVersion } from "./api/use-presets";
import { formatCalendarDate } from "./lib/dates";
import { useRuleValueText } from "./lib/rule-value";
import { VersionDiffDialog } from "./version-diff-dialog";

type Field = AccountPresetState["fields"][number];

function FieldRow({ field, rowVersion, accountId }: { field: Field; rowVersion: number; accountId: string }) {
  const t = useTranslations("presets.account");
  const tc = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const valueText = useRuleValueText();
  const restore = useRestorePresetValue(accountId);
  const [open, setOpen] = useState(false);
  const account = field.account_value === undefined ? null : valueText(field.field, field.account_value);
  const preset = field.preset_value === undefined ? null : valueText(field.field, field.preset_value);

  async function onRestore() {
    try {
      await restore.mutateAsync({ field: field.field, rowVersion });
      toast.success(t("restored"));
      setOpen(false);
    } catch (error) {
      toast.error(error instanceof ApiError && error.code === "common.version_conflict" ? t("conflict") : errorMessage(tc, error).message);
    }
  }

  return (
    <div className="flex flex-wrap items-center gap-x-3 gap-y-1">
      <dt className="min-w-44 text-muted-foreground">{t(`fields.${field.field}`)}</dt>
      <dd className="flex flex-wrap items-center gap-2">
        <span>{field.origin === "unresolved" ? t("unresolved") : (account ?? "—")}</span>
        {field.origin === "edited" ? (
          <Popover open={open} onOpenChange={setOpen}>
            <PopoverTrigger asChild>
              <Button type="button" variant="ghost" size="sm" className="h-7 px-1" aria-label={t("editedFor", { field: t(`fields.${field.field}`) })}>
                <Badge tone="info"><PencilLine aria-hidden="true" />{t("edited")}</Badge>
              </Button>
            </PopoverTrigger>
            <PopoverContent className="space-y-2 text-sm">
              <p>{preset === null ? t("presetValueUnknown") : t("presetValue", { value: preset })}</p>
              <Button type="button" size="sm" disabled={restore.isPending || preset === null} onClick={() => void onRestore()}>{t("restore")}</Button>
            </PopoverContent>
          </Popover>
        ) : null}
        {field.origin === "chosen_from_conflict" ? <span className="text-xs text-muted-foreground">{t("chosen", { date: field.chosen_at ? formatCalendarDate(field.chosen_at.slice(0, 10), locale) : "" })}</span> : null}
      </dd>
    </div>
  );
}

/** What the account stores from its preset, with each value's origin (S16-04), and the new-version notice (S16-05). */
export function AccountPresetPanel({ accountId, renderOptIn }: { accountId: string; renderOptIn: ComponentProps<typeof VersionDiffDialog>["renderOptIn"] }) {
  const t = useTranslations("presets.account");
  const tc = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const preset = useAccountPreset(accountId);
  const current = usePresetVersion(preset.data?.preset_version_id);
  const firm = current.data?.firm_name ?? "";
  const [review, setReview] = useState(false);
  // No preset (404): free-text prop or real account, nothing to show. Another failure is shown with a retry; the account itself still works.
  if (preset.isError && !(preset.error instanceof ApiError && preset.error.status === 404))
    return <ErrorState title={errorMessage(tc, preset.error).message} code={errorMessage(tc, preset.error).code} onRetry={() => void preset.refetch()} />;
  if (!preset.data) return null;
  const state = preset.data;
  return (
    <Card>
      <CardHeader><CardTitle>{t("title")}</CardTitle></CardHeader>
      <CardContent className="space-y-3 text-sm">
        <p className="text-muted-foreground">{t("version", { n: state.release.version_number, date: formatCalendarDate(state.release.verified_on, locale) })}</p>
        {state.newer_version ? (
          <Alert tone={state.newer_version.suspends ? "warning" : "info"}>
            <div className="space-y-2">
              <p>{t("newer", { firm, n: state.newer_version.release.version_number, date: formatCalendarDate(state.newer_version.release.published_at.slice(0, 10), locale) })}</p>
              {state.newer_version.suspends ? <p>{t("suspends")}</p> : null}
              <Button type="button" variant="outline" size="sm" onClick={() => setReview(true)}>{t("seeChanges")}</Button>
            </div>
          </Alert>
        ) : null}
        <dl className="space-y-1.5">
          {state.fields.map((field) => <FieldRow key={field.field} field={field} rowVersion={state.row_version} accountId={accountId} />)}
        </dl>
      </CardContent>
      {review && state.newer_version ? (
        <VersionDiffDialog accountId={accountId} firm={firm} targetId={state.newer_version.preset_version_id} rowVersion={state.row_version} accountPlatform={state.firm_platform ?? ""} renderOptIn={renderOptIn} onClose={() => setReview(false)} />
      ) : null}
    </Card>
  );
}
