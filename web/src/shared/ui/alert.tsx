import { cva, type VariantProps } from "class-variance-authority";
import type { ComponentProps } from "react";
import { cn } from "@/shared/lib/utils";

const alertVariants = cva("flex gap-3 rounded-lg border p-3 text-sm outline-none focus-visible:ring-2 focus-visible:ring-ring [&>svg]:mt-0.5 [&>svg]:size-4 [&>svg]:shrink-0", {
  variants: {
    tone: {
      info: "border-status-info/30 bg-status-info-soft text-status-info",
      success: "border-status-success/30 bg-status-success-soft text-status-success",
      warning: "border-status-warning/30 bg-status-warning-soft text-status-warning",
      danger: "border-status-danger/30 bg-status-danger-soft text-status-danger",
    },
  },
  defaultVariants: { tone: "info" },
});

/** `danger` and `warning` announce themselves (role=alert); `info` and `success` are polite status messages. */
export function Alert({ className, tone, role, ...props }: ComponentProps<"div"> & VariantProps<typeof alertVariants>) {
  const implied = tone === "danger" || tone === "warning" ? "alert" : "status";
  return <div role={role ?? implied} className={cn(alertVariants({ tone }), className)} {...props} />;
}
