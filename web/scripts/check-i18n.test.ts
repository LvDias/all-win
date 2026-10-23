import { describe, expect, it } from "vitest";
import { compareCatalogs, flatten } from "./check-i18n";

describe("compareCatalogs", () => {
  it("passes identical key sets", () => {
    const a = flatten({ x: { y: "1" }, z: "2" });
    expect(compareCatalogs(a, flatten({ x: { y: "um" }, z: "dois" }), "a", "b")).toEqual([]);
  });

  it("reports missing keys on both sides and empty strings", () => {
    const a = flatten({ only_a: "1", empty: "", shared: "x" });
    const b = flatten({ only_b: "1", empty: "e", shared: " " });
    expect(compareCatalogs(a, b, "pt-BR", "en")).toEqual([
      "only_a: missing in en",
      "empty: empty in pt-BR",
      "only_b: missing in pt-BR",
      "shared: empty in en",
    ]);
  });
});
