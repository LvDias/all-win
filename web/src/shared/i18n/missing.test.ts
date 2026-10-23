import { IntlError, IntlErrorCode } from "next-intl";
import { handleIntlError, messageFallback } from "./missing";

describe("messageFallback", () => {
  afterEach(() => vi.unstubAllEnvs());

  it("renders a visible marker with the key in marker mode", () => {
    vi.stubEnv("NEXT_PUBLIC_I18N_MISSING", "marker");
    expect(messageFallback({ namespace: "sources.create", key: "title" })).toBe("⟦missing: sources.create.title⟧");
    expect(messageFallback({ key: "common.x" })).toBe("⟦missing: common.x⟧");
  });

  it("reports only the key, never user data, and never shows the key in production", () => {
    const spy = vi.spyOn(console, "error").mockImplementation(() => {});
    handleIntlError(new IntlError(IntlErrorCode.MISSING_MESSAGE, "Could not resolve `a.b` in messages for locale `en`."));
    expect(spy).toHaveBeenCalledWith(expect.stringContaining("[i18n] missing message"));
    spy.mockRestore();
  });

  it("never shows the key name in production", () => {
    vi.stubEnv("NEXT_PUBLIC_I18N_MISSING", "");
    vi.stubEnv("NODE_ENV", "production");
    expect(messageFallback({ namespace: "a", key: "b" })).toBe("");
  });
});
