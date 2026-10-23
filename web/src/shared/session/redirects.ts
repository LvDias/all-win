import type { Locale } from "@/shared/i18n/config";
import { splitLocalePath, withLocale } from "@/shared/i18n/paths";

export type LoginReason = "session_expired" | "signed_out" | "sessions_ended" | "pending_expired";

/** `session.ended` reasons from the event stream map onto the login page messages (S3 spec §1). */
export function loginReasonFor(endedReason: string): LoginReason {
  switch (endedReason) {
    case "logout":
      return "signed_out";
    case "password_reset":
    case "password_changed":
    case "totp_changed":
    case "admin_totp_reset":
      return "sessions_ended";
    default:
      return "session_expired";
  }
}

/** Only same-origin relative paths are accepted; anything else falls back to the app home (open-redirect guard). */
export function safeReturnTo(value: string | null | undefined): string | null {
  if (!value || !value.startsWith("/") || value.startsWith("//") || value.includes("\\") || /[\u0000-\u001f]/.test(value)) return null;
  return value;
}

export function loginPath(locale: Locale, options: { reason?: LoginReason; returnTo?: string } = {}): string {
  const params = new URLSearchParams();
  if (options.reason) params.set("reason", options.reason);
  const returnTo = safeReturnTo(options.returnTo);
  if (returnTo) params.set("returnTo", returnTo);
  const query = params.toString();
  return `${withLocale(locale, "/login")}${query ? `?${query}` : ""}`;
}

/** Where to land after sign-in: `returnTo` re-prefixed with the user's profile language, else the app home. */
export function landingPath(userLocale: Locale, returnTo: string | null | undefined): string {
  const safe = safeReturnTo(returnTo);
  if (!safe) return withLocale(userLocale, "/");
  const queryAt = safe.search(/[?#]/);
  const path = queryAt === -1 ? safe : safe.slice(0, queryAt);
  const tail = queryAt === -1 ? "" : safe.slice(queryAt);
  return `${withLocale(userLocale, splitLocalePath(path).rest)}${tail}`;
}
