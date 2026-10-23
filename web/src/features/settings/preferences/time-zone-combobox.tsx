"use client";

import { Check, ChevronDown } from "lucide-react";
import { useTranslations } from "next-intl";
import { useId, useMemo, useState } from "react";
import { Popover, PopoverContent, PopoverTrigger } from "@/shared/ui/popover";
import { cn } from "@/shared/lib/utils";
import { buildTimeZoneOptions, filterTimeZones, groupTimeZones, supportedTimeZones } from "./time-zones";

type TimeZoneComboboxProps = {
  id: string;
  value: string;
  onChange: (timeZone: string) => void;
  disabled?: boolean;
  invalid?: boolean;
  describedBy?: string;
};

/** Searchable IANA time-zone picker (Popover + listbox, WAI-ARIA combobox pattern). */
export function TimeZoneCombobox({ id, value, onChange, disabled, invalid, describedBy }: TimeZoneComboboxProps) {
  const t = useTranslations("settings.preferences.timeZone");
  const listId = useId();
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [active, setActive] = useState(0);

  const options = useMemo(() => buildTimeZoneOptions(supportedTimeZones(value), new Date()), [value]);
  const filtered = useMemo(() => filterTimeZones(options, query), [options, query]);
  const groups = useMemo(() => groupTimeZones(filtered), [filtered]);
  const selected = options.find((option) => option.id === value);
  const activeOption = filtered[Math.min(active, filtered.length - 1)];

  function choose(timeZone: string) {
    setOpen(false);
    if (timeZone !== value) onChange(timeZone);
  }

  return (
    <Popover
      open={open}
      onOpenChange={(next) => {
        setOpen(next);
        setQuery("");
        setActive(Math.max(0, options.findIndex((option) => option.id === value)));
      }}
    >
      <PopoverTrigger asChild>
        <button
          id={id}
          type="button"
          role="combobox"
          aria-expanded={open}
          aria-controls={listId}
          aria-invalid={invalid}
          aria-describedby={describedBy}
          disabled={disabled}
          className="flex min-h-11 w-full items-center justify-between gap-2 rounded-md border border-input bg-background px-3 py-2 text-left text-base outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive"
        >
          <span className="truncate">{selected?.label ?? value}</span>
          <ChevronDown aria-hidden="true" className="size-4 shrink-0 opacity-60" />
        </button>
      </PopoverTrigger>
      <PopoverContent>
        <input
          type="text"
          role="combobox"
          aria-expanded="true"
          aria-controls={listId}
          aria-activedescendant={activeOption ? `${listId}-${activeOption.id}` : undefined}
          aria-label={t("search")}
          autoComplete="off"
          value={query}
          onChange={(event) => {
            setQuery(event.target.value);
            setActive(0);
          }}
          onKeyDown={(event) => {
            if (event.key === "ArrowDown") {
              event.preventDefault();
              setActive((index) => Math.min(index + 1, filtered.length - 1));
            } else if (event.key === "ArrowUp") {
              event.preventDefault();
              setActive((index) => Math.max(index - 1, 0));
            } else if (event.key === "Enter" && activeOption) {
              event.preventDefault();
              choose(activeOption.id);
            }
          }}
          className="w-full border-b bg-transparent px-3 py-3 text-base outline-none"
        />
        <div id={listId} role="listbox" aria-label={t("label")} className="max-h-72 overflow-y-auto p-1">
          {filtered.length === 0 ? <p className="px-3 py-4 text-sm text-muted-foreground">{t("noResults")}</p> : null}
          {groups.map((group) => (
            <div key={group.region} role="group" aria-label={group.region}>
              <p aria-hidden="true" className="px-2 pt-2 pb-1 text-xs font-medium text-muted-foreground">
                {group.region}
              </p>
              {group.options.map((option) => (
                <div
                  key={option.id}
                  id={`${listId}-${option.id}`}
                  role="option"
                  aria-selected={option.id === value}
                  onClick={() => choose(option.id)}
                  onMouseMove={() => setActive(filtered.indexOf(option))}
                  ref={(node) => {
                    if (node && option.id === activeOption?.id) node.scrollIntoView?.({ block: "nearest" });
                  }}
                  className={cn("flex min-h-11 cursor-default items-center gap-2 rounded-sm px-2 text-sm", option.id === activeOption?.id && "bg-accent text-accent-foreground")}
                >
                  <Check aria-hidden="true" className={cn("size-4 shrink-0", option.id !== value && "invisible")} />
                  {option.label}
                </div>
              ))}
            </div>
          ))}
        </div>
      </PopoverContent>
    </Popover>
  );
}
