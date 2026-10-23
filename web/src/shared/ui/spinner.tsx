import { Loader2 } from "lucide-react";
import { cn } from "@/shared/lib/utils";

export function Spinner({ className }: { className?: string }) {
  return <Loader2 aria-hidden="true" className={cn("size-4 animate-spin motion-reduce:animate-none", className)} />;
}
