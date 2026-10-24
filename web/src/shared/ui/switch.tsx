"use client";

import { Switch as SwitchPrimitive } from "radix-ui";
import type { ComponentProps } from "react";
import { cn } from "@/shared/lib/utils";

export function Switch({ className, ...props }: ComponentProps<typeof SwitchPrimitive.Root>) {
  return (
    <SwitchPrimitive.Root className={cn("inline-flex h-6 w-11 shrink-0 items-center rounded-full border border-transparent bg-input outline-none transition-colors focus-visible:ring-2 focus-visible:ring-ring data-[state=checked]:bg-primary disabled:opacity-50", className)} {...props}>
      <SwitchPrimitive.Thumb className="block size-5 translate-x-0.5 rounded-full bg-background transition-transform data-[state=checked]:translate-x-[22px] motion-reduce:transition-none" />
    </SwitchPrimitive.Root>
  );
}
