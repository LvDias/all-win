import { TriangleAlert } from "lucide-react";
import { useLocale, useTranslations } from "next-intl";
import { useId } from "react";
import { formatDate } from "@/shared/format";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { Alert } from "@/shared/ui/alert";
import { Checkbox } from "@/shared/ui/checkbox";
import { Label } from "@/shared/ui/label";
import { RadioGroup, RadioGroupItem } from "@/shared/ui/radio-group";
import type { OptInChoice, OptInKind, RuleVersion } from "./api/types";
import { FirmRuleQuote } from "@/shared/components/firm-rule-quote";
import type { RuleQuote } from "./api/types";

export type OptInValue = { choice: OptInChoice | ""; ack: boolean };

type Props = {
  kind: OptInKind;
  rule: RuleVersion;
  value: OptInValue;
  onChange: (value: OptInValue) => void;
  /** Translated error under the legend (no choice) and under the checkbox (no ack). */
  choiceError?: string;
  ackError?: string;
  /** Info notice above the section (rule changed with the selection / stale rule). */
  notice?: string;
  /** Previous version text shown next to the new one in the review dialog. */
  previous?: { rule: RuleVersion; label: string };
  firmFallback: string;
};

/**
 * Per-account opt-in (spec §5.3). DOM order = reading order: summary, quote, risk warning, then the choice.
 * Nothing is ever preselected. `allowed` shows an info line and no question (the caller omits the group by not
 * rendering this component when the choice is not required).
 */
export function OptInSection({ kind, rule, value, onChange, choiceError, ackError, notice, previous, firmFallback }: Props) {
  const t = useTranslations(`accounts.optIn.${kind}`);
  const tc = useTranslations("accounts.optIn");
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const warningId = useId();
  const errorId = useId();
  const firm = rule.firm_name ?? firmFallback;
  const stance = rule.firm_stance === "allowed" ? null : rule.firm_stance;

  return (
    <fieldset className="space-y-4" aria-describedby={choiceError ? errorId : undefined}>
      <legend className="text-base font-semibold">{t("legend")}</legend>
      {kind === "automation" ? <p className="text-sm text-muted-foreground">{t("help")}</p> : null}
      {notice ? <Alert tone="warning" role="status">{notice}</Alert> : null}

      {stance === null ? (
        <p className="text-sm">{t("allowed", { firm })}</p>
      ) : (
        <>
          <div className="space-y-2 text-sm">
            <p className="font-medium">{tc("summaryLabel")}</p>
            <p>{`${t(`stance.${stance}`, { firm })} ${t("stanceSecond")}`}</p>
          </div>
          {stance === "not_confirmed" ? <p className="text-sm font-medium">{tc("conflicting")}</p> : null}
          {rule.quotes.map((q) => <RuleQuoteItem key={q.url + q.accessed_at + q.text.slice(0, 20)} quote={q} firm={firm} />)}
          {stance === "unknown" && rule.research ? (
            <div className="space-y-1 text-sm">
              <p>{tc("researched", { date: formatDate(`${rule.research.searched_on}T00:00:00Z`, locale, "UTC") })}</p>
              <ul className="list-disc pl-5">
                {rule.research.sources.map((s) => (
                  <li key={s.url}>
                    <a className="underline underline-offset-4" href={s.url} target="_blank" rel="noopener noreferrer">{s.page_title ?? s.url}</a>
                  </li>
                ))}
              </ul>
            </div>
          ) : null}
          {previous ? (
            <details className="text-sm">
              <summary className="min-h-11 cursor-pointer">{previous.label}</summary>
              <div className="space-y-2 pt-2">
                {previous.rule.quotes.map((q) => <RuleQuoteItem key={q.url + q.accessed_at} quote={q} firm={previous.rule.firm_name ?? firm} />)}
              </div>
            </details>
          ) : null}

          <Alert tone="danger" id={warningId}>
            <TriangleAlert aria-hidden />
            <span>{tc("risk")}</span>
          </Alert>

          <RadioGroup
            aria-label={t("legend")}
            aria-describedby={choiceError ? `${warningId} ${errorId}` : warningId}
            aria-invalid={choiceError ? true : undefined}
            required
            value={value.choice}
            onValueChange={(choice) => onChange({ choice: choice as OptInChoice, ack: false })}
          >
            {(["declined", "accepted"] as const).map((option) => {
              const id = `${warningId}-${option}`;
              const key = option === "declined" ? "decline" : "accept";
              return (
                <Label key={option} htmlFor={id} className="flex min-h-11 cursor-pointer items-start gap-3 rounded-lg border p-3">
                  <RadioGroupItem id={id} value={option} />
                  <span className="space-y-1">
                    <span className="block font-medium">{t(`${key}.label`)}</span>
                    <span className="block text-sm font-normal text-muted-foreground">{t(`${key}.description`)}</span>
                  </span>
                </Label>
              );
            })}
          </RadioGroup>
          {choiceError ? <p id={errorId} className="text-sm text-destructive">{choiceError}</p> : null}

          {value.choice === "accepted" ? (
            <div className="space-y-1">
              <Label className="flex min-h-11 items-start gap-3 font-normal">
                <Checkbox checked={value.ack} aria-invalid={ackError ? true : undefined} onCheckedChange={(ack) => onChange({ ...value, ack: ack === true })} />
                <span>{t("ack")}</span>
              </Label>
              {ackError ? <p className="text-sm text-destructive">{ackError}</p> : null}
            </div>
          ) : null}
        </>
      )}
    </fieldset>
  );
}

/** Plain text only: the firm's wording is never HTML and never translated. Noon UTC keeps the date-only day stable in any time zone. */
function RuleQuoteItem({ quote, firm }: { quote: RuleQuote; firm: string }) {
  return <FirmRuleQuote firm={firm} pageTitle={[quote.page_title, quote.section].filter(Boolean).join(" · ")} accessedAt={`${quote.accessed_at}T12:00:00Z`} url={quote.url} text={quote.text} lang={quote.language} />;
}
