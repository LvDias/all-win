import { hasLocale } from "next-intl";
import { getRequestConfig } from "next-intl/server";
import { defaultLocale, locales } from "./config";
import { loadMessages } from "./messages";
import { handleIntlError, messageFallback } from "./missing";

export default getRequestConfig(async ({ requestLocale }) => {
  const requested = await requestLocale;
  const locale = hasLocale(locales, requested) ? requested : defaultLocale;
  return {
    locale,
    messages: await loadMessages(locale),
    // Display time zones come from the profile; server rendering stays on UTC so output is deterministic.
    timeZone: "UTC",
    onError: handleIntlError,
    getMessageFallback: messageFallback,
  };
});
