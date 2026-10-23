"use client";

import { useTranslations } from "next-intl";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useId, useRef, useState, type FormEvent, type ReactNode } from "react";
import { toast } from "sonner";
import { ApiError } from "@/shared/api/client";
import { errorMessage } from "@/shared/api/error-message";
import type { Account, AccountRiskRules, RiskRules, RiskRulesRequest } from "@/shared/api/types";
import { PageHeader } from "@/shared/components/page-header";
import { formatMoney, parseDecimalInput } from "@/shared/format";
import { useUnsavedChanges } from "@/shared/i18n/locale-switch";
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
import { canonicalAmount } from "./account-update";
import { useAccount, useAccountRiskRules, usePreviewRiskRules, useRefreshAfterEdit, useSaveRiskRules, useUpdateCommission } from "./api";

type Field = "drawdownAmount" | "drawdownType" | "dailyLoss" | "profitTarget" | "profitTargetOnReach" | "consistency" | "commission" | "resetTime";
type Errors = Partial<Record<Field, string>>;
type DrawdownType = "trailing_intraday" | "trailing_eod" | "static" | "other";
/** `other` is the firm's own rule: offered only when the account already has it, so it is kept, never newly chosen. */
const chosenTypes: DrawdownType[] = ["trailing_intraday", "trailing_eod", "static"];

/** JSON pointers the Core answers under `/rules/...` (contract `RiskRulesRejected`) → the field that owns them. */
/** Visual order of the fields, for "focus the first invalid one". */
const fieldOrder: Field[] = ["drawdownAmount", "drawdownType", "dailyLoss", "profitTarget", "profitTargetOnReach", "consistency", "commission", "resetTime"];

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
  "/rules/trading_day_reset": "resetTime",
  "/rules/trading_day_reset/local_time": "resetTime",
  "/commission_per_contract_side": "commission",
};

/** S13-02: edit the account's risk rules. Tightening applies at once, loosening from the next trading day (Q48): the Core decides, the form only tells. */
export function RulesForm({ accountId }: { accountId: string }) {
  const t = useTranslations("risk.edit");
  const root = useTranslations();
  const rules = useAccountRiskRules(accountId);
  const account = useAccount(accountId);
  const path = useLocalePath();
  const header = <PageHeader title={t("title")} />;
  if (rules.isPending || account.isPending)
    return (
      <div className="grid gap-4" aria-busy="true">
        {header}
        <Skeleton className="h-40 w-full max-w-180" />
        <Skeleton className="h-40 w-full max-w-180" />
      </div>
    );
  if (rules.isError || account.isError) {
    const failure = errorMessage(root, rules.error ?? account.error);
    const notFound = [rules.error, account.error].some((error) => error instanceof ApiError && error.status === 404);
    if (notFound)
      return (
        <div className="grid gap-4">
          {header}
          <ErrorState title={root("errors.common.not_found")} code={failure.code} />
        </div>
      );
    return (
      <div className="grid gap-4">
        {header}
        <ErrorState title={t("loadError")} code={failure.code} onRetry={() => {
            void rules.refetch();
            void account.refetch();
          }} />
        <Link href={path(`/accounts/${accountId}/risk`)} className="text-sm underline underline-offset-4">
          {t("back")}
        </Link>
      </div>
    );
  }
  return <Editor key={rules.data.version} accountId={accountId} current={rules.data} account={account.data} onReload={() => {
    void rules.refetch();
    void account.refetch();
  }} />;
}

