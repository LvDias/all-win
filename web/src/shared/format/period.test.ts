import { describe, expect, it } from "vitest";
import { periodWindow } from "./period";

const now = new Date("2026-10-23T17:31:05Z");

describe("periodWindow", () => {
  it("presets start at local midnight and have no end", () => {
    expect(periodWindow("today", "", "", now, "America/Sao_Paulo")).toEqual({ from: "2026-10-23T03:00:00Z" });
    expect(periodWindow("7d", "", "", now, "UTC")).toEqual({ from: "2026-10-16T00:00:00Z" });
    expect(periodWindow("30d", "", "", now, "UTC")).toEqual({ from: "2026-09-23T00:00:00Z" });
  });

  it("a custom period is half-open: the end is the start of the day after the last day, in the display zone", () => {
    expect(periodWindow("custom", "2026-10-01", "2026-10-31", now, "America/Sao_Paulo")).toEqual({ from: "2026-10-01T03:00:00Z", to: "2026-11-01T03:00:00Z" });
    expect(periodWindow("custom", "2026-12-31", "2026-12-31", now, "UTC")).toEqual({ from: "2026-12-31T00:00:00Z", to: "2027-01-01T00:00:00Z" });
  });

  it("an open custom period leaves the missing side out", () => {
    expect(periodWindow("custom", "2026-10-01", "", now, "UTC")).toEqual({ from: "2026-10-01T00:00:00Z" });
    expect(periodWindow("custom", "", "2026-10-01", now, "UTC")).toEqual({ to: "2026-10-02T00:00:00Z" });
  });
});
