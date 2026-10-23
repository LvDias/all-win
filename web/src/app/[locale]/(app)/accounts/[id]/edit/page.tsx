import { notFound } from "next/navigation";
import { setRequestLocale } from "next-intl/server";
import { parseRouteId } from "@/shared/api/ids";
import { AccountFormPage } from "@/features/accounts";

export default async function EditAccountPage({ params }: { params: Promise<{ locale: string; id: string }> }) {
  const { locale, id } = await params;
  setRequestLocale(locale);
  const accountId = parseRouteId(id);
  if (!accountId) notFound();
  return <AccountFormPage accountId={accountId} />;
}
