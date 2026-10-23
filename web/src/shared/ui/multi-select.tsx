"use client";

import { ChevronDown } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "./button";
import { Checkbox } from "./checkbox";
import { Label } from "./label";
import { Popover, PopoverContent, PopoverTrigger } from "./popover";

export type MultiSelectOption = { value: string; label: string };

type MultiSelectProps = {
  label: string;
  options: MultiSelectOption[];
  selected: readonly string[];
  onChange: (selected: string[]) => void;
};

/** A filter control: a trigger that names the field and the count, and a checkbox list. Selection applies as it changes. */
export function MultiSelect({ label, options, selected, onChange }: MultiSelectProps) {
  const t = useTranslations("common.multiSelect");
  const toggle = (value: string, checked: boolean) => onChange(checked ? [...selected, value] : selected.filter((item) => item !== value));
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="outline" aria-label={selected.length > 0 ? t("triggerWithCount", { label, count: selected.length }) : label} className="justify-between gap-2">
          <span>{selected.length > 0 ? t("withCount", { label, count: selected.length }) : label}</span>
          <ChevronDown aria-hidden="true" className="size-4" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-auto max-w-xs p-2">
        <div role="group" aria-label={label} className="grid max-h-72 overflow-y-auto">
          {options.length === 0 ? <p className="p-2 text-sm text-muted-foreground">{t("none")}</p> : null}
          {options.map((option) => (
            <Label key={option.value} className="flex min-h-11 cursor-pointer items-center gap-3 rounded px-2 hover:bg-accent">
              <Checkbox checked={selected.includes(option.value)} onCheckedChange={(checked) => toggle(option.value, checked === true)} />
              <span className="text-sm font-normal">{option.label}</span>
            </Label>
          ))}
        </div>
        {selected.length > 0 ? (
          <Button variant="ghost" size="sm" className="mt-1 w-full" onClick={() => onChange([])}>
            {t("clear")}
          </Button>
        ) : null}
      </PopoverContent>
    </Popover>
  );
}
