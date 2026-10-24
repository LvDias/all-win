"use client";

import { Landmark, Plus } from "lucide-react";
import type { ReactNode } from "react";
import Link from "next/link";
import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { useLocale, useTranslations } from "next-intl";
import { errorMessage } from "@/shared/api/error-message";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { withLocale } from "@/shared/i18n/paths";
import { useIsDesktop } from "@/shared/lib/use-media-query";
import { Button, buttonVariants } from "@/shared/ui/button";
import { Card } from "@/shared/ui/card";
import { EmptyState } from "@/shared/ui/empty-state";
import { ErrorState } from "@/shared/ui/error-state";
import { Input } from "@/shared/ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import { Skeleton } from "@/shared/ui/skeleton";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/shared/ui/table";
import { AccountActions } from "./account-actions";
import { AccountStateBadges } from "./account-state-badges";
import { PlatformAccountText, useAccountSummary } from "./account-summary";
import { useAccounts } from "./api/use-accounts";
import type { Account } from "./api/types";
import { hasPendingChoice } from "./lib/opt-in";
import { PageHeader } from "@/shared/components/page-header";

type Filters = { q: string; type: string; state: string };
const stateValues = ["all", "active", "paused", "disabled", "choice_pending"] as const;
const stateLabel = { all: "all", active: "active", paused: "paused", disabled: "disabled", choice_pending: "choicePending" } as const;

export function filterAccounts(accounts: Account[], f: Filters): Account[] {
  const q = f.q.trim().toLowerCase();
  return accounts.filter((a) => {
    if (q && !a.display_name.toLowerCase().includes(q)) return false;
    if (f.type === "prop" || f.type === "real") if (a.kind !== f.type) return false;
    if (f.state === "choice_pending") return hasPendingChoice(a);
    if (f.state === "active" || f.state === "paused" || f.state === "disabled") return a.lifecycle_state === f.state;
    return true;
  });
}

/** An extra column composed by the page (features do not import each other): the header text and one cell per account. Absent = no column. */
export type AccountListColumn = { header: string; cell: (account: Account) => ReactNode };

