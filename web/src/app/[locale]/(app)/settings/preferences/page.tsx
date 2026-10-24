import { getTranslations, setRequestLocale } from "next-intl/server";
import { PreferencesForm } from "@/features/settings/preferences";

export default async function PreferencesPage({ params }: { params: Promise<{ locale: string }> }) {
  const { locale } = await params;
  setRequestLocale(locale);
  const t = await getTranslations("settings.preferences");
  return (
    <div className="space-y-6">
      <h1 className="text-xl font-semibold md:text-2xl">{t("title")}</h1>
      <PreferencesForm />
    </div>
  );
}
