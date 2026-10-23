"use client";

import { useTranslations } from "next-intl";
import type { ReactNode } from "react";
import { FirmRuleQuote } from "@/shared/components/firm-rule-quote";
import { StanceBadge, type StanceValues } from "@/shared/components/stance-badge";
import { formatDate } from "@/shared/format";
import { useDisplay } from "@/shared/session/display";

export type StanceQuote = { text: string; language: string; page_title: string; url: string; accessed_at: string };

/** A block shows one concrete position: `varies_by_platform` is expanded into one block per platform before it gets here. */
type BlockValues = { [S in keyof StanceValues]: Exclude<StanceValues[S], "varies_by_platform"> };
type Stance = { [S in keyof BlockValues]: { subject: S; value: BlockValues[S] } }[keyof BlockValues];

/** Exactly the summary keys the catalogs define (one per valid subject and value). */
type SummaryKey = { [S in keyof BlockValues]: `summary.${S}.${BlockValues[S]}` }[keyof BlockValues];

type StanceBlockProps = Stance & {
  firm: string;
  /** The firm's own texts, verbatim. With `not_confirmed` every conflicting text is listed. */
  quotes: StanceQuote[];
  /** Set for `unknown`: when and where we looked. */
  searchedOn?: string;
  /** Platform this stance applies to, when it differs per platform. */
  platformLabel?: string;
  /** "Verified on {date}" line. */
  verifiedOn?: string;
  children?: ReactNode;
};

/** Heading + badge + the firm's verbatim quotes (S16 §7). The summary is fixed, reviewed copy per subject and value (Aquarela), never free text. */
export function StanceBlock({ firm, quotes, searchedOn, platformLabel, verifiedOn, children, ...stance }: StanceBlockProps) {
  const t = useTranslations("stanceBlock");
  const { locale } = useDisplay();
  const { subject, value } = stance;
  // `unknown` has no quote to show; every other position quotes the firm (with `not_confirmed`, all the conflicting texts).
  const quoted = value !== "unknown";
  return (
    <section className="space-y-3 rounded-lg border p-4" aria-label={t(`heading.${subject}`)}>
      <div className="flex flex-wrap items-center gap-2">
        <h3 className="text-base font-medium">{t(`heading.${subject}`)}</h3>
        <StanceBadge {...stance} />
        {platformLabel ? <span className="text-sm text-muted-foreground">{platformLabel}</span> : null}
      </div>
      <p className="text-sm">
        <span className="font-medium">{t("summaryLabel")}</span> {t(`summary.${subject}.${value}` as SummaryKey, { firm })}
      </p>
      {value === "not_confirmed" ? <p className="text-sm font-medium">{t("diverge")}</p> : null}
      {value === "unknown" && searchedOn ? <p className="text-sm">{t("unknownSearched", { date: formatDate(`${searchedOn}T12:00:00Z`, locale, "UTC") })}</p> : null}
      {quoted ? quotes.map((q, index) => <FirmRuleQuote key={`${index}|${q.url}`} firm={firm} pageTitle={q.page_title} accessedAt={`${q.accessed_at}T12:00:00Z`} url={q.url} text={q.text} lang={q.language} />) : null}
      {children}
      {verifiedOn ? <p className="text-xs text-muted-foreground">{t("verifiedOn", { date: formatDate(`${verifiedOn}T12:00:00Z`, locale, "UTC") })}</p> : null}
    </section>
  );
}
