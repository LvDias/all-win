import { otherValue } from "@/shared/components/conflict-picker";
import { emptyDraft } from "../other-value";
import { ids, unconfirmedDrawdown, version } from "../testing/fixtures";
import { buildOutcome, emptyPresetDraft, type PresetDraft } from "./preset-selection";

const draft = (resolutions: PresetDraft["resolutions"] = {}): PresetDraft => ({ ...emptyPresetDraft, presetVersionId: ids.v50, resolutions });
const build = (v = version(ids.v50, "50000.00"), d = draft()) => buildOutcome(v, d, "pt-BR", "Apex · Plan A");

describe("buildOutcome", () => {
  it("a fully verified preset is valid, sends no rule values and caps the position at the preset's", () => {
    const outcome = build();
    expect(outcome).toMatchObject({ valid: true, ruleValues: {}, maxPositionCap: 60, maxPositionDefault: 60, firmPlatform: null, sizeAmount: "50000.00" });
  });

  it("a required not-confirmed field blocks the preset until the user picks a published value or types another", () => {
    const v = version(ids.v50, "50000.00", { drawdown: unconfirmedDrawdown });
    expect(build(v)).toMatchObject({ valid: false, problems: { drawdown: "unresolved" } });
    expect(build(v, draft({ drawdown: { selected: "1", other: emptyDraft } }))).toMatchObject({ valid: true, ruleValues: { drawdown: { type: "static", amount: "2000.00" } } });
    expect(build(v, draft({ drawdown: { selected: otherValue, other: { a: "2250,50", b: "static" } } }))).toMatchObject({ valid: true, ruleValues: { drawdown: { type: "static", amount: "2250.5" } } });
    expect(build(v, draft({ drawdown: { selected: otherValue, other: { a: "", b: "static" } } }))).toMatchObject({ valid: false, problems: { drawdown: "required" } });
  });

  it("an optional not-confirmed field may stay unresolved", () => {
    const v = version(ids.v50, "50000.00", { consistency_rule: { status: "not_confirmed", published_values: [{ value: { text: "40%" }, sources: [] }, { value: { text: "30%" }, sources: [] }] } });
    expect(build(v)).toMatchObject({ valid: true, ruleValues: {} });
  });

  it("an edited verified value is sent as the account's own value", () => {
    const outcome = build(undefined, draft({ evaluation_profit_target: { selected: otherValue, other: { a: "3500", b: "" } } }));
    expect(outcome.ruleValues).toEqual({ evaluation_profit_target: { amount: "3500" } });
  });

  it("a not-confirmed maximum position defaults to the chosen value and allows up to the highest published one", () => {
    const v = version(ids.v50, "50000.00", { max_position_qty: { status: "not_confirmed", published_values: [{ value: { contracts: 40 }, sources: [] }, { value: { contracts: 60 }, sources: [] }] } });
    expect(build(v)).toMatchObject({ valid: false, problems: { max_position_qty: "unresolved" } });
    expect(build(v, draft({ max_position_qty: { selected: "0", other: emptyDraft } }))).toMatchObject({ valid: true, maxPositionDefault: 40, maxPositionCap: 60, ruleValues: {} });
  });

  describe("per-platform rule sets", () => {
    const src = [{ url: "https://firm.example/a", accessed_at: "2026-10-18" }];
    const rules = (contracts: number) => ({
      ...version(ids.v50, "50000.00").rule_sets[0]?.rules,
      max_position_qty: { status: "verified" as const, value: { contracts }, sources: src },
    });
    const base = version(ids.v50, "50000.00");
    const twoPlatforms = { ...base, rule_sets: [{ firm_platform: "rithmic", rules: rules(40) }, { firm_platform: "tradovate", rules: rules(60) }] } as typeof base;
    const withPlatform = (firmPlatform: string) => ({ ...draft(), firmPlatform });

    it("is unavailable, not prefilled, and invalid before a platform is chosen (never the first platform's rules)", () => {
      const outcome = buildOutcome(twoPlatforms, draft(), "pt-BR", "Apex");
      expect(outcome).toMatchObject({ unavailable: true, valid: false, maxPositionCap: null, maxPositionDefault: null, ruleValues: {} });
    });

    it("uses exactly the chosen platform's set", () => {
      expect(buildOutcome(twoPlatforms, withPlatform("tradovate"), "pt-BR", "Apex")).toMatchObject({ unavailable: false, valid: true, maxPositionCap: 60, firmPlatform: "tradovate" });
      expect(buildOutcome(twoPlatforms, withPlatform("rithmic"), "pt-BR", "Apex")).toMatchObject({ maxPositionCap: 40 });
    });

    it("is unavailable for a platform the preset has no rules for", () => {
      expect(buildOutcome(twoPlatforms, withPlatform("ninja"), "pt-BR", "Apex")).toMatchObject({ unavailable: true, valid: false });
    });
  });
});
