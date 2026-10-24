"use client";

import { Eye } from "lucide-react";
import Link from "next/link";
import { useLocale, useTranslations } from "next-intl";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { PageHeader } from "@/shared/components/page-header";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { withLocale } from "@/shared/i18n/paths";
import { Alert } from "@/shared/ui/alert";
import { ErrorState } from "@/shared/ui/error-state";
import { Skeleton } from "@/shared/ui/skeleton";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/shared/ui/tabs";
import { FirmRuleQuote } from "@/shared/components/firm-rule-quote";
import { useFirm } from "./api/use-presets";
import { formatCalendarDate } from "./lib/dates";
import { PlansTab } from "./plans-tab";
import { StancesTab } from "./stances-tab";
import { VersionsTab } from "./versions-tab";

/** S16-02: one firm's stances, plans with their rules and release history. */
export function FirmDetailPage({ firmId }: { firmId: string }) {
  const t = useTranslations("presets");
  const tc = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const firm = useFirm(firmId);
  const back = <Link className="text-sm underline underline-offset-4" href={withLocale(locale, "/presets")}>{t("firm.backToCatalog")}</Link>;

  if (firm.isPending)
    return (
      <div className="space-y-4" aria-busy="true" role="status">
        <span className="sr-only">{tc("common.states.loading")}</span>
        <Skeleton className="h-10 w-64" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  if (firm.isError) {
    const notFound = firm.error instanceof ApiError && firm.error.status === 404;
    return (
      <div className="space-y-4">
        <ErrorState title={notFound ? t("firm.notFound") : errorMessage(tc, firm.error).message} code={errorMessage(tc, firm.error).code} onRetry={notFound ? undefined : () => void firm.refetch()} />
        {back}
      </div>
    );
  }
  const data = firm.data;
  const { summary } = data;
  return (
    <div className="space-y-6">
      <PageHeader title={summary.name} description={t("firm.verified", { date: formatCalendarDate(summary.release.verified_on, locale), n: summary.release.version_number })} />
      {summary.manual_only ? (
        <Alert tone="info">
          <div className="space-y-3">
            <p className="flex items-center gap-2 font-medium"><Eye aria-hidden="true" className="size-4" />{t("manualOnly")}</p>
            <FirmRuleQuote firm={summary.name} pageTitle={summary.manual_only.reason.page_title} accessedAt={`${summary.manual_only.reason.accessed_at}T12:00:00Z`} url={summary.manual_only.reason.url} text={summary.manual_only.reason.text} lang={summary.manual_only.reason.language} />
            <div className="grid gap-2 text-sm sm:grid-cols-2">
              <div><p className="font-medium">{t("manualOnlyWorks.title")}</p><p>{t("manualOnlyWorks.text")}</p></div>
              <div><p className="font-medium">{t("manualOnlyFails.title")}</p><p>{t("manualOnlyFails.text")}</p></div>
            </div>
          </div>
        </Alert>
      ) : null}
      <Tabs defaultValue="stances">
        <TabsList>
          <TabsTrigger value="stances">{t("firm.tabs.stances")}</TabsTrigger>
          <TabsTrigger value="plans">{t("firm.tabs.plans")}</TabsTrigger>
          <TabsTrigger value="versions">{t("firm.tabs.versions")}</TabsTrigger>
        </TabsList>
        <TabsContent value="stances"><StancesTab firm={data} /></TabsContent>
        <TabsContent value="plans"><PlansTab firm={data} /></TabsContent>
        <TabsContent value="versions"><VersionsTab firmId={firmId} firmName={summary.name} platformLabel={(code) => (code === undefined ? undefined : (data.platforms.find((p) => p.code === code)?.label ?? code))} /></TabsContent>
      </Tabs>
      {back}
    </div>
  );
}
