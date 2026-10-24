import { setRequestLocale } from "next-intl/server";
import { LoginForm } from "@/features/auth";

export default async function Page({ params }: { params: Promise<{ locale: string }> }) {
  const { locale } = await params;
  setRequestLocale(locale);
  return <LoginForm />;
}
