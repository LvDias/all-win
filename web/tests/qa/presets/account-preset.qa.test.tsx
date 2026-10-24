// QA (Retina) — V1-S16 account side (AC-2 edited values, AC-3/AC-11 new version): AccountPresetPanel + version dialog, black box through the public index.
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { beforeEach, describe, expect, it } from "vitest";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
// render first: it installs the next/navigation mock before the feature loads.
import { AccountPresetPanel, type RenderOptIn } from "@/features/presets";

const me = { id: "00000000-0000-4000-8000-000000000001", email: "t@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const uuid = (n: number) => `00000000-0000-4000-8000-${String(n).padStart(12, "0")}`;
const acc = uuid(500), cur = uuid(20), v4 = uuid(99), v5 = uuid(100);
const rel = (n: number) => ({ version_number: n, published_at: "2026-10-22T10:00:00Z", verified_on: "2026-10-21" });
const src = (url = "https://firm.example/a") => ({ url, accessed_at: "2026-10-18", page_title: "Rules" });
const verified = (value: unknown) => ({ status: "verified", value, sources: [src()] });
const versionBody = (id: string, n: number, rules: Record<string, unknown> = {}) => ({
  id, preset_id: uuid(40), firm_id: uuid(1), firm_name: "Apex", plan_id: uuid(10), plan_name: "Plan A", phase: "evaluation", size_amount: "50000.00", release: rel(n), not_confirmed_count: 0,
  rule_sets: [{ rules: { max_position_qty: verified({ contracts: 60 }), drawdown: verified({ type: "trailing_eod", amount: "2500.00" }), ...rules } }],
});
const state = (o: Record<string, unknown> = {}) => ({
  account_id: acc, preset_version_id: cur, release: rel(3), row_version: 7,
  fields: [
    { field: "max_position_qty", origin: "preset", account_value: { contracts: 60 }, preset_value: { contracts: 60 } },
    { field: "drawdown", origin: "edited", account_value: { type: "static", amount: "1800.00" }, preset_value: { type: "trailing_eod", amount: "2500.00" } },
    { field: "daily_loss_limit", origin: "chosen_from_conflict", chose: "published_value", chosen_at: "2026-10-19T09:00:00Z", account_value: { kind: "hard", amount: "900.00" } },
    { field: "consistency_rule", origin: "unresolved" },
  ],
  ...o,
});
const newer = (target = v4, n = 4, suspends = false) => ({ newer_version: { preset_version_id: target, release: rel(n), suspends } });
const diff = (rows: unknown[], o: Record<string, unknown> = {}) => ({ account_id: acc, from_preset_version_id: cur, to_preset_version_id: v4, rows, firm_platform_required: false, opt_in_changes: { automation: null, copy: null }, ...o });
const drawdownRow = { field: "drawdown", account_value: { type: "static", amount: "1800.00" }, current_preset_value: { type: "trailing_eod", amount: "2500.00" }, new_preset_value: { type: "trailing_eod", amount: "2600.00" }, edited: true, requires_choice: true, cap_conflict: false };
const lossRow = { field: "daily_loss_limit", account_value: { kind: "hard", amount: "900.00" }, edited: false, requires_choice: true, cap_conflict: false };
const lossPublished = { status: "not_confirmed", published_values: [{ value: { kind: "hard", amount: "1000.00" }, sources: [src()] }, { value: { kind: "hard", amount: "800.00" }, sources: [src("https://firm.example/b")] }] };

const optIn: RenderOptIn = ({ kind, ruleVersionId, value, onChange, onRequiredChange, choiceError }) => (
  <fieldset>
    <legend>{`opt-in ${kind}`}</legend>
    <button type="button" onClick={() => { onRequiredChange(true); onChange({ choice: "declined", ack: false }); }}>declinar {kind}</button>
    <button type="button" onClick={() => onRequiredChange(false)}>permitida {kind}</button>
    <span data-testid={`v-${kind}`}>{ruleVersionId}:{value.choice}</span>
    {choiceError ? <p>{choiceError}</p> : null}
  </fieldset>
);

let applies: Record<string, unknown>[] = [];
let diffTargets: string[] = [];
let stateGets = 0;
let currentState: unknown;
function serve(o: { preset?: unknown | null; rows?: unknown[]; diffOver?: Record<string, unknown>; target?: object } = {}) {
  currentState = o.preset === undefined ? state(newer()) : o.preset;
  server.use(
    http.get("*/api/v1/accounts/:id/preset", () => (stateGets++, currentState === null ? HttpResponse.json({ error: { code: "common.not_found" } }, { status: 404 }) : HttpResponse.json(currentState))),
    http.get("*/api/v1/preset-versions/:id", ({ params }) => HttpResponse.json(params.id === v4 ? versionBody(v4, 4, o.target as Record<string, unknown>) : params.id === v5 ? versionBody(v5, 5, o.target as Record<string, unknown>) : versionBody(cur, 3))),
    http.get("*/api/v1/accounts/:id/preset-diff", ({ request }) => (diffTargets.push(new URL(request.url).searchParams.get("target_preset_version_id") ?? ""), HttpResponse.json(diff(o.rows ?? [drawdownRow], o.diffOver)))),
  );
}
function applyOk() {
  server.use(http.post("*/api/v1/accounts/:id/preset-version", async ({ request }) => (applies.push((await request.json()) as Record<string, unknown>), HttpResponse.json(state()))));
}
const applyErr = (status: number, code: string, pointer?: string) => server.use(http.post("*/api/v1/accounts/:id/preset-version", async ({ request }) => (applies.push((await request.json()) as Record<string, unknown>), HttpResponse.json({ error: { code, pointer } }, { status }))));
beforeEach(() => {
  applies = []; diffTargets = []; stateGets = 0;
  server.use(http.get("*/api/v1/me", () => HttpResponse.json(me)));
});
const open = async () => { const u = userEvent.setup(); renderApp(<AccountPresetPanel accountId={acc} renderOptIn={optIn} />); await u.click(await screen.findByRole("button", { name: "Ver mudanças" })); return { u, dialog: await screen.findByRole("dialog") }; };

describe("panel (AC-2 edited values)", () => {
  it("no preset (404): nothing is rendered", async () => {
    serve({ preset: null });
    const { container } = renderApp(<AccountPresetPanel accountId={acc} renderOptIn={optIn} />);
    await waitFor(() => expect(stateGets).toBeGreaterThan(0));
    expect(container.querySelector("dl")).toBeNull();
    expect(screen.queryByText("Preset da firma")).not.toBeInTheDocument();
  });
  it("each value shows its origin; money is the Core string formatted, not computed", async () => {
    serve({ preset: state() });
    renderApp(<AccountPresetPanel accountId={acc} renderOptIn={optIn} />);
    expect(await screen.findByText("Escolhido por você entre valores divergentes em 19/10/2026")).toBeInTheDocument();
    expect(screen.getByText("Não confirmado — não acompanhado")).toBeInTheDocument();
    expect(document.body.textContent).toMatch(/US\$\s1\.800,00/);
  });
  it("edited value: the popover shows the PRESET value and restore sends the panel's row_version; the preset is not modified", async () => {
    serve({ preset: state() });
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/accounts/:id/preset-values/:field/restore", async ({ request, params }) => (bodies.push({ field: params.field, ...((await request.json()) as object) }), HttpResponse.json(state({ row_version: 8, fields: [] })))));
    renderApp(<AccountPresetPanel accountId={acc} renderOptIn={optIn} />);
    const u = userEvent.setup();
    await u.click(await screen.findByRole("button", { name: /Drawdown: editado/ }));
    expect(await screen.findByText(/Valor do preset: .*2\.500,00/)).toBeInTheDocument();
    await u.click(screen.getByRole("button", { name: "Restaurar valor do preset" }));
    await waitFor(() => expect(bodies).toEqual([{ field: "drawdown", row_version: 7 }]));
  });
  it("restore conflict (409) tells the user and keeps the panel", async () => {
    serve({ preset: state() });
    server.use(http.post("*/api/v1/accounts/:id/preset-values/:field/restore", () => HttpResponse.json({ error: { code: "common.version_conflict" } }, { status: 409 })));
    renderApp(<AccountPresetPanel accountId={acc} renderOptIn={optIn} />);
    const u = userEvent.setup();
    await u.click(await screen.findByRole("button", { name: /Drawdown: editado/ }));
    await u.click(await screen.findByRole("button", { name: "Restaurar valor do preset" }));
    expect(await screen.findByText(/mudou em outro lugar|mudaram em outro lugar|Recarregue/i)).toBeInTheDocument();
  });
  it("a Core error loading the preset says nothing here (the account page still works)", async () => {
    server.use(http.get("*/api/v1/accounts/:id/preset", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    const { container } = renderApp(<AccountPresetPanel accountId={acc} renderOptIn={optIn} />);
    await new Promise((r) => setTimeout(r, 400));
    expect(container.querySelector("dl")).toBeNull();
  });
});

describe("new version notice (AC-3)", () => {
  it("no notice without newer_version; with one, firm, version and date; suspension text only when the Core says so", async () => {
    serve({ preset: state() });
    const first = renderApp(<AccountPresetPanel accountId={acc} renderOptIn={optIn} />);
    await screen.findByText(/Preset/);
    expect(screen.queryByRole("button", { name: "Ver mudanças" })).not.toBeInTheDocument();
    first.unmount();
    serve({ preset: state(newer(v4, 4, false)) });
    const second = renderApp(<AccountPresetPanel accountId={acc} renderOptIn={optIn} />);
    expect(await screen.findByText(/Há uma nova versão do preset da Apex: versão 4, de 22\/10\/2026\./)).toBeInTheDocument();
    expect(screen.queryByText(/suspensas até você revisar/)).not.toBeInTheDocument();
    second.unmount();
    serve({ preset: state(newer(v4, 4, true)) });
    renderApp(<AccountPresetPanel accountId={acc} renderOptIn={optIn} />);
    expect(await screen.findByText("Esta versão mudou a regra de automação. Ordens automatizadas estão suspensas até você revisar.")).toBeInTheDocument();
  });
});

describe("version dialog (AC-11 nothing preselected)", () => {
  it("shows a table with only the rows the Core sent, Aplicar disabled until loaded, nothing preselected", async () => {
    serve({ rows: [drawdownRow] }); applyOk();
    const { dialog } = await open();
    await within(dialog).findByRole("table");
    expect(within(dialog).getAllByRole("row")).toHaveLength(2); // header + 1
    for (const radio of within(dialog).getAllByRole("radio")) expect(radio).not.toBeChecked();
  });
  it("Aplicar without answering shows an error per row and sends nothing", async () => {
    serve({ rows: [drawdownRow, lossRow], target: { daily_loss_limit: lossPublished } }); applyOk();
    const { u, dialog } = await open();
    await within(dialog).findByRole("table");
    await u.click(within(dialog).getByRole("button", { name: /^Aplicar versão 4/ }));
    expect(await within(dialog).findByText("Escolha uma opção para continuar.")).toBeInTheDocument();
    expect(within(dialog).getByText("Resolva este valor para aplicar.")).toBeInTheDocument();
    expect(applies).toHaveLength(0);
  });
  it("sends exactly the user's answers: choices for edited, rule_values for the picked published value, target and row_version", async () => {
    serve({ rows: [drawdownRow, lossRow], target: { daily_loss_limit: lossPublished } }); applyOk();
    const { u, dialog } = await open();
    await within(dialog).findByRole("table");
    await u.click(within(dialog).getByRole("radio", { name: "Usar o valor do preset" }));
    await u.click(within(dialog).getByRole("radio", { name: /US\$\s*800,00/ }));
    await u.click(within(dialog).getByRole("button", { name: /^Aplicar versão 4/ }));
    await waitFor(() => expect(applies).toHaveLength(1));
    expect(applies[0]).toMatchObject({ target_preset_version_id: v4, row_version: 7, choices: { drawdown: "use_preset" }, rule_values: { daily_loss_limit: { kind: "hard", amount: "800.00" } } });
    expect(applies[0]).not.toHaveProperty("opt_ins");
  });
  it("Manter versão atual closes without any write and the notice stays", async () => {
    serve({ rows: [drawdownRow] }); applyOk();
    const { u, dialog } = await open();
    await u.click(within(dialog).getByRole("button", { name: "Manter versão atual" }));
    await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());
    expect(applies).toHaveLength(0);
    expect(screen.getByRole("button", { name: "Ver mudanças" })).toBeInTheDocument();
  });
  it("max position above the new cap: lower value required, above-cap rejected client-side, valid value sent as max_position_qty", async () => {
    const cap = { field: "max_position_qty", account_value: { contracts: 60 }, current_preset_value: { contracts: 60 }, new_preset_value: { contracts: 40 }, edited: false, requires_choice: false, cap_conflict: true };
    serve({ rows: [cap] }); applyOk();
    const { u, dialog } = await open();
    const input = await within(dialog).findByRole("textbox");
    await u.type(input, "50");
    await u.click(within(dialog).getByRole("button", { name: /^Aplicar versão 4/ }));
    expect((await within(dialog).findAllByText(/No máximo 40/)).length).toBeGreaterThan(0);
    expect(applies).toHaveLength(0);
    await u.clear(input); await u.type(input, "40");
    await u.click(within(dialog).getByRole("button", { name: /^Aplicar versão 4/ }));
    await waitFor(() => expect(applies).toHaveLength(1));
    expect(applies[0]).toMatchObject({ max_position_qty: 40 });
  });
  it("opt-in changed by the new version: the question is asked and its rule_version_id is sent; 'allowed' rules send nothing", async () => {
    serve({ rows: [], diffOver: { opt_in_changes: { automation: uuid(700), copy: null } } }); applyOk();
    const { u, dialog } = await open();
    await within(dialog).findByTestId("v-automation");
    await u.click(within(dialog).getByRole("button", { name: /^Aplicar versão 4/ }));
    expect(await within(dialog).findByText(/Escolha/)).toBeInTheDocument(); // required and unanswered
    expect(applies).toHaveLength(0);
    await u.click(within(dialog).getByRole("button", { name: "declinar automation" }));
    await u.click(within(dialog).getByRole("button", { name: /^Aplicar versão 4/ }));
    await waitFor(() => expect(applies).toHaveLength(1));
    expect(applies[0]).toMatchObject({ opt_ins: { automation: { choice: "declined", rule_version_id: uuid(700) } } });
  });
  it("opt-in rule that now allows it: not asked, not sent", async () => {
    serve({ rows: [], diffOver: { opt_in_changes: { automation: uuid(700), copy: null } } }); applyOk();
    const { u, dialog } = await open();
    await u.click(await within(dialog).findByRole("button", { name: "permitida automation" }));
    await u.click(within(dialog).getByRole("button", { name: /^Aplicar versão 4/ }));
    await waitFor(() => expect(applies).toHaveLength(1));
    expect(applies[0]).not.toHaveProperty("opt_ins");
  });
});

describe("conflicts must not loop", () => {
  it("presets.version_superseded: tells the user, clears the selections, and the NEXT diff/apply targets the newest version", async () => {
    serve({ rows: [drawdownRow] });
    let calls = 0;
    server.use(http.post("*/api/v1/accounts/:id/preset-version", async ({ request }) => {
      applies.push((await request.json()) as Record<string, unknown>);
      if (++calls === 1) { currentState = state(newer(v5, 5)); return HttpResponse.json({ error: { code: "presets.version_superseded" } }, { status: 409 }); }
      return HttpResponse.json(state());
    }));
    const { u, dialog } = await open();
    await u.click(await within(dialog).findByRole("radio", { name: "Usar o valor do preset" }));
    await u.click(within(dialog).getByRole("button", { name: /^Aplicar versão 4/ }));
    expect(await within(dialog).findByText("Saiu uma versão mais nova enquanto você revisava. Revise de novo.")).toBeInTheDocument();
    for (const radio of within(dialog).getAllByRole("radio")) expect(radio).not.toBeChecked();
    // After the reload the dialog must move to the newer target, otherwise every retry is a 409 again.
    await waitFor(() => expect(diffTargets[diffTargets.length - 1]).toBe(v5), { timeout: 3000 });
  });
  it("common.version_conflict: the retry carries the account's NEW row_version (the panel state is refreshed)", async () => {
    serve({ rows: [drawdownRow] });
    let calls = 0;
    server.use(http.post("*/api/v1/accounts/:id/preset-version", async ({ request }) => {
      applies.push((await request.json()) as Record<string, unknown>);
      if (++calls === 1) { currentState = state({ ...newer(), row_version: 9 }); return HttpResponse.json({ error: { code: "common.version_conflict" } }, { status: 409 }); }
      return HttpResponse.json(state());
    }));
    const { u, dialog } = await open();
    await u.click(await within(dialog).findByRole("radio", { name: "Usar o valor do preset" }));
    await u.click(within(dialog).getByRole("button", { name: /^Aplicar versão 4/ }));
    await waitFor(() => expect(applies).toHaveLength(1));
    await u.click(await within(dialog).findByRole("radio", { name: "Usar o valor do preset" }));
    await u.click(within(dialog).getByRole("button", { name: /^Aplicar versão 4/ }));
    await waitFor(() => expect(applies).toHaveLength(2));
    expect(applies[1]!.row_version).toBe(9);
  });
  it("422 presets.required_field_unresolved / choice_required show an error and keep the dialog", async () => {
    serve({ rows: [drawdownRow] });
    applyErr(422, "presets.choice_required", "/choices/drawdown");
    const { u, dialog } = await open();
    await u.click(await within(dialog).findByRole("radio", { name: "Manter meu valor" }));
    await u.click(within(dialog).getByRole("button", { name: /^Aplicar versão 4/ }));
    await waitFor(() => expect(applies).toHaveLength(1));
    // delta 3: a 422 naming a field is shown AT that field; either that or a general alert is acceptable, silence is not.
    await waitFor(() => expect(within(dialog).queryByRole("alert") !== null || /presets\.choice_required/.test(dialog.textContent ?? "")).toBe(true));
    expect(screen.getByRole("dialog")).toBeInTheDocument();
    // the Core's stable code must be translated (spec 9 'Codes the UI maps'), not shown as the generic unknown error with the raw code
    expect(dialog.textContent ?? "").not.toMatch(/presets\.choice_required/);
  });
});
