import { describe, expect, it } from "vitest";
import { accountKeys } from "./account-keys";

describe("accountKeys", () => {
  it("nests list and detail under the root, so invalidating the root reaches both", () => {
    expect(accountKeys.list().slice(0, 1)).toEqual(accountKeys.all);
    expect(accountKeys.detail("a1").slice(0, 1)).toEqual(accountKeys.all);
    expect(accountKeys.detail("a1")).toEqual(["accounts", "detail", "a1"]);
    expect(accountKeys.detail("a1")).not.toEqual(accountKeys.detail("a2"));
  });
});
