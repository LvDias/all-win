import { describe, expect, it } from "vitest";
import { decideRedirect, type RedirectInput } from "./redirect-decision";

const base: RedirectInput = { pathname: "/", search: "", cookieLocale: undefined, hasSession: false, acceptLanguage: null };

describe("decideRedirect", () => {
  it("sends unprefixed paths to the browser language, keeping path and query", () => {
    expect(decideRedirect({ ...base, pathname: "/settings/preferences", search: "?a=1", acceptLanguage: "en-US,en;q=0.9" })).toBe("/en/settings/preferences?a=1");
    expect(decideRedirect(base)).toBe("/pt-BR");
  });

  it("prefers the NEXT_LOCALE cookie over Accept-Language", () => {
    expect(decideRedirect({ ...base, cookieLocale: "pt-BR", acceptLanguage: "en" })).toBe("/pt-BR");
  });

  it("ignores an invalid cookie value", () => {
    expect(decideRedirect({ ...base, cookieLocale: "fr", acceptLanguage: "en" })).toBe("/en");
  });

  it("serves prefixed paths as is when signed out, even if the cookie differs", () => {
    expect(decideRedirect({ ...base, pathname: "/en/sources", cookieLocale: "pt-BR" })).toBeNull();
  });

  it("redirects a signed-in user to the persisted language", () => {
    expect(decideRedirect({ ...base, pathname: "/en/sources", search: "?x=1", cookieLocale: "pt-BR", hasSession: true })).toBe("/pt-BR/sources?x=1");
  });

  it("does not redirect a signed-in user already on the right language", () => {
    expect(decideRedirect({ ...base, pathname: "/pt-BR/sources", cookieLocale: "pt-BR", hasSession: true })).toBeNull();
  });

  it.each(["/en/invite", "/en/reset-password"])("keeps the locale embedded in %s", (pathname) => {
    expect(decideRedirect({ ...base, pathname, cookieLocale: "pt-BR", hasSession: true })).toBeNull();
  });
});
