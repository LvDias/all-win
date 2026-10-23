import type { components } from "@/shared/api/schema.gen";

type S = components["schemas"];
const id = (n: number) => `00000000-0000-4000-8000-${String(n).padStart(12, "0")}`;
export const ids = { apex: id(1), topstep: id(2), plan: id(10), v50: id(20), v100: id(21) };

const release = { version_number: 3, published_at: "2026-10-20T12:00:00Z", verified_on: "2026-10-18" };
const quote = { text: "Automated trading is permitted.", language: "en", page_title: "Rules", url: "https://firm.example/rules", accessed_at: "2026-10-18" };
const source = { url: "https://firm.example/rules", accessed_at: "2026-10-18", page_title: "Rules" };

export const apex: S["FirmSummary"] = {
  id: ids.apex, slug: "apex", name: "Apex", plan_names: ["Plan A", "Plan B", "Plan C"],
  stances: { automation: "allowed", copy: "not_confirmed", vps: "allowed", personal_device: "unknown" },
  not_confirmed_count: 2, release,
};
export const topstep: S["FirmSummary"] = {
  id: ids.topstep, slug: "topstep", name: "Topstep", plan_names: ["Combine"],
  stances: { automation: "forbidden", copy: "forbidden", vps: "forbidden", personal_device: "required" },
  not_confirmed_count: 0, release,
  manual_only: { reason: { ...quote, text: "Orders must originate from your personal device." } },
};

export const apexDetail: S["FirmDetail"] = {
  summary: apex,
  stances: {
    automation: [{ id: id(30), kind: "automation", firm_stance: "allowed", is_generic: false, firm_id: ids.apex, firm_name: "Apex", version_number: 3, published_at: release.published_at, quotes: [quote] }],
    copy: [{ id: id(31), kind: "copy", firm_stance: "not_confirmed", is_generic: false, firm_id: ids.apex, firm_name: "Apex", version_number: 3, published_at: release.published_at, quotes: [quote, { ...quote, text: "Copy trading is not allowed.", url: "https://firm.example/faq" }] }],
    vps: { value: "allowed", quotes: [quote] },
    personal_device: { value: "unknown", quotes: [], research: { searched_on: "2026-10-18", sources: [source] } },
  },
  platforms: [],
  plans: [{ plan_id: ids.plan, name: "Plan A", phases: [{ phase: "evaluation", sizes: [
    { preset_id: id(40), preset_version_id: ids.v50, size_amount: "50000.00", size_label: "50K", platform_specific: false, not_confirmed_count: 1 },
    { preset_id: id(41), preset_version_id: ids.v100, size_amount: "100000.00", size_label: "100K", platform_specific: false, not_confirmed_count: 0 },
  ] }] }],
  excluded_plans: ["Legacy"],
};

const verified = <V,>(value: V) => ({ status: "verified" as const, value, sources: [source] });
export const version = (versionId: string, size: string, over: Partial<S["PresetRules"]> = {}): S["PresetVersion"] => ({
  id: versionId, preset_id: id(40), firm_id: ids.apex, firm_name: "Apex", plan_id: ids.plan, plan_name: "Plan A", phase: "evaluation",
  size_amount: size, release, not_confirmed_count: 0,
  rule_sets: [{ rules: {
    max_position_qty: verified({ contracts: 60 }),
    drawdown: verified({ type: "trailing_eod", amount: "2500.00" }),
    daily_loss_limit: verified({ kind: "none" }),
    trading_day_reset: verified({ local_time: "17:00", time_zone: "America/Chicago", published_text: "5:00 PM CT" }),
    evaluation_profit_target: verified({ amount: "3000.00" }),
    ...over,
  } }],
});
export const unconfirmedDrawdown: S["PresetRules"]["drawdown"] = {
  status: "not_confirmed",
  published_values: [
    { value: { type: "trailing_eod", amount: "2500.00" }, sources: [source] },
    { value: { type: "static", amount: "2000.00" }, sources: [{ ...source, url: "https://firm.example/faq", page_title: "FAQ" }] },
  ],
};
