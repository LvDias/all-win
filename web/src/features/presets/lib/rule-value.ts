"use client";

import { useLocale, useTranslations } from "next-intl";
import { formatMoney } from "@/shared/format";
import { defaultLocale, isLocale } from "@/shared/i18n/config";
import type { RuleField } from "../api/account-preset";

const record = (value: unknown): Record<string, unknown> | null => (typeof value === "object" && value !== null ? (value as Record<string, unknown>) : null);
const text = (value: unknown): string | null => (typeof value === "string" ? value : null);

/**
 * A rule value from the Core (typed by `field` in the contract, opaque in generated types) as plain text, or null when
 * it has an unexpected shape. Money stays a string until `formatMoney`; nothing is computed here.
 */
export function useRuleValueText() {
  const t = useTranslations("presets");
  const raw = useLocale();
  const locale = isLocale(raw) ? raw : defaultLocale;
  const money = (amount: unknown) => {
    const value = text(amount);
    return value === null ? null : formatMoney(value, locale);
  };
  return (field: RuleField, value: unknown): string | null => {
    const v = record(value);
    if (v === null) return null;
    switch (field) {
      case "max_position_qty":
        return text(v.published_text) ?? (typeof v.contracts === "number" ? String(v.contracts) : null);
      case "drawdown": {
        const amount = money(v.amount);
        return amount === null || typeof v.type !== "string" ? null : `${amount} · ${t(`plans.drawdown.${v.type as "static"}`)}`;
      }
      case "daily_loss_limit": {
        if (v.kind === "none") return t("plans.dailyLoss.none");
        const amount = money(v.amount);
        return amount === null || typeof v.kind !== "string" ? null : `${amount} · ${t(`plans.dailyLoss.${v.kind as "hard"}`)}`;
      }
      case "trading_day_reset":
        return text(v.published_text) ?? (text(v.local_time) && text(v.time_zone) ? `${text(v.local_time)} ${text(v.time_zone)}` : null);
      case "consistency_rule":
        return text(v.text);
      default:
        return money(v.amount);
    }
  };
}

export const ruleFieldLabelKey = (field: RuleField): `account.fields.${RuleField}` => `account.fields.${field}`;
