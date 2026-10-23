"use client";

import { Languages } from "lucide-react";
import { useLocale, useTranslations } from "next-intl";
import { Button } from "@/shared/ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubTrigger, DropdownMenuTrigger } from "@/shared/ui/dropdown-menu";
import { Spinner } from "@/shared/ui/spinner";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/shared/ui/tooltip";
import { isLocale, localeNames, locales } from "./config";
import { useLocaleSwitch } from "./locale-switch";

function LocaleRadioItems() {
  const { switchLocale, pending } = useLocaleSwitch();
  const locale = useLocale();
  return (
    <DropdownMenuRadioGroup value={locale} onValueChange={(value) => isLocale(value) && switchLocale(value)}>
      {locales.map((option) => (
        <DropdownMenuRadioItem key={option} value={option} lang={option} disabled={pending}>
          {localeNames[option].name}
          {pending && option !== locale ? <Spinner className="ml-auto" /> : null}
        </DropdownMenuRadioItem>
      ))}
    </DropdownMenuRadioGroup>
  );
}

/** Public pages: trigger button with the current language (spec V1-S4 §3). */
export function LanguageSwitcher() {
  const t = useTranslations("language");
  const locale = useLocale();
  const { locked, pending } = useLocaleSwitch();
  const names = isLocale(locale) ? localeNames[locale] : localeNames["pt-BR"];
  const accessibleName = t("triggerName", { language: names.name });

  const trigger = (
    <Button variant="outline" disabled={locked} aria-label={accessibleName} aria-busy={pending}>
      {pending ? <Spinner /> : <Languages aria-hidden="true" className="size-4" />}
      <span className="hidden sm:inline" aria-hidden="true">
        {names.name}
      </span>
      <span className="sm:hidden" aria-hidden="true">
        {names.short}
      </span>
    </Button>
  );

  if (locked) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>
          <span tabIndex={0}>{trigger}</span>
        </TooltipTrigger>
        <TooltipContent>{t("locked")}</TooltipContent>
      </Tooltip>
    );
  }
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>{trigger}</DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <LocaleRadioItems />
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

/** Signed-in app: sub-menu to be placed inside the user menu's `DropdownMenuContent`. */
export function LanguageMenuSub() {
  const t = useTranslations("language");
  const { locked } = useLocaleSwitch();
  return (
    <DropdownMenuSub>
      <DropdownMenuSubTrigger disabled={locked}>
        <Languages aria-hidden="true" className="size-4" />
        {t("label")}
      </DropdownMenuSubTrigger>
      <DropdownMenuSubContent>
        <LocaleRadioItems />
      </DropdownMenuSubContent>
    </DropdownMenuSub>
  );
}
