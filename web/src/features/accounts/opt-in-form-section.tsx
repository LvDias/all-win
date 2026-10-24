"use client";

import { useTranslations } from "next-intl";
import { useWatch, type UseFormReturn } from "react-hook-form";
import type { RuleVersion } from "./api/types";
import { useFieldError } from "./account-fields";
import type { AccountFormValues } from "./lib/account-form";
import { OptInSection } from "./opt-in-section";

/**
 * The automation opt-in bound to the form (fields `automationChoice`, `automationAck`, `automationRuleId`).
 * A choice stored for another rule version than the one shown reads as empty (spec §5.2) and shows a notice.
 */
export function OptInFormSection({ form, rule, staleNotice }: { form: UseFormReturn<AccountFormValues>; rule: RuleVersion; staleNotice: boolean }) {
  const t = useTranslations("accounts.optIn");
  const fieldError = useFieldError(form);
  const [choice, ack, ruleId, typedFirm] = useWatch({ control: form.control, name: ["automationChoice", "automationAck", "automationRuleId", "firmName"] });
  const stale = ruleId !== "" && ruleId !== rule.id;
  return (
    <OptInSection
      kind="automation"
      rule={rule}
      value={stale ? { choice: "", ack: false } : { choice, ack }}
      onChange={(v) => {
        form.setValue("automationChoice", v.choice, { shouldDirty: true });
        form.setValue("automationAck", v.ack, { shouldDirty: true });
        form.setValue("automationRuleId", rule.id, { shouldDirty: true });
        form.clearErrors(["automationChoice", "automationAck"]);
      }}
      choiceError={fieldError("automationChoice")}
      ackError={fieldError("automationAck")}
      notice={staleNotice ? t("ruleStale") : stale ? t("ruleChanged") : undefined}
      // The generic rule names no firm: the sentence uses the one the user typed ("da Apex"); "esta firma" only if none was typed.
      firmFallback={typedFirm.trim() !== "" ? typedFirm.trim() : t("automation.genericFirm")}
    />
  );
}
