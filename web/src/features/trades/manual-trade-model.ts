import { z } from "zod";
import type { ManualTradeFields } from "@/shared/api/types";
import { parseDecimalInput, zonedToInstant } from "@/shared/format";
import type { Locale } from "@/shared/i18n/config";

export type ManualTradeValues = {
  accountId: string;
  direction: "long" | "short" | "";
  quantity: string;
  entryDate: string;
  entryTime: string;
  entryPrice: string;
  exitDate: string;
  exitTime: string;
  exitPrice: string;
  commission: string;
  mae: string;
  mfe: string;
};

export const emptyValues = (accountId = ""): ManualTradeValues => ({ accountId, direction: "", quantity: "", entryDate: "", entryTime: "", entryPrice: "", exitDate: "", exitTime: "", exitPrice: "", commission: "", mae: "", mfe: "" });

/** The fields each step owns, for validation and for mapping the Core's pointers. */
export const stepFields = {
  trade: ["accountId", "direction", "quantity"],
  times: ["entryDate", "entryTime", "entryPrice", "exitDate", "exitTime", "exitPrice", "commission"],
  excursions: ["mae", "mfe"],
} as const satisfies Record<string, readonly (keyof ManualTradeValues)[]>;

/** Core JSON pointers → the form field (the Core knows `entry_time`, the form has a date and a time). */
export const pointerToField: Record<string, keyof ManualTradeValues> = {
  "/account_id": "accountId",
  "/direction": "direction",
  "/quantity": "quantity",
  "/entry_time": "entryDate",
  "/entry_price": "entryPrice",
  "/exit_time": "exitDate",
  "/exit_price": "exitPrice",
  "/commission": "commission",
  "/mae": "mae",
  "/mfe": "mfe",
};

type Env = { locale: Locale; timeZone: string; tick: string | undefined; now: () => number };
type Messages = (key: string) => string;

const hasMessage = (path: string[], message: string, ctx: z.RefinementCtx) => ctx.addIssue({ code: "custom", path, message });

function price(text: string, env: Env) {
  return parseDecimalInput(text, env.locale, { tick: env.tick });
}
function money(text: string, env: Env) {
  return parseDecimalInput(text, env.locale, { scale: 2 });
}

export function tradeSchema(m: Messages) {
  return z.object({
    accountId: z.string().min(1, m("errors.accountRequired")),
    direction: z.enum(["long", "short"], { error: m("errors.directionRequired") }),
    quantity: z.string().regex(/^[0-9]+$/, m("errors.quantityInvalid")).refine((value) => Number(value) >= 1 && Number(value) <= 10000, m("errors.quantityInvalid")),
  });
}

export function timesSchema(m: Messages, env: Env) {
  return z
    .object({ entryDate: z.string(), entryTime: z.string(), entryPrice: z.string(), exitDate: z.string(), exitTime: z.string(), exitPrice: z.string(), commission: z.string() })
    .superRefine((value, ctx) => {
      const entry = zonedToInstant(value.entryDate, value.entryTime, env.timeZone);
      const exit = zonedToInstant(value.exitDate, value.exitTime, env.timeZone);
      if (!entry) hasMessage(["entryDate"], m("errors.timeRequired"), ctx);
      if (!exit) hasMessage(["exitDate"], m("errors.timeRequired"), ctx);
      if (entry && Date.parse(entry) > env.now()) hasMessage(["entryDate"], m("errors.inFuture"), ctx);
      if (exit && Date.parse(exit) > env.now()) hasMessage(["exitDate"], m("errors.inFuture"), ctx);
      // Both are UTC instants in the same ISO format, so the text order is the time order.
      if (entry && exit && exit <= entry) hasMessage(["exitDate"], m("errors.exitBeforeEntry"), ctx);
      for (const [field, text] of [["entryPrice", value.entryPrice], ["exitPrice", value.exitPrice]] as const) {
        const parsed = price(text, env);
        if (!parsed.ok) hasMessage([field], parsed.reason === "tick" ? m("errors.notTickMultiple") : m("errors.priceInvalid"), ctx);
      }
      if (value.commission.trim() !== "") {
        const parsed = money(value.commission, env);
        if (!parsed.ok) hasMessage(["commission"], parsed.reason === "ambiguous" ? m("errors.amountAmbiguous") : m("errors.moneyInvalid"), ctx);
      }
    });
}

export function excursionsSchema(m: Messages, env: Env) {
  return z.object({ mae: z.string(), mfe: z.string() }).superRefine((value, ctx) => {
    for (const field of ["mae", "mfe"] as const) {
      if (value[field].trim() === "") continue;
      const parsed = money(value[field], env);
      if (!parsed.ok) hasMessage([field], parsed.reason === "ambiguous" ? m("errors.amountAmbiguous") : m("errors.moneyInvalid"), ctx);
    }
  });
}

/** The request body, or null while a required field is missing or invalid (the preview then waits). */
export function toRequest(values: ManualTradeValues, instrumentId: string | undefined, env: Env): ManualTradeFields | null {
  if (!instrumentId || !values.accountId || (values.direction !== "long" && values.direction !== "short")) return null;
  const quantity = /^[0-9]+$/.test(values.quantity) ? Number(values.quantity) : 0;
  const entry = zonedToInstant(values.entryDate, values.entryTime, env.timeZone);
  const exit = zonedToInstant(values.exitDate, values.exitTime, env.timeZone);
  const entryPrice = price(values.entryPrice, env);
  const exitPrice = price(values.exitPrice, env);
  if (quantity < 1 || quantity > 10000 || !entry || !exit || !entryPrice.ok || !exitPrice.ok) return null;
  const body: ManualTradeFields = { account_id: values.accountId, instrument_id: instrumentId, direction: values.direction, quantity, entry_time: entry, entry_price: entryPrice.value, exit_time: exit, exit_price: exitPrice.value };
  for (const [key, text] of [["commission", values.commission], ["mae", values.mae], ["mfe", values.mfe]] as const) {
    if (text.trim() === "") continue;
    const parsed = money(text, env);
    if (!parsed.ok) return null;
    body[key] = parsed.value;
  }
  return body;
}
