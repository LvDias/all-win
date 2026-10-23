import type { ReactNode } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/shared/ui/card";

/** Centered card for the public pages (login, invite, reset). */
export function AuthCard({ title, titleId, children, footer }: { title: string; titleId: string; children: ReactNode; footer?: ReactNode }) {
  return (
    <div className="mx-auto flex w-full max-w-sm flex-col gap-4">
      <Card aria-labelledby={titleId}>
        <CardHeader>
          <CardTitle id={titleId}>
            <h1 className="text-lg font-semibold">{title}</h1>
          </CardTitle>
        </CardHeader>
        <CardContent>{children}</CardContent>
      </Card>
      {footer ? <p className="text-center text-sm text-muted-foreground">{footer}</p> : null}
    </div>
  );
}
