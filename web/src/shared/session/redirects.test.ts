import { landingPath, loginPath, loginReasonFor, safeReturnTo } from "./redirects";

describe("redirects", () => {
  it("maps session.ended reasons", () => {
    expect(loginReasonFor("expired")).toBe("session_expired");
    expect(loginReasonFor("logout")).toBe("signed_out");
    for (const reason of ["password_reset", "password_changed", "totp_changed", "admin_totp_reset"]) expect(loginReasonFor(reason)).toBe("sessions_ended");
    expect(loginReasonFor("something_new")).toBe("session_expired");
  });

  it("accepts only same-origin relative paths", () => {
    expect(safeReturnTo("/pt-BR/connections/sources?x=1")).toBe("/pt-BR/connections/sources?x=1");
    for (const bad of ["//evil.com", "https://evil.com", "/\\evil.com", "javascript:alert(1)", "", null, undefined, "/a\nb"]) expect(safeReturnTo(bad)).toBeNull();
  });

  it("builds the login path with reason and a safe returnTo", () => {
    expect(loginPath("en", { reason: "session_expired", returnTo: "/en/settings/security" })).toBe("/en/login?reason=session_expired&returnTo=%2Fen%2Fsettings%2Fsecurity");
    expect(loginPath("en", { returnTo: "//evil.com" })).toBe("/en/login");
  });

  it("re-prefixes the landing path with the profile language", () => {
    expect(landingPath("en", "/pt-BR/connections/sources?x=1")).toBe("/en/connections/sources?x=1");
    expect(landingPath("pt-BR", "//evil.com")).toBe("/pt-BR");
    expect(landingPath("en", null)).toBe("/en");
  });
});
