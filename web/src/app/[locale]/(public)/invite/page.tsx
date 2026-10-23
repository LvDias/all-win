import { setRequestLocale } from "next-intl/server";
import { InviteForm } from "@/features/auth";

export default async function Page({ params }: { params: Promise<{ locale: string }> }) {
  const { locale } = await params;
  setRequestLocale(locale);
  return <InviteForm />;
}
