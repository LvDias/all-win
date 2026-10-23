"use client";

import { Pencil } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useLocale, useTranslations } from "next-intl";
import { useState, type ReactNode } from "react";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { formatDate, formatDateTime } from "@/shared/format";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { withLocale } from "@/shared/i18n/paths";
import { Alert } from "@/shared/ui/alert";
import { Button, buttonVariants } from "@/shared/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/shared/ui/card";
import { ErrorState } from "@/shared/ui/error-state";
import { Skeleton } from "@/shared/ui/skeleton";
import { AccountActions } from "./account-actions";
import { AccountStateBadges, OptInChoiceBadge } from "./account-state-badges";
import { PlatformAccountText, useAccountSummary } from "./account-summary";
import { useAccount } from "./api/use-accounts";
import type { Account, OptInKind, OptInState } from "./api/types";
import { useLifecycleQuickAction } from "./lifecycle-dialog";
import { visibleOptInCards } from "./lib/opt-in";
import { OptInReviewDialog } from "./opt-in-review-dialog";
import { useRuleVersion } from "./api/use-accounts";
import type { StepUpRunner } from "./lib/use-step-up";
import { PageHeader } from "@/shared/components/page-header";

function OptInCard({ account, state, onReview }: { account: Account; state: OptInState; onReview: (kind: OptInKind) => void }) {
  const t = useTranslations("accounts.detail");
  const tr = useTranslations("accounts.optIn.review");
  const tStance = useTranslations("accounts.stance");
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const kind = state.kind;
  const current = useRuleVersion(state.current_rule_version_id);
  const chosenRule = useRuleVersion(state.state === "chosen" ? state.rule_version_id : undefined);
  const firm = account.firm_name ?? "";
  const pending = state.state === "pending_review";
  const change = current.data?.change;
  const ref = current.data;
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t(`cards.${kind}`)}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3 text-sm">
        {pending ? (
          <>
            <Alert tone="warning">
              <div className="space-y-1">
                <p>
                  {state.pending_reason === "preset_attached" || !ref
                    ? t("pendingNoPrevious")
                    : t(kind === "automation" ? "pendingAutomation" : "pendingCopy", { firm, date: formatDate(ref.published_at, locale, "UTC"), n: ref.version_number })}
                </p>
                {change?.cause === "stance_changed" ? <p>{t("whatChanged.stance", { old: tStance(change.previous_firm_stance), new: ref ? tStance(ref.firm_stance) : "" })}</p> : null}
                {change?.cause === "text_changed" && change.reason ? <p>{t("whatChanged.text", { reason: change.reason[locale === "en" ? "en" : "pt-BR"] })}</p> : null}
              </div>
            </Alert>
            <Button type="button" onClick={() => onReview(kind)}>{tr("now")}</Button>
          </>
        ) : state.choice && state.chosen_at ? (
          <>
            <OptInChoiceBadge kind={kind} choice={state.choice} />
            <p>{t("chosenBy", { date: formatDateTime(state.chosen_at, locale, "UTC"), firm: chosenRule.data?.firm_name ?? firm, n: chosenRule.data?.version_number ?? "" })}</p>
            <Button type="button" variant="outline" onClick={() => onReview(kind)}>{tr("change")}</Button>
          </>
        ) : null}
      </CardContent>
    </Card>
  );
}

