import type { Account, FirmStance, OptInChoices, OptInKind, OptInState } from "../api/types";

/** Spec §5.1 / AC-10: any stance other than "allowed" needs an explicit choice. */
export function choiceRequired(stance: FirmStance): boolean {
  return stance !== "allowed";
}

/** Account-detail cards (§5.5): one per kind with a state other than `not_required`. */
export function visibleOptInCards(account: Account): OptInState[] {
  return [account.opt_ins.automation, account.opt_ins.copy].filter((s) => s.state !== "not_required");
}

export function hasPendingChoice(account: Account): boolean {
  return account.opt_ins.automation.state === "pending_review" || account.opt_ins.copy.state === "pending_review";
}

/** True when saving this choice may need step-up: only an accepted choice (declined never does, D9). */
export function needsStepUp(choices: OptInChoices | undefined): boolean {
  return Object.values(choices ?? {}).some((c) => c.choice === "accepted");
}

/** Picks the opt-in kind named by a pointer like `/opt_ins/automation`. */
export function optInKindFromPointer(pointer: string | undefined): OptInKind | null {
  const match = /^\/opt_ins\/(automation|copy)$/.exec(pointer ?? "");
  return match ? (match[1] as OptInKind) : null;
}
