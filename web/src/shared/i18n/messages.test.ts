import { describe, expect, it } from "vitest";
import { loadMessages, mergeMessages } from "./messages";

describe("mergeMessages", () => {
  it("fills missing and empty keys from the base, keeping translations", () => {
    const base = { a: "A", b: { c: "C", d: "D" }, e: "E" };
    const over = { a: "", b: { c: "c" }, f: "f" };
    expect(mergeMessages(base, over)).toEqual({ a: "A", b: { c: "c", d: "D" }, e: "E", f: "f" });
  });
});

describe("loadMessages", () => {
  it("returns the catalog of the locale", async () => {
    const en = await loadMessages("en");
    expect(JSON.stringify(en)).toContain("Skip to content");
  });
});