/** `presetSection` is composed by the page (features never import each other): the S16 preset panel. */
export function AccountDetail({ accountId, stepUp, presetSection }: { accountId: string; stepUp?: StepUpRunner; presetSection?: ReactNode }) {
  const t = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const router = useRouter();
  const account = useAccount(accountId);
  const summary = useAccountSummary();
  const quick = useLifecycleQuickAction();
  const [review, setReview] = useState<OptInKind | null>(null);
  const listHref = withLocale(locale, "/accounts");

  if (account.isPending)
    return (
      <div className="space-y-4" aria-busy="true" role="status">
        <span className="sr-only">{t("common.states.loading")}</span>
        <Skeleton className="h-10 w-64" />
        {[0, 1, 2, 3].map((i) => <Skeleton key={i} className="h-28 w-full" />)}
      </div>
    );
  if (account.isError) {
    const notFound = account.error instanceof ApiError && account.error.status === 404;
    return (
      <div className="space-y-4">
        <ErrorState title={t(notFound ? "accounts.detail.notFound" : "accounts.detail.loadError")} code={errorMessage(t, account.error).code} onRetry={notFound ? undefined : () => void account.refetch()} />
        <Link className="text-sm underline underline-offset-4" href={listHref}>{t("accounts.detail.backToList")}</Link>
      </div>
    );
  }

  const a = account.data;
  const cards = visibleOptInCards(a);
  const inactive = a.lifecycle_state !== "active";
  return (
    <div className="space-y-6">
      <PageHeader
        title={a.display_name}
        description={`${t(`accounts.type.${a.kind}`)} · ${summary(a)}`}
        toggle={<AccountStateBadges account={a} max={4} />}
        actions={
          <>
          <Link className={buttonVariants({ variant: "outline" })} href={withLocale(locale, `/accounts/${a.id}/edit`)}><Pencil aria-hidden />{t("accounts.actions.edit")}</Link>
          {inactive ? (
            <Button type="button" disabled={quick.pending} onClick={() => quick.run(a)}>
              {t(a.lifecycle_state === "paused" ? "accounts.actions.resume" : "accounts.actions.enable")}
            </Button>
          ) : null}
          <AccountActions account={a} showView={false} onDeleted={() => router.push(listHref)} />
          </>
        }
      />

      {inactive ? <Alert tone="info">{t(a.lifecycle_state === "paused" ? "accounts.detail.pausedNote" : "accounts.detail.disabledNote")}</Alert> : null}

      <Card>
        <CardHeader><CardTitle>{t("accounts.detail.cards.data")}</CardTitle></CardHeader>
        <CardContent className="space-y-1 text-sm">
          <p>{summary(a)}</p>
          {a.firm_platform ? <p>{t("accounts.detail.firmPlatform", { value: a.firm_platform })}</p> : null}
          <p>{`${t("accounts.list.columns.maxPosition")}: ${t("accounts.list.contracts", { count: a.max_position_qty })} · ${t("accounts.list.columns.multiplier")}: ${a.multiplier}×`}</p>
          <p className="text-muted-foreground">{t("accounts.detail.createdAt", { date: formatDateTime(a.created_at, locale, "UTC") })}</p>
        </CardContent>
      </Card>

      {presetSection}

      <Card>
        <CardHeader><CardTitle>{t("accounts.detail.cards.platform")}</CardTitle></CardHeader>
        <CardContent className="space-y-2 text-sm">
          <PlatformAccountText account={a} />
          {a.binding && a.binding.oco_location !== "server" ? <p className="text-muted-foreground">{t("accounts.form.binding.oco")}</p> : null}
          <div>
            <Link className={buttonVariants({ variant: "outline" })} href={withLocale(locale, `/accounts/${a.id}/edit`)}>
              {t(a.binding ? "accounts.actions.edit" : "accounts.detail.link")}
            </Link>
          </div>
        </CardContent>
      </Card>

      {cards.map((s) => <OptInCard key={s.kind} account={a} state={s} onReview={setReview} />)}

      <Card>
        <CardHeader><CardTitle>{t("accounts.detail.cards.position")}</CardTitle></CardHeader>
        <CardContent className="text-sm">{t(a.has_open_position ? "accounts.detail.openPosition.yes" : "accounts.detail.openPosition.no")}</CardContent>
      </Card>

      {review ? <OptInReviewDialog account={a} kind={review} stepUp={stepUp} onClose={() => setReview(null)} /> : null}
    </div>
  );
}
