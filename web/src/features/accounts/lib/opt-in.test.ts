import { describe, expect, it } from "vitest";
import { choiceRequired, needsStepUp, optInKindFromPointer } from "./opt-in";

describe("opt-in rules", () => {
  it.each([
    ["allowed", false],
    ["forbidden", true],
    ["conditional", true],
    ["unknown", true],
    ["not_confirmed", true],
  ] as const)("choice required for %s: %s", (stance, expected) => {
    expect(choiceRequired(stance)).toBe(expected);
  });

  it("step-up only for accepted", () => {
    expect(needsStepUp({ automation: { choice: "declined", rule_version_id: "r" } })).toBe(false);
    expect(needsStepUp({ automation: { choice: "accepted", rule_version_id: "r" } })).toBe(true);
    expect(needsStepUp(undefined)).toBe(false);
  });

  it("maps pointers to kinds", () => {
    expect(optInKindFromPointer("/opt_ins/copy")).toBe("copy");
    expect(optInKindFromPointer("/display_name")).toBeNull();
  });
});
