"use client";

import { Eye, SearchX } from "lucide-react";
import Link from "next/link";
import { useLocale, useTranslations } from "next-intl";
import { useState } from "react";
import { errorMessage } from "@/shared/api/error-message";
import { PageHeader } from "@/shared/components/page-header";
import { StanceBadge } from "@/shared/components/stance-badge";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { withLocale } from "@/shared/i18n/paths";
import { Badge } from "@/shared/ui/badge";
import { Button } from "@/shared/ui/button";
import { EmptyState } from "@/shared/ui/empty-state";
import { ErrorState } from "@/shared/ui/error-state";
import { Input } from "@/shared/ui/input";
import { Skeleton } from "@/shared/ui/skeleton";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/shared/ui/table";
import { useFirms } from "./api/use-presets";
import type { FirmSummary } from "./api/types";
import { formatCalendarDate } from "./lib/dates";

const matches = (firm: FirmSummary, query: string) => {
  const q = query.trim().toLowerCase();
  return q === "" || firm.name.toLowerCase().includes(q) || firm.plan_names.some((plan) => plan.toLowerCase().includes(q));
};

function PlansText({ names }: { names: string[] }) {
  const t = useTranslations("presets.catalog");
  if (names.length === 0) return <>{"—"}</>;
  const extra = names.length - 2;
  return <>{`${names.slice(0, 2).join(", ")}${extra > 0 ? ` ${t("more", { count: extra })}` : ""}`}</>;
}

function ManualOnlyBadge() {
  const t = useTranslations("presets");
  return (
    <Badge tone="neutral">
      <Eye aria-hidden="true" />
      {t("manualOnly")}
    </Badge>
  );
}

/** S16-01: which firms have presets and each firm's stance, before registering an account. */
export function PresetsCatalog() {
  const t = useTranslations("presets");
  const tc = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const firms = useFirms();
  const [query, setQuery] = useState("");

  const header = <PageHeader title={t("catalog.title")} description={t("catalog.intro")} />;
  if (firms.isPending)
    return (
      <div className="space-y-6">
        {header}
        <div className="space-y-2" aria-busy="true" role="status">
          <span className="sr-only">{tc("common.states.loading")}</span>
          {[0, 1, 2, 3, 4, 5].map((i) => <Skeleton key={i} className="h-12 w-full" />)}
        </div>
      </div>
    );
  if (firms.isError)
    return (
      <div className="space-y-6">
        {header}
        <ErrorState title={errorMessage(tc, firms.error).message} code={errorMessage(tc, firms.error).code} onRetry={() => void firms.refetch()} />
      </div>
    );

  const all = [...firms.data.firms].sort((a, b) => a.name.localeCompare(b.name));
  if (all.length === 0)
    return (
      <div className="space-y-6">
        {header}
        <EmptyState icon={SearchX} title={t("catalog.empty")} description="">
          <Button type="button" onClick={() => void firms.refetch()}>{tc("common.actions.retry")}</Button>
        </EmptyState>
      </div>
    );
  const rows = all.filter((firm) => matches(firm, query));
  const href = (firm: FirmSummary) => withLocale(locale, `/presets/${firm.id}`);
  const verified = (firm: FirmSummary) => formatCalendarDate(firm.release.verified_on, locale);
  const unconfirmed = (firm: FirmSummary) => (firm.not_confirmed_count > 0 ? <span className="text-status-warning">{t("catalog.notConfirmed", { count: firm.not_confirmed_count })}</span> : "—");

  return (
    <div className="space-y-6">
      {header}
      <Input type="search" aria-label={t("catalog.search")} placeholder={t("catalog.search")} value={query} onChange={(event) => setQuery(event.target.value)} className="max-w-sm" />
      {rows.length === 0 ? (
        <EmptyState icon={SearchX} title={t("catalog.noResults")} description="">
          <Button type="button" variant="outline" onClick={() => setQuery("")}>{t("catalog.clearSearch")}</Button>
        </EmptyState>
      ) : (
        <>
          <Table className="hidden md:table">
            <caption className="sr-only">{t("catalog.title")}</caption>
            <TableHeader>
              <TableRow>
                <TableHead scope="col">{t("catalog.columns.firm")}</TableHead>
                <TableHead scope="col">{t("catalog.columns.plans")}</TableHead>
                <TableHead scope="col">{t("catalog.columns.automation")}</TableHead>
                <TableHead scope="col">{t("catalog.columns.copy")}</TableHead>
                <TableHead scope="col">{t("catalog.columns.unconfirmed")}</TableHead>
                <TableHead scope="col">{t("catalog.columns.verified")}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {rows.map((firm) => (
                <TableRow key={firm.id}>
                  <TableCell>
                    <div className="flex flex-wrap items-center gap-2">
                      <Link className="font-medium underline underline-offset-4" href={href(firm)}>{firm.name}</Link>
                      {firm.manual_only ? <ManualOnlyBadge /> : null}
                    </div>
                  </TableCell>
                  <TableCell><PlansText names={firm.plan_names} /></TableCell>
                  <TableCell><StanceBadge subject="automation" value={firm.stances.automation} /></TableCell>
                  <TableCell><StanceBadge subject="copy" value={firm.stances.copy} /></TableCell>
                  <TableCell>{unconfirmed(firm)}</TableCell>
                  <TableCell>{verified(firm)}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
          <ul className="grid gap-3 md:hidden">
            {rows.map((firm) => (
              <li key={firm.id} className="space-y-2 rounded-lg border p-3">
                <div className="flex flex-wrap items-center gap-2">
                  <Link className="font-medium underline underline-offset-4" href={href(firm)}>{firm.name}</Link>
                  {firm.manual_only ? <ManualOnlyBadge /> : null}
                </div>
                <p className="text-sm text-muted-foreground"><PlansText names={firm.plan_names} /></p>
                <div className="flex flex-wrap gap-2">
                  <span className="flex items-center gap-1 text-xs">{t("catalog.columns.automation")} <StanceBadge subject="automation" value={firm.stances.automation} /></span>
                  <span className="flex items-center gap-1 text-xs">{t("catalog.columns.copy")} <StanceBadge subject="copy" value={firm.stances.copy} /></span>
                </div>
                <p className="text-xs">{unconfirmed(firm)}</p>
                <p className="text-xs text-muted-foreground">{`${t("catalog.columns.verified")}: ${verified(firm)}`}</p>
              </li>
            ))}
          </ul>
        </>
      )}
    </div>
  );
}
