import type { Account, OptInRules, OptInState, RuleVersion } from "../api/types";

export const ids = {
  conn: "5a2b8c3d-1e4f-4a6b-8c7d-9e0f1a2b3c4d",
  acc1: "11111111-1111-4111-8111-111111111111",
  acc2: "22222222-2222-4222-8222-222222222222",
  ruleAuto: "1f2e3d4c-5b6a-4978-8a9b-0c1d2e3f4a5b",
  ruleAuto2: "2a3b4c5d-6e7f-4a8b-9c0d-1e2f3a4b5c6d",
  ruleCopy: "3b4c5d6e-7f8a-4b9c-8d1e-2f3a4b5c6d7e",
};

const notRequired = (kind: OptInState["kind"], stance?: OptInState["firm_stance"]): OptInState => ({ kind, state: "not_required", ...(stance ? { firm_stance: stance } : {}) });

export function realAccount(over: Partial<Account> = {}): Account {
  return {
    id: ids.acc2,
    kind: "real",
    bracket_mode: "individual",
    sync: { state: "in_sync", version: 1, observable: true, reasons: [], reasons_total: 0 },
    display_name: "Conta real",
    size_amount: "25000.00",
    currency: "USD",
    multiplier: 1,
    max_position_qty: 3,
    lifecycle_state: "active",
    opt_ins: { automation: notRequired("automation"), copy: notRequired("copy") },
    has_open_position: false,
    row_version: 1,
    created_at: "2026-10-20T12:00:00Z",
    updated_at: "2026-10-20T12:00:00Z",
    ...over,
  };
}

export function propAccount(over: Partial<Account> = {}): Account {
  return {
    ...realAccount(),
    id: ids.acc1,
    kind: "prop",
    display_name: "Apex 50K 1",
    firm_name: "Apex Trader Funding",
    plan_name: "50K Rithmic",
    phase: "Evaluation",
    size_amount: "50000.00",
    max_position_qty: 2,
    opt_ins: {
      automation: { kind: "automation", firm_stance: "forbidden", state: "chosen", choice: "declined", chosen_at: "2026-10-21T10:00:00Z", rule_version_id: ids.ruleAuto, current_rule_version_id: ids.ruleAuto },
      copy: notRequired("copy", "forbidden"),
    },
    ...over,
  };
}

export function ruleVersion(over: Partial<RuleVersion> = {}): RuleVersion {
  return {
    id: ids.ruleAuto,
    kind: "automation",
    firm_stance: "forbidden",
    is_generic: false,
    firm_id: "9c0d1e2f-3a4b-4c5d-8e6f-7a8b9c0d1e2f",
    firm_name: "Apex Trader Funding",
    version_number: 3,
    published_at: "2026-10-01T00:00:00Z",
    quotes: [{ text: "No Automation or Algorithm Usage allowed.", language: "en", page_title: "Prohibited Activities", url: "https://example.com/rules", accessed_at: "2026-10-23" }],
    ...over,
  };
}

export const genericRules: OptInRules = {
  automation: ruleVersion({ id: ids.ruleAuto, firm_stance: "unknown", is_generic: true, firm_id: undefined, firm_name: undefined, quotes: [] }),
  copy: ruleVersion({ id: ids.ruleCopy, kind: "copy", firm_stance: "unknown", is_generic: true, firm_id: undefined, firm_name: undefined, quotes: [] }),
};
