"use client";

import { Check, ChevronDown } from "lucide-react";
import { useTranslations } from "next-intl";
import { useId, useMemo, useState } from "react";
import { cn } from "@/shared/lib/utils";
import { Popover, PopoverContent, PopoverTrigger } from "@/shared/ui/popover";

export type ComboboxOption = { value: string; label: string; group?: string; /** Extra text matched by the search. */ keywords?: string };

type ComboboxProps = {
  id: string;
  options: ComboboxOption[];
  value: string | null;
  onChange: (value: string) => void;
  /** Accessible name of the list and the search field. */
  label: string;
  placeholder?: string;
  disabled?: boolean;
  invalid?: boolean;
  describedBy?: string;
};

const fold = (text: string) => text.normalize("NFD").replace(/\p{Diacritic}/gu, "").toLowerCase();

/** Searchable single-choice picker (Popover + listbox, WAI-ARIA combobox). For short, known option lists. */
export function Combobox({ id, options, value, onChange, label, placeholder, disabled, invalid, describedBy }: ComboboxProps) {
  const t = useTranslations("common.combobox");
  const listId = useId();
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [active, setActive] = useState(0);
  const filtered = useMemo(() => {
    const needle = fold(query.trim());
    return needle ? options.filter((option) => fold(`${option.label} ${option.keywords ?? ""}`).includes(needle)) : options;
  }, [options, query]);
  const selected = options.find((option) => option.value === value);
  const activeOption = filtered[Math.min(active, filtered.length - 1)];

  function choose(next: string) {
    setOpen(false);
    if (next !== value) onChange(next);
  }

  return (
    <Popover
      open={open}
      onOpenChange={(next) => {
        setOpen(next);
        setQuery("");
        setActive(Math.max(0, options.findIndex((option) => option.value === value)));
      }}
    >
      <PopoverTrigger asChild>
        <button
          id={id}
          type="button"
          role="combobox"
          aria-expanded={open}
          aria-controls={listId}
          aria-label={`${label}: ${selected?.label ?? placeholder ?? t("choose")}`}
          aria-invalid={invalid}
          aria-describedby={describedBy}
          disabled={disabled}
          className="flex min-h-11 w-full items-center justify-between gap-2 rounded-md border border-input bg-background px-3 py-2 text-left text-base outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive"
        >
          <span className={cn("truncate", !selected && "text-muted-foreground")}>{selected?.label ?? placeholder ?? t("choose")}</span>
          <ChevronDown aria-hidden="true" className="size-4 shrink-0 opacity-60" />
        </button>
      </PopoverTrigger>
      <PopoverContent>
        <input
          type="text"
          role="combobox"
          aria-expanded="true"
          aria-controls={listId}
          aria-activedescendant={activeOption ? `${listId}-${activeOption.value}` : undefined}
          aria-label={t("search", { label })}
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
              choose(activeOption.value);
            }
          }}
          className="w-full border-b bg-transparent px-3 py-3 text-base outline-none"
        />
        <div id={listId} role="listbox" aria-label={label} className="max-h-72 overflow-y-auto p-1">
          {filtered.length === 0 ? <p className="px-3 py-4 text-sm text-muted-foreground">{t("noResults")}</p> : null}
          {filtered.map((option, position) => (
            <div key={option.value}>
              {option.group && option.group !== filtered[position - 1]?.group ? (
                <p aria-hidden="true" className="px-2 pt-2 pb-1 text-xs font-medium text-muted-foreground">
                  {option.group}
                </p>
              ) : null}
              <div
                id={`${listId}-${option.value}`}
                role="option"
                aria-selected={option.value === value}
                onClick={() => choose(option.value)}
                onMouseMove={() => setActive(position)}
                ref={(node) => {
                  if (node && option.value === activeOption?.value) node.scrollIntoView?.({ block: "nearest" });
                }}
                className={cn("flex min-h-11 cursor-default items-center gap-2 rounded-sm px-2 text-sm", option.value === activeOption?.value && "bg-accent text-accent-foreground")}
              >
                <Check aria-hidden="true" className={cn("size-4 shrink-0", option.value !== value && "invisible")} />
                {option.label}
              </div>
            </div>
          ))}
        </div>
      </PopoverContent>
    </Popover>
  );
}
