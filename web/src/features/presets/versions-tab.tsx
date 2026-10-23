"use client";

import { useLocale, useTranslations } from "next-intl";
import { errorMessage } from "@/shared/api/error-message";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import { Badge } from "@/shared/ui/badge";
import { ErrorState } from "@/shared/ui/error-state";
import { Skeleton } from "@/shared/ui/skeleton";
import { useFirmReleases } from "./api/use-presets";
import type { FirmRelease } from "./api/types";
import { formatCalendarDate } from "./lib/dates";

type Change = FirmRelease["changes"][number];

function ChangeText({ change }: { change: Change }) {
  const t = useTranslations("presets.versions");
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const stance = change.stance ? t(`stance.${change.stance}`) : "";
  switch (change.kind) {
    case "stance_changed":
      return <>{t("changes.stanceChanged", { stance, from: change.from ?? "", to: change.to ?? "" })}</>;
    case "stance_text_changed":
      return <>{t("changes.stanceTextChanged", { stance, reason: change.reason?.[locale === "en" ? "en" : "pt-BR"] ?? "" })}</>;
    case "rule_value_changed":
      return <>{t("changes.ruleValueChanged")}</>;
    case "preset_added":
      return <>{t("changes.presetAdded")}</>;
    case "preset_removed":
      return <>{t("changes.presetRemoved")}</>;
  }
}

/** S16-02 tab "Versions": newest first, read-only. A material text change is flagged by the publisher, never by us. */
export function VersionsTab({ firmId }: { firmId: string }) {
  const t = useTranslations("presets.versions");
  const tc = useTranslations();
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const releases = useFirmReleases(firmId, true);
  if (releases.isPending) return <Skeleton className="h-32 w-full" />;
  if (releases.isError) return <ErrorState title={errorMessage(tc, releases.error).message} code={errorMessage(tc, releases.error).code} onRetry={() => void releases.refetch()} />;
  const list = [...releases.data.releases].sort((a, b) => b.version_number - a.version_number);
  return (
    <ol className="grid gap-3">
      {list.map((release) => (
        <li key={release.version_number} className="space-y-2 rounded-lg border p-4">
          <div className="flex flex-wrap items-center gap-2">
            <h3 className="font-medium">{t("version", { n: release.version_number })}</h3>
            {release.changes.some((c) => c.material) ? <Badge tone="warning">{t("material")}</Badge> : null}
          </div>
          <p className="text-sm text-muted-foreground">{t("dates", { published: formatCalendarDate(release.published_at.slice(0, 10), locale), verified: formatCalendarDate(release.verified_on, locale) })}</p>
          {release.changes.length === 0 ? <p className="text-sm">{t("noChanges")}</p> : (
            <ul className="list-disc space-y-1 pl-5 text-sm">
              {release.changes.map((change, index) => <li key={index}><ChangeText change={change} /></li>)}
            </ul>
          )}
        </li>
      ))}
    </ol>
  );
}
