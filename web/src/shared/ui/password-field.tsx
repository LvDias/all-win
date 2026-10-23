"use client";

import { Eye, EyeOff } from "lucide-react";
import { useTranslations } from "next-intl";
import { useState, type ComponentProps } from "react";
import { Button } from "@/shared/ui/button";
import { Input } from "@/shared/ui/input";

/** Password input with a show/hide toggle (also the typo check at sign-up, S3-03). */
export function PasswordField({ ref, ...props }: Omit<ComponentProps<typeof Input>, "type">) {
  const t = useTranslations("common.password");
  const [shown, setShown] = useState(false);
  return (
    <div className="flex gap-2">
      <Input ref={ref} type={shown ? "text" : "password"} spellCheck={false} autoCapitalize="none" {...props} />
      <Button type="button" variant="outline" size="icon" aria-pressed={shown} aria-label={t("toggle")} onClick={() => setShown((current) => !current)}>
        {shown ? <EyeOff aria-hidden="true" /> : <Eye aria-hidden="true" />}
      </Button>
    </div>
  );
}
