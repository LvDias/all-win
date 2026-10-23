import { describe, expect, it } from "vitest";
import { parseRouteId, segment } from "./ids";

describe("route ids", () => {
  it("accepts a UUID and rejects anything that could change the API path", () => {
    const id = "00000000-0000-4000-8000-0000000000a1";
    expect(parseRouteId(id)).toBe(id);
    for (const bad of ["../me", "..%2Fme", `${id}?x=1`, `${id}/risk`, "", " ", "abc", `${id}\n`, undefined]) expect(parseRouteId(bad)).toBeNull();
  });

  it("encodes a path segment", () => {
    expect(segment("a/b?c#d")).toBe("a%2Fb%3Fc%23d");
  });
});
