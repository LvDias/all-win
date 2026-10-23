// INTERIM: mirrors contract 2.1.0 (draft, modules/accounts.yaml + presets.yaml) until Vitral regenerates
// src/shared/api/schema.gen.ts. Then this file re-exports from `@/shared/api/types` and nothing else changes.

export type AccountKind = "prop" | "real";
export type LifecycleState = "active" | "paused" | "disabled";
export type OptInKind = "automation" | "copy";
export type FirmStance = "allowed" | "forbidden" | "conditional" | "unknown" | "not_confirmed";
export type OptInChoice = "accepted" | "declined";
export type PendingReason = "stance_changed" | "text_changed" | "preset_attached";
export type PlatformAccountKind = "simulation" | "playback" | "live" | "unknown";
export type ConnectionStatus = "awaiting_pairing" | "connected" | "disconnected" | "outdated" | "unpaired";

export type OptInState = {
  kind: OptInKind;
  firm_stance?: FirmStance;
  state: "not_required" | "chosen" | "pending_review";
  pending_reason?: PendingReason;
  choice?: OptInChoice;
  chosen_at?: string;
  rule_version_id?: string;
  current_rule_version_id?: string;
};

export type AccountBinding = {
  connection_id: string;
  connection_name: string;
  connection_status: ConnectionStatus;
  platform_account: string;
  kind: PlatformAccountKind;
  oco_location: "local" | "server" | "unknown";
  bound_at?: string;
};

export type Account = {
  id: string;
  kind: AccountKind;
  display_name: string;
  firm_name?: string;
  plan_name?: string;
  phase?: string;
  firm_id?: string;
  preset_version_id?: string;
  firm_platform?: string;
  size_amount: string;
  currency: string;
  multiplier: number;
  max_position_qty: number;
  lifecycle_state: LifecycleState;
  binding?: AccountBinding;
  opt_ins: { automation: OptInState; copy: OptInState };
  has_open_position: boolean;
  row_version: number;
  created_at: string;
  updated_at: string;
};

export type PlatformAccountRef = { connection_id: string; platform_account: string };
export type OptInChoiceRequest = { choice: OptInChoice; rule_version_id: string };
export type OptInChoices = Partial<Record<OptInKind, OptInChoiceRequest>>;
export type StepUpProof = { challenge_id: string; code: string };

export type AccountWriteBody = {
  display_name: string;
  firm_name?: string;
  plan_name?: string;
  phase?: string;
  preset_version_id?: string;
  firm_platform?: string;
  size_amount?: string;
  multiplier?: number;
  max_position_qty: number;
  binding?: PlatformAccountRef;
  opt_ins?: OptInChoices;
  step_up?: StepUpProof;
};
export type AccountCreateBody = AccountWriteBody & { kind: AccountKind };
export type AccountUpdateBody = AccountWriteBody & { row_version: number };

export type ArchiveImpact = {
  has_open_position: boolean;
  routes: { source_id: string; source_name: string }[];
  follower_links: { account_id: string; display_name: string; role: "master" | "follower" }[];
};

export type RuleQuote = { text: string; language: string; page_title: string; section?: string; url: string; accessed_at: string };
export type LocalizedText = { "pt-BR": string; en: string };
export type RuleVersion = {
  id: string;
  kind: OptInKind;
  firm_stance: FirmStance;
  is_generic: boolean;
  firm_id?: string;
  firm_name?: string;
  version_number: number;
  published_at: string;
  quotes: RuleQuote[];
  research?: { searched_on: string; sources: { page_title?: string; url: string }[] };
  change?: { previous_rule_version_id: string; previous_firm_stance: FirmStance; cause: "stance_changed" | "text_changed" | "not_material"; reason?: LocalizedText };
};
export type OptInRules = { automation: RuleVersion; copy: RuleVersion };
