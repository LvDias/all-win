"use client";

import { useTranslations } from "next-intl";
import { StanceBlock, type StanceQuote } from "@/shared/components/stance-block";
import type { FirmDetail } from "./api/types";

/** S16-02 tab "Firm positions": automation, copying, VPS/server and personal device, each with the firm's own text. */
export function StancesTab({ firm }: { firm: FirmDetail }) {
  const t = useTranslations("presets");
  const name = firm.summary.name;
  const verifiedOn = firm.summary.release.verified_on;
  const platformLabel = (code: string | undefined) => (code === undefined ? undefined : (firm.platforms.find((p) => p.code === code)?.label ?? code));
  const quotes = (list: StanceQuote[]) => list;
  return (
    <div className="grid gap-4">
      {firm.stances.automation.map((rule) => (
        <StanceBlock key={rule.id} subject="automation" firm={name} value={rule.firm_stance} quotes={quotes(rule.quotes)} searchedOn={rule.research?.searched_on} platformLabel={platformLabel(rule.firm_platform)} verifiedOn={verifiedOn} />
      ))}
      {firm.stances.copy.map((rule) => (
        <StanceBlock key={rule.id} subject="copy" firm={name} value={rule.firm_stance} quotes={quotes(rule.quotes)} searchedOn={rule.research?.searched_on} platformLabel={platformLabel(rule.firm_platform)} verifiedOn={verifiedOn} />
      ))}
      <StanceBlock subject="vps" firm={name} value={firm.stances.vps.value} quotes={firm.stances.vps.quotes} searchedOn={firm.stances.vps.research?.searched_on} verifiedOn={verifiedOn} />
      <StanceBlock subject="personalDevice" firm={name} value={firm.stances.personal_device.value} quotes={firm.stances.personal_device.quotes} searchedOn={firm.stances.personal_device.research?.searched_on} verifiedOn={verifiedOn} />
      <p className="text-xs text-muted-foreground">{t("stances.disclaimer")}</p>
    </div>
  );
}
