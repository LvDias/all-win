import { z } from "zod";
import type { Locale } from "@/shared/i18n/config";
import { parseDecimalInput } from "@/shared/format";
import type { AccountCreateBody, AccountKind, AccountUpdateBody, OptInChoice, OptInChoices } from "../api/types";

/** Messages are i18n keys under `accounts.form.errors.*`; the component translates them. */
export const formErrorKeys = {
  required: "required",
  tooLong: "nameTooLong",
  size: "sizeInvalid",
  integer: "integerMin1",
  maxPosition: "maxPositionInvalid",
  optInRequired: "optInRequired",
  ackRequired: "ackRequired",
} as const;

const wholeNumber = (value: string) => /^[0-9]{1,5}$/.test(value.trim()) && Number(value) >= 1;
const text = z.string().trim().max(80, formErrorKeys.tooLong);
const requiredText = z.string().trim().min(1, formErrorKeys.required).max(80, formErrorKeys.tooLong);
type Fail = (path: string, message: string) => void;

function refineData(v: { size: string; maxPosition: string; multiplier: string }, locale: Locale, fail: Fail) {
  const size = parseDecimalInput(v.size, locale, { scale: 2 });
  if (!size.ok || Number(size.value) <= 0) fail("size", formErrorKeys.size);
  if (!wholeNumber(v.maxPosition)) fail("maxPosition", formErrorKeys.maxPosition);
  if (v.multiplier.trim() !== "" && !wholeNumber(v.multiplier)) fail("multiplier", formErrorKeys.integer);
}

function refineOptIn(v: { automationChoice: string; automationAck: boolean }, fail: Fail) {
  if (v.automationChoice === "") fail("automationChoice", formErrorKeys.optInRequired);
  else if (v.automationChoice === "accepted" && !v.automationAck) fail("automationAck", formErrorKeys.ackRequired);
}

const issueFailer = (ctx: z.RefinementCtx): Fail => (path, message) => ctx.addIssue({ code: "custom", path: [path], message });

/** One schema per step of "Nova conta"; `Continue` validates only the current step (F§10.1). */
export function stepSchemas(locale: Locale, current: { automationRuleId: string | undefined }) {
  return {
    type: z.object({ kind: z.enum(["prop", "real"], { message: formErrorKeys.required }) }),
    firm: z.object({ firmName: requiredText, planName: requiredText, phase: requiredText }),
    data: z
      .object({ displayName: requiredText, size: z.string(), maxPosition: z.string(), multiplier: z.string() })
      .superRefine((v, ctx) => refineData(v, locale, issueFailer(ctx))),
    platform: z.object({ binding: z.string() }),
    optIn: z
      .object({ automationChoice: z.enum(["accepted", "declined", ""]), automationAck: z.boolean(), automationRuleId: z.string() })
      .superRefine((v, ctx) => {
        const fail = issueFailer(ctx);
        // A choice made against another rule version than the one now shown is not a choice (spec §5.2).
        if (v.automationChoice !== "" && v.automationRuleId !== current.automationRuleId) fail("automationChoice", formErrorKeys.optInRequired);
        else refineOptIn(v, fail);
      }),
  };
}

/** Whole-form schema, used by the edit page (sections, not steps). */
export function accountFormSchema(locale: Locale, opts: { kind: AccountKind; optInRequired: boolean }) {
  return z
    .object({
      kind: z.enum(["prop", "real", ""]),
      displayName: requiredText,
      firmName: text,
      planName: text,
      phase: text,
      size: z.string(),
      maxPosition: z.string(),
      multiplier: z.string(),
      /** `"<connection_id>|<platform_account>"` or empty for unbound. */
      binding: z.string(),
      automationChoice: z.enum(["accepted", "declined", ""]),
      automationAck: z.boolean(),
      /** Rule version the choice was made against; compared with the version now shown. */
      automationRuleId: z.string(),
    })
    .superRefine((v, ctx) => {
      const fail = issueFailer(ctx);
      if (opts.kind === "prop") {
        if (v.firmName === "") fail("firmName", formErrorKeys.required);
        if (v.planName === "") fail("planName", formErrorKeys.required);
        if (v.phase === "") fail("phase", formErrorKeys.required);
      }
      refineData(v, locale, fail);
      if (opts.kind === "prop" && opts.optInRequired) refineOptIn(v, fail);
    });
}

export type AccountFormValues = z.input<ReturnType<typeof accountFormSchema>>;

export const emptyAccountForm: AccountFormValues = {
  kind: "",
  displayName: "",
  firmName: "",
  planName: "",
  phase: "",
  size: "",
  maxPosition: "",
  multiplier: "1",
  binding: "",
  automationChoice: "",
  automationAck: false,
  automationRuleId: "",
};

export function bindingToValue(b: { connection_id: string; platform_account: string } | undefined): string {
  return b ? `${b.connection_id}|${b.platform_account}` : "";
}

function parseBinding(value: string) {
  const at = value.indexOf("|");
  return at < 0 ? undefined : { connection_id: value.slice(0, at), platform_account: value.slice(at + 1) };
}

/** Builds the write body from validated values. Sizes stay decimal strings; no arithmetic. */
export function toWriteBody(
  v: AccountFormValues,
  ctx: { kind: AccountKind; locale: Locale; automationRuleVersionId?: string },
): Omit<AccountCreateBody, "kind"> {
  const size = parseDecimalInput(v.size, ctx.locale, { scale: 2 });
  const choice: OptInChoice | undefined = v.automationChoice === "" ? undefined : v.automationChoice;
  const optIns: OptInChoices | undefined =
    ctx.kind === "prop" && choice && ctx.automationRuleVersionId
      ? { automation: { choice, rule_version_id: ctx.automationRuleVersionId } }
      : undefined;
  const binding = parseBinding(v.binding);
  return {
    display_name: v.displayName.trim(),
    ...(ctx.kind === "prop" ? { firm_name: v.firmName.trim(), plan_name: v.planName.trim(), phase: v.phase.trim() } : {}),
    size_amount: size.ok ? size.value : v.size,
    max_position_qty: Number(v.maxPosition),
    ...(v.multiplier.trim() === "" ? {} : { multiplier: Number(v.multiplier) }),
    ...(binding ? { binding } : {}),
    ...(optIns ? { opt_ins: optIns } : {}),
  };
}

export const toUpdateBody = (body: Omit<AccountCreateBody, "kind">, rowVersion: number): AccountUpdateBody => ({ ...body, row_version: rowVersion });

/** Maps an API error pointer to a form field name, or null when it has no field. */
/** True when a form error message is one of our i18n keys (translated by the field) rather than server text. */
export const isFormErrorKey = (message: string | undefined): message is string => message !== undefined && (Object.values(formErrorKeys) as string[]).includes(message);

export function fieldFromPointer(pointer: string | undefined): keyof AccountFormValues | null {
  switch (pointer) {
    case "/display_name":
      return "displayName";
    case "/max_position_qty":
      return "maxPosition";
    case "/multiplier":
      return "multiplier";
    case "/size_amount":
      return "size";
    case "/binding":
      return "binding";
    case "/firm_name":
      return "firmName";
    case "/plan_name":
      return "planName";
    case "/phase":
      return "phase";
    case "/opt_ins/automation":
      return "automationChoice";
    default:
      return null;
  }
}
