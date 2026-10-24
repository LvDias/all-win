import { describe, expect, it } from "vitest";
import { parseDecimalInput } from "./parse-decimal";

describe("parseDecimalInput", () => {
  it("accepts comma and dot as decimal separator in pt-BR", () => {
    expect(parseDecimalInput("1234,50", "pt-BR")).toEqual({ ok: true, value: "1234.5" });
    expect(parseDecimalInput("1234.50", "pt-BR")).toEqual({ ok: true, value: "1234.5" });
  });

  it("accepts only the dot in en", () => {
    expect(parseDecimalInput("1234.50", "en")).toEqual({ ok: true, value: "1234.5" });
    expect(parseDecimalInput("1234,50", "en")).toEqual({ ok: false, reason: "grouping" });
  });

  it.each([
    ["1.234,50", "pt-BR"],
    ["1,234.50", "en"],
    ["1.234.567", "pt-BR"],
  ] as const)("rejects grouping separators: %s", (text, locale) => {
    expect(parseDecimalInput(text, locale)).toEqual({ ok: false, reason: "grouping" });
  });

  it("rejects the ambiguous 1.234 through scale validation", () => {
    expect(parseDecimalInput("1.234", "pt-BR", { scale: 2 })).toEqual({ ok: false, reason: "ambiguous" });
  });

  it("validates tick multiples exactly", () => {
    expect(parseDecimalInput("21234,25", "pt-BR", { tick: "0.25" })).toEqual({ ok: true, value: "21234.25" });
    expect(parseDecimalInput("21234.30", "en", { tick: "0.25" })).toEqual({ ok: false, reason: "tick" });
    expect(parseDecimalInput("0.3", "en", { tick: "0.1" })).toEqual({ ok: true, value: "0.3" });
  });

  it("handles signs only where allowed", () => {
    expect(parseDecimalInput("-5", "en")).toEqual({ ok: false, reason: "sign" });
    expect(parseDecimalInput("-5,0", "pt-BR", { allowSign: true })).toEqual({ ok: true, value: "-5" });
    expect(parseDecimalInput("-0", "en", { allowSign: true })).toEqual({ ok: true, value: "0" });
  });

  it("normalizes leading and trailing zeros", () => {
    expect(parseDecimalInput("007.500", "en")).toEqual({ ok: true, value: "7.5" });
  });

  it.each(["", "  "])("reports empty input", (text) => {
    expect(parseDecimalInput(text, "en")).toEqual({ ok: false, reason: "empty" });
  });

  it.each(["abc", "1e3", "1.", ".5", "1 2", "--1"])("rejects invalid text %s", (text) => {
    const result = parseDecimalInput(text, "en", { allowSign: true });
    expect(result.ok).toBe(false);
  });

  it("keeps values beyond float precision exact", () => {
    expect(parseDecimalInput("9007199254740993.01", "en")).toEqual({ ok: true, value: "9007199254740993.01" });
  });

  describe("money fields refuse a pt-BR separator followed by exactly three digits (thousands or decimals?)", () => {
    it.each(["2.500", "2,500", "999.999", "10,000"])("pt-BR %s is ambiguous", (text) => {
      expect(parseDecimalInput(text, "pt-BR", { scale: 4 })).toEqual({ ok: false, reason: "ambiguous" });
    });

    it("still accepts the unambiguous forms", () => {
      expect(parseDecimalInput("2500,00", "pt-BR", { scale: 2 })).toEqual({ ok: true, value: "2500" });
      expect(parseDecimalInput("2,5", "pt-BR", { scale: 2 })).toEqual({ ok: true, value: "2.5" });
      expect(parseDecimalInput("0,350", "pt-BR", { scale: 4 })).toEqual({ ok: true, value: "0.35" });
      expect(parseDecimalInput("2500", "pt-BR", { scale: 2 })).toEqual({ ok: true, value: "2500" });
      expect(parseDecimalInput("12.3456", "pt-BR", { scale: 4 })).toEqual({ ok: true, value: "12.3456" });
    });

    it("in en the dot is always the decimal point and the comma is rejected", () => {
      expect(parseDecimalInput("2.500", "en", { scale: 4 })).toEqual({ ok: true, value: "2.5" });
      expect(parseDecimalInput("2,500", "en", { scale: 4 })).toEqual({ ok: false, reason: "grouping" });
    });

    it("leaves fields without a scale (prices, distances) alone", () => {
      expect(parseDecimalInput("21234.250", "pt-BR", { tick: "0.25" })).toEqual({ ok: true, value: "21234.25" });
    });
  });

  it("returns a prefilled value verbatim when the text is unchanged, even if typed it would be ambiguous", () => {
    expect(parseDecimalInput("1.125", "pt-BR", { scale: 4 })).toEqual({ ok: false, reason: "ambiguous" });
    expect(parseDecimalInput("1.125", "pt-BR", { scale: 4, unchanged: "1.125" })).toEqual({ ok: true, value: "1.125" });
    expect(parseDecimalInput(" 1.125 ", "pt-BR", { scale: 4, unchanged: "1.125" })).toEqual({ ok: true, value: "1.125" });
    // Any edit goes back through the normal rules.
    expect(parseDecimalInput("1.126", "pt-BR", { scale: 4, unchanged: "1.125" })).toEqual({ ok: false, reason: "ambiguous" });
    expect(parseDecimalInput("2.500", "pt-BR", { scale: 4, unchanged: "1.125" })).toEqual({ ok: false, reason: "ambiguous" });
  });
});
