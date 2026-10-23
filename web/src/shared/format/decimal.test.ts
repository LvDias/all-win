import { describe, expect, it } from "vitest";
import { formatCount, formatDecimal, formatMoney, formatPercentTruncated, formatPrice, formatSignedMoney } from "./decimal";

// Intl may emit a no-break space between symbol and number; compare with plain spaces.
const plain = (text: string) => text.replace(/\s/g, " ");

describe("money", () => {
  it("formats USD per locale without changing the value", () => {
    expect(plain(formatMoney("1234.5", "pt-BR"))).toBe("US$ 1.234,50");
    expect(formatMoney("1234.5", "en")).toBe("$1,234.50");
  });

  it("keeps stored precision beyond 2 decimals", () => {
    expect(formatMoney("1.2345", "en")).toBe("$1.2345");
  });

  it("is exact for values a float cannot hold", () => {
    expect(formatMoney("9007199254740993.01", "en")).toBe("$9,007,199,254,740,993.01");
  });

  it("signs P&L except zero", () => {
    expect(plain(formatSignedMoney("1234.5", "pt-BR"))).toBe("+US$ 1.234,50");
    expect(plain(formatSignedMoney("-320", "pt-BR"))).toBe("-US$ 320,00");
    expect(plain(formatSignedMoney("0", "pt-BR"))).toBe("US$ 0,00");
    expect(formatSignedMoney("1234.5", "en")).toBe("+$1,234.50");
    expect(formatSignedMoney("-320", "en")).toBe("-$320.00");
    expect(formatSignedMoney("0.00", "en")).toBe("$0.00");
  });

  it("rejects non-decimal input", () => {
    expect(() => formatMoney("1e3", "en")).toThrow(RangeError);
    expect(() => formatMoney("abc", "en")).toThrow(RangeError);
  });
});

describe("price, decimal, count, percent", () => {
  it("formats price with tick decimals and grouping", () => {
    expect(formatPrice("21234.5", "pt-BR", 2)).toBe("21.234,50");
    expect(formatPrice("21234.5", "en", 2)).toBe("21,234.50");
  });

  it("does not drop significant digits of the value", () => {
    expect(formatPrice("21234.125", "en", 2)).toBe("21,234.125");
  });

  it("formats a distance without unit", () => {
    expect(formatDecimal("12.5", "pt-BR", 2)).toBe("12,50");
    expect(formatDecimal("12.5", "en", 2)).toBe("12.50");
  });

  it("formats counts with grouping", () => {
    expect(formatCount("1204", "pt-BR")).toBe("1.204");
    expect(formatCount("1204", "en")).toBe("1,204");
    expect(() => formatCount("1.5", "en")).toThrow(RangeError);
  });

  it("truncates percent instead of rounding", () => {
    expect(formatPercentTruncated("79.9", "pt-BR")).toBe("79%");
    expect(formatPercentTruncated("79.9", "en")).toBe("79%");
    expect(formatPercentTruncated("79.99999999999999999", "en")).toBe("79%");
  });
});
