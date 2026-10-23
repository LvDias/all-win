import { describe, expect, it } from "vitest";
import { accountFormSchema, emptyAccountForm } from "./account-form";
import { firstStepWithErrors, issuesForStep, stepOfField, stepsFor } from "./account-steps";

describe("stepsFor", () => {
  it("prop: type, firm, data, platform, opt-in, review", () => {
    expect(stepsFor("prop", true)).toEqual(["type", "firm", "data", "platform", "optIn", "review"]);
  });
  it("drops the opt-in step when the firm allows it, and the firm step for real", () => {
    expect(stepsFor("prop", false)).toEqual(["type", "firm", "data", "platform", "review"]);
    expect(stepsFor("real", true)).toEqual(["type", "data", "platform", "review"]);
  });
  it("shows only the type step until one is chosen", () => expect(stepsFor(null, true)).toEqual(["type"]));
});

describe("step ownership", () => {
  it("maps a pointer field back to its step", () => {
    expect(stepOfField("displayName")).toBe("data");
    expect(stepOfField("binding")).toBe("platform");
    expect(stepOfField("automationChoice")).toBe("optIn");
    expect(stepOfField(null)).toBeNull();
  });

  it("validates only the current step", () => {
    const r = accountFormSchema("en", { kind: "prop", optInRequired: true }).safeParse(emptyAccountForm);
    const issues = r.success ? [] : r.error.issues;
    expect(Object.keys(issuesForStep(issues, "firm"))).toEqual(["firmName", "planName", "phase"]);
    expect(Object.keys(issuesForStep(issues, "data")).sort()).toEqual(["displayName", "maxPosition", "size"]);
    expect(issuesForStep(issues, "platform")).toEqual({});
  });

  it("finds the earliest step with an error", () => {
    expect(firstStepWithErrors(stepsFor("prop", true), { automationChoice: "x", size: "y" })).toBe("data");
  });
});
