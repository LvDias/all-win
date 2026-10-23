import { getTranslations, setRequestLocale } from "next-intl/server";
import { SecurityPage } from "@/features/security";

export default async function Page({ params }: { params: Promise<{ locale: string }> }) {
  const { locale } = await params;
  setRequestLocale(locale);
  const t = await getTranslations("security");
  return (
    <div className="space-y-6">
      <h1 className="text-xl font-semibold md:text-2xl">{t("title")}</h1>
      <SecurityPage />
    </div>
  );
}