function Editor({ accountId, current, account, onReload }: { accountId: string; current: AccountRiskRules; account: Account; onReload: () => void }) {
  const t = useTranslations("risk.edit");
  const root = useTranslations();
  const router = useRouter();
  const path = useLocalePath();
  const { locale } = useDisplay();
  const preview = usePreviewRiskRules(accountId);
  const save = useSaveRiskRules(accountId);
  const updateCommission = useUpdateCommission(accountId);
  const refresh = useRefreshAfterEdit();
  const submitting = useRef(false);
  /** The rules call may succeed and the commission call fail: a retry then only repeats the commission. */
  const rulesSaved = useRef(false);
  const ids = { drawdownAmount: useId(), drawdownType: useId(), dailyLoss: useId(), profitTarget: useId(), profitTargetOnReach: useId(), consistency: useId(), commission: useId(), resetTime: useId() };
  const rules = current.rules;
  const dailyLossNone = rules.daily_loss_limit?.kind === "none";
  const drawdownOther = rules.drawdown?.type === "other";
  const reset = rules.trading_day_reset;

  const [drawdownAmount, setDrawdownAmount] = useState(rules.drawdown?.amount ?? "");
  const [drawdownType, setDrawdownType] = useState<DrawdownType | "">(rules.drawdown?.type ?? "");
  const [dailyLoss, setDailyLoss] = useState(rules.daily_loss_limit?.amount ?? "");
  const [profitTarget, setProfitTarget] = useState(rules.daily_profit_target?.amount ?? "");
  const [onReach, setOnReach] = useState<RiskRules["profit_target_on_reach"]>(rules.profit_target_on_reach);
  const [consistency, setConsistency] = useState(rules.consistency_threshold_pct ?? "");
  const [commission, setCommission] = useState(account.commission_per_contract_side ?? "");
  const [resetTime, setResetTime] = useState(reset?.local_time ?? "");
  const [errors, setErrors] = useState<Errors>({});
  const [failure, setFailure] = useState<{ message: string; reload: boolean } | null>(null);
  const [confirmReached, setConfirmReached] = useState<Pending | null>(null);
  const [pending, setPending] = useState(false);
  const [discarding, setDiscarding] = useState(false);
  const [focusRequest, setFocusRequest] = useState<{ field: Field; n: number } | null>(null);
  const fieldIds = useRef(ids);
  useEffect(() => {
    fieldIds.current = ids;
  });
  // Focus moves from an effect, once the error text is on screen (F§10: first invalid field).
  useEffect(() => {
    if (focusRequest) document.getElementById(fieldIds.current[focusRequest.field])?.focus();
  }, [focusRequest]);

  const dirty =
    drawdownAmount !== (rules.drawdown?.amount ?? "") ||
    drawdownType !== (rules.drawdown?.type ?? "") ||
    dailyLoss !== (rules.daily_loss_limit?.amount ?? "") ||
    profitTarget !== (rules.daily_profit_target?.amount ?? "") ||
    onReach !== rules.profit_target_on_reach ||
    consistency !== (rules.consistency_threshold_pct ?? "") ||
    commission !== (account.commission_per_contract_side ?? "") ||
    resetTime !== (reset?.local_time ?? "");
  useUnsavedChanges(dirty);
  useEffect(() => {
    if (!dirty) return;
    const guard = (event: BeforeUnloadEvent) => event.preventDefault();
    window.addEventListener("beforeunload", guard);
    return () => window.removeEventListener("beforeunload", guard);
  }, [dirty]);

  function focusFirst(found: Errors) {
    const first = fieldOrder.find((field) => found[field]);
    if (first) setFocusRequest((previous) => ({ field: first, n: (previous?.n ?? 0) + 1 }));
  }

  const backHref = path(`/accounts/${accountId}/risk`);
  const hasConsistency = rules.consistency_threshold_pct !== undefined;
  const flattenAvailable = current.enforcement_enabled;

  function money(text: string, scale = 2): { value?: string; error?: string } {
    if (text.trim() === "") return {};
    const parsed = parseDecimalInput(text, locale, { scale });
    if (parsed.ok) return { value: parsed.value };
    return { error: parsed.reason === "ambiguous" ? t("errors.amountAmbiguous") : t("errors.amountInvalid") };
  }

  /** What the Core will receive, shown before saving ("understood as US$ 2.500,00"), so a misread separator is caught by the user. */
  function understood(text: string, scale: number): string | undefined {
    if (text.trim() === "") return undefined;
    const parsed = parseDecimalInput(text, locale, { scale });
    return parsed.ok ? t("understood", { value: formatMoney(parsed.value, locale) }) : undefined;
  }

  function build(): { pending?: Pending; errors: Errors } {
    const found: Errors = {};
    // A value the user did not change keeps the Core's own text ("2500.00"), so an untouched form is not a change.
    const same = (original: string | undefined, typed: { value?: string; error?: string }) => (typed.value !== undefined && original !== undefined && canonicalAmount(original) === typed.value ? { value: original } : typed);
    const dd = same(rules.drawdown?.amount, money(drawdownAmount));
    const dl = dailyLossNone ? {} : same(rules.daily_loss_limit?.amount, money(dailyLoss));
    const pt = same(rules.daily_profit_target?.amount, money(profitTarget));
    const fee = money(commission, 4);
    if (dd.error) found.drawdownAmount = dd.error;
    if (dl.error) found.dailyLoss = dl.error;
    if (pt.error) found.profitTarget = pt.error;
    if (fee.error) found.commission = fee.error === t("errors.amountAmbiguous") ? fee.error : t("errors.commissionInvalid");
    // Type and amount are one object: both set or both empty.
    if (dd.value && !drawdownType) found.drawdownType = t("errors.drawdownTypeRequired");
    if (!dd.value && drawdownType && !dd.error) found.drawdownAmount = t("errors.drawdownAmountRequired");
    let threshold: string | undefined;
    if (hasConsistency && consistency.trim() !== "") {
      if (/^(100(\.0)?|[0-9]{1,2}(\.[0-9])?)$/.test(consistency.trim())) threshold = consistency.trim();
      else found.consistency = t("errors.consistencyInvalid");
    }
    if (reset && !/^([01][0-9]|2[0-3]):[0-5][0-9]$/.test(resetTime)) found.resetTime = t("errors.resetInvalid");
    if (Object.keys(found).length > 0) return { errors: found };

    const next: RiskRules = {
      ...rules,
      profit_target_on_reach: pt.value ? onReach : "notify",
      // An "other" drawdown keeps its published description; a "none" daily loss stays "none" (the firm states it).
      drawdown: dd.value && drawdownType ? { ...rules.drawdown, type: drawdownType, amount: dd.value } : undefined,
      daily_loss_limit: dailyLossNone ? rules.daily_loss_limit : dl.value ? { kind: rules.daily_loss_limit?.kind === "soft_pause" ? "soft_pause" : "hard", amount: dl.value } : undefined,
      daily_profit_target: pt.value ? { amount: pt.value } : undefined,
      consistency_threshold_pct: threshold,
      trading_day_reset: reset && resetTime !== reset.local_time ? { local_time: resetTime, time_zone: reset.time_zone } : reset,
    };
    const rulesBody = { version: current.version, rules: dropUndefined(next) };
    const before = account.commission_per_contract_side;
    // `undefined` keeps the floor, `null` removes it (contract: absent = unchanged).
    const commissionChange: string | null | undefined = fee.value === undefined ? (before === undefined ? undefined : null) : before !== undefined && canonicalAmount(before) === fee.value ? undefined : fee.value;
    const changed = stable(rulesBody.rules) !== stable(dropUndefined(rules));
    return { pending: { rules: changed ? rulesBody : null, commission: commissionChange }, errors: found };
  }

  function mapFailure(error: unknown) {
    if (error instanceof ApiError && error.status === 409 && error.code === "common.version_conflict") return setFailure({ message: t("errors.conflict"), reload: true });
    const field = error instanceof ApiError && error.pointer ? pointerField[error.pointer] : undefined;
    const message = errorMessage(root, error).message;
    if (field) {
      setErrors({ [field]: message });
      focusFirst({ [field]: message });
    }
    if (rulesSaved.current) setFailure({ message: field ? t("errors.rulesSavedCommissionFailed") : t("errors.rulesSavedCommissionFailedWith", { message }), reload: false });
    else if (!field) setFailure({ message, reload: false });
  }

  async function submit(event: FormEvent) {
    event.preventDefault();
    if (submitting.current) return;
    const built = build();
    setErrors(built.errors);
    setFailure(null);
    focusFirst(built.errors);
    if (!built.pending) return;
    submitting.current = true;
    setPending(true);
    try {
      if (built.pending.rules && !rulesSaved.current) {
        const effect = await preview.mutateAsync(built.pending.rules);
        if (effect.would_be_reached) {
          setConfirmReached({ rules: { ...built.pending.rules, acknowledge_reached: true }, commission: built.pending.commission });
          return;
        }
      }
      await persist(built.pending);
    } catch (error) {
      mapFailure(error);
    } finally {
      submitting.current = false;
      setPending(false);
    }
  }

  /** Rules first (the commission change bumps the limits version they were read at), then the account's commission floor. */
  async function persist(work: Pending) {
    if (work.rules && !rulesSaved.current) {
      await save.mutateAsync(work.rules);
      rulesSaved.current = true;
    }
    if (work.commission !== undefined) await updateCommission.mutateAsync({ account, commission: work.commission });
    refresh();
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
          <FieldRow id={ids.drawdownAmount} label={t("fields.drawdownAmount")} error={errors.drawdownAmount} help={t("tighterNow")} parsed={understood(drawdownAmount, 2)}>
            <Input id={ids.drawdownAmount} inputMode="decimal" value={drawdownAmount} onChange={(event) => setDrawdownAmount(event.target.value)} autoComplete="off" aria-invalid={errors.drawdownAmount ? true : undefined} aria-describedby={describedBy(ids.drawdownAmount, errors.drawdownAmount)} />
          </FieldRow>
          <FieldRow id={ids.drawdownType} label={t("fields.drawdownType")} error={errors.drawdownType}>
            <Select value={drawdownType} onValueChange={(value) => setDrawdownType(value as DrawdownType)}>
              <SelectTrigger id={ids.drawdownType} aria-invalid={errors.drawdownType ? true : undefined} aria-describedby={describedBy(ids.drawdownType, errors.drawdownType)}>
                <SelectValue placeholder={t("choose")} />
              </SelectTrigger>
              <SelectContent>
                {(drawdownOther ? [...chosenTypes, "other" as const] : chosenTypes).map((type) => (
                  <SelectItem key={type} value={type}>
                    {root(`risk.drawdownType.${type}`)}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {drawdownOther && rules.drawdown?.description ? <p className="text-xs text-muted-foreground">{t("drawdownOther", { description: rules.drawdown.description })}</p> : null}
          </FieldRow>
          {dailyLossNone ? (
            <div className="grid gap-1">
              <p className="text-sm font-medium">{t("fields.dailyLoss")}</p>
              <p className="text-sm text-muted-foreground">{t("dailyLossNone")}</p>
            </div>
          ) : (
            <FieldRow id={ids.dailyLoss} label={t("fields.dailyLoss")} error={errors.dailyLoss} help={t("looserLater")} parsed={understood(dailyLoss, 2)}>
              <Input id={ids.dailyLoss} inputMode="decimal" value={dailyLoss} onChange={(event) => setDailyLoss(event.target.value)} autoComplete="off" aria-invalid={errors.dailyLoss ? true : undefined} aria-describedby={describedBy(ids.dailyLoss, errors.dailyLoss)} />
            </FieldRow>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("sections.target")}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <FieldRow id={ids.profitTarget} label={t("fields.profitTarget")} error={errors.profitTarget} parsed={understood(profitTarget, 2)}>
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

      <Card id="commission">
        <CardHeader>
          <CardTitle className="text-base">{t("sections.commission")}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <FieldRow id={ids.commission} label={t("fields.commission")} error={errors.commission} help={t("commission.help")} parsed={understood(commission, 4)}>
            <Input id={ids.commission} inputMode="decimal" value={commission} onChange={(event) => setCommission(event.target.value)} autoComplete="off" aria-invalid={errors.commission ? true : undefined} aria-describedby={describedBy(ids.commission, errors.commission)} />
          </FieldRow>
          {commission.trim() === "" ? <Alert tone="warning">{t("commission.empty")}</Alert> : null}
          <p className="text-xs text-muted-foreground">{t("commission.lowerLater")}</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t("sections.tradingDay")}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          {reset ? (
            <FieldRow id={ids.resetTime} label={t("fields.resetTime", { zone: reset.time_zone })} error={errors.resetTime} help={t("tradingDay.editLater")}>
              <Input id={ids.resetTime} type="time" value={resetTime} onChange={(event) => setResetTime(event.target.value)} aria-invalid={errors.resetTime ? true : undefined} aria-describedby={describedBy(ids.resetTime, errors.resetTime)} />
            </FieldRow>
          ) : (
            <p className="text-sm">{t("tradingDay.cme")}</p>
          )}
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
        {dirty ? (
          <Button type="button" variant="outline" onClick={() => setDiscarding(true)}>
            {root("common.actions.cancel")}
          </Button>
        ) : (
          <Button asChild variant="outline">
            <Link href={backHref}>{root("common.actions.cancel")}</Link>
          </Button>
        )}
        <Button type="submit" disabled={pending}>
          {pending ? <Spinner /> : null}
          {t("save")}
        </Button>
      </div>

      <AlertDialog open={discarding} onOpenChange={setDiscarding}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t("discard.title")}</AlertDialogTitle>
            <AlertDialogDescription>{t("discard.body")}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel autoFocus>{t("discard.stay")}</AlertDialogCancel>
            <AlertDialogAction onClick={() => router.push(backHref)}>{t("discard.confirm")}</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

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

type Pending = { rules: RiskRulesRequest | null; commission: string | null | undefined };

/** JSON with sorted keys, to compare two rule sets regardless of key order. */
function stable(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(stable).join(",")}]`;
  if (value !== null && typeof value === "object") return `{${Object.entries(value).sort(([a], [b]) => a.localeCompare(b)).map(([k, v]) => `${JSON.stringify(k)}:${stable(v)}`).join(",")}}`;
  return JSON.stringify(value);
}

const describedBy = (id: string, error?: string) => (error ? `${id}-error` : undefined);

function FieldRow({ id, label, error, help, parsed, children }: { id: string; label: string; error?: string; help?: string; parsed?: string; children: ReactNode }) {
  return (
    <div className="grid gap-2">
      <Label htmlFor={id}>{label}</Label>
      {children}
      {parsed ? <p className="text-xs text-muted-foreground">{parsed}</p> : null}
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
