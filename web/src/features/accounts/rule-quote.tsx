import { ExternalLink } from "lucide-react";
import { useLocale, useTranslations } from "next-intl";
import { useState } from "react";
import { formatDate } from "@/shared/format";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { Button } from "@/shared/ui/button";
import type { RuleQuote } from "./api/types";

const previewLength = 280;

/**
 * INTERIM for the shared `FirmRuleQuote` (F§7.4). The firm's text is rendered as plain text only,
 * in the firm's language (`lang`), never translated, never as HTML.
 */
export function RuleQuoteBlock({ quote, firmName }: { quote: RuleQuote; firmName: string }) {
  const t = useTranslations("accounts.optIn.quote");
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const [expanded, setExpanded] = useState(false);
  const long = quote.text.length > previewLength;
  const text = expanded || !long ? quote.text : `${quote.text.slice(0, previewLength)}…`;
  return (
    <blockquote className="border-l-4 border-border pl-3 text-sm">
      <p lang={quote.language} className="whitespace-pre-line">{text}</p>
      <footer className="mt-1 space-y-1 text-xs text-muted-foreground">
        <span>{t("source", { title: [quote.page_title, quote.section].filter(Boolean).join(" · "), date: formatDate(`${quote.accessed_at}T00:00:00Z`, locale, "UTC") })}</span>
        <div className="flex flex-wrap items-center gap-2">
          <a className="inline-flex min-h-11 items-center gap-1 underline underline-offset-4" href={quote.url} target="_blank" rel="noopener noreferrer">
            {t("open", { firm: firmName })}
            <ExternalLink aria-hidden className="size-3" />
          </a>
          {long ? (
            <Button type="button" variant="ghost" size="sm" aria-expanded={expanded} onClick={() => setExpanded((v) => !v)}>
              {expanded ? t("hide") : t("show")}
            </Button>
          ) : null}
        </div>
      </footer>
    </blockquote>
  );
}
