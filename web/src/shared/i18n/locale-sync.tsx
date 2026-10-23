"use client";

import { useLocale } from "next-intl";
import { usePathname, useRouter } from "next/navigation";
import { useEffect } from "react";
import { useMe } from "@/shared/session/me";
import { isLocale } from "./config";
import { writeLocaleCookie } from "./cookie";
import { focusHeadingFlag } from "./locale-switch";
import { isLocaleExempt, splitLocalePath, withLocale } from "./paths";

/**
 * The profile language always wins (AC-1): if the signed-in user's locale differs from the route's,
 * navigate to the profile language. Invite and reset links keep the locale embedded in the link.
 * Also moves focus to the page h1 after a language switch (spec §3.7).
 */
export function LocaleSync() {
  const locale = useLocale();
  const pathname = usePathname();
  const router = useRouter();
  const me = useMe();

  const profileLocale = me.data?.locale;
  useEffect(() => {
    if (!isLocale(profileLocale) || profileLocale === locale) return;
    const { rest } = splitLocalePath(pathname);
    if (isLocaleExempt(rest)) return;
    writeLocaleCookie(profileLocale);
    router.replace(`${withLocale(profileLocale, rest)}${window.location.search}${window.location.hash}`, { scroll: false });
  }, [profileLocale, locale, pathname, router]);

  useEffect(() => {
    try {
      if (sessionStorage.getItem(focusHeadingFlag) !== "1") return;
      sessionStorage.removeItem(focusHeadingFlag);
    } catch {
      return;
    }
    const heading = document.querySelector<HTMLElement>("main h1");
    heading?.setAttribute("tabindex", "-1");
    heading?.focus();
  }, [locale]);

  return null;
}
