import { apiFetch } from "@/shared/api/client";
import { accountKeys as sharedAccountKeys } from "@/shared/api/account-keys";
import { segment } from "@/shared/api/ids";
import type {
  Account,
  AccountCreateBody,
  AccountUpdateBody,
  ArchiveImpact,
  LifecycleState,
  OptInChoice,
  OptInKind,
  OptInRules,
  OptInState,
  RuleVersion,
  StepUpProof,
} from "./types";

const base = "/api/v1/accounts";

/** The shared account keys (list, detail) plus this feature's own, all under `accountKeys.all` where they belong to an account. */
export const accountKeys = {
  ...sharedAccountKeys,
  impact: (id: string) => [...sharedAccountKeys.all, "impact", id] as const,
  rules: (presetVersionId: string | null, firmPlatform: string | null) => ["opt-in-rules", presetVersionId, firmPlatform] as const,
  ruleVersion: (id: string) => ["rule-version", id] as const,
};

export const listAccounts = (signal?: AbortSignal) => apiFetch<{ accounts: Account[] }>(base, { signal }).then((r) => r.accounts);
export const getAccount = (id: string, signal?: AbortSignal) => apiFetch<Account>(`${base}/${segment(id)}`, { signal });

/** `idempotencyKey` is generated once per form submission and reused on retries (the retry after a step-up too). */
export const createAccount = (body: AccountCreateBody, idempotencyKey: string) =>
  apiFetch<Account>(base, { method: "POST", body, headers: { "Idempotency-Key": idempotencyKey } });

export const updateAccount = (id: string, body: AccountUpdateBody) => apiFetch<Account>(`${base}/${segment(id)}`, { method: "PUT", body });
export const archiveAccount = (id: string) => apiFetch<void>(`${base}/${segment(id)}`, { method: "DELETE" });
export const getArchiveImpact = (id: string, signal?: AbortSignal) => apiFetch<ArchiveImpact>(`${base}/${segment(id)}/archive-impact`, { signal });

export const setLifecycle = (id: string, state: LifecycleState, rowVersion: number) =>
  apiFetch<Account>(`${base}/${segment(id)}/lifecycle`, { method: "PUT", body: { state, row_version: rowVersion } });

export const setOptIn = (id: string, kind: OptInKind, body: { choice: OptInChoice; rule_version_id: string; step_up?: StepUpProof }) =>
  apiFetch<OptInState>(`${base}/${segment(id)}/opt-ins/${kind}`, { method: "PUT", body });

export function getOptInRules(params: { presetVersionId: string | null; firmPlatform: string | null }, signal?: AbortSignal) {
  const query = new URLSearchParams();
  if (params.presetVersionId) query.set("preset_version_id", params.presetVersionId);
  if (params.firmPlatform) query.set("firm_platform", params.firmPlatform);
  const qs = query.size > 0 ? `?${query.toString()}` : "";
  return apiFetch<OptInRules>(`/api/v1/opt-in-rules${qs}`, { signal });
}

export const getRuleVersion = (id: string, signal?: AbortSignal) => apiFetch<RuleVersion>(`/api/v1/rule-versions/${segment(id)}`, { signal });

/**
 * The standard size(s) a non-standard account size copies its profile from (S6 read). The candidates depend on the size
 * only; the instrument just decides `has_default`, so the first catalog instrument is enough here.
 */
export async function getNearestProfileSizeCandidates(sizeAmount: string, signal?: AbortSignal) {
  const { instruments } = await apiFetch<{ instruments: { id: string }[] }>("/api/v1/instruments", { signal });
  const first = instruments[0];
  if (!first) return [];
  const query = new URLSearchParams({ size_amount: sizeAmount, instrument_id: first.id });
  const result = await apiFetch<{ candidates: { size_amount: string }[] }>(`/api/v1/profile-sizes/nearest?${query.toString()}`, { signal });
  return result.candidates.map((c) => c.size_amount);
}
