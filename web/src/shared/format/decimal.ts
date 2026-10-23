import { intlLocales, type Locale } from "@/shared/i18n/config";

/**
 * Decimals arrive as strings and are formatted from the string, never through `Number` (F§6.1).
 * `Intl.NumberFormat.format` treats a string as an exact decimal (ECMA-402 NumberFormat v3).
 */
const decimalPattern = /^-?\d+(\.\d+)?$/;

export function fractionDigits(value: string): number {
  return value.split(".")[1]?.length ?? 0;
}

/** Validates the wire format; the template type is what `Intl.NumberFormat.format` accepts as an exact decimal. */
function exact(value: string): `${number}` {
  if (!decimalPattern.test(value)) throw new RangeError(`Not a decimal string: ${value}`);
  return value as `${number}`;
}

let exactStrings: boolean | undefined;

/** Older engines coerce the string to a Number without error; refuse to format money there (F§6.1.1). */
function assertExactStringSupport() {
  exactStrings ??= new Intl.NumberFormat("en-US", { useGrouping: false, maximumFractionDigits: 3 }).format("9007199254740993.123" as `${number}`) === "9007199254740993.123";
  if (!exactStrings) throw new Error("This browser cannot format decimal strings exactly (needs Intl.NumberFormat v3).");
}

function numberFormat(locale: Locale, options: Intl.NumberFormatOptions) {
  assertExactStringSupport();
  return new Intl.NumberFormat(intlLocales[locale], options);
}

/** Display precision is the larger of the type's minimum and the stored precision: nothing is rounded away (F§6.1.2). */
function precision(value: string, minimum: number) {
  const digits = Math.max(minimum, fractionDigits(value));
  return { minimumFractionDigits: digits, maximumFractionDigits: digits };
}

export function formatMoney(value: string, locale: Locale): string {
  return numberFormat(locale, { style: "currency", currency: "USD", ...precision(value, 2) }).format(exact(value));
}

/** P&L: explicit sign on non-zero values, because color alone is not enough (F§6.2). */
export function formatSignedMoney(value: string, locale: Locale): string {
  return numberFormat(locale, { style: "currency", currency: "USD", signDisplay: "exceptZero", ...precision(value, 2) }).format(exact(value));
}

export function formatPrice(value: string, locale: Locale, tickDecimals: number): string {
  return numberFormat(locale, { useGrouping: true, ...precision(value, tickDecimals) }).format(exact(value));
}

/** Points distance, without the unit (the unit is a translated word). */
export function formatDecimal(value: string, locale: Locale, minimumDecimals: number): string {
  return numberFormat(locale, precision(value, minimumDecimals)).format(exact(value));
}

export function formatCount(value: string, locale: Locale): string {
  if (!/^-?\d+$/.test(value)) throw new RangeError(`Not an integer string: ${value}`);
  return numberFormat(locale, { maximumFractionDigits: 0 }).format(exact(value));
}

/** Percent of a limit used, truncated so 80 % is never shown while the Core state is still normal (F§6.2). */
export function formatPercentTruncated(value: string, locale: Locale): string {
  return numberFormat(locale, { style: "unit", unit: "percent", unitDisplay: "narrow", maximumFractionDigits: 0, roundingMode: "trunc" }).format(exact(value));
}
