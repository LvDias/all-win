import { Ban, CircleCheck, CircleHelp, TriangleAlert, type LucideIcon } from "lucide-react";
import { useTranslations } from "next-intl";
import { Badge, type Tone } from "@/shared/ui/badge";

/** Firm positions on automation, copying, VPS/server and a personal device (S16 §7; F§7.2). */
type Position = "allowed" | "forbidden" | "conditional" | "unknown" | "not_confirmed";
/** The values the contract allows per subject (S16 §7); a badge for another value does not type-check. */
export type StanceValues = {
  automation: Position | "varies_by_platform";
  copy: Position | "varies_by_platform";
  vps: Position;
  personalDevice: "required" | "not_required" | "unknown" | "not_confirmed";
};
export type StanceSubject = keyof StanceValues;
export type StanceValue = StanceValues[StanceSubject];
type StanceProps = { [S in StanceSubject]: { subject: S; value: StanceValues[S] } }[StanceSubject];

const looks: Record<StanceValue, { tone: Tone; icon: LucideIcon }> = {
  allowed: { tone: "success", icon: CircleCheck },
  forbidden: { tone: "danger", icon: Ban },
  conditional: { tone: "warning", icon: TriangleAlert },
  unknown: { tone: "neutral", icon: CircleHelp },
  not_confirmed: { tone: "warning", icon: CircleHelp },
  varies_by_platform: { tone: "info", icon: TriangleAlert },
  required: { tone: "warning", icon: TriangleAlert },
  not_required: { tone: "success", icon: CircleCheck },
};

/** Icon plus text, never color alone. The wording depends on the subject (a VPS is "Permitido", automation "Permitida"). */
export function StanceBadge({ subject, value, className }: StanceProps & { className?: string }) {
  const t = useTranslations("stanceBadge");
  const { tone, icon: Icon } = looks[value satisfies StanceValue];
  return (
    <Badge tone={tone} className={className}>
      <Icon aria-hidden="true" />
      {t(`${subject === "vps" ? "masculine" : subject === "personalDevice" ? "device" : "feminine"}.${value}`)}
    </Badge>
  );
}
