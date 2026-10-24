"use client";

import { RadioGroup as RadioGroupPrimitive } from "radix-ui";
import type { ComponentProps } from "react";
import { cn } from "@/shared/lib/utils";

export function RadioGroup({ className, ...props }: ComponentProps<typeof RadioGroupPrimitive.Root>) {
  return <RadioGroupPrimitive.Root className={cn("grid gap-2", className)} {...props} />;
}

/** The visible circle is 20 px; wrap it with its label so the hit area is at least 44 px. */
export function RadioGroupItem({ className, ...props }: ComponentProps<typeof RadioGroupPrimitive.Item>) {
  return (
    <RadioGroupPrimitive.Item
      className={cn("flex size-5 shrink-0 items-center justify-center rounded-full border border-input bg-background outline-none focus-visible:ring-2 focus-visible:ring-ring data-[state=checked]:border-primary disabled:opacity-50", className)}
      {...props}
    >
      <RadioGroupPrimitive.Indicator className="size-2.5 rounded-full bg-primary" />
    </RadioGroupPrimitive.Item>
  );
}
