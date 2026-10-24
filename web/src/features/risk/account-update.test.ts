import { describe, expect, it } from "vitest";
import type { Account } from "@/shared/api/types";
import { accountUpdateBody, canonicalAmount } from "./account-update";

const base = { id: "a", kind: "prop", display_name: "X", firm_name: "Apex", plan_name: "50K", phase: "evaluation", size_amount: "50000", currency: "USD", multiplier: 1, max_position_qty: 3, lifecycle_state: "active", opt_ins: {}, has_open_position: false, row_version: 4, created_at: "", updated_at: "" } as unknown as Account;

describe("accountUpdateBody", () => {
  it("sends the manual prop fields and size, no preset fields, and leaves the commission out when unchanged", () => {
    expect(accountUpdateBody(base, undefined)).toEqual({ row_version: 4, display_name: "X", max_position_qty: 3, multiplier: 1, firm_name: "Apex", plan_name: "50K", phase: "evaluation", size_amount: "50000" });
  });

  it("sends only the size for a real account and omits an absent binding", () => {
    const real = { ...base, kind: "real", firm_name: undefined, plan_name: undefined, phase: undefined } as unknown as Account;
    expect(accountUpdateBody(real, "0.5")).toEqual({ row_version: 4, display_name: "X", max_position_qty: 3, multiplier: 1, size_amount: "50000", commission_per_contract_side: "0.5" });
  });

  it("sends null to remove the floor", () => {
    expect(accountUpdateBody(base, null).commission_per_contract_side).toBeNull();
  });
});

describe("canonicalAmount", () => {
  it("drops trailing zeros without arithmetic", () => {
    expect([canonicalAmount("0.3500"), canonicalAmount("1.00"), canonicalAmount("10"), canonicalAmount("0.35")]).toEqual(["0.35", "1", "10", "0.35"]);
  });
});
