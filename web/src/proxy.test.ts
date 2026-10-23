import { NextRequest } from "next/server";
import { config, proxy } from "./proxy";

describe("proxy", () => {
  it("adds the security headers with a fresh nonce on pages and on redirects", () => {
    const page = proxy(new NextRequest("http://localhost/pt-BR"));
    const redirect = proxy(new NextRequest("http://localhost/"));
    expect(redirect.status).toBe(307);
    for (const response of [page, redirect]) {
      expect(response.headers.get("Content-Security-Policy")).toMatch(/script-src 'self' 'nonce-[A-Za-z0-9+/=]+' 'strict-dynamic'/);
      expect(response.headers.get("X-Content-Type-Options")).toBe("nosniff");
      expect(response.headers.get("Referrer-Policy")).toBe("strict-origin-when-cross-origin");
      expect(response.headers.get("Permissions-Policy")).toBe("camera=(), microphone=(), geolocation=()");
    }
    expect(page.headers.get("Content-Security-Policy")).not.toBe(redirect.headers.get("Content-Security-Policy"));
  });

  it("matcher skips only whole segments", () => {
    const pattern = new RegExp(`^${(config.matcher[0] ?? "").replace(/\(\?:/g, "(?:")}$`);
    for (const path of ["/api/v1/me", "/ingress/v1/x", "/adapter/v1", "/_next/static/a", "/favicon.ico", "/api"]) expect(pattern.test(path)).toBe(false);
    for (const path of ["/apiary", "/adapters-guide", "/ingress-help", "/pt-BR/login"]) expect(pattern.test(path)).toBe(true);
  });
});
