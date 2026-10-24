"use client";

import { useTranslations } from "next-intl";
import type { ReactNode } from "react";
import { useMe } from "@/shared/session/me";
import { ErrorState } from "@/shared/ui/error-state";
import { Skeleton } from "@/shared/ui/skeleton";

/** The generic not-found screen: the same one a non-admin gets for any unknown route (does not reveal admin routes). */
export function AdminNotFound() {
  const t = useTranslations("admin");
  return <ErrorState title={t("notFound")} code="common.not_found" />;
}

/** Renders children only for platform admins; the Core still authorizes every call. */
export function AdminGate({ children }: { children: ReactNode }) {
  const me = useMe();
  if (me.isPending) return <Skeleton className="h-40 w-full" />;
  if (!me.data?.is_platform_admin) return <AdminNotFound />;
  return <>{children}</>;
}
