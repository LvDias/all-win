import { Ban, Bot, BotOff, CircleArrowUp, CircleCheck, CircleDashed, CircleHelp, CircleOff, Hourglass, Lock, OctagonX, Pause, PlugZap, RefreshCcwDot, TriangleAlert, Unplug, type LucideIcon } from "lucide-react";
import { useTranslations } from "next-intl";
import { Badge, type Tone } from "@/shared/ui/badge";

/** Account, connection, source and risk states (foundations F§7.1 and F§7.2): tone, icon and text are fixed here. */
const kinds = {
  blocked: { tone: "danger", icon: Lock },
  blockedByTarget: { tone: "success", icon: Lock },
  paused: { tone: "neutral", icon: Pause },
  disabled: { tone: "neutral", icon: CircleOff },
  outOfSync: { tone: "danger", icon: RefreshCcwDot },
  connected: { tone: "success", icon: PlugZap },
  disconnected: { tone: "danger", icon: Unplug },
  awaitingPairing: { tone: "info", icon: Hourglass },
  outdated: { tone: "warning", icon: CircleArrowUp },
  unpaired: { tone: "neutral", icon: Ban },
  revoked: { tone: "neutral", icon: Ban },
  active: { tone: "success", icon: CircleCheck },
  stale: { tone: "neutral", icon: CircleDashed },
  acceptsAutomation: { tone: "info", icon: Bot },
  noAutomation: { tone: "neutral", icon: BotOff },
  choicePending: { tone: "warning", icon: CircleHelp },
  riskWarning: { tone: "warning", icon: TriangleAlert },
  riskReached: { tone: "danger", icon: OctagonX },
  targetReached: { tone: "success", icon: CircleCheck },
} satisfies Record<string, { tone: Tone; icon: LucideIcon }>;

export type StatusKind = keyof typeof kinds;

type TimedKind = "blocked" | "blockedByTarget";

/** The two "blocked until" kinds need their time: a badge reading "Blocked until " with nothing after it is not allowed by the type. */
type StatusBadgeProps = { className?: string } & ({ kind: TimedKind; time: string } | { kind: Exclude<StatusKind, TimedKind>; time?: undefined });

/** Icon plus text, never color alone. `time` fills "Blocked until {time}". */
export function StatusBadge({ kind, time, className }: StatusBadgeProps) {
  const t = useTranslations("status");
  const { tone, icon: Icon } = kinds[kind];
  return (
    <Badge tone={tone} className={className}>
      <Icon aria-hidden="true" />
      {t(kind, { time: time ?? "" })}
    </Badge>
  );
}
