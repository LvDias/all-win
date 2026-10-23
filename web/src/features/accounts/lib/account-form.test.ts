import { describe, expect, it } from "vitest";
import { accountFormSchema, emptyAccountForm, fieldFromPointer, toWriteBody, type AccountFormValues } from "./account-form";

const valid: AccountFormValues = { ...emptyAccountForm, displayName: "Apex 1", firmName: "Apex", planName: "50K", phase: "Evaluation", size: "50000", maxPosition: "2" };

function issues(values: AccountFormValues, kind: "prop" | "real" = "prop", optInRequired = false) {
  const r = accountFormSchema("en", { kind, optInRequired }).safeParse(values);
  return r.success ? {} : Object.fromEntries(r.error.issues.map((i) => [String(i.path[0]), i.message]));
}

describe("account form schema", () => {
  it("accepts a valid prop account", () => expect(issues(valid)).toEqual({}));
  it("requires prop firm fields only for prop", () => {
    const blank = { ...valid, firmName: "", planName: "", phase: "" };
    expect(Object.keys(issues(blank))).toEqual(["firmName", "planName", "phase"]);
    expect(issues(blank, "real")).toEqual({});
  });
  it("rejects fractional or zero multiplier and max position", () => {
    expect(issues({ ...valid, multiplier: "1.5" }).multiplier).toBe("integerMin1");
    expect(issues({ ...valid, multiplier: "0" }).multiplier).toBe("integerMin1");
    expect(issues({ ...valid, maxPosition: "0" }).maxPosition).toBe("maxPositionInvalid");
    expect(issues({ ...valid, maxPosition: "" }).maxPosition).toBe("maxPositionInvalid");
  });
  it("refuses an ambiguous pt-BR size instead of guessing 25 or 25000", () => {
    const ptIssues = (size: string) => {
      const r = accountFormSchema("pt-BR", { kind: "real", optInRequired: false }).safeParse({ ...valid, size });
      return r.success ? undefined : r.error.issues.find((i) => i.path[0] === "size")?.message;
    };
    expect(ptIssues("25.000")).toBe("sizeAmbiguous");
    expect(ptIssues("25,000")).toBe("sizeAmbiguous");
    expect(ptIssues("25000")).toBeUndefined();
    expect(ptIssues("25000,00")).toBeUndefined();
    expect(issues({ ...valid, size: "25.000" }, "real").size).toBeUndefined();
  });
  it("rejects zero size and more than 2 decimals", () => {
    expect(issues({ ...valid, size: "0" }).size).toBe("sizeInvalid");
    expect(issues({ ...valid, size: "10.001" }).size).toBe("sizeInvalid");
  });
  it("requires the choice with no default, and the ack when accepting", () => {
    expect(issues(valid, "prop", true).automationChoice).toBe("optInRequired");
    expect(issues({ ...valid, automationChoice: "accepted" }, "prop", true).automationAck).toBe("ackRequired");
    expect(issues({ ...valid, automationChoice: "declined" }, "prop", true)).toEqual({});
  });
});

describe("toWriteBody", () => {
  it("keeps money as a string and sends the rule version seen", () => {
    const body = toWriteBody({ ...valid, size: "50000,50", automationChoice: "declined", binding: "c1|Sim101" }, { kind: "prop", locale: "pt-BR", automationRuleVersionId: "rv1" });
    expect(body).toMatchObject({ size_amount: "50000.5", max_position_qty: 2, multiplier: 1, binding: { connection_id: "c1", platform_account: "Sim101" }, opt_ins: { automation: { choice: "declined", rule_version_id: "rv1" } } });
  });
  it("sends no prop fields or opt_ins for real", () => {
    const body = toWriteBody({ ...valid, automationChoice: "declined" }, { kind: "real", locale: "en", automationRuleVersionId: "rv1" });
    expect(body).not.toHaveProperty("firm_name");
    expect(body).not.toHaveProperty("opt_ins");
  });
});

describe("fieldFromPointer", () => {
  it("maps known pointers", () => {
    expect(fieldFromPointer("/opt_ins/automation")).toBe("automationChoice");
    expect(fieldFromPointer("/binding")).toBe("binding");
    expect(fieldFromPointer("/nope")).toBeNull();
  });
});
