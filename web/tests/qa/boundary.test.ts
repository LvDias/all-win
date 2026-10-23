import { PreferencesForm } from "@/features/settings";
import { describe, expect, it } from "vitest";

describe("tests/qa boundary", () => {
  it("reaches a feature through its public index", () => {
    expect(PreferencesForm).toBeTypeOf("function");
  });
});
