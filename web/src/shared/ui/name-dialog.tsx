"use client";

import { useTranslations } from "next-intl";
import { useId, useState, type FormEvent } from "react";
import { ApiError, networkErrorCode } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import { StepUpCancelled } from "@/shared/session/step-up";
import { Alert } from "@/shared/ui/alert";
import { Button } from "@/shared/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/shared/ui/dialog";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { Spinner } from "@/shared/ui/spinner";

type Props = {
  title: string;
  description: string;
  submitLabel: string;
  label: string;
  placeholder: string;
  /** Shown when the request failed without an answer, so it is unknown whether the object was created. */
  unknownOutcome: string;
  initialName?: string;
  /** Resolves when done; throws ApiError / StepUpCancelled. */
  onSubmit: (name: string) => Promise<void>;
  onClose: () => void;
  onRefresh: () => void;
};

/** Name step shared by create and rename (S3-09, S3-10): 1 to 60 characters, trimmed, unique among active sources. */
export function NameDialog({ title, description, label, placeholder, unknownOutcome: unknownOutcomeText, submitLabel, initialName = "", onSubmit, onClose, onRefresh }: Props) {
  const t = useTranslations();
  const [name, setName] = useState(initialName);
  const [pending, setPending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [unknownOutcome, setUnknownOutcome] = useState(false);
  const id = useId();
  const trimmed = name.trim();

  async function submit(event: FormEvent) {
    event.preventDefault();
    if (pending || trimmed.length < 1 || trimmed.length > 60) return;
    setPending(true);
    setError(null);
    try {
      await onSubmit(trimmed);
    } catch (failure) {
      if (failure instanceof StepUpCancelled) return;
      if (failure instanceof ApiError && failure.code === networkErrorCode) setUnknownOutcome(true);
      else setError(errorMessage(t, failure).message);
    } finally {
      setPending(false);
    }
  }

  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <form onSubmit={(event) => void submit(event)} className="grid gap-4">
          <DialogHeader>
            <DialogTitle>{title}</DialogTitle>
            <DialogDescription>{description}</DialogDescription>
          </DialogHeader>
          {unknownOutcome ? (
            <Alert tone="warning">
              <div className="grid gap-2">
                <p>{unknownOutcomeText}</p>
                <Button type="button" size="sm" variant="outline" onClick={onRefresh}>
                  {t("common.actions.refresh")}
                </Button>
              </div>
            </Alert>
          ) : null}
          {error ? <Alert tone="danger" tabIndex={-1} ref={(node) => node?.focus()}>{error}</Alert> : null}
          <div className="grid gap-2">
            <Label htmlFor={id}>{label}</Label>
            <Input id={id} value={name} onChange={(event) => setName(event.target.value)} maxLength={60} placeholder={placeholder} autoComplete="off" autoFocus />
          </div>
          <DialogFooter>
            <Button type="button" variant="outline" onClick={onClose}>
              {t("common.actions.cancel")}
            </Button>
            <Button type="submit" disabled={pending || trimmed.length < 1}>
              {pending ? <Spinner /> : null}
              {submitLabel}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
