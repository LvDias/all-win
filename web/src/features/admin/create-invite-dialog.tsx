"use client";

import { useTranslations } from "next-intl";
import { useId, useState, type FormEvent } from "react";
import { errorMessage } from "@/shared/api/error-message";
import { isLocale, localeNames, locales, type Locale } from "@/shared/i18n/config";
import { useAppLocale } from "@/shared/i18n/navigation";
import { formatDateTime } from "@/shared/format";
import { useMe } from "@/shared/session/me";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import { Spinner } from "@/shared/ui/spinner";
import { useCreateInvitation, type InvitationCreated } from "./api";
import { OneTimeLinkDialog } from "./one-time-link-dialog";

const emailPattern = /^\S+@\S+\.\S+$/;

export function CreateInviteDialog({ open, onClose }: { open: boolean; onClose: () => void }) {
  return open ? <CreateInviteFlow onClose={onClose} /> : null;
}

function CreateInviteFlow({ onClose }: { onClose: () => void }) {
  const t = useTranslations();
  const appLocale = useAppLocale();
  const me = useMe();
  const create = useCreateInvitation();
  const [email, setEmail] = useState("");
  const [locale, setLocale] = useState<Locale>(appLocale);
  const [error, setError] = useState<string | null>(null);
  const [result, setResult] = useState<InvitationCreated | null>(null);
  const emailId = useId();
  const localeId = useId();

  async function submit(event: FormEvent) {
    event.preventDefault();
    const trimmed = email.trim();
    if (create.isPending || !emailPattern.test(trimmed)) return;
    setError(null);
    try {
      setResult(await create.mutateAsync({ email: trimmed, locale }));
    } catch (failure) {
      setError(errorMessage(t, failure).message);
    }
  }

  if (result) {
    const expires = formatDateTime(result.invitation.expires_at, appLocale, me.data?.time_zone ?? "UTC", { zone: true });
    return (
      <OneTimeLinkDialog
        title={t("admin.invites.resultTitle")}
        description={result.email_sent ? t("admin.invites.sent", { email: result.invitation.email }) : t("admin.invites.manualDelivery")}
        url={result.invite_url}
        label={t("admin.invites.linkLabel")}
        guardClose={!result.email_sent}
        onClose={onClose}
      >
        {result.email_sent ? null : <Alert tone="warning">{t("admin.invites.emailFailed")}</Alert>}
        <p className="text-sm text-muted-foreground">{t("admin.invites.expiresAt", { date: expires })}</p>
      </OneTimeLinkDialog>
    );
  }

  return (
    <Dialog open onOpenChange={(next) => !next && onClose()}>
      <DialogContent>
        <form onSubmit={(event) => void submit(event)} className="grid gap-4">
          <DialogHeader>
            <DialogTitle>{t("admin.invites.createTitle")}</DialogTitle>
            <DialogDescription>{t("admin.invites.createBody")}</DialogDescription>
          </DialogHeader>
          {error ? <Alert tone="danger">{error}</Alert> : null}
          <div className="grid gap-2">
            <Label htmlFor={emailId}>{t("admin.invites.email")}</Label>
            <Input id={emailId} type="email" autoComplete="off" autoFocus value={email} onChange={(event) => setEmail(event.target.value)} />
          </div>
          <div className="grid gap-2">
            <Label htmlFor={localeId}>{t("admin.invites.language")}</Label>
            <Select value={locale} onValueChange={(value) => isLocale(value) && setLocale(value)}>
              <SelectTrigger id={localeId}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {locales.map((option) => (
                  <SelectItem key={option} value={option} lang={option}>
                    {localeNames[option].name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <DialogFooter>
            <Button type="button" variant="outline" onClick={onClose}>
              {t("common.actions.cancel")}
            </Button>
            <Button type="submit" disabled={create.isPending || !emailPattern.test(email.trim())}>
              {create.isPending ? <Spinner /> : null}
              {t("admin.invites.createSubmit")}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
