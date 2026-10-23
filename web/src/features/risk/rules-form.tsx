"use client";

import { useTranslations } from "next-intl";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useId, useRef, useState, type FormEvent, type ReactNode } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import type { AccountRiskRules, RiskRules, RiskRulesRequest } from "@/shared/api/types";
import { PageHeader } from "@/shared/components/page-header";
import { parseDecimalInput } from "@/shared/format";
import { useLocalePath } from "@/shared/i18n/navigation";
import { useDisplay } from "@/shared/session/display";
import { Alert } from "@/shared/ui/alert";
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/shared/ui/alert-dialog";
import { Button } from "@/shared/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/shared/ui/card";
import { ErrorState } from "@/shared/ui/error-state";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { RadioGroup, RadioGroupItem } from "@/shared/ui/radio-group";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/shared/ui/select";
import { Skeleton } from "@/shared/ui/skeleton";
import { Spinner } from "@/shared/ui/spinner";
import { useAccountRiskRules, usePreviewRiskRules, useSaveRiskRules } from "./api";

type Field = "drawdownAmount" | "drawdownType" | "dailyLoss" | "profitTarget" | "profitTargetOnReach" | "consistency";
type Errors = Partial<Record<Field, string>>;
type DrawdownType = "trailing_intraday" | "trailing_eod" | "static";
const drawdownTypes: DrawdownType[] = ["trailing_intraday", "trailing_eod", "static"];

/** JSON pointers the Core answers under `/rules/...` (contract `RiskRulesRejected`) → the field that owns them. */
const pointerField: Record<string, Field> = {
  "/rules/daily_loss_limit": "dailyLoss",
  "/rules/daily_loss_limit/amount": "dailyLoss",
  "/rules/drawdown": "drawdownAmount",
  "/rules/drawdown/amount": "drawdownAmount",
  "/rules/drawdown/type": "drawdownType",
  "/rules/daily_profit_target": "profitTarget",
  "/rules/daily_profit_target/amount": "profitTarget",
  "/rules/profit_target_on_reach": "profitTargetOnReach",
  "/rules/consistency_threshold_pct": "consistency",
};

/** S13-02: edit the account's risk rules. Tightening applies at once, loosening from the next trading day (Q48): the Core decides, the form only tells. */
export function RulesForm({ accountId }: { accountId: string }) {
  const t = useTranslations("risk.edit");
  const root = useTranslations();
  const rules = useAccountRiskRules(accountId);
  const path = useLocalePath();
  const header = <PageHeader title={t("title")} />;
  if (rules.isPending)
    return (
      <div className="grid gap-4" aria-busy="true">
        {header}
        <Skeleton className="h-40 w-full max-w-180" />
        <Skeleton className="h-40 w-full max-w-180" />
      </div>
    );
  if (rules.isError) {
    const failure = errorMessage(root, rules.error);
    return (
      <div className="grid gap-4">
        {header}
        <ErrorState title={t("loadError")} code={failure.code} onRetry={() => void rules.refetch()} />
        <Link href={path(`/accounts/${accountId}/risk`)} className="text-sm underline underline-offset-4">
          {t("back")}
        </Link>
      </div>
    );
  }
  return <Editor key={rules.data.version} accountId={accountId} current={rules.data} onReload={() => void rules.refetch()} />;
}

