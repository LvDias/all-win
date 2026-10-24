"use client";

import { ArrowDownRight, ArrowUpRight, PencilLine } from "lucide-react";
import { useTranslations } from "next-intl";
import type { Trade } from "@/shared/api/types";
import { formatDateTime, formatMoney, formatPrice, formatSignedMoney } from "@/shared/format";
import type { Locale } from "@/shared/i18n/config";
import { cn } from "@/shared/lib/utils";
import { useDisplay } from "@/shared/session/display";
import { Badge } from "@/shared/ui/badge";

/** Time in trade as "12 min 30 s" or "2 h 05 min": up to two units; the unit abbreviations are the same in both languages. */
export function formatDuration(totalSeconds: number): string {
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;
  if (hours > 0) return `${hours} h ${String(minutes).padStart(2, "0")} min`;
  if (minutes > 0) return `${minutes} min ${String(seconds).padStart(2, "0")} s`;
  return `${seconds} s`;
}

/** Signed money (the sign is part of the text: color is secondary). The string is the Core's; nothing is computed here. */
export function PnlText({ value, locale, className }: { value: string; locale: Locale; className?: string }) {
  const negative = value.startsWith("-");
  const zero = /^-?0+(\.0+)?$/.test(value);
  return <span className={cn("tabular-nums", !zero && (negative ? "text-status-danger" : "text-status-success"), className)}>{formatSignedMoney(value, locale)}</span>;
}

export function DirectionLabel({ direction }: { direction: Trade["direction"] }) {
  const t = useTranslations("trades.direction");
  const Icon = direction === "long" ? ArrowUpRight : ArrowDownRight;
  return (
    <span className="inline-flex items-center gap-1">
      <Icon aria-hidden="true" className="size-4" />
      {t(direction)}
    </span>
  );
}

/** Where the trade came from (spec §3.4): source and signal, `Manual`, master replication or the platform with no signal. */
export function OriginLabel({ trade }: { trade: Trade }) {
  const t = useTranslations("trades.origin");
  if (trade.origin === "manual")
    return (
      <Badge tone="neutral">
        <PencilLine aria-hidden="true" />
        {t("manual")}
      </Badge>
    );
  if (trade.origin === "platform") return <span>{t("platform")}</span>;
  if (trade.origin === "master") return <span>{t("master", { account: trade.master_account_name ?? "" })}</span>;
  return (
    <span className="inline-flex flex-wrap items-center gap-1">
      <span>{trade.source_name}</span>
      {trade.signal_type ? <Badge tone="info">{t(`signal.${trade.signal_type}`)}</Badge> : null}
    </span>
  );
}

/** MAE or MFE in USD; `—` with "Not provided" when the trade has none (manual trades, or the plugin did not see the whole interval). */
export function ExcursionValue({ value }: { value: string | undefined }) {
  const t = useTranslations("trades");
  const { locale } = useDisplay();
  if (value === undefined)
    return (
      <span title={t("notProvided")} aria-label={t("notProvided")}>
        {"—"}
      </span>
    );
  return <span className="tabular-nums">{formatMoney(value, locale)}</span>;
}

export function Excursions({ trade }: { trade: Trade }) {
  const t = useTranslations("trades.table");
  return (
    <span className="grid text-xs">
      <span>
        {t("mae")} <ExcursionValue value={trade.mae} />
      </span>
      <span>
        {t("mfe")} <ExcursionValue value={trade.mfe} />
      </span>
    </span>
  );
}

/** A time with seconds, and the price under it (spec §3.3). */
export function TimePrice({ time, price }: { time: string; price: string }) {
  const { locale, timeZone } = useDisplay();
  return (
    <span className="grid">
      <time dateTime={time} className="whitespace-nowrap">
        {formatDateTime(time, locale, timeZone, { seconds: true })}
      </time>
      <span className="text-xs tabular-nums text-muted-foreground">{formatPrice(price, locale, 2)}</span>
    </span>
  );
}
