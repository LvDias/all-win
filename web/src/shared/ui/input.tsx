import type { ComponentProps } from "react";
import { cn } from "@/shared/lib/utils";

export function Input({ className, type = "text", ...props }: ComponentProps<"input">) {
  return (
    <input
      type={type}
      className={cn(
        "flex min-h-11 w-full rounded-md border border-input bg-background px-3 py-2 text-base outline-none placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring read-only:bg-muted disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-status-danger md:text-sm",
        className,
      )}
      {...props}
    />
  );
}
