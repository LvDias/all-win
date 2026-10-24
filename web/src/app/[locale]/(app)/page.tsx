import Link from "next/link";
import { getTranslations, setRequestLocale } from "next-intl/server";
import { GettingStarted } from "@/features/home";

export default async function HomePage({ params }: { params: Promise<{ locale: string }> }) {
  const { locale } = await params;
  setRequestLocale(locale);
  const t = await getTranslations("home");
  return (
    <div className="space-y-6">
      <h1 className="text-xl font-semibold md:text-2xl">{t("title")}</h1>
      <p className="text-muted-foreground">{t("intro")}</p>
      <GettingStarted />
      <Link className="inline-flex min-h-11 items-center text-sm underline underline-offset-4" href={`/${locale}/settings/preferences`}>
        {t("preferences")}
      </Link>
    </div>
  );
}
