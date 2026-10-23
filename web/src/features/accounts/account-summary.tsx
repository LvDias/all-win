import { useLocale, useTranslations } from "next-intl";
import { formatMoney } from "@/shared/format";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import type { Account } from "./api/types";

/** `Apex · 50K Rithmic · Evaluation · US$ 50,000.00` for prop; the size alone for real. */
export function useAccountSummary() {
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  return (account: Account) => {
    const size = formatMoney(account.size_amount, locale);
    return account.kind === "prop" ? [account.firm_name, account.plan_name, account.phase, size].filter(Boolean).join(" · ") : size;
  };
}

export function PlatformAccountText({ account }: { account: Account }) {
  const t = useTranslations("accounts");
  if (!account.binding) return <span className="text-muted-foreground">{t("list.unbound")}</span>;
  return (
    <span>
      {`${account.binding.platform_account} · ${account.binding.connection_name} `}
      <span className="text-xs text-muted-foreground">{`(${t(`platformKind.${account.binding.kind}`)})`}</span>
    </span>
  );
}
