"use client";

import { useQueryClient } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import Link from "next/link";
import { useRouter, useSearchParams } from "next/navigation";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import { resendChallenge } from "@/shared/api/challenges";
import { CodeChallenge } from "@/shared/components/code-challenge";
import { lifetimeMinutes } from "@/shared/format";
import { useViewOnceLock } from "@/shared/i18n/locale-switch";
import { useLocalePath } from "@/shared/i18n/navigation";
import { meQueryKey } from "@/shared/session/me";
import { landingPath, safeReturnTo } from "@/shared/session/redirects";
import { buttonVariants } from "@/shared/ui/button";
import { cn } from "@/shared/lib/utils";
import { requestLoginCode, verifyLoginCode } from "./api";
import { AuthCard } from "./auth-card";
import { clearPendingLogin, readPendingLogin, updatePendingChallenge } from "./pending-login";

/** S3-02. Without a pending login (reload, direct visit) the user is sent back to the e-mail page. */
export function CodeForm() {
  const t = useTranslations();
  const path = useLocalePath();
  const router = useRouter();
  const search = useSearchParams();
  const queryClient = useQueryClient();
  const pending = readPendingLogin();
  const [minutes] = useState(() => (pending ? lifetimeMinutes(pending.challenge.expires_at) : 0));
  const returnTo = safeReturnTo(search.get("returnTo"));
  // A pending challenge is lost on a language switch, so the switcher stays off (S4 spec §3).
  useViewOnceLock(pending !== null);

  useEffect(() => {
    if (!pending) router.replace(path("/login"));
  }, [pending, path, router]);

  if (!pending) return null;
  const { email, remember } = pending;

  return (
    <AuthCard title={t("auth.code.title")} titleId="code-title">
      <CodeChallenge
        challenge={pending.challenge}
        description={t("auth.code.sent", { email, minutes })}
        onVerify={async (code) => {
          const result = await verifyLoginCode({ challenge_id: readPendingLogin()?.challenge.challenge_id ?? pending.challenge.challenge_id, code, remember_me: remember });
          clearPendingLogin();
          queryClient.clear();
          queryClient.setQueryData(meQueryKey, result.user);
          if (result.new_device) toast.info(t("auth.code.newDevice"));
          router.replace(landingPath(result.user.locale, returnTo));
        }}
        onResend={async () => {
          const next = await resendChallenge(readPendingLogin()?.challenge.challenge_id ?? pending.challenge.challenge_id);
          updatePendingChallenge(next);
          return next;
        }}
        onNewChallenge={async () => {
          const next = await requestLoginCode(email);
          updatePendingChallenge(next);
          return next;
        }}
        secondaryActions={
          <Link
            href={path("/login")}
            onClick={() => clearPendingLogin()}
            className={cn(buttonVariants({ variant: "ghost" }))}
          >
            {t("auth.code.otherEmail")}
          </Link>
        }
      />
    </AuthCard>
  );
}
