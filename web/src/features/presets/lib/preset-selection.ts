import type { Locale } from "@/shared/i18n/config";
import { otherValue } from "@/shared/components/conflict-picker";
import type { RuleField } from "../api/account-preset";
import type { PresetVersion } from "../api/types";
import { parseOther, type OtherDraft } from "../other-value";

/** The user's answer for one rule field. `selected`: "" (none), the index of a published value, or `otherValue`. */
export type Resolution = { selected: string; other: OtherDraft };
export type PresetDraft = {
  firmId: string;
  planId: string;
  phase: string;
  presetVersionId: string;
  firmPlatform: string;
  /** Not-confirmed fields: the choice. Verified fields: `selected: otherValue` means the user edited the value (`Editado`). */
  resolutions: Partial<Record<RuleField, Resolution>>;
};

export const emptyPresetDraft: PresetDraft = { firmId: "", planId: "", phase: "", presetVersionId: "", firmPlatform: "", resolutions: {} };

/** S16 AC-6: a preset cannot be saved while one of these is unresolved. */
export const requiredFields: readonly RuleField[] = ["max_position_qty", "drawdown", "daily_loss_limit", "trading_day_reset"];
const ruleFields: readonly RuleField[] = ["max_position_qty", "drawdown", "daily_loss_limit", "daily_profit_target", "trading_day_reset", "consistency_rule", "commission_per_contract_side", "evaluation_profit_target"];

type RuleLike = { status: "verified" | "not_confirmed"; value?: unknown; published_values?: { value: unknown }[] };
const ruleOf = (version: PresetVersion, platform: string, field: RuleField): RuleLike | undefined =>
  (version.rule_sets.find((set) => (set.firm_platform ?? "") === platform) ?? version.rule_sets[0])?.rules[field] as RuleLike | undefined;

export type FieldProblem = "unresolved" | "required" | "invalid";

/** What the account form sends for the chosen preset (`createAccount`): ids, the user's values, the cap. No money arithmetic. */
export type PresetOutcome = {
  presetVersionId: string;
  firmPlatform: string | null;
  label: string;
  sizeAmount: string;
  /** Highest maximum position the account may use (the preset's; for a not-confirmed one, the highest published value, or the typed one). */
  maxPositionCap: number | null;
  /** The value to prefill the account's maximum position with (the preset's verified one, or the one the user resolved). */
  maxPositionDefault: number | null;
  ruleValues: Record<string, unknown>;
  valid: boolean;
  problems: Partial<Record<RuleField, FieldProblem>>;
};

/** Pure: turns the loaded preset version and the user's draft into the outcome the form needs. */
export function buildOutcome(version: PresetVersion, draft: PresetDraft, locale: Locale, label: string): PresetOutcome {
  const ruleValues: Record<string, unknown> = {};
  const problems: PresetOutcome["problems"] = {};
  let cap: number | null = null;
  let defaultQty: number | null = null;
  for (const field of ruleFields) {
    const rule = ruleOf(version, draft.firmPlatform, field);
    if (!rule) continue;
    const resolution = draft.resolutions[field];
    const required = requiredFields.includes(field);
    if (rule.status === "verified") {
      if (field === "max_position_qty") {
        cap = defaultQty = (rule.value as { contracts: number }).contracts;
        continue;
      }
      if (resolution?.selected === otherValue) {
        const parsed = parseOther(field, resolution.other, locale);
        if (parsed.ok) ruleValues[field] = parsed.value;
        else problems[field] = parsed.reason;
      }
      continue;
    }
    const published = rule.published_values ?? [];
    if (!resolution || resolution.selected === "") {
      if (required) problems[field] = "unresolved";
      continue;
    }
    if (resolution.selected === otherValue) {
      const parsed = parseOther(field, resolution.other, locale);
      if (!parsed.ok) problems[field] = parsed.reason;
      else if (field === "max_position_qty") cap = defaultQty = (parsed.value as { contracts: number }).contracts;
      else ruleValues[field] = parsed.value;
      continue;
    }
    const chosen = published[Number(resolution.selected)]?.value;
    if (chosen === undefined) {
      problems[field] = "invalid";
      continue;
    }
    if (field === "max_position_qty") {
      defaultQty = (chosen as { contracts: number }).contracts;
      cap = Math.max(...published.map((entry) => (entry.value as { contracts: number }).contracts));
    } else ruleValues[field] = chosen;
  }
  // A not-confirmed maximum position: any value up to the highest published one is allowed (contract); a typed one is its own cap.
  const maxRule = ruleOf(version, draft.firmPlatform, "max_position_qty");
  if (maxRule?.status === "not_confirmed" && draft.resolutions.max_position_qty?.selected !== otherValue && cap !== null) cap = Math.max(...(maxRule.published_values ?? []).map((entry) => (entry.value as { contracts: number }).contracts));
  return {
    presetVersionId: version.id,
    firmPlatform: draft.firmPlatform === "" ? null : draft.firmPlatform,
    label,
    sizeAmount: version.size_amount,
    maxPositionCap: cap,
    maxPositionDefault: defaultQty,
    ruleValues,
    valid: Object.keys(problems).length === 0,
    problems,
  };
}
