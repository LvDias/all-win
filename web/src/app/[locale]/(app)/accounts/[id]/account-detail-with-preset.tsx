"use client";

import { AccountDetail, OptInChoiceField } from "@/features/accounts";
import { AccountPresetPanel, type RenderOptIn } from "@/features/presets";

// Features never import each other: the page composes the account detail with the preset panel, and lends the panel
// the S5 opt-in question for its version dialog.
const renderOptIn: RenderOptIn = ({ kind, ruleVersionId, value, onChange, onRequiredChange, choiceError, ackError }) => (
  <OptInChoiceField kind={kind} ruleVersionId={ruleVersionId} value={value} onChange={onChange} onRequiredChange={onRequiredChange} choiceError={choiceError} ackError={ackError} firmFallback="" />
);

export function AccountDetailWithPreset({ accountId }: { accountId: string }) {
  return <AccountDetail accountId={accountId} presetSection={<AccountPresetPanel accountId={accountId} renderOptIn={renderOptIn} />} />;
}
