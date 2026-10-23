import type { ComponentType, ReactNode } from "react";

/** Empty list (F§10): icon, one-sentence title, one-sentence explanation, one primary action. */
export function EmptyState({ icon: Icon, title, description, children }: { icon: ComponentType<{ className?: string; "aria-hidden"?: boolean }>; title: string; description: ReactNode; children?: ReactNode }) {
  return (
    <div className="flex flex-col items-center gap-3 rounded-lg border border-dashed p-8 text-center">
      <Icon aria-hidden className="size-8 text-muted-foreground" />
      <h2 className="text-lg font-semibold">{title}</h2>
      <div className="max-w-md text-sm text-muted-foreground">{description}</div>
      {children}
    </div>
  );
}
