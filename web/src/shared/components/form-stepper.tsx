"use client";

import { useTranslations } from "next-intl";
import { useEffect, useId, useRef, useState, type FormEvent, type ReactNode } from "react";
import type { FieldPath, FieldValues, UseFormReturn } from "react-hook-form";
import type { ZodType } from "zod";
import { useUnsavedChanges } from "@/shared/i18n/locale-switch";
import { useIsDesktop } from "@/shared/lib/use-media-query";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Button } from "@/shared/ui/button";
import { Spinner } from "@/shared/ui/spinner";
import { Stepper } from "@/shared/ui/stepper";

export type ReviewRow = { label: string; value: ReactNode | null | undefined };

export type FormStep<V extends FieldValues> = {
  id: string;
  label: string;
  /** Skippable; a skipped step shows "Not set" in the review. */
  optional?: boolean;
  /** Form fields owned by this step: used for validation focus and for mapping server `pointer`s. */
  fields: FieldPath<V>[];
  /** The step exists only while this is true (for example `kind === "prop"`). */
  when?: (values: V) => boolean;
  /** Validation run by `Continue` for this step only. */
  schema: ZodType;
  content: ReactNode;
  /** Rows of this step's group in the review. */
  review: (values: V) => ReviewRow[];
};

/** A server error mapped to the step that owns the field. `pointer` is a JSON pointer such as `/platform_account`. */
export type SubmitError = { pointer?: string; message: string };

type FormStepperProps<V extends FieldValues> = {
  form: UseFormReturn<V>;
  steps: FormStep<V>[];
  /** Called once from the review. Returns the errors the Core refused the request with, or nothing on success. */
  onSubmit: (values: V) => Promise<SubmitError[] | void>;
  submitLabel: string;
  onCancel: () => void;
  /** Rendered in the review above the footer, for example a warning that must be read and its acknowledgment control. */
  reviewExtra?: ReactNode;
  /** Blocks the final action until the feature's own condition holds (for example an acknowledgment). */
  canSubmit?: boolean;
  /** Overrides the breakpoint choice: dialogs use `compact`. */
  variant?: "full" | "compact";
};

const reviewId = "review";

