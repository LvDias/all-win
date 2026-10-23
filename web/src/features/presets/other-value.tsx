"use client";

import { useTranslations } from "next-intl";
import { parseDecimalInput } from "@/shared/format";
import type { Locale } from "@/shared/i18n/config";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import type { RuleField } from "./api/account-preset";

/** What the user types for "another value": two text boxes at most, interpreted per field. */
export type OtherDraft = { a: string; b: string };
export const emptyDraft: OtherDraft = { a: "", b: "" };

export type ParsedOther = { ok: true; value: Record<string, unknown> } | { ok: false; reason: "required" | "invalid" };

const drawdownTypes = ["trailing_intraday", "trailing_eod", "static"] as const;
const lossKinds = ["hard", "soft_pause", "none"] as const;

/** Parses a draft into the contract's value for the field (exact decimals as strings; never a float). `max_position_qty` is the top-level contracts count. */
export function parseOther(field: RuleField, draft: OtherDraft, locale: Locale): ParsedOther {
  const amount = (text: string): { ok: true; value: string } | { ok: false; reason: "required" | "invalid" } => {
    const parsed = parseDecimalInput(text, locale, { scale: 2 });
    if (parsed.ok) return Number(parsed.value) < 0 ? { ok: false, reason: "invalid" } : { ok: true, value: parsed.value };
    return { ok: false, reason: parsed.reason === "empty" ? "required" : "invalid" };
  };
  switch (field) {
    case "max_position_qty": {
      if (draft.a.trim() === "") return { ok: false, reason: "required" };
      return /^[1-9][0-9]{0,5}$/.test(draft.a.trim()) ? { ok: true, value: { contracts: Number(draft.a.trim()) } } : { ok: false, reason: "invalid" };
    }
    case "drawdown": {
      if (!(drawdownTypes as readonly string[]).includes(draft.b)) return { ok: false, reason: "required" };
      const a = amount(draft.a);
      return a.ok ? { ok: true, value: { type: draft.b, amount: a.value } } : a;
    }
    case "daily_loss_limit": {
      if (!(lossKinds as readonly string[]).includes(draft.b)) return { ok: false, reason: "required" };
      if (draft.b === "none") return { ok: true, value: { kind: "none" } };
      const a = amount(draft.a);
      return a.ok ? { ok: true, value: { kind: draft.b, amount: a.value } } : a;
    }
    case "trading_day_reset": {
      if (draft.a.trim() === "" || draft.b.trim() === "") return { ok: false, reason: "required" };
      return /^([01][0-9]|2[0-3]):[0-5][0-9]$/.test(draft.a.trim()) ? { ok: true, value: { local_time: draft.a.trim(), time_zone: draft.b.trim() } } : { ok: false, reason: "invalid" };
    }
    case "consistency_rule":
      return draft.a.trim() === "" ? { ok: false, reason: "required" } : { ok: true, value: { text: draft.a.trim() } };
    default: {
      const a = amount(draft.a);
      return a.ok ? { ok: true, value: { amount: a.value } } : a;
    }
  }
}

/** The field's own inputs for "another value" (S16-03: validation is the field's own). */
export function OtherValueEditor({ field, id, draft, onChange, invalid }: { field: RuleField; id: string; draft: OtherDraft; onChange: (draft: OtherDraft) => void; invalid?: boolean }) {
  const t = useTranslations("presets.other");
  const tp = useTranslations("presets.plans");
  const aria = invalid ? true : undefined;
  const amountInput = (label: string) => (
    <div className="grid gap-1.5">
      <Label htmlFor={`${id}-a`}>{label}</Label>
      <Input id={`${id}-a`} inputMode="decimal" value={draft.a} aria-invalid={aria} onChange={(event) => onChange({ ...draft, a: event.target.value })} />
    </div>
  );
  const choice = (label: string, options: readonly string[], labelOf: (value: string) => string) => (
    <div className="grid gap-1.5">
      <Label htmlFor={`${id}-b`}>{label}</Label>
      <Select value={draft.b} onValueChange={(b) => onChange({ ...draft, b })}>
        <SelectTrigger id={`${id}-b`} aria-invalid={aria}><SelectValue placeholder={t("choose")} /></SelectTrigger>
        <SelectContent>{options.map((option) => <SelectItem key={option} value={option}>{labelOf(option)}</SelectItem>)}</SelectContent>
      </Select>
    </div>
  );
  switch (field) {
    case "max_position_qty":
      return (
        <div className="grid gap-1.5">
          <Label htmlFor={`${id}-a`}>{t("contracts")}</Label>
          <Input id={`${id}-a`} inputMode="numeric" value={draft.a} aria-invalid={aria} onChange={(event) => onChange({ ...draft, a: event.target.value })} />
        </div>
      );
    case "drawdown":
      return <div className="grid gap-3 sm:grid-cols-2">{choice(t("type"), drawdownTypes, (v) => tp(`drawdown.${v as "static"}`))}{amountInput(t("amount"))}</div>;
    case "daily_loss_limit":
      return <div className="grid gap-3 sm:grid-cols-2">{choice(t("kind"), lossKinds, (v) => tp(`dailyLoss.${v as "hard"}`))}{draft.b === "none" ? null : amountInput(t("amount"))}</div>;
    case "trading_day_reset":
      return (
        <div className="grid gap-3 sm:grid-cols-2">
          <div className="grid gap-1.5">
            <Label htmlFor={`${id}-a`}>{t("time")}</Label>
            <Input id={`${id}-a`} placeholder="17:00" value={draft.a} aria-invalid={aria} onChange={(event) => onChange({ ...draft, a: event.target.value })} />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor={`${id}-b`}>{t("zone")}</Label>
            <Input id={`${id}-b`} placeholder="America/Chicago" value={draft.b} aria-invalid={aria} onChange={(event) => onChange({ ...draft, b: event.target.value })} />
          </div>
        </div>
      );
    case "consistency_rule":
      return (
        <div className="grid gap-1.5">
          <Label htmlFor={`${id}-a`}>{t("text")}</Label>
          <Input id={`${id}-a`} value={draft.a} aria-invalid={aria} onChange={(event) => onChange({ ...draft, a: event.target.value })} />
        </div>
      );
    default:
      return amountInput(t("amount"));
  }
}