function Editor({ accountId, current, onReload }: { accountId: string; current: AccountRiskRules; onReload: () => void }) {
  const t = useTranslations("risk.edit");
  const root = useTranslations();
  const router = useRouter();
  const path = useLocalePath();
  const { locale } = useDisplay();
  const preview = usePreviewRiskRules(accountId);
  const save = useSaveRiskRules(accountId);
  const submitting = useRef(false);
  const ids = { drawdownAmount: useId(), drawdownType: useId(), dailyLoss: useId(), profitTarget: useId(), consistency: useId() };
  const rules = current.rules;

  const [drawdownAmount, setDrawdownAmount] = useState(rules.drawdown?.amount ?? "");
  const [drawdownType, setDrawdownType] = useState<DrawdownType | "">(rules.drawdown && rules.drawdown.type !== "other" ? rules.drawdown.type : "");
  const [dailyLoss, setDailyLoss] = useState(rules.daily_loss_limit?.amount ?? "");
  const [profitTarget, setProfitTarget] = useState(rules.daily_profit_target?.amount ?? "");
  const [onReach, setOnReach] = useState<RiskRules["profit_target_on_reach"]>(rules.profit_target_on_reach);
  const [consistency, setConsistency] = useState(rules.consistency_threshold_pct ?? "");
  const [errors, setErrors] = useState<Errors>({});
  const [failure, setFailure] = useState<{ message: string; reload: boolean } | null>(null);
  const [confirmReached, setConfirmReached] = useState<RiskRulesRequest | null>(null);
  const [pending, setPending] = useState(false);

  const backHref = path(`/accounts/${accountId}/risk`);
  const hasConsistency = rules.consistency_threshold_pct !== undefined;
  const flattenAvailable = current.enforcement_enabled;

  function money(text: string): { value?: string; error?: string } {
    if (text.trim() === "") return {};
    const parsed = parseDecimalInput(text, locale, { scale: 2 });
    return parsed.ok ? { value: parsed.value } : { error: t("errors.amountInvalid") };
  }

  function build(): { body?: RiskRulesRequest; errors: Errors } {
    const found: Errors = {};
    const dd = money(drawdownAmount);
    const dl = money(dailyLoss);
    const pt = money(profitTarget);
    if (dd.error) found.drawdownAmount = dd.error;
    if (dl.error) found.dailyLoss = dl.error;
    if (pt.error) found.profitTarget = pt.error;
    // Type and amount are one object: both set or both empty.
    if (dd.value && !drawdownType) found.drawdownType = t("errors.drawdownTypeRequired");
    if (!dd.value && drawdownType && !dd.error) found.drawdownAmount = t("errors.drawdownAmountRequired");
    let threshold: string | undefined;
    if (hasConsistency && consistency.trim() !== "") {
      if (/^(100(\.0)?|[0-9]{1,2}(\.[0-9])?)$/.test(consistency.trim())) threshold = consistency.trim();
      else found.consistency = t("errors.consistencyInvalid");
    }
    if (Object.keys(found).length > 0) return { errors: found };
    const next: RiskRules = {
      ...rules,
      profit_target_on_reach: pt.value ? onReach : "notify",
      drawdown: dd.value && drawdownType ? { ...rules.drawdown, type: drawdownType, amount: dd.value } : undefined,
      daily_loss_limit: dl.value ? { kind: rules.daily_loss_limit?.kind === "soft_pause" ? "soft_pause" : "hard", amount: dl.value } : undefined,
      daily_profit_target: pt.value ? { amount: pt.value } : undefined,
      consistency_threshold_pct: threshold,
    };
    return { body: { version: current.version, rules: dropUndefined(next) }, errors: found };
  }

  function mapFailure(error: unknown) {
    if (error instanceof ApiError && error.status === 409 && error.code === "common.version_conflict") return setFailure({ message: t("errors.conflict"), reload: true });
    const field = error instanceof ApiError && error.pointer ? pointerField[error.pointer] : undefined;
    const message = errorMessage(root, error).message;
    if (field) setErrors({ [field]: message });
    else setFailure({ message, reload: false });
  }

  async function submit(event: FormEvent) {
    event.preventDefault();
    if (submitting.current) return;
    const built = build();
    setErrors(built.errors);
    setFailure(null);
    if (!built.body) return;
    submitting.current = true;
    setPending(true);
    try {
      const effect = await preview.mutateAsync(built.body);
      if (effect.would_be_reached) setConfirmReached({ ...built.body, acknowledge_reached: true });
      else await persist(built.body);
    } catch (error) {
      mapFailure(error);
    } finally {
      submitting.current = false;
      setPending(false);
    }
  }

  async function persist(body: RiskRulesRequest) {
    await save.mutateAsync(body);
    toast.success(t("saved"));
    router.push(backHref);
  }

  async function confirmAndSave() {
    if (!confirmReached || submitting.current) return;
    submitting.current = true;
    setPending(true);
    try {
      await persist(confirmReached);
    } catch (error) {
      setConfirmReached(null);
      mapFailure(error);
    } finally {
      submitting.current = false;
      setPending(false);
    }
  }

  const errorList = Object.values(errors).filter((message): message is string => Boolean(message));

  return (
    <form onSubmit={(event) => void submit(event)} className="grid max-w-180 gap-4" noValidate>
      <PageHeader title={t("title")} description={t("description")} />
      {errorList.length >= 3 ? (
        <Alert tone="danger" tabIndex={-1} ref={(node) => node?.focus()}>
          {t("summary", { count: errorList.length })}
        </Alert>
      ) : null}
      {failure ? (
        <Alert tone="danger" tabIndex={-1} ref={(node) => node?.focus()}>
          <div className="grid gap-2">
            <p>{failure.message}</p>
            {failure.reload ? (
              <Button type="button" size="sm" variant="outline" onClick={onReload}>
                {t("reload")}
              </Button>
            ) : null}
          </div>
        </Alert>
      ) : null}

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("sections.loss")}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <FieldRow id={ids.drawdownAmount} label={t("fields.drawdownAmount")} error={errors.drawdownAmount} help={t("tighterNow")}>
            <Input id={ids.drawdownAmount} inputMode="decimal" value={drawdownAmount} onChange={(event) => setDrawdownAmount(event.target.value)} autoComplete="off" aria-invalid={errors.drawdownAmount ? true : undefined} aria-describedby={describedBy(ids.drawdownAmount, errors.drawdownAmount)} />
          </FieldRow>
          <FieldRow id={ids.drawdownType} label={t("fields.drawdownType")} error={errors.drawdownType}>
            <Select value={drawdownType} onValueChange={(value) => setDrawdownType(value as DrawdownType)}>
              <SelectTrigger id={ids.drawdownType} aria-invalid={errors.drawdownType ? true : undefined} aria-describedby={describedBy(ids.drawdownType, errors.drawdownType)}>
                <SelectValue placeholder={t("choose")} />
              </SelectTrigger>
              <SelectContent>
                {drawdownTypes.map((type) => (
                  <SelectItem key={type} value={type}>
                    {root(`risk.drawdownType.${type}`)}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </FieldRow>
          <FieldRow id={ids.dailyLoss} label={t("fields.dailyLoss")} error={errors.dailyLoss} help={t("looserLater")}>
            <Input id={ids.dailyLoss} inputMode="decimal" value={dailyLoss} onChange={(event) => setDailyLoss(event.target.value)} autoComplete="off" aria-invalid={errors.dailyLoss ? true : undefined} aria-describedby={describedBy(ids.dailyLoss, errors.dailyLoss)} />
          </FieldRow>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("sections.target")}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <FieldRow id={ids.profitTarget} label={t("fields.profitTarget")} error={errors.profitTarget}>
            <Input id={ids.profitTarget} inputMode="decimal" value={profitTarget} onChange={(event) => setProfitTarget(event.target.value)} autoComplete="off" aria-invalid={errors.profitTarget ? true : undefined} aria-describedby={describedBy(ids.profitTarget, errors.profitTarget)} />
          </FieldRow>
          <fieldset className="grid gap-2">
            <legend className="mb-1 text-sm font-medium">{t("fields.onReach")}</legend>
            <RadioGroup value={onReach} onValueChange={(value) => (value === "notify" || (value === "flatten_and_block" && flattenAvailable)) && setOnReach(value)}>
              <Label className="flex min-h-11 items-start gap-3 rounded-md border p-3">
                <RadioGroupItem value="notify" />
                <span className="grid gap-1">
                  <span className="font-medium">{t("onReach.notify.label")}</span>
                  <span className="text-sm text-muted-foreground">{t("onReach.notify.body")}</span>
                </span>
              </Label>
              <Label aria-disabled={!flattenAvailable} className="flex min-h-11 items-start gap-3 rounded-md border p-3 aria-disabled:opacity-60">
                <RadioGroupItem value="flatten_and_block" aria-disabled={!flattenAvailable} aria-describedby={`${ids.profitTarget}-flatten`} />
                <span className="grid gap-1">
                  <span className="font-medium">{t("onReach.flatten.label")}</span>
                  <span className="text-sm text-muted-foreground">{t("onReach.flatten.body")}</span>
                  {!flattenAvailable ? (
                    <span id={`${ids.profitTarget}-flatten`} className="text-sm">
                      {t("onReach.flatten.unavailable")}
                    </span>
                  ) : null}
                </span>
              </Label>
            </RadioGroup>
            {errors.profitTargetOnReach ? <p className="text-sm text-status-danger">{errors.profitTargetOnReach}</p> : null}
          </fieldset>
        </CardContent>
      </Card>

      {hasConsistency ? (
        <Card>
          <CardHeader>
            <CardTitle className="text-base">{t("sections.consistency")}</CardTitle>
          </CardHeader>
          <CardContent className="grid gap-4">
            <FieldRow id={ids.consistency} label={t("fields.consistency")} error={errors.consistency} help={t("consistencyHelp")}>
              <Input id={ids.consistency} inputMode="decimal" value={consistency} onChange={(event) => setConsistency(event.target.value)} autoComplete="off" aria-invalid={errors.consistency ? true : undefined} aria-describedby={describedBy(ids.consistency, errors.consistency)} />
            </FieldRow>
          </CardContent>
        </Card>
      ) : null}

      <p className="text-sm text-muted-foreground">{current.enforcement_enabled ? t("enforcement.on") : t("enforcement.off")}</p>

      <div className="sticky bottom-0 -mx-1 flex flex-wrap justify-end gap-2 border-t bg-background p-3">
        <Button asChild variant="outline">
          <Link href={backHref}>{root("common.actions.cancel")}</Link>
        </Button>
        <Button type="submit" disabled={pending}>
          {pending ? <Spinner /> : null}
          {t("save")}
        </Button>
      </div>

      <AlertDialog open={confirmReached !== null} onOpenChange={(open) => !open && !pending && setConfirmReached(null)}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("reached.title")}</AlertDialogTitle>
            <AlertDialogDescription>{current.enforcement_enabled ? t("reached.bodyEnforced") : t("reached.body")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel autoFocus>{t("reached.back")}</AlertDialogCancel>
            <AlertDialogAction
              disabled={pending}
              onClick={(event) => {
                event.preventDefault();
                void confirmAndSave();
              }}
            >
              {pending ? <Spinner /> : null}
              {t("reached.confirm")}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </form>
  );
}

const describedBy = (id: string, error?: string) => (error ? `${id}-error` : undefined);

function FieldRow({ id, label, error, help, children }: { id: string; label: string; error?: string; help?: string; children: ReactNode }) {
  return (
    <div className="grid gap-2">
      <Label htmlFor={id}>{label}</Label>
      {children}
      {help ? <p className="text-xs text-muted-foreground">{help}</p> : null}
      {error ? (
        <p id={`${id}-error`} className="text-sm text-status-danger">
          {error}
        </p>
      ) : null}
    </div>
  );
}

/** The contract rejects `null`; an absent rule means "not set". */
function dropUndefined(rules: RiskRules): RiskRules {
  return Object.fromEntries(Object.entries(rules).filter(([, value]) => value !== undefined)) as RiskRules;
}
