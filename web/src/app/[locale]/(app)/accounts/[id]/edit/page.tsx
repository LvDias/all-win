import { setRequestLocale } from "next-intl/server";
import { AccountFormPage } from "@/features/accounts";

export default async function EditAccountPage({ params }: { params: Promise<{ locale: string; id: string }> }) {
  const { locale, id } = await params;
  setRequestLocale(locale);
  return <AccountFormPage accountId={id} />;
}
