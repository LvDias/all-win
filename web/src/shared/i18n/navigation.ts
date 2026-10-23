"use client";

import { useLocale } from "next-intl";
import { defaultLocale, isLocale, type Locale } from "./config";
import { withLocale } from "./paths";

/** The active locale, narrowed to the supported set. */
export function useAppLocale(): Locale {
  const locale = useLocale();
  return isLocale(locale) ? locale : defaultLocale;
}

/** Builds a locale-prefixed path for an in-app route, e.g. `useLocalePath()("/login")`. */
export function useLocalePath(): (rest: string) => string {
  const locale = useAppLocale();
  return (rest) => withLocale(locale, rest);
}
