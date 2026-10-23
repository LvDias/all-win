import { describe, expect, it } from "vitest";
import { resolveFromAcceptLanguage } from "./resolve-locale";

describe("resolveFromAcceptLanguage", () => {
  it.each([
    [null, "pt-BR"],
    ["", "pt-BR"],
    ["pt-PT,en;q=0.5", "pt-BR"],
    ["en-GB,en;q=0.9", "en"],
    ["fr-FR,en;q=0.8,pt;q=0.7", "en"],
    ["fr-FR,de;q=0.8", "pt-BR"],
    ["en;q=0.4,pt;q=0.9", "pt-BR"],
    ["en;q=0,fr", "pt-BR"],
    ["EN-us", "en"],
    ["*", "pt-BR"],
  ])("%s -> %s", (header, expected) => {
    expect(resolveFromAcceptLanguage(header)).toBe(expected);
  });
});
