"use client";

import { ExternalLink } from "lucide-react";
import { useTranslations } from "next-intl";
import { useState } from "react";
import { formatDate } from "@/shared/format";
import { useDisplay } from "@/shared/session/display";
import { cn } from "@/shared/lib/utils";
import { Button } from "@/shared/ui/button";

type FirmRuleQuoteProps = {
  firm: string;
  pageTitle: string;
  /** ISO instant the page was read. */
  accessedAt: string;
  url: string;
  /** The rule exactly as published, never translated or summarized. */
  text: string;
  /** Language of the published text (`en` for an English page), so screen readers switch pronunciation. */
  lang: string;
  /** Optional plain-language summary shown above the quote, labeled as ours. */
  summary?: string;
};

/** An official firm rule, verbatim (foundations F§7.4). The full text is always in the DOM; only its height is limited. */
/** Only https links are rendered as links: the URL comes from the preset data, never from the user, but a `javascript:` value must not become an href. */
const safeUrl = (value: string): string | null => (/^https:\/\//i.test(value) ? value : null);

export function FirmRuleQuote({ firm, pageTitle, accessedAt, url, text, lang, summary }: FirmRuleQuoteProps) {
  const t = useTranslations("firmRule");
  const { locale, timeZone } = useDisplay();
  const [expanded, setExpanded] = useState(false);
  const long = text.split("\n").length > 6 || text.length > 360;
  return (
    <figure className="grid gap-2">
      {summary ? (
        <p className="text-sm">
          <span className="font-medium">{t("summaryLabel")}</span> {summary}
        </p>
      ) : null}
      <blockquote cite={safeUrl(url) ?? undefined} lang={lang} className={cn("rounded-r-md border-l-4 border-status-info bg-muted p-3 text-sm whitespace-pre-line", long && !expanded && "line-clamp-6")}>
        {text}
      </blockquote>
      {long ? (
        <div>
          <Button type="button" variant="link" size="sm" aria-expanded={expanded} onClick={() => setExpanded((open) => !open)}>
            {expanded ? t("showLess") : t("showFull")}
          </Button>
        </div>
      ) : null}
      <figcaption className="flex flex-wrap items-center gap-x-2 text-xs text-muted-foreground">
        <span>{t("caption", { firm, page: pageTitle, date: formatDate(accessedAt, locale, timeZone) })}</span>
        {safeUrl(url) ? (
          <a href={safeUrl(url) ?? undefined} target="_blank" rel="noopener noreferrer" className="inline-flex items-center gap-1 underline underline-offset-4">
            {t("viewPage")}
            <ExternalLink aria-hidden="true" className="size-3" />
            <span className="sr-only">{t("newTab")}</span>
          </a>
        ) : null}
      </figcaption>
    </figure>
  );
}
