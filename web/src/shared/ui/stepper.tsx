import { Check } from "lucide-react";
import { useTranslations } from "next-intl";
import { cn } from "@/shared/lib/utils";

type StepperProps = { steps: { id: string; label: string }[]; current: number; variant?: "full" | "compact"; className?: string };

/** Position in a short linear flow. Steps are not clickable; going back uses the screen's own button. */
export function Stepper({ steps, current, variant = "compact", className }: StepperProps) {
  const t = useTranslations("stepper");
  const active = steps[current];
  if (variant === "compact") {
    return (
      <div className={className}>
        <p className="text-sm text-muted-foreground">{t("progress", { current: current + 1, total: steps.length, label: active?.label ?? "" })}</p>
        <div aria-hidden="true" className="mt-2 flex gap-1">
          {steps.map((step, index) => (
            <span key={step.id} className={cn("h-1 flex-1 rounded-full", index <= current ? "bg-primary" : "bg-muted")} />
          ))}
        </div>
      </div>
    );
  }
  return (
    <ol className={cn("flex flex-wrap items-center gap-4", className)}>
      {steps.map((step, index) => (
        <li key={step.id} aria-current={index === current ? "step" : undefined} className="flex items-center gap-2 text-sm">
          <span
            className={cn(
              "flex size-6 items-center justify-center rounded-full text-xs font-medium",
              index < current ? "bg-status-success text-white" : index === current ? "bg-primary text-primary-foreground" : "bg-muted text-muted-foreground",
            )}
          >
            {index < current ? <Check aria-hidden="true" className="size-3.5" /> : index + 1}
          </span>
          <span className={index === current ? "font-medium" : "text-muted-foreground"}>{step.label}</span>
          {index < current ? <span className="sr-only">{t("done")}</span> : null}
        </li>
      ))}
    </ol>
  );
}
