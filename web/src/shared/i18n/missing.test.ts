import { messageFallback } from "./missing";

describe("messageFallback", () => {
  afterEach(() => vi.unstubAllEnvs());

  it("renders a visible marker with the key in marker mode", () => {
    vi.stubEnv("NEXT_PUBLIC_I18N_MISSING", "marker");
    expect(messageFallback({ namespace: "sources.create", key: "title" })).toBe("⟦missing: sources.create.title⟧");
    expect(messageFallback({ key: "common.x" })).toBe("⟦missing: common.x⟧");
  });

  it("never shows the key name in production", () => {
    vi.stubEnv("NEXT_PUBLIC_I18N_MISSING", "");
    vi.stubEnv("NODE_ENV", "production");
    expect(messageFallback({ namespace: "a", key: "b" })).toBe("");
  });
});
