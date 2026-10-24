import type { ReactNode } from "react";

type PageHeaderProps = {
  title: string;
  description?: string;
  /** Primary action (and secondary ones), aligned right from `sm`, stacked below it on narrow screens. */
  actions?: ReactNode;
  /** A control that belongs to the page, such as the "show revoked" switch. */
  toggle?: ReactNode;
};

/** The page's only `h1`, with its purpose, primary action and optional toggle (components.md C§4). */
export function PageHeader({ title, description, actions, toggle }: PageHeaderProps) {
  return (
    <header className="grid gap-3">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
        <div className="min-w-0 grid gap-1">
          <h1 className="text-xl font-semibold md:text-2xl">{title}</h1>
          {description ? <p className="text-sm text-muted-foreground">{description}</p> : null}
        </div>
        {actions ? <div className="flex shrink-0 flex-wrap gap-2">{actions}</div> : null}
      </div>
      {toggle}
    </header>
  );
}
