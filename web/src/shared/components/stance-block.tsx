import { useTranslations } from "next-intl";
import type { ReactNode } from "react";
import { FirmRuleQuote } from "@/shared/components/firm-rule-quote";
import { StanceBadge, type StanceSubject, type StanceValue } from "@/shared/components/stance-badge";
import { formatDate } from "@/shared/format";
import { useDisplay } from "@/shared/session/display";

export type StanceQuote = { text: string; language: string; page_title: string; url: string; accessed_at: string };

type StanceBlockProps = {
  subject: StanceSubject;
  firm: string;
  value: StanceValue;
  /** The firm's own texts, verbatim. With `not_confirmed` every conflicting text is listed. */
  quotes: StanceQuote[];
  /** Set for `unknown`: when and where we looked. */
  searchedOn?: string;
  /** Platform this stance applies to, when it differs per platform. */
  platformLabel?: string;
  /** "Verified on {date}" line. */
  verifiedOn?: string;
  /** Hides the quotes behind a disclosure (compact form in pickers and dialogs). */
  children?: ReactNode;
};

/** Heading + badge + the firm's verbatim quotes (S16 §7). Our reading of a stance is never invented here: the summary is fixed copy per value. */
export function StanceBlock({ subject, firm, value, quotes, searchedOn, platformLabel, verifiedOn, children }: StanceBlockProps) {
  const t = useTranslations("stanceBlock");
  const { locale } = useDisplay();
  const known = value !== "unknown" && value !== "varies_by_platform";
  return (
    <section className="space-y-3 rounded-lg border p-4" aria-label={t(`heading.${subject}`)}>
      <div className="flex flex-wrap items-center gap-2">
        <h3 className="text-base font-medium">{t(`heading.${subject}`)}</h3>
        <StanceBadge subject={subject} value={value} />
        {platformLabel ? <span className="text-sm text-muted-foreground">{platformLabel}</span> : null}
      </div>
      <p className="text-sm">
        <span className="font-medium">{t("summaryLabel")}</span> {t(`summary.${subject}.${value}`)}
      </p>
      {value === "not_confirmed" ? <p className="text-sm font-medium">{t("diverge")}</p> : null}
      {value === "unknown" && searchedOn ? <p className="text-sm">{t("unknownSearched", { date: formatDate(`${searchedOn}T12:00:00Z`, locale, "UTC") })}</p> : null}
      {known
        ? quotes.map((q) => <FirmRuleQuote key={`${q.url}|${q.text}`} firm={firm} pageTitle={q.page_title} accessedAt={q.accessed_at} url={q.url} text={q.text} lang={q.language} />)
        : null}
      {children}
      {verifiedOn ? <p className="text-xs text-muted-foreground">{t("verifiedOn", { date: formatDate(`${verifiedOn}T12:00:00Z`, locale, "UTC") })}</p> : null}
    </section>
  );
}
