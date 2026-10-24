import { Bot, BotOff } from "lucide-react";
import { useTranslations } from "next-intl";
import { StatusBadge, type StatusKind } from "@/shared/components/status-badge";
import { Badge } from "@/shared/ui/badge";
import type { Account } from "./api/types";
import { hasPendingChoice } from "./lib/opt-in";

/** Badges by F§7.2 priority, most severe first: pending choice, paused, disabled. Active shows none. */
export function AccountStateBadges({ account, max = 2 }: { account: Account; max?: number }) {
  const t = useTranslations("accounts.state");
  const items: StatusKind[] = [];
  if (hasPendingChoice(account)) items.push("choicePending");
  if (account.lifecycle_state === "paused") items.push("paused");
  if (account.lifecycle_state === "disabled") items.push("disabled");
  const shown = items.slice(0, max);
  const extra = items.length - shown.length;
  return (
    <span className="inline-flex flex-wrap items-center gap-1">
      {account.lifecycle_state === "active" && items.length === 0 ? <span>{t("active")}</span> : null}
      {shown.map((kind) => <StatusBadge key={kind} kind={kind} />)}
      {extra > 0 ? <span className="text-xs text-muted-foreground">{`+${extra}`}</span> : null}
    </span>
  );
}

export function OptInChoiceBadge({ kind, choice }: { kind: "automation" | "copy"; choice: "accepted" | "declined" }) {
  const t = useTranslations("accounts.state");
  const accepted = choice === "accepted";
  if (kind === "automation") return <StatusBadge kind={accepted ? "acceptsAutomation" : "noAutomation"} />;
  return <Badge tone={accepted ? "info" : "neutral"}>{accepted ? <Bot aria-hidden /> : <BotOff aria-hidden />}{accepted ? t("acceptsCopy") : t("noCopy")}</Badge>;
}
