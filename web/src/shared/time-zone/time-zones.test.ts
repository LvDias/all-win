import { describe, expect, it } from "vitest";
import { buildTimeZoneOptions, filterTimeZones, groupTimeZones, offsetLabel } from "./time-zones";

const at = new Date("2026-10-23T12:00:00Z");

describe("time zones", () => {
  it("computes the current offset for display", () => {
    expect(offsetLabel("America/Sao_Paulo", at)).toBe("GMT-03:00");
    expect(offsetLabel("UTC", at)).toBe("GMT+00:00");
  });

  it("builds labels and groups by region", () => {
    const options = buildTimeZoneOptions(["America/Sao_Paulo", "America/Chicago", "Europe/Paris", "UTC"], at);
    expect(options[0]?.label).toBe("(GMT-03:00) Sao Paulo");
    expect(groupTimeZones(options).map((g) => [g.region, g.options.length])).toEqual([["America", 2], ["Europe", 1], ["UTC", 1]]);
  });

  it("searches ignoring case and accents", () => {
    const options = buildTimeZoneOptions(["America/Sao_Paulo", "America/Chicago"], at);
    expect(filterTimeZones(options, "SÃO paulo").map((o) => o.id)).toEqual(["America/Sao_Paulo"]);
    expect(filterTimeZones(options, "gmt-05").map((o) => o.id)).toEqual(["America/Chicago"]);
    expect(filterTimeZones(options, "")).toHaveLength(2);
  });
});