function pointerField(pointer: string): string {
  return pointer.replace(/^\//, "").split("/").map((part) => part.replace(/~1/g, "/").replace(/~0/g, "~")).join(".");
}

/**
 * The step-by-step create flow (components.md C§2.1, foundations F§10.1). It owns the current step, the footer and the
 * review; the feature owns the form (react-hook-form), the step contents and the request.
 */
export function FormStepper<V extends FieldValues>({ form, steps, onSubmit, submitLabel, onCancel, reviewExtra, canSubmit = true, variant }: FormStepperProps<V>) {
  const t = useTranslations("forms");
  const desktop = useIsDesktop();
  const headingId = useId();
  const heading = useRef<HTMLHeadingElement>(null);
  const values = form.watch();
  const active = steps.filter((step) => step.when?.(values) ?? true);
  const [currentId, setCurrentId] = useState<string>(steps[0]?.id ?? reviewId);
  const [backToReview, setBackToReview] = useState(false);
  const [submitting, setSubmitting] = useState(false);
  const [formErrors, setFormErrors] = useState<string[]>([]);
  const [brokenSteps, setBrokenSteps] = useState<string[]>([]);
  const [summary, setSummary] = useState(0);
  const [discarding, setDiscarding] = useState(false);
  const [announce, setAnnounce] = useState("");

  const ids = [...active.map((step) => step.id), reviewId];
  // A step that stopped applying (its gate changed) falls back to the closest remaining one.
  const index = ids.includes(currentId) ? ids.indexOf(currentId) : Math.min(ids.length - 1, Math.max(0, steps.findIndex((step) => step.id === currentId)));
  const current = active[index];
  const inReview = index === ids.length - 1;
  const label = inReview ? t("review.title") : (current?.label ?? "");
  const dirty = form.formState.isDirty;
  useUnsavedChanges(dirty);

  useEffect(() => {
    if (!dirty) return;
    const guard = (event: BeforeUnloadEvent) => event.preventDefault();
    window.addEventListener("beforeunload", guard);
    return () => window.removeEventListener("beforeunload", guard);
  }, [dirty]);

  function go(id: string) {
    setCurrentId(id);
    setFormErrors([]);
    setSummary(0);
    setAnnounce(t("step.announce", { n: Math.max(1, ids.indexOf(id) + 1), total: ids.length, label: id === reviewId ? t("review.title") : (active.find((step) => step.id === id)?.label ?? "") }));
    window.setTimeout(() => heading.current?.focus(), 0);
  }

  function validate(step: FormStep<V>): boolean {
    const picked = Object.fromEntries(step.fields.map((field) => [field, form.getValues(field)]));
    const result = step.schema.safeParse(picked);
    form.clearErrors(step.fields);
    if (result.success) return true;
    let first: string | null = null;
    const seen = new Set<string>();
    for (const issue of result.error.issues) {
      const path = issue.path.join(".");
      if (seen.has(path)) continue;
      seen.add(path);
      form.setError(path as FieldPath<V>, { type: "validate", message: issue.message });
      first ??= path;
    }
    setSummary(seen.size);
    if (first) form.setFocus(first as FieldPath<V>);
    return false;
  }

  function next() {
    if (!current || !validate(current)) return;
    go(backToReview ? reviewId : (ids[index + 1] ?? reviewId));
    setBackToReview(false);
  }

  function back() {
    go(ids[Math.max(0, index - 1)] ?? ids[0] ?? reviewId);
  }

  function edit(id: string) {
    setBackToReview(true);
    go(id);
  }

  async function submit() {
    if (submitting || !canSubmit) return;
    for (const step of active) {
      if (!validate(step)) {
        go(step.id);
        setBrokenSteps((list) => [...new Set([...list, step.id])]);
        return;
      }
    }
    setSubmitting(true);
    setFormErrors([]);
    setBrokenSteps([]);
    try {
      const errors = (await onSubmit(form.getValues())) ?? [];
      const loose: string[] = [];
      const broken: string[] = [];
      for (const error of errors) {
        const field = error.pointer ? pointerField(error.pointer) : null;
        const owner = field ? active.find((step) => step.fields.some((candidate) => field === candidate || field.startsWith(`${candidate}.`))) : undefined;
        if (owner && field) {
          form.setError(field as FieldPath<V>, { type: "server", message: error.message });
          broken.push(owner.id);
        } else loose.push(error.message);
      }
      setFormErrors(loose);
      setBrokenSteps([...new Set(broken)]);
      const target = active.find((step) => broken.includes(step.id));
      if (target) go(target.id);
    } finally {
      setSubmitting(false);
    }
  }

  function onFormSubmit(event: FormEvent) {
    event.preventDefault();
    // Enter inside a text field presses Continue; in the review it never submits.
    if (!inReview) next();
  }

  function cancel() {
    if (dirty) setDiscarding(true);
    else onCancel();
  }

  const compact = (variant ?? (desktop ? "full" : "compact")) === "compact";
  const stepperSteps = [...active.map((step) => ({ id: step.id, label: step.label })), { id: reviewId, label: t("review.title") }];

  return (
    <form onSubmit={onFormSubmit} noValidate className="grid gap-6">
      <Stepper steps={stepperSteps} current={index} variant={compact ? "compact" : "full"} />
      <p className="sr-only" aria-live="polite">
        {announce}
      </p>
      <section aria-labelledby={headingId} className="grid gap-4">
        <h2 id={headingId} ref={heading} tabIndex={-1} className="text-lg font-semibold outline-none">
          {label}
        </h2>
        {summary >= 3 ? <Alert tone="danger">{t("errorsSummary", { count: summary })}</Alert> : null}
        {inReview ? (
          <>
            {brokenSteps.length > 0 ? (
              <Alert tone="danger">
                <p>
                  {brokenSteps.map((id, position) => (
                    <span key={id}>
                      {position > 0 ? " " : null}
                      <Button type="button" variant="link" size="sm" onClick={() => edit(id)}>
                        {t("review.fixStep", { n: active.findIndex((step) => step.id === id) + 1 })}
                      </Button>
                    </span>
                  ))}
                </p>
              </Alert>
            ) : null}
            {formErrors.map((message) => (
              <Alert key={message} tone="danger">
                {message}
              </Alert>
            ))}
            {active.map((step) => (
              <section key={step.id} aria-labelledby={`${headingId}-${step.id}`} className="grid gap-2 rounded-lg border p-4">
                <div className="flex items-center justify-between gap-2">
                  <h3 id={`${headingId}-${step.id}`} className="font-medium">
                    {step.label}
                  </h3>
                  <Button type="button" variant="link" size="sm" onClick={() => edit(step.id)} aria-label={t("review.edit", { group: step.label })}>
                    {t("review.editShort")}
                  </Button>
                </div>
                <dl className="grid gap-2 text-sm sm:grid-cols-[minmax(0,12rem)_1fr] sm:gap-x-4">
                  {step.review(values).map((row) => (
                    <div key={row.label} className="contents">
                      <dt className="text-muted-foreground">{row.label}</dt>
                      <dd className="min-w-0 break-words">{row.value === null || row.value === undefined || row.value === "" ? t("review.notSet") : row.value}</dd>
                    </div>
                  ))}
                </dl>
              </section>
            ))}
            {reviewExtra}
          </>
        ) : (
          // Keyed so each step mounts its own fields: sibling steps with the same markup must not share DOM nodes.
          <div key={current?.id} className="grid gap-4">
            {current?.content}
          </div>
        )}
      </section>
      <div className="flex flex-col gap-2 pb-[env(safe-area-inset-bottom)] sm:flex-row sm:justify-end">
        {index === 0 ? (
          <Button type="button" variant="outline" onClick={cancel}>
            {t("cancel")}
          </Button>
        ) : (
          <Button type="button" variant="outline" onClick={back}>
            {t("back")}
          </Button>
        )}
        {!inReview && current?.optional ? (
          <Button type="button" variant="ghost" onClick={() => go(backToReview ? reviewId : (ids[index + 1] ?? reviewId))}>
            {t("skip")}
          </Button>
        ) : null}
        {inReview ? (
          <Button type="button" onClick={() => void submit()} disabled={submitting || !canSubmit} className="max-sm:order-first">
            {submitting ? <Spinner /> : null}
            {submitLabel}
          </Button>
        ) : (
          <Button type="submit" className="max-sm:order-first">
            {t("continue")}
          </Button>
        )}
      </div>
      <AlertDialog open={discarding} onOpenChange={setDiscarding}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("discard.title")}</AlertDialogTitle>
            <AlertDialogDescription>{t("discard.body")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{t("discard.stay")}</AlertDialogCancel>
            <AlertDialogAction variant="destructive" onClick={onCancel}>
              {t("discard.confirm")}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </form>
  );
}
