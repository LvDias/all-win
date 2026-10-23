import type { Locale } from "@/shared/i18n/config";

export type ParseFailure = "empty" | "invalid" | "grouping" | "sign" | "scale" | "tick";
export type ParseResult = { ok: true; value: string } | { ok: false; reason: ParseFailure };

export type ParseOptions = {
  allowSign?: boolean;
  /** Maximum number of decimals (money: 2). */
  scale?: number;
  /** Value must be a multiple of this decimal string (instrument tick, e.g. "0.25"). */
  tick?: string;
};

/**
 * Exact input parser (F§6.3): text in, canonical decimal string out ("1234.50"). No floats.
 * pt-BR accepts "," and "." as the decimal separator; en only ".". Grouping separators are rejected.
 */
export function parseDecimalInput(text: string, locale: Locale, options: ParseOptions = {}): ParseResult {
  const raw = text.trim();
  if (raw === "") return { ok: false, reason: "empty" };

  const signMatch = /^[+-]/.exec(raw);
  if (signMatch && !options.allowSign) return { ok: false, reason: "sign" };
  const unsigned = signMatch ? raw.slice(1) : raw;
  const negative = signMatch?.[0] === "-";

  if (!/^[\d.,]+$/.test(unsigned)) return { ok: false, reason: "invalid" };

  const dots = (unsigned.match(/\./g) ?? []).length;
  const commas = (unsigned.match(/,/g) ?? []).length;
  if (locale === "en" && commas > 0) return { ok: false, reason: "grouping" };
  if (dots + commas > 1) return { ok: false, reason: "grouping" };

  const [intPart = "", fracPart = ""] = unsigned.split(/[.,]/);
  if (intPart === "" || !/^\d+$/.test(intPart) || (dots + commas === 1 && !/^\d+$/.test(fracPart))) {
    return { ok: false, reason: "invalid" };
  }

  const integer = intPart.replace(/^0+(?=\d)/, "");
  const fraction = fracPart.replace(/0+$/, "");
  if (options.scale !== undefined && fraction.length > options.scale) return { ok: false, reason: "scale" };

  if (options.tick !== undefined && !isMultipleOf(integer, fraction, options.tick)) return { ok: false, reason: "tick" };

  const isZero = /^0*$/.test(integer + fraction);
  const canonical = `${negative && !isZero ? "-" : ""}${integer}${fraction === "" ? "" : `.${fraction}`}`;
  return { ok: true, value: canonical };
}

function isMultipleOf(integer: string, fraction: string, tick: string): boolean {
  const [tickInt = "0", tickFrac = ""] = tick.split(".");
  const digits = Math.max(fraction.length, tickFrac.length);
  const scaled = (int: string, frac: string) => BigInt(int + frac.padEnd(digits, "0"));
  const step = scaled(tickInt, tickFrac);
  return step !== 0n && scaled(integer, fraction) % step === 0n;
}
