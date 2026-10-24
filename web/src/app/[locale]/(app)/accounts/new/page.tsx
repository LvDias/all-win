import { setRequestLocale } from "next-intl/server";
import { NewAccountWithPresets } from "./new-account-with-presets";

export default async function NewAccountPage({ params }: { params: Promise<{ locale: string }> }) {
  const { locale } = await params;
  setRequestLocale(locale);
  return <NewAccountWithPresets />;
}
