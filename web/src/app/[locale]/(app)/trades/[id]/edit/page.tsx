import { notFound } from "next/navigation";
import { setRequestLocale } from "next-intl/server";
import { ManualTradeEdit } from "@/features/trades";
import { parseRouteId } from "@/shared/api/ids";

export default async function Page({ params }: { params: Promise<{ locale: string; id: string }> }) {
  const { locale, id } = await params;
  setRequestLocale(locale);
  const tradeId = parseRouteId(id);
  if (!tradeId) notFound();
  return <ManualTradeEdit tradeId={tradeId} />;
}
