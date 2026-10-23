import type { components } from "@/shared/api/schema.gen";
import type { Account } from "@/shared/api/types";

type AccountUpdateBody = components["schemas"]["AccountUpdateRequest"];

/** The decimal text without trailing zeros, so "0.3500" and "0.35" compare equal (exact strings, no arithmetic). */
export function canonicalAmount(value: string): string {
  return value.includes(".") ? value.replace(/0+$/, "").replace(/\.$/, "") : value;
}

/**
 * `updateAccount` replaces the whole editable state (an absent `binding` unbinds), so changing only the commission floor
 * sends the stored account back unchanged plus the new floor. `opt_ins` is left out: nothing about the automation rule
 * version changes. `commission`: a string sets the floor, `null` removes it, `undefined` keeps it (absent = unchanged).
 */
export function accountUpdateBody(account: Account, commission: string | null | undefined): AccountUpdateBody {
  const body: AccountUpdateBody = {
    row_version: account.row_version,
    display_name: account.display_name,
    max_position_qty: account.max_position_qty,
    multiplier: account.multiplier,
  };
  if (account.preset_version_id) {
    body.preset_version_id = account.preset_version_id;
    if (account.firm_platform) body.firm_platform = account.firm_platform;
  } else if (account.kind === "prop") {
    body.firm_name = account.firm_name;
    body.plan_name = account.plan_name;
    body.phase = account.phase;
    body.size_amount = account.size_amount;
  } else {
    body.size_amount = account.size_amount;
  }
  if (account.binding) body.binding = { connection_id: account.binding.connection_id, platform_account: account.binding.platform_account };
  if (commission !== undefined) body.commission_per_contract_side = commission;
  return body;
}
