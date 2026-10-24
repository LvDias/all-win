import { useTranslations } from "next-intl";
import { cn } from "@/shared/lib/utils";
import type { RiskState } from "@/shared/api/types";
import { StatusBadge } from "./status-badge";

type RiskMeterProps = {
  /** Limit name, already translated. */
  label: string;
  /** Formatted "used of limit" sentence part, e.g. `US$ 820,00 de US$ 1.000,00` (the caller formats the Core strings). */
  value: string;
  /** Truncated percent from the Core; the bar is clamped to 0–100 for display only. */
  percent: number;
  state: RiskState;
  /** Extra line under the value (drawdown type, trailing floor, firm threshold). */
  detail?: string;
  /** `progress` meters (daily profit target) use the success tone; consistency uses warning at most. */
  tone?: "limit" | "progress";
  dimmed?: boolean;
};

const fill: Record<RiskState, string> = {
  normal: "bg-foreground/70",
  warning: "bg-status-warning",
  reached: "bg-status-danger",
  target_reached: "bg-status-success",
};

/** F§7.3: label, value, percent, bar with a fixed 80 % tick, and a badge when not normal. Figures and state come from the Core. */
export function RiskMeter({ label, value, percent, state, detail, tone = "limit", dimmed }: RiskMeterProps) {
  const t = useTranslations("risk.meter");
  const clamped = Math.min(100, Math.max(0, percent));
  const stateText = state === "normal" ? "" : `, ${t(`state.${state}`)}`;
  const color = tone === "progress" && state !== "target_reached" ? "bg-status-success" : fill[state];
  return (
    <div className={cn("grid gap-1.5", dimmed && "opacity-60")}>
      <div className="flex flex-wrap items-baseline justify-between gap-x-3 gap-y-1">
        <span className="text-sm font-medium">{label}</span>
        <span className="flex flex-wrap items-center gap-2 text-sm">
          <span className="tabular-nums">{value}</span>
          <span className="tabular-nums text-muted-foreground">{t("percent", { percent })}</span>
          {state === "warning" ? <StatusBadge kind="riskWarning" /> : null}
          {state === "reached" ? <StatusBadge kind="riskReached" /> : null}
          {state === "target_reached" ? <StatusBadge kind="targetReached" /> : null}
        </span>
      </div>
      <div role="meter" aria-label={label} aria-valuemin={0} aria-valuemax={100} aria-valuenow={clamped} aria-valuetext={`${label}: ${value}, ${t("percent", { percent })}${stateText}`} className="relative h-2.5 overflow-hidden rounded-full bg-muted">
        <div className={cn("h-full rounded-full", color)} style={{ width: `${clamped}%` }} />
        <div aria-hidden="true" className="absolute inset-y-0 left-[80%] w-px bg-foreground/50" />
      </div>
      {detail ? <p className="text-xs text-muted-foreground">{detail}</p> : null}
    </div>
  );
}
