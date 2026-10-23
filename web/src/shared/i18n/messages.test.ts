import { describe, expect, it } from "vitest";
import { buildCatalog, loadMessages, mergeMessages } from "./messages";

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

describe("buildCatalog", () => {
  const ptBR = { a: "A", b: "B" };
  const en = { a: "a" };

  it("production: a key missing in en reads as pt-BR text", () => {
    expect(buildCatalog(en, ptBR, false)).toEqual({ a: "a", b: "B" });
  });

  it("marker mode: the key stays missing so the marker shows", () => {
    expect(buildCatalog(en, ptBR, true)).toEqual({ a: "a" });
  });
});

describe("loadMessages and the fallback mode", () => {
  afterEach(() => vi.unstubAllEnvs());

  it("loads the same keys either way when the catalogs are in parity", async () => {
    vi.stubEnv("NEXT_PUBLIC_I18N_MISSING", "marker");
    const marker = await loadMessages("en");
    vi.stubEnv("NEXT_PUBLIC_I18N_MISSING", "");
    vi.stubEnv("NODE_ENV", "production");
    expect(await loadMessages("en")).toEqual(marker);
  });
});
