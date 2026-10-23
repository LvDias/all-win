import { IntlErrorCode, type IntlError } from "next-intl";
import { isMarkerMode } from "./config";

/**
 * Missing translations (spec V1-S4 §7). Dev/QA/staging: a visible marker and a console error.
 * Production: the catalogs are already merged with pt-BR, so only the key (no user data) is reported
 * and the key name is never shown to the user.
 */
export function handleIntlError(error: IntlError): void {
  if (error.code === IntlErrorCode.MISSING_MESSAGE) {
    console.error(`[i18n] missing message: ${error.message}`);
    return;
  }
  console.error(`[i18n] ${error.code}: ${error.message}`);
}

export function messageFallback({ namespace, key }: { namespace?: string; key: string }): string {
  const path = namespace ? `${namespace}.${key}` : key;
  return isMarkerMode() ? `⟦missing: ${path}⟧` : "";
}
