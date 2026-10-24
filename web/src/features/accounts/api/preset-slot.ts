import type { ReactNode } from "react";

/**
 * What "Nova conta" needs from a chosen firm preset (S16). Defined here, not imported: features do not import each
 * other, so the page that composes both passes an object of this shape.
 */
export type PresetOutcome = {
  presetVersionId: string;
  firmPlatform: string | null;
  /** "Firm · Plan · Phase · Size", for the review step. */
  label: string;
  /** The preset's size as the Core publishes it (decimal string). */
  sizeAmount: string;
  /** The account's maximum position may not exceed this (S5 AC-3). */
  maxPositionCap: number | null;
  /** Prefill for the maximum position. */
  maxPositionDefault: number | null;
  /** The user's own values where they differ from the preset or resolve a not-confirmed field (`rule_values`). */
  ruleValues: Record<string, unknown>;
  /** False while a required rule is unresolved (S16 AC-6). */
  valid: boolean;
};

export type PresetSlot = {
  outcome: PresetOutcome | null;
  /** The picker. `switchToManual` switches the form to the free-text path (Q40). */
  render: (actions: { switchToManual: () => void; error?: string }) => ReactNode;
};
