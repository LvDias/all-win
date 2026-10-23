import { ApiError, networkErrorCode } from "./client";

type Translator = {
  (key: string, values?: Record<string, string>): string;
  has: (key: string) => boolean;
};

/**
 * Maps a Core error code 1:1 to `errors.<code>`; unknown codes use `errors.unknown` with the code (spec V1-S4 §5).
 * `t` is a translator for the root namespace.
 */
export function errorMessage(t: Translator, error: unknown): { message: string; code: string } {
  if (error instanceof ApiError) {
    if (error.code === networkErrorCode) return { message: t("common.errors.network"), code: error.code };
    const key = `errors.${error.code}`;
    return t.has(key) ? { message: t(key), code: error.code } : { message: t("errors.unknown", { code: error.code }), code: error.code };
  }
  return { message: t("common.errors.failed"), code: "client.unknown" };
}
