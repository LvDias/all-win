"use client";

import { AccountFormPage } from "@/features/accounts";
import { PresetPicker, usePresetSelection } from "@/features/presets";

// Features never import each other: the page composes the edit form with the preset picker. The picker's draft lives
// here, above the form, so switching between "keep", "pick another" and "free text" keeps what was chosen.
export function EditAccountWithPresets({ accountId }: { accountId: string }) {
  const selection = usePresetSelection();
  return (
    <AccountFormPage
      accountId={accountId}
      preset={{
        outcome: selection.outcome,
        render: ({ switchToManual, error }) => <PresetPicker selection={selection} onManual={switchToManual} error={error} />,
      }}
    />
  );
}
