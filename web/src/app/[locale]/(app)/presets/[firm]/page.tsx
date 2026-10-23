import { notFound } from "next/navigation";
import { setRequestLocale } from "next-intl/server";
import { FirmDetailPage } from "@/features/presets";
import { parseRouteId } from "@/shared/api/ids";

export default async function Page({ params }: { params: Promise<{ locale: string; firm: string }> }) {
  const { locale, firm } = await params;
  setRequestLocale(locale);
  const firmId = parseRouteId(firm);
  if (firmId === null) notFound();
  return <FirmDetailPage firmId={firmId} />;
}
