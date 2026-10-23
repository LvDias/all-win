import { describe, expect, it } from "vitest";
import { formatDate, formatDateTime, formatTime } from "./datetime";

const instant = "2026-10-23T17:31:05Z";

describe("date and time", () => {
  it("formats in the display time zone", () => {
    expect(formatDate(instant, "pt-BR", "America/Sao_Paulo")).toBe("23/10/2026");
    expect(formatDate(instant, "en", "America/Sao_Paulo")).toBe("Oct 23, 2026");
    expect(formatDate("2026-10-24T01:00:00Z", "en", "America/Sao_Paulo")).toBe("Oct 23, 2026");
  });

  it("uses a 24-hour clock in both languages", () => {
    expect(formatTime(instant, "pt-BR", "America/Sao_Paulo", { seconds: true })).toBe("14:31:05");
    expect(formatTime(instant, "en", "America/Sao_Paulo", { seconds: true })).toBe("14:31:05");
    expect(formatTime("2026-10-23T03:05:00Z", "en", "UTC")).toBe("03:05");
    expect(formatTime("2026-10-23T00:05:00Z", "en", "UTC")).toBe("00:05");
  });

  it("joins date and time per locale", () => {
    expect(formatDateTime(instant, "pt-BR", "America/Sao_Paulo", { seconds: true })).toBe("23/10/2026 14:31:05");
    expect(formatDateTime(instant, "en", "America/Sao_Paulo", { seconds: true })).toBe("Oct 23, 2026, 14:31:05");
  });

  it("appends the short zone name when asked", () => {
    expect(formatTime(instant, "en", "America/Sao_Paulo", { zone: true })).toMatch(/^14:31 GMT-3$/);
  });

  it("rejects invalid instants", () => {
    expect(() => formatDate("nope", "en", "UTC")).toThrow(RangeError);
  });
});
