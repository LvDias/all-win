"use client";

import { useLocale, useTranslations } from "next-intl";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { useDisplay } from "@/shared/session/display";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/shared/ui/tooltip";
import { resetInUserZone } from "./lib/zones";

/** The trading-day reset as published, with the firm's and the user's clocks together in a tooltip (and for screen readers). */
export function ResetTime({ localTime, timeZone, publishedText }: { localTime: string; timeZone: string; publishedText?: string }) {
  const t = useTranslations("presets.plans");
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const display = useDisplay();
  const shown = publishedText ?? `${localTime} ${timeZone}`;
  const zones = resetInUserZone(localTime, timeZone, display.timeZone, locale, new Date());
  if (!zones) return <>{shown}</>;
  const both = t("resetBoth", { firm: zones.firm, user: zones.user });
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        {/* `relative`: the screen-reader text is absolutely positioned, and without a positioned parent inside the table's scroll box it
            would stretch the PAGE sideways (WCAG 1.4.10 at 320 px). */}
        <span tabIndex={0} className="relative underline decoration-dotted underline-offset-4">
          {shown}
          <span className="sr-only">{both}</span>
        </span>
      </TooltipTrigger>
      <TooltipContent>{both}</TooltipContent>
    </Tooltip>
  );
}
