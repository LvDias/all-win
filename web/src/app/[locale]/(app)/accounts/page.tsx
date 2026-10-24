import { setRequestLocale } from "next-intl/server";
import { Suspense } from "react";
import { AccountsList } from "@/features/accounts";

export default async function AccountsPage({ params }: { params: Promise<{ locale: string }> }) {
  const { locale } = await params;
  setRequestLocale(locale);
  return (
    <Suspense>
      <AccountsList />
    </Suspense>
  );
}
