"use client";

import { AccountCreateStepper } from "@/features/accounts";
import { PresetPicker, usePresetSelection } from "@/features/presets";

// Features never import each other: the page composes "Nova conta" with the preset picker. The picker's draft lives
// here, above the stepper, so going back a step keeps the choices.
export function NewAccountWithPresets() {
  const selection = usePresetSelection();
  return (
    <AccountCreateStepper
      preset={{
        outcome: selection.outcome,
        render: ({ switchToManual, error }) => <PresetPicker selection={selection} onManual={switchToManual} error={error} />,
      }}
    />
  );
}
