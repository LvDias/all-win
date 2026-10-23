"use client";

import { useAppLocale } from "@/shared/i18n/navigation";
import { useMe } from "./me";

/** Language and IANA time zone used to display dates for the signed-in user (UTC until the profile loads). */
export function useDisplay() {
  const locale = useAppLocale();
  const me = useMe();
  return { locale, timeZone: me.data?.time_zone ?? "UTC" };
}
