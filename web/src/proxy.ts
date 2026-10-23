import { NextResponse, type NextRequest } from "next/server";
import { localeCookie, sessionCookie } from "@/shared/i18n/config";
import { decideRedirect } from "@/shared/i18n/redirect-decision";
import { createNonce, securityHeaders } from "@/shared/security/headers";

export function proxy(request: NextRequest) {
  const nonce = createNonce();
  const headers = securityHeaders(nonce, process.env.NODE_ENV === "development");

  const { pathname, search } = request.nextUrl;
  const target = decideRedirect({
    pathname,
    search,
    cookieLocale: request.cookies.get(localeCookie)?.value,
    hasSession: request.cookies.has(sessionCookie),
    acceptLanguage: request.headers.get("accept-language"),
  });

  // Next.js reads the nonce from the request's CSP header and applies it to its own scripts.
  const requestHeaders = new Headers(request.headers);
  requestHeaders.set("x-nonce", nonce);
  requestHeaders.set("Content-Security-Policy", headers["Content-Security-Policy"] ?? "");

  const response = target === null ? NextResponse.next({ request: { headers: requestHeaders } }) : NextResponse.redirect(new URL(target, request.url));
  for (const [name, value] of Object.entries(headers)) response.headers.set(name, value);
  return response;
}

export const config = {
  // Segment-anchored so /apiary or /adapters-guide still get locale handling.
  matcher: ["/((?!(?:api|ingress|adapter|_next)(?:/|$)|.*\\..*).*)"],
};
