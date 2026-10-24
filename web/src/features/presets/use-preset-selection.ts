"use client";

import { useLocale, useTranslations } from "next-intl";
import { useMemo, useState } from "react";
import { formatMoney } from "@/shared/format";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { usePresetVersion } from "./api/use-presets";
import { buildOutcome, emptyPresetDraft, type PresetDraft, type PresetOutcome } from "./lib/preset-selection";

/** The preset the user is choosing for a new account: the draft lives above the stepper so going back keeps it. */
export function usePresetSelection() {
  const t = useTranslations("presets.plans");
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const [draft, setDraft] = useState<PresetDraft>(emptyPresetDraft);
  const complete = draft.presetVersionId !== "";
  const version = usePresetVersion(complete ? draft.presetVersionId : undefined);
  const outcome: PresetOutcome | null = useMemo(() => {
    const data = version.data;
    if (!data) return null;
    const size = data.size_label ?? formatMoney(data.size_amount, locale);
    return buildOutcome(data, draft, locale, `${data.firm_name} · ${data.plan_name} · ${t(`phase.${data.phase}`)} · ${size}`);
  }, [version.data, draft, locale, t]);
  return { draft, setDraft, version, outcome };
}

export type PresetSelection = ReturnType<typeof usePresetSelection>;
