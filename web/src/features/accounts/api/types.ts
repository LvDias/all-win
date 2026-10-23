import type { components } from "@/shared/api/schema.gen";

type S = components["schemas"];

export type Account = S["Account"];
export type AccountKind = S["AccountKind"];
export type LifecycleState = S["LifecycleState"];
export type OptInKind = S["OptInKind"];
export type FirmStance = S["FirmStance"];
export type OptInState = S["OptInState"];
export type OptInChoices = S["OptInChoices"];
export type OptInChoiceRequest = S["OptInChoiceRequest"];
export type OptInChoice = OptInChoiceRequest["choice"];
export type PendingReason = NonNullable<OptInState["pending_reason"]>;
export type AccountBinding = S["AccountBinding"];
export type PlatformAccountRef = S["PlatformAccountRef"];
export type StepUpProof = S["StepUpProof"];
export type AccountCreateBody = S["AccountCreateRequest"];
export type AccountUpdateBody = S["AccountUpdateRequest"];
export type ArchiveImpact = S["ArchiveImpact"];
export type RuleQuote = S["RuleQuote"];
export type RuleVersion = S["RuleVersion"];
export type OptInRules = S["OptInRules"];
export type LocalizedText = S["LocalizedText"];