export function AccountsList({ riskColumn }: { riskColumn?: AccountListColumn }) {
  const t = useTranslations("accounts");
  const tAll = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const router = useRouter();
  const pathname = usePathname();
  const params = useSearchParams();
  const desktop = useIsDesktop();
  const accounts = useAccounts();
  const summary = useAccountSummary();
  const filters: Filters = { q: params.get("q") ?? "", type: params.get("type") ?? "all", state: params.get("state") ?? "all" };
  const filtered = filters.q !== "" || filters.type !== "all" || filters.state !== "all";

  function setFilter(key: keyof Filters, value: string) {
    const next = new URLSearchParams(params.toString());
    if (value === "" || value === "all") next.delete(key);
    else next.set(key, value);
    const qs = next.toString();
    router.replace(qs ? `${pathname}?${qs}` : pathname);
  }

  const newHref = withLocale(locale, "/accounts/new");
  const header = (
    <PageHeader title={t("list.title")} actions={<Link className={buttonVariants()} href={newHref}><Plus aria-hidden />{t("list.new")}</Link>} />
  );

  if (accounts.isPending)
    return (
      <div className="space-y-6" aria-busy="true" role="status">
        {header}
        <span className="sr-only">{tAll("common.states.loading")}</span>
        <Skeleton className="h-11 w-full" />
        <Skeleton className="h-11 w-full" />
        <Skeleton className="h-11 w-full" />
      </div>
    );
  if (accounts.isError)
    return (
      <div className="space-y-6">
        {header}
        <ErrorState title={t("list.loadError")} code={errorMessage(tAll, accounts.error).code} onRetry={() => void accounts.refetch()} />
      </div>
    );

  if (accounts.data.length === 0)
    return (
      <div className="space-y-6">
        {header}
        <EmptyState icon={Landmark} title={t("list.empty.title")} description={t("list.empty.body")}>
          <Link className={buttonVariants()} href={newHref}>{t("list.new")}</Link>
        </EmptyState>
      </div>
    );

  const rows = filterAccounts(accounts.data, filters).sort((a, b) => a.display_name.localeCompare(b.display_name, locale));

  return (
    <div className="space-y-6">
      {header}
      <div className="flex flex-col gap-3 sm:flex-row">
        <Input type="search" aria-label={t("list.search")} placeholder={t("list.search")} defaultValue={filters.q} onChange={(e) => setFilter("q", e.target.value)} className="sm:max-w-xs" />
        <Select value={filters.type} onValueChange={(v) => setFilter("type", v)}>
          <SelectTrigger aria-label={t("list.filters.type")} className="sm:w-40"><SelectValue /></SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{t("list.filters.all")}</SelectItem>
            <SelectItem value="prop">{t("list.filters.prop")}</SelectItem>
            <SelectItem value="real">{t("list.filters.real")}</SelectItem>
          </SelectContent>
        </Select>
        <Select value={filters.state} onValueChange={(v) => setFilter("state", v)}>
          <SelectTrigger aria-label={t("list.filters.state")} className="sm:w-48"><SelectValue /></SelectTrigger>
          <SelectContent>
            {stateValues.map((s) => <SelectItem key={s} value={s}>{t(`list.filters.${stateLabel[s]}`)}</SelectItem>)}
          </SelectContent>
        </Select>
      </div>

      {rows.length === 0 ? (
        <div className="space-y-3 text-sm text-muted-foreground">
          <p>{t("list.emptyFiltered")}</p>
          {filtered ? <Button variant="outline" onClick={() => router.replace(pathname)}>{t("list.filters.clear")}</Button> : null}
        </div>
      ) : desktop ? (
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t("list.columns.name")}</TableHead>
              <TableHead>{t("list.columns.type")}</TableHead>
              <TableHead>{t("list.columns.details")}</TableHead>
              <TableHead>{t("list.columns.platformAccount")}</TableHead>
              <TableHead>{t("list.columns.multiplier")}</TableHead>
              <TableHead>{t("list.columns.maxPosition")}</TableHead>
              {riskColumn ? <TableHead>{riskColumn.header}</TableHead> : null}
              <TableHead>{t("list.columns.state")}</TableHead>
              <TableHead><span className="sr-only">{t("list.columns.actions")}</span></TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {rows.map((a) => (
              <TableRow key={a.id}>
                <TableCell><Link className="font-medium underline-offset-4 hover:underline" href={withLocale(locale, `/accounts/${a.id}`)}>{a.display_name}</Link></TableCell>
                <TableCell>{t(`type.${a.kind}`)}</TableCell>
                <TableCell>{summary(a)}</TableCell>
                <TableCell><PlatformAccountText account={a} /></TableCell>
                <TableCell>{`${a.multiplier}×`}</TableCell>
                <TableCell>{t("list.contracts", { count: a.max_position_qty })}</TableCell>
                {riskColumn ? <TableCell>{riskColumn.cell(a)}</TableCell> : null}
                <TableCell><AccountStateBadges account={a} /></TableCell>
                <TableCell><AccountActions account={a} /></TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      ) : (
        <ul className="space-y-3">
          {rows.map((a) => (
            <li key={a.id}>
              <Card className="space-y-2 p-4">
                <div className="flex items-start justify-between gap-2">
                  <Link className="font-medium underline-offset-4 hover:underline" href={withLocale(locale, `/accounts/${a.id}`)}>{a.display_name}</Link>
                  <AccountActions account={a} />
                </div>
                <p className="text-sm text-muted-foreground">{`${t(`type.${a.kind}`)} · ${summary(a)}`}</p>
                <p className="text-sm"><PlatformAccountText account={a} /></p>
                <p className="text-sm">{`${a.multiplier}× · ${t("list.contracts", { count: a.max_position_qty })}`}</p>
                {riskColumn ? <div className="text-sm"><span className="text-muted-foreground">{`${riskColumn.header}: `}</span>{riskColumn.cell(a)}</div> : null}
                <AccountStateBadges account={a} />
              </Card>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
