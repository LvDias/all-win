import { Bot, BotOff, CircleHelp, CircleOff, Pause } from "lucide-react";
import { useTranslations } from "next-intl";
import { Badge } from "@/shared/ui/badge";
import type { Account } from "./api/types";
import { hasPendingChoice } from "./lib/opt-in";

/** Badges by F§7.2 priority, most severe first: pending choice, paused, disabled. Active shows none. */
export function AccountStateBadges({ account, max = 2 }: { account: Account; max?: number }) {
  const t = useTranslations("accounts.state");
  const items: { key: string; node: React.ReactNode }[] = [];
  if (hasPendingChoice(account))
    items.push({ key: "pending", node: <Badge tone="warning"><CircleHelp aria-hidden />{t("choicePending")}</Badge> });
  if (account.lifecycle_state === "paused") items.push({ key: "paused", node: <Badge tone="neutral"><Pause aria-hidden />{t("paused")}</Badge> });
  if (account.lifecycle_state === "disabled") items.push({ key: "disabled", node: <Badge tone="neutral"><CircleOff aria-hidden />{t("disabled")}</Badge> });
  const shown = items.slice(0, max);
  const extra = items.length - shown.length;
  return (
    <span className="inline-flex flex-wrap items-center gap-1">
      {account.lifecycle_state === "active" && items.length === 0 ? <span>{t("active")}</span> : null}
      {shown.map((i) => <span key={i.key}>{i.node}</span>)}
      {extra > 0 ? <span className="text-xs text-muted-foreground">{`+${extra}`}</span> : null}
    </span>
  );
}

export function OptInChoiceBadge({ kind, choice }: { kind: "automation" | "copy"; choice: "accepted" | "declined" }) {
  const t = useTranslations("accounts.state");
  const accepted = choice === "accepted";
  const label = kind === "automation" ? (accepted ? t("acceptsAutomation") : t("noAutomation")) : accepted ? t("acceptsCopy") : t("noCopy");
  return <Badge tone={accepted ? "info" : "neutral"}>{accepted ? <Bot aria-hidden /> : <BotOff aria-hidden />}{label}</Badge>;
}
