import { describe, expect, it } from "vitest";
import { formatDate, formatDateTime, formatTime, instantToZoned, startOfDayInstant, zonedToInstant } from "./datetime";

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

describe("zoned wall-clock conversion", () => {
  it("converts a time typed in the display zone to the UTC instant", () => {
    expect(zonedToInstant("2026-10-23", "14:31:05", "America/Sao_Paulo")).toBe("2026-10-23T17:31:05Z");
    expect(zonedToInstant("2026-10-23", "14:31", "UTC")).toBe("2026-10-23T14:31:00Z");
    expect(zonedToInstant("2026-01-15", "09:30:00", "America/New_York")).toBe("2026-01-15T14:30:00Z");
    expect(zonedToInstant("2026-07-15", "09:30:00", "America/New_York")).toBe("2026-07-15T13:30:00Z");
  });

  it("rejects text that is not a real date or time", () => {
    expect(zonedToInstant("2026-02-30", "10:00", "UTC")).toBeNull();
    expect(zonedToInstant("2026-10-23", "24:00", "UTC")).toBeNull();
    expect(zonedToInstant("23/10/2026", "10:00", "UTC")).toBeNull();
  });

  it("round-trips through the zone", () => {
    expect(instantToZoned("2026-10-23T17:31:05Z", "America/Sao_Paulo")).toEqual({ date: "2026-10-23", time: "14:31:05" });
    expect(zonedToInstant(...(Object.values(instantToZoned("2026-03-08T07:30:00Z", "America/New_York")) as [string, string]), "America/New_York")).toBe("2026-03-08T07:30:00Z");
  });

  it("finds the start of a day in the display zone", () => {
    expect(startOfDayInstant(new Date("2026-10-23T17:31:05Z"), 0, "America/Sao_Paulo")).toBe("2026-10-23T03:00:00Z");
    expect(startOfDayInstant(new Date("2026-10-23T01:00:00Z"), 0, "America/Sao_Paulo")).toBe("2026-10-22T03:00:00Z");
    expect(startOfDayInstant(new Date("2026-10-23T17:31:05Z"), 7, "UTC")).toBe("2026-10-16T00:00:00Z");
  });
});
