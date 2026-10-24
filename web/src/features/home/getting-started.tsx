"use client";

import { useQuery } from "@tanstack/react-query";
import { CircleCheck, Circle } from "lucide-react";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { apiFetch } from "@/shared/api/client";
import type { Connection, Source } from "@/shared/api/types";
import { useLocalePath } from "@/shared/i18n/navigation";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/shared/ui/card";
import { Skeleton } from "@/shared/ui/skeleton";

/** Done flags come from the list counts the API already returns; no extra rule. */
function useProgress() {
  const sources = useQuery({ queryKey: ["sources", { includeRevoked: false }], queryFn: ({ signal }) => apiFetch<Source[]>("/api/v1/sources", { signal }) });
  const connections = useQuery({ queryKey: ["connections", { includeUnpaired: false }], queryFn: ({ signal }) => apiFetch<Connection[]>("/api/v1/connections", { signal }) });
  return {
    loading: sources.isPending || connections.isPending,
    source: (sources.data?.length ?? 0) > 0,
    pairing: (connections.data ?? []).some((connection) => connection.status !== "awaiting_pairing" && connection.status !== "unpaired"),
  };
}

export function GettingStarted() {
  const t = useTranslations("home.gettingStarted");
  const path = useLocalePath();
  const progress = useProgress();

  const steps = [
    { key: "source", done: progress.source, href: "/connections/sources", guide: t("alertGuide") },
    { key: "pairing", done: progress.pairing, href: "/connections/ninjatrader", guide: t("installGuide") },
  ] as const;

  return (
    <Card aria-labelledby="getting-started-title" className="max-w-2xl">
      <CardHeader>
        <CardTitle id="getting-started-title">{t("title")}</CardTitle>
        <CardDescription>{t("description")}</CardDescription>
      </CardHeader>
      <CardContent>
        {progress.loading ? (
          <div className="grid gap-2" aria-busy="true">
            <Skeleton className="h-11 w-full" />
            <Skeleton className="h-11 w-full" />
          </div>
        ) : (
          <ol className="grid gap-3">
            {steps.map((step, index) => (
              <li key={step.key} className="flex flex-wrap items-center gap-x-3 gap-y-1">
                {step.done ? <CircleCheck aria-hidden="true" className="size-5 text-status-success" /> : <Circle aria-hidden="true" className="size-5 text-muted-foreground" />}
                <Link href={path(step.href)} className="inline-flex min-h-11 items-center text-sm font-medium underline-offset-4 hover:underline">
                  {`${index + 1}. ${t(`${step.key}.label`)}`}
                </Link>
                <span className="sr-only">{step.done ? t("done") : t("todo")}</span>
                {/* The guides arrive with S17 and the plugin; until then they are visible but inactive. */}
                <span aria-disabled="true" className="text-xs text-muted-foreground">
                  {`${step.guide} · ${t("soon")}`}
                </span>
              </li>
            ))}
          </ol>
        )}
      </CardContent>
    </Card>
  );
}
