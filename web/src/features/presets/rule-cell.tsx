"use client";

import { Info } from "lucide-react";
import { useLocale, useTranslations } from "next-intl";
import type { ReactNode } from "react";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { Badge } from "@/shared/ui/badge";
import { Popover, PopoverContent, PopoverTrigger } from "@/shared/ui/popover";
import type { SourceRef } from "./api/types";
import { formatCalendarDate } from "./lib/dates";

export type Provenance<V> = {
  status: "verified" | "not_confirmed";
  value?: V;
  sources?: SourceRef[];
  published_values?: { value: V; sources: SourceRef[] }[];
};

/** Only https links from preset data reach an `href`. */
const safeUrl = (url: string) => (/^https:\/\//i.test(url) ? url : undefined);

export function SourceList({ sources }: { sources: SourceRef[] }) {
  const t = useTranslations("presets.rule");
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  return (
    <ul className="space-y-1 text-xs">
      {sources.map((source, index) => {
        const href = safeUrl(source.url);
        const text = t("source", { page: source.page_title ?? source.url, date: formatCalendarDate(source.accessed_at, locale) });
        return (
          <li key={`${index}|${source.url}`}>
            {href ? (
              <a className="underline underline-offset-4" href={href} target="_blank" rel="noopener noreferrer">
                {text}
                <span className="sr-only"> {t("newTab")}</span>
              </a>
            ) : (
              text
            )}
          </li>
        );
      })}
    </ul>
  );
}

/**
 * One rule value of a preset with its provenance (F§7.4): verified → the value and a source popover; not confirmed →
 * the badge and every published value with its sources, nothing chosen (the choice is made on the account).
 */
export function RuleCell<V>({ rule, format, label }: { rule: Provenance<V> | undefined; format: (value: V) => ReactNode; label: string }) {
  const t = useTranslations("presets.rule");
  if (rule === undefined) return <>{"—"}</>;
  if (rule.status === "not_confirmed")
    return (
      <div className="space-y-1">
        <Badge tone="warning">{t("notConfirmed")}</Badge>
        <ul className="space-y-1 text-xs">
          {(rule.published_values ?? []).map((published, index) => (
            <li key={index}>
              <div>{format(published.value)}</div>
              <SourceList sources={published.sources} />
            </li>
          ))}
        </ul>
      </div>
    );
  if (rule.value === undefined) return <>{"—"}</>;
  return (
    <span className="inline-flex items-center gap-1">
      {format(rule.value)}
      {rule.sources && rule.sources.length > 0 ? (
        <Popover>
          <PopoverTrigger className="inline-flex size-6 items-center justify-center rounded-md hover:bg-muted" aria-label={t("sourcesFor", { field: label })}>
            <Info aria-hidden="true" className="size-4" />
          </PopoverTrigger>
          <PopoverContent>
            <SourceList sources={rule.sources} />
          </PopoverContent>
        </Popover>
      ) : null}
    </span>
  );
}
