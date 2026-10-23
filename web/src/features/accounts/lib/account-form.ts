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

export function accountFormSchema(locale: Locale, opts: { kind: AccountKind; optInRequired: boolean }) {
  return z
    .object({
      displayName: z.string().trim().min(1, formErrorKeys.required).max(80, formErrorKeys.tooLong),
      firmName: z.string().trim().max(80, formErrorKeys.tooLong),
      planName: z.string().trim().max(80, formErrorKeys.tooLong),
      phase: z.string().trim().max(80, formErrorKeys.tooLong),
      size: z.string(),
      maxPosition: z.string(),
      multiplier: z.string(),
      /** `"<connection_id>|<platform_account>"` or empty for unbound. */
      binding: z.string(),
      automationChoice: z.enum(["accepted", "declined", ""]),
      automationAck: z.boolean(),
    })
    .superRefine((v, ctx) => {
      const fail = (path: string, message: string) => ctx.addIssue({ code: "custom", path: [path], message });
      if (opts.kind === "prop") {
        if (v.firmName === "") fail("firmName", formErrorKeys.required);
        if (v.planName === "") fail("planName", formErrorKeys.required);
        if (v.phase === "") fail("phase", formErrorKeys.required);
      }
      const size = parseDecimalInput(v.size, locale, { scale: 2 });
      if (!size.ok || Number(size.value) <= 0) fail("size", formErrorKeys.size);
      if (!wholeNumber(v.maxPosition)) fail("maxPosition", formErrorKeys.maxPosition);
      if (v.multiplier.trim() !== "" && !wholeNumber(v.multiplier)) fail("multiplier", formErrorKeys.integer);
      if (opts.kind === "prop" && opts.optInRequired) {
        if (v.automationChoice === "") fail("automationChoice", formErrorKeys.optInRequired);
        else if (v.automationChoice === "accepted" && !v.automationAck) fail("automationAck", formErrorKeys.ackRequired);
      }
    });
}

export type AccountFormValues = z.input<ReturnType<typeof accountFormSchema>>;

export const emptyAccountForm: AccountFormValues = {
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
