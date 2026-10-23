"use client";

import { CircleArrowUp, CircleCheck, Clock, PlugZap, Unlink, type LucideIcon } from "lucide-react";
import { useTranslations } from "next-intl";
import type { Connection } from "@/shared/api/types";
import { formatRelative } from "@/shared/format";
import { useDisplay } from "@/shared/session/display";
import { Alert } from "@/shared/ui/alert";
import { Badge, type Tone } from "@/shared/ui/badge";

const statuses: Record<Connection["status"], { tone: Tone; icon: LucideIcon }> = {
  awaiting_pairing: { tone: "neutral", icon: Clock },
  connected: { tone: "success", icon: CircleCheck },
  disconnected: { tone: "danger", icon: PlugZap },
  outdated: { tone: "warning", icon: CircleArrowUp },
  unpaired: { tone: "neutral", icon: Unlink },
};

/** Status is icon + text + color, never color alone (F§7.2). */
export function StatusBadge({ connection }: { connection: Connection }) {
  const t = useTranslations("connections.status");
  const { locale } = useDisplay();
  const { tone, icon: Icon } = statuses[connection.status];
  return (
    <span className="flex flex-wrap items-center gap-x-2 gap-y-1">
      <Badge tone={tone}>
        <Icon aria-hidden="true" />
        {t(connection.status)}
      </Badge>
      <time className="text-xs text-muted-foreground" dateTime={connection.status_changed_at}>
        {formatRelative(connection.status_changed_at, locale)}
      </time>
    </span>
  );
}

/** Consequence wording for the states that stop orders (S3-10). */
export function StatusNotice({ connection }: { connection: Connection }) {
  const t = useTranslations("connections.notice");
  if (connection.status === "disconnected") {
    // `disconnect_reason` can be absent (gateway not yet recording the close code) or unknown: fall back to the generic text.
    const key = connection.disconnect_reason === "user" ? "disconnectedByUser" : connection.disconnect_reason === "network" ? "disconnectedNetwork" : "disconnected";
    return <Alert tone="danger">{t(key)}</Alert>;
  }
  if (connection.status === "outdated") return <Alert tone="warning">{t("outdated")}</Alert>;
  return null;
}

/** NinjaTrader accounts reported by the instance, with their binding. */
export function AccountsList({ connection }: { connection: Connection }) {
  const t = useTranslations("connections.accounts");
  const accounts = connection.visible_accounts ?? [];
  if (accounts.length === 0) return <p className="text-sm text-muted-foreground">{t("none")}</p>;
  return (
    <ul className="grid gap-1 text-sm">
      {accounts.map((account) => (
        <li key={account.platform_account} className="flex flex-wrap items-center gap-x-2">
          <span className="font-mono">{account.platform_account}</span>
          <Badge tone={account.account_id ? "success" : "neutral"}>{account.account_id ? t("bound") : t("unbound")}</Badge>
        </li>
      ))}
    </ul>
  );
}

export function ConnectionDetails({ connection }: { connection: Connection }) {
  const t = useTranslations("connections");
  return (
    <div className="grid gap-3">
      <StatusNotice connection={connection} />
      <div>
        <h3 className="mb-1 text-sm font-medium">{t("accounts.title")}</h3>
        <AccountsList connection={connection} />
      </div>
      {connection.instance_id ? (
        <p className="text-xs text-muted-foreground">
          {t("details.instance")} <span className="font-mono">{connection.instance_id}</span>
        </p>
      ) : null}
    </div>
  );
}
