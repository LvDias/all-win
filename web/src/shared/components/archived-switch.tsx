"use client";

import { useId } from "react";
import { useTranslations } from "next-intl";
import { Label } from "@/shared/ui/label";
import { Switch } from "@/shared/ui/switch";

type Props = { checked: boolean; onChange: (checked: boolean) => void };

/** "Include archived accounts" (Q79): the Core hides trades of archived accounts unless asked. */
export function ArchivedSwitch({ checked, onChange }: Props) {
  const t = useTranslations("common");
  const id = useId();
  return (
    <div className="flex min-h-11 items-center gap-2">
      <Switch id={id} checked={checked} onCheckedChange={onChange} />
      <Label htmlFor={id} className="text-sm">
        {t("includeArchived")}
      </Label>
    </div>
  );
}
