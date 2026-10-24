"use client";

import { useQueryClient } from "@tanstack/react-query";
import { useLocale, useTranslations } from "next-intl";
import { useState } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { StepUpCancelled } from "@/shared/session/step-up";
import { errorMessage } from "@/shared/api/error-message";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { Skeleton } from "@/shared/ui/skeleton";
import { accountKeys } from "./api/accounts-api";
import { useRuleVersion, useSetOptIn } from "./api/use-accounts";
import type { Account, OptInKind } from "./api/types";
import { OptInSection, type OptInValue } from "./opt-in-section";
import { useStepUpRunner, type StepUpRunner } from "./lib/use-step-up";

/**
 * Change or review one opt-in on an existing account (spec §6, S5-05). Nothing is preselected, not even the
 * previous choice, which is shown as text only. `Save` calls setAccountOptIn; `accepted` goes through step-up.
 */
export function OptInReviewDialog({ account, kind, onClose, stepUp }: { account: Account; kind: OptInKind; onClose: () => void; stepUp?: StepUpRunner }) {
  const t = useTranslations();
  const locale = useLocale();
  const state = account.opt_ins[kind];
  const rule = useRuleVersion(state.current_rule_version_id);
  const previous = useRuleVersion(rule.data?.change?.previous_rule_version_id);
  const setOptIn = useSetOptIn(account.id);
  const queryClient = useQueryClient();
  const interim = useStepUpRunner();
  const run = stepUp ?? interim;
  const [value, setValue] = useState<OptInValue>({ choice: "", ack: false });
  const [errors, setErrors] = useState<{ choice?: string; ack?: string }>({});
  const [stale, setStale] = useState(false);
  const [failure, setFailure] = useState<string | null>(null);

  async function save() {
    if (!rule.data) return;
    if (value.choice === "") return setErrors({ choice: t(`accounts.optIn.${kind}.required`) });
    if (value.choice === "accepted" && !value.ack) return setErrors({ ack: t("accounts.form.errors.ackRequired") });
    setErrors({});
    setFailure(null);
    const choice = value.choice;
    const ruleVersionId = rule.data.id;
    try {
      if (choice === "accepted") await run({ action: "account_opt_in_accept", target: account.id, description: t("accounts.form.stepUpDescriptionEdit"), send: (proof) => setOptIn.mutateAsync({ kind, choice, ruleVersionId, stepUp: proof }) });
      else await setOptIn.mutateAsync({ kind, choice, ruleVersionId });
      toast.success(t("accounts.form.success.saved"));
      onClose();
    } catch (error) {
      if (error instanceof StepUpCancelled) return;
      if (error instanceof ApiError && error.code === "accounts.rule_version_changed") {
        setStale(true);
        setValue({ choice: "", ack: false });
        // The rule shown is keyed on the account's current version: refresh the account so the dialog loads the new one.
        void queryClient.invalidateQueries({ queryKey: accountKeys.detail(account.id) });
        void queryClient.invalidateQueries({ queryKey: accountKeys.list() });
        return;
      }
      setFailure(errorMessage(t, error).message);
    }
  }

  const change = rule.data?.change;
  const previousChoice = state.choice ? t(`accounts.optIn.review.${state.choice}`) : null;

  return (
    <Dialog open onOpenChange={(open) => (open ? undefined : onClose())}>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>{t("accounts.optIn.review.title")}</DialogTitle>
          <DialogDescription>{account.display_name}</DialogDescription>
        </DialogHeader>
        {rule.isPending ? <Skeleton className="h-32 w-full" /> : null}
        {rule.isError ? <Alert tone="danger">{t("accounts.optIn.rulesError")}</Alert> : null}
        {rule.data ? (
          <>
            {previousChoice ? <p className="text-sm">{t("accounts.optIn.review.previousChoice", { choice: previousChoice })}</p> : null}
            {change?.cause === "text_changed" && change.reason ? (
              <p className="text-sm">{t("accounts.detail.whatChanged.text", { reason: change.reason[locale === "en" ? "en" : "pt-BR"] })}</p>
            ) : null}
            <OptInSection
              kind={kind}
              rule={rule.data}
              value={value}
              onChange={(v) => {
                setStale(false);
                setErrors({});
                setValue(v);
              }}
              choiceError={errors.choice}
              ackError={errors.ack}
              notice={stale ? t("accounts.optIn.ruleStale") : undefined}
              previous={previous.data ? { rule: previous.data, label: t("accounts.optIn.review.previousText", { n: previous.data.version_number }) } : undefined}
              firmFallback={account.firm_name ?? t("accounts.optIn.automation.genericFirm")}
            />
          </>
        ) : null}
        {failure ? <Alert tone="danger">{failure}</Alert> : null}
        <DialogFooter>
          <Button type="button" variant="outline" onClick={onClose}>{t("common.actions.cancel")}</Button>
          <Button type="button" disabled={!rule.data || setOptIn.isPending} onClick={() => void save()}>
            {setOptIn.isPending ? t("common.states.saving") : t("accounts.form.save")}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
