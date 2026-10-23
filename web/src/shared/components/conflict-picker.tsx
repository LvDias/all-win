"use client";

import { ExternalLink } from "lucide-react";
import { useId, type ReactNode } from "react";
import { useTranslations } from "next-intl";
import { formatDate } from "@/shared/format";
import { useDisplay } from "@/shared/session/display";
import { Label } from "@/shared/ui/label";
import { RadioGroup, RadioGroupItem } from "@/shared/ui/radio-group";

export type ConflictSource = { url: string; accessed_at: string; page_title?: string };
export type ConflictOption = { id: string; label: string; sources: ConflictSource[] };

/** The sentinel `value` for "another value"; option ids come from the caller and never equal it. */
export const otherValue = "__other__";

type ConflictPickerProps = {
  legend: string;
  options: ConflictOption[];
  /** Selected option id, `otherValue`, or "" — nothing is preselected and nothing is ever suggested (S16 AC-6, F§7.4). */
  value: string;
  onValueChange: (value: string) => void;
  /** The field's own input, shown when "another value" is chosen. */
  otherInput?: ReactNode;
  error?: string;
  /** Pass the form field's id so the error can be tied to it. */
  id?: string;
};

/** Only https links (as `FirmRuleQuote`) from firm data reach an `href`. */
const safeUrl = (url: string) => (/^https:\/\//i.test(url) ? url : undefined);

/** Official pages disagree: one choice per published value with its sources, plus "another value" (foundations F§7.4). */
export function ConflictPicker({ legend, options, value, onValueChange, otherInput, error, id }: ConflictPickerProps) {
  const t = useTranslations("conflictPicker");
  const { locale } = useDisplay();
  const base = useId();
  const errorId = `${base}-error`;
  return (
    <fieldset id={id} className="space-y-2 rounded-lg border border-status-warning p-3" aria-describedby={error ? errorId : undefined}>
      <legend className="px-1 text-sm font-medium">{legend}</legend>
      <p className="text-sm">{t("intro")}</p>
      <RadioGroup value={value} onValueChange={onValueChange} aria-label={legend} aria-invalid={error ? true : undefined}>
        {options.map((option) => (
          <div key={option.id} className="space-y-1">
            <Label htmlFor={`${base}-${option.id}`} className="flex min-h-11 cursor-pointer items-center gap-3 rounded-lg border p-3">
              <RadioGroupItem id={`${base}-${option.id}`} value={option.id} />
              {option.label}
            </Label>
            <ul className="space-y-0.5 pl-3 text-xs text-muted-foreground">
              {option.sources.map((source, index) => {
                const href = safeUrl(source.url);
                const text = t("source", { page: source.page_title ?? source.url, date: formatDate(`${source.accessed_at}T12:00:00Z`, locale, "UTC") });
                return (
                  <li key={`${index}|${source.url}`}>
                    {href ? (
                      <a href={href} target="_blank" rel="noopener noreferrer" className="inline-flex items-center gap-1 underline underline-offset-4">
                        {text}
                        <ExternalLink aria-hidden="true" className="size-3" />
                        <span className="sr-only">{t("newTab")}</span>
                      </a>
                    ) : (
                      text
                    )}
                  </li>
                );
              })}
            </ul>
          </div>
        ))}
        <Label htmlFor={`${base}-other`} className="flex min-h-11 cursor-pointer items-center gap-3 rounded-lg border p-3">
          <RadioGroupItem id={`${base}-other`} value={otherValue} />
          {t("other")}
        </Label>
      </RadioGroup>
      {value === otherValue ? otherInput : null}
      {error ? (
        <p id={errorId} className="text-sm text-status-danger">
          {error}
        </p>
      ) : null}
    </fieldset>
  );
}
