import { getTranslations, setRequestLocale } from "next-intl/server";
import { InvitesPage } from "@/features/admin";

export default async function Page({ params }: { params: Promise<{ locale: string }> }) {
  const { locale } = await params;
  setRequestLocale(locale);
  const t = await getTranslations("admin.invites");
  return (
    <div className="space-y-6">
      <h1 className="text-xl font-semibold md:text-2xl">{t("title")}</h1>
      <InvitesPage />
    </div>
  );
}
