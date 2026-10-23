import { NextResponse, type NextRequest } from "next/server";
import { localeCookie, sessionCookie } from "@/shared/i18n/config";
import { decideRedirect } from "@/shared/i18n/redirect-decision";

export function proxy(request: NextRequest) {
  const { pathname, search } = request.nextUrl;
  const target = decideRedirect({
    pathname,
    search,
    cookieLocale: request.cookies.get(localeCookie)?.value,
    hasSession: request.cookies.has(sessionCookie),
    acceptLanguage: request.headers.get("accept-language"),
  });
  if (target === null) return NextResponse.next();
  return NextResponse.redirect(new URL(target, request.url));
}

export const config = {
  matcher: ["/((?!api|ingress|adapter|_next|.*\\..*).*)"],
};
