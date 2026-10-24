import { setRequestLocale } from "next-intl/server";
import { TradesPage } from "@/features/trades";

export default async function Page({ params }: { params: Promise<{ locale: string }> }) {
  const { locale } = await params;
  setRequestLocale(locale);
  return <TradesPage />;
}
