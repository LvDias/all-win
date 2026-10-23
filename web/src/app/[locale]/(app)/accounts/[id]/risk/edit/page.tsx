import { notFound } from "next/navigation";
import { setRequestLocale } from "next-intl/server";
import { RulesForm } from "@/features/risk";
import { parseRouteId } from "@/shared/api/ids";

export default async function Page({ params }: { params: Promise<{ locale: string; id: string }> }) {
  const { locale, id } = await params;
  setRequestLocale(locale);
  const accountId = parseRouteId(id);
  if (!accountId) notFound();
  return <RulesForm accountId={accountId} />;
}
