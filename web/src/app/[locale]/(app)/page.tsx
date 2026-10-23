import Link from "next/link";
import { getTranslations, setRequestLocale } from "next-intl/server";

export default async function HomePage({ params }: { params: Promise<{ locale: string }> }) {
  const { locale } = await params;
  setRequestLocale(locale);
  const t = await getTranslations("home");
  return (
    <div className="space-y-4">
      <h1 className="text-xl font-semibold md:text-2xl">{t("title")}</h1>
      <p className="text-muted-foreground">{t("intro")}</p>
      <Link className="text-sm underline underline-offset-4" href={`/${locale}/settings/preferences`}>
        {t("preferences")}
      </Link>
    </div>
  );
}
