"use client";

import { useEffect, useRef, type ReactNode } from "react";
import { Card, CardContent, CardHeader } from "@/shared/ui/card";

type Props = {
  title: string;
  titleId: string;
  children: ReactNode;
  footer?: ReactNode;
  /** Moves focus to the heading on mount, for result pages that replace a form (S3-05 success). */
  focusTitle?: boolean;
};

/** Centered card for the public pages (login, invite, reset). The title is the page's only heading. */
export function AuthCard({ title, titleId, children, footer, focusTitle = false }: Props) {
  const heading = useRef<HTMLHeadingElement>(null);
  useEffect(() => {
    if (focusTitle) heading.current?.focus();
  }, [focusTitle]);
  return (
    <div className="mx-auto flex w-full max-w-sm flex-col gap-4">
      <Card aria-labelledby={titleId}>
        <CardHeader>
          <h1 id={titleId} ref={heading} tabIndex={-1} className="text-lg leading-tight font-semibold outline-none">
            {title}
          </h1>
        </CardHeader>
        <CardContent>{children}</CardContent>
      </Card>
      {footer ? <p className="text-center text-sm text-muted-foreground">{footer}</p> : null}
    </div>
  );
}
