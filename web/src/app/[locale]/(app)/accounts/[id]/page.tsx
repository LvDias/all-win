import { setRequestLocale } from "next-intl/server";
import { AccountDetail } from "@/features/accounts";

export default async function AccountDetailPage({ params }: { params: Promise<{ locale: string; id: string }> }) {
  const { locale, id } = await params;
  setRequestLocale(locale);
  return <AccountDetail accountId={id} />;
}
