import { apiFetch } from "@/shared/api/client";
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

export const accountKeys = {
  all: ["accounts"] as const,
  list: () => [...accountKeys.all, "list"] as const,
  detail: (id: string) => [...accountKeys.all, "detail", id] as const,
  impact: (id: string) => [...accountKeys.all, "impact", id] as const,
  rules: (presetVersionId: string | null, firmPlatform: string | null) => ["opt-in-rules", presetVersionId, firmPlatform] as const,
  ruleVersion: (id: string) => ["rule-version", id] as const,
};

export const listAccounts = (signal?: AbortSignal) => apiFetch<{ accounts: Account[] }>(base, { signal }).then((r) => r.accounts);
export const getAccount = (id: string, signal?: AbortSignal) => apiFetch<Account>(`${base}/${id}`, { signal });

/** `idempotencyKey` is generated once per form submission and reused on retries (the retry after a step-up too). */
export const createAccount = (body: AccountCreateBody, idempotencyKey: string) =>
  apiFetch<Account>(base, { method: "POST", body, headers: { "Idempotency-Key": idempotencyKey } });

export const updateAccount = (id: string, body: AccountUpdateBody) => apiFetch<Account>(`${base}/${id}`, { method: "PUT", body });
export const archiveAccount = (id: string) => apiFetch<void>(`${base}/${id}`, { method: "DELETE" });
export const getArchiveImpact = (id: string, signal?: AbortSignal) => apiFetch<ArchiveImpact>(`${base}/${id}/archive-impact`, { signal });

export const setLifecycle = (id: string, state: LifecycleState, rowVersion: number) =>
  apiFetch<Account>(`${base}/${id}/lifecycle`, { method: "PUT", body: { state, row_version: rowVersion } });

export const setOptIn = (id: string, kind: OptInKind, body: { choice: OptInChoice; rule_version_id: string; step_up?: StepUpProof }) =>
  apiFetch<OptInState>(`${base}/${id}/opt-ins/${kind}`, { method: "PUT", body });

export function getOptInRules(params: { presetVersionId: string | null; firmPlatform: string | null }, signal?: AbortSignal) {
  const query = new URLSearchParams();
  if (params.presetVersionId) query.set("preset_version_id", params.presetVersionId);
  if (params.firmPlatform) query.set("firm_platform", params.firmPlatform);
  const qs = query.size > 0 ? `?${query.toString()}` : "";
  return apiFetch<OptInRules>(`/api/v1/opt-in-rules${qs}`, { signal });
}

export const getRuleVersion = (id: string, signal?: AbortSignal) => apiFetch<RuleVersion>(`/api/v1/rule-versions/${id}`, { signal });
