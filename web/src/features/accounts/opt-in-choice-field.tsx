"use client";

import { useTranslations } from "next-intl";
import { useEffect } from "react";
import { Alert } from "@/shared/ui/alert";
import { Skeleton } from "@/shared/ui/skeleton";
import { useRuleVersion } from "./api/use-accounts";
import type { OptInKind } from "./api/types";
import { OptInSection, type OptInValue } from "./opt-in-section";

export type { OptInValue };

type Props = {
  kind: OptInKind;
  /** The rule version the caller will send as `rule_version_id` (the text shown here is exactly that version's). */
  ruleVersionId: string;
  value: OptInValue;
  onChange: (value: OptInValue) => void;
  /** Reported once the rule is known: `false` when the firm allows it, so no choice is asked or sent. */
  onRequiredChange?: (required: boolean) => void;
  choiceError?: string;
  ackError?: string;
  firmFallback: string;
};

/**
 * One opt-in question for a rule version id, for other features to compose (the preset version dialog, S16-05).
 * Loads the rule and shows the S5 section; nothing is preselected.
 */
export function OptInChoiceField({ kind, ruleVersionId, value, onChange, onRequiredChange, choiceError, ackError, firmFallback }: Props) {
  const t = useTranslations("accounts.optIn");
  const rule = useRuleVersion(ruleVersionId);
  const required = rule.data ? rule.data.firm_stance !== "allowed" : undefined;
  useEffect(() => {
    if (required !== undefined) onRequiredChange?.(required);
  }, [required, onRequiredChange]);
  if (rule.isError) return <Alert tone="danger">{t("rulesError")}</Alert>;
  if (!rule.data) return <Skeleton className="h-40 w-full" />;
  return <OptInSection kind={kind} rule={rule.data} value={value} onChange={onChange} choiceError={choiceError} ackError={ackError} firmFallback={firmFallback} />;
}
