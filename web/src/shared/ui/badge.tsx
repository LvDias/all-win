import { cva, type VariantProps } from "class-variance-authority";
import type { ComponentProps } from "react";
import { cn } from "@/shared/lib/utils";

/** Status is never conveyed by color alone: callers pass an icon and text (F§7). */
const badgeVariants = cva("inline-flex items-center gap-1 rounded-md px-2 py-0.5 text-xs font-medium whitespace-nowrap [&_svg]:size-3.5", {
  variants: {
    tone: {
      success: "bg-status-success-soft text-status-success",
      warning: "bg-status-warning-soft text-status-warning",
      danger: "bg-status-danger-soft text-status-danger",
      info: "bg-status-info-soft text-status-info",
      neutral: "bg-status-neutral-soft text-status-neutral",
    },
  },
  defaultVariants: { tone: "neutral" },
});

export type Tone = NonNullable<VariantProps<typeof badgeVariants>["tone"]>;

export function Badge({ className, tone, ...props }: ComponentProps<"span"> & VariantProps<typeof badgeVariants>) {
  return <span className={cn(badgeVariants({ tone }), className)} {...props} />;
}
