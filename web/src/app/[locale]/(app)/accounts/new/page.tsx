import { setRequestLocale } from "next-intl/server";
import { AccountCreateStepper } from "@/features/accounts";

export default async function NewAccountPage({ params }: { params: Promise<{ locale: string }> }) {
  const { locale } = await params;
  setRequestLocale(locale);
  return <AccountCreateStepper />;
}
