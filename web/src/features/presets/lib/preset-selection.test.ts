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
});
