import { setRequestLocale } from "next-intl/server";
import { RulesForm } from "@/features/risk";

export default async function Page({ params }: { params: Promise<{ locale: string; id: string }> }) {
  const { locale, id } = await params;
  setRequestLocale(locale);
  return <RulesForm accountId={id} />;
}
