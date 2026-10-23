import type { AccountKind } from "../api/types";
import type { AccountFormValues } from "./account-form";

export type StepId = "type" | "firm" | "data" | "platform" | "optIn" | "review";

type FieldName = keyof AccountFormValues;

/**
 * Fields owned by each step (F§10.1: a 422 with a pointer goes back to the step that owns the field).
 * Assumption until S5 rev. 4 is published: phase travels with firm and plan ("Firma e plano").
 */
export const stepFields: Record<StepId, FieldName[]> = {
  type: [],
  firm: ["firmName", "planName", "phase"],
  data: ["displayName", "size", "maxPosition", "multiplier"],
  platform: ["binding"],
  optIn: ["automationChoice", "automationAck"],
  review: [],
};

/** Steps that do not apply are left out of the list (F§10.1); `review` is always last. */
export function stepsFor(kind: AccountKind | null, optInApplies: boolean): StepId[] {
  if (kind === null) return ["type"];
  return [
    "type",
    ...(kind === "prop" ? (["firm"] as const) : []),
    "data",
    "platform",
    ...(kind === "prop" && optInApplies ? (["optIn"] as const) : []),
    "review",
  ];
}

export function stepOfField(field: FieldName | null): StepId | null {
  if (field === null) return null;
  return (Object.keys(stepFields) as StepId[]).find((s) => stepFields[s].includes(field)) ?? null;
}

/** "Continuar" validates only the current step: keep the issues whose field belongs to it. */
export function issuesForStep(issues: { path: PropertyKey[]; message: string }[], step: StepId): Partial<Record<FieldName, string>> {
  const out: Partial<Record<FieldName, string>> = {};
  for (const issue of issues) {
    const field = issue.path[0] as FieldName;
    if (stepFields[step].includes(field)) out[field] ??= issue.message;
  }
  return out;
}

/** First step (in list order) that owns an error, so a failed review/submit returns to it. */
export function firstStepWithErrors(steps: StepId[], errors: Partial<Record<FieldName, string>>): StepId | null {
  return steps.find((s) => stepFields[s].some((f) => errors[f] !== undefined)) ?? null;
}
