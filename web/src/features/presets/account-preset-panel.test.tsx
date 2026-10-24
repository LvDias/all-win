import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
import { AccountPresetPanel } from "./account-preset-panel";
import type { RenderOptIn } from "./version-diff-dialog";
import { ids, unconfirmedDrawdown, version } from "./testing/fixtures";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "t@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const accountId = "00000000-0000-4000-8000-0000000000aa";
const newVersionId = "00000000-0000-4000-8000-000000000099";
const release = { version_number: 4, published_at: "2026-10-22T10:00:00Z", verified_on: "2026-10-21" };

const state = (over: Record<string, unknown> = {}) => ({
  account_id: accountId,
  preset_version_id: ids.v50,
  release: { version_number: 3, published_at: "2026-10-20T12:00:00Z", verified_on: "2026-10-18" },
  row_version: 7,
  fields: [
    { field: "max_position_qty", origin: "preset", account_value: { contracts: 60 }, preset_value: { contracts: 60 } },
    { field: "drawdown", origin: "edited", account_value: { type: "static", amount: "1800.00" }, preset_value: { type: "trailing_eod", amount: "2500.00" } },
    { field: "daily_loss_limit", origin: "chosen_from_conflict", chose: "published_value", chosen_at: "2026-10-19T09:00:00Z", account_value: { kind: "hard", amount: "900.00" } },
    { field: "consistency_rule", origin: "unresolved" },
  ],
  ...over,
});

const optIn: RenderOptIn = ({ kind, ruleVersionId, value, onChange, onRequiredChange, choiceError }) => (
  <fieldset>
    <legend>{`opt-in ${kind} ${ruleVersionId}`}</legend>
    <button type="button" onClick={() => { onRequiredChange(true); onChange({ choice: "declined", ack: false }); }}>declinar</button>
    {choiceError ? <p>{choiceError}</p> : <span>{value.choice}</span>}
  </fieldset>
);

const diffBody = (rows: unknown[], over: Record<string, unknown> = {}) => ({
  account_id: accountId,
  from_preset_version_id: ids.v50,
  to_preset_version_id: newVersionId,
  rows,
  firm_platform_required: false,
  opt_in_changes: { automation: null, copy: null },
  ...over,
});

function serve(over: { preset?: unknown; diff?: unknown } = {}) {
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(user)),
    http.get("*/api/v1/accounts/:id/preset", () => (over.preset === null ? HttpResponse.json({ error: { code: "common.not_found" } }, { status: 404 }) : HttpResponse.json(over.preset ?? state()))),
    http.get("*/api/v1/preset-versions/:id", ({ params }) => HttpResponse.json(params.id === newVersionId ? { ...version(newVersionId, "50000.00", { drawdown: unconfirmedDrawdown }), release } : version(ids.v50, "50000.00"))),
    http.get("*/api/v1/accounts/:id/preset-diff", () => HttpResponse.json(over.diff ?? diffBody([]))),
  );
}

const withNewer = (suspends = false) => state({ newer_version: { preset_version_id: newVersionId, release, suspends } });

describe("AccountPresetPanel", () => {
  it("renders nothing for an account without a preset", async () => {
    serve({ preset: null });
    const { container } = renderApp(<AccountPresetPanel accountId={accountId} renderOptIn={optIn} />);
    await waitFor(() => expect(container.querySelector("dl")).toBeNull());
    expect(screen.queryByText("Preset da firma")).not.toBeInTheDocument();
  });

  it("marks each value's origin and restores an edited value with the account's row_version", async () => {
    serve();
    let body: unknown = null;
    server.use(http.post("*/api/v1/accounts/:id/preset-values/:field/restore", async ({ request, params }) => {
      body = { field: params.field, ...(await request.json() as object) };
      return HttpResponse.json(state({ fields: [] }));
    }));
    renderApp(<AccountPresetPanel accountId={accountId} renderOptIn={optIn} />);
    expect(await screen.findByText("Escolhido por você entre valores divergentes em 19/10/2026")).toBeInTheDocument();
    expect(screen.getByText("Não confirmado — não acompanhado")).toBeInTheDocument();
    await userEvent.click(screen.getByRole("button", { name: /Drawdown: editado/ }));
    expect(await screen.findByText(/Valor do preset: .*2\.500,00 · Trailing fim do dia/)).toBeInTheDocument();
    await userEvent.click(screen.getByRole("button", { name: "Restaurar valor do preset" }));
    await waitFor(() => expect(body).toEqual({ field: "drawdown", row_version: 7 }));
  });

  it("announces a new version and says automation is suspended when the Core says so", async () => {
    serve({ preset: withNewer(true) });
    renderApp(<AccountPresetPanel accountId={accountId} renderOptIn={optIn} />);
    expect(await screen.findByText(/Há uma nova versão do preset da Apex: versão 4, de 22\/10\/2026\./)).toBeInTheDocument();
    expect(screen.getByText("Esta versão mudou a regra de automação. Ordens automatizadas estão suspensas até você revisar.")).toBeInTheDocument();
  });

  it("asks for a choice on each edited value and each not-confirmed value, preselecting nothing, then applies once", async () => {
    serve({
      preset: withNewer(),
      diff: diffBody([
        { field: "drawdown", account_value: { type: "static", amount: "1800.00" }, current_preset_value: { type: "trailing_eod", amount: "2500.00" }, new_preset_value: { type: "trailing_eod", amount: "2600.00" }, edited: true, requires_choice: true, cap_conflict: false },
        { field: "daily_loss_limit", account_value: { kind: "hard", amount: "900.00" }, edited: false, requires_choice: true, cap_conflict: false },
      ]),
    });
    let sent: Record<string, unknown> | null = null;
    server.use(http.post("*/api/v1/accounts/:id/preset-version", async ({ request }) => {
      sent = (await request.json()) as Record<string, unknown>;
      return HttpResponse.json(state());
    }));
    // The new version's daily loss is not confirmed in the fixture: give it two published values.
    server.use(http.get("*/api/v1/preset-versions/:id", ({ params }) =>
      HttpResponse.json(params.id === newVersionId
        ? { ...version(newVersionId, "50000.00", { daily_loss_limit: { status: "not_confirmed", published_values: [{ value: { kind: "hard", amount: "1000.00" }, sources: [{ url: "https://firm.example/a", accessed_at: "2026-10-18", page_title: "A" }] }, { value: { kind: "soft_pause", amount: "800.00" }, sources: [{ url: "https://firm.example/b", accessed_at: "2026-10-18", page_title: "B" }] }] } }), release }
        : version(ids.v50, "50000.00"))));
    renderApp(<AccountPresetPanel accountId={accountId} renderOptIn={optIn} />);
    await userEvent.click(await screen.findByRole("button", { name: "Ver mudanças" }));
    const dialog = await screen.findByRole("dialog");
    await within(dialog).findByRole("group", { name: /Drawdown: manter seu valor/ });
    for (const radio of within(dialog).getAllByRole("radio")) expect(radio).not.toBeChecked();

    await userEvent.click(within(dialog).getByRole("button", { name: "Aplicar versão 4" }));
    expect(await within(dialog).findByText("Escolha uma opção para continuar.")).toBeInTheDocument();
    expect(within(dialog).getByText("Resolva este valor para aplicar.")).toBeInTheDocument();
    expect(sent).toBeNull();

    await userEvent.click(within(dialog).getByRole("radio", { name: "Manter meu valor" }));
    await userEvent.click(within(dialog).getByRole("radio", { name: /US\$\s*800,00 · pausa do dia/ }));
    await userEvent.click(within(dialog).getByRole("button", { name: "Aplicar versão 4" }));
    await waitFor(() => expect(sent).not.toBeNull());
    expect(sent).toEqual({
      target_preset_version_id: newVersionId,
      row_version: 7,
      choices: { drawdown: "keep_mine" },
      rule_values: { daily_loss_limit: { kind: "soft_pause", amount: "800.00" } },
    });
  });

  it("blocks apply until the maximum position is lowered to the new cap", async () => {
    serve({ preset: withNewer(), diff: diffBody([{ field: "max_position_qty", account_value: { contracts: 60 }, current_preset_value: { contracts: 60 }, new_preset_value: { contracts: 40 }, edited: false, requires_choice: false, cap_conflict: true }]) });
    let sent: Record<string, unknown> | null = null;
    server.use(http.post("*/api/v1/accounts/:id/preset-version", async ({ request }) => { sent = (await request.json()) as Record<string, unknown>; return HttpResponse.json(state()); }));
    renderApp(<AccountPresetPanel accountId={accountId} renderOptIn={optIn} />);
    await userEvent.click(await screen.findByRole("button", { name: "Ver mudanças" }));
    const dialog = await screen.findByRole("dialog");
    expect(await within(dialog).findByText("A nova versão limita a 40 contratos")).toBeInTheDocument();
    const input = within(dialog).getByLabelText("Posição máxima: reduza para aplicar");
    await userEvent.type(input, "45");
    await userEvent.click(within(dialog).getByRole("button", { name: "Aplicar versão 4" }));
    expect(await within(dialog).findByText("No máximo 40.")).toBeInTheDocument();
    expect(sent).toBeNull();
    await userEvent.clear(input);
    await userEvent.type(input, "40");
    await userEvent.click(within(dialog).getByRole("button", { name: "Aplicar versão 4" }));
    await waitFor(() => expect(sent).toEqual({ target_preset_version_id: newVersionId, row_version: 7, max_position_qty: 40 }));
  });

  it("asks the opt-in question for a changed rule version and sends it with that version id", async () => {
    const ruleVersion = "00000000-0000-4000-8000-0000000000bb";
    serve({ preset: withNewer(true), diff: diffBody([], { opt_in_changes: { automation: ruleVersion, copy: null } }) });
    let sent: Record<string, unknown> | null = null;
    server.use(http.post("*/api/v1/accounts/:id/preset-version", async ({ request }) => { sent = (await request.json()) as Record<string, unknown>; return HttpResponse.json(state()); }));
    renderApp(<AccountPresetPanel accountId={accountId} renderOptIn={optIn} />);
    await userEvent.click(await screen.findByRole("button", { name: "Ver mudanças" }));
    const dialog = await screen.findByRole("dialog");
    await within(dialog).findByText(`opt-in automation ${ruleVersion}`);
    await userEvent.click(within(dialog).getByRole("button", { name: "Aplicar versão 4" }));
    expect(await within(dialog).findByText("Escolha se esta conta aceita ordens automatizadas.")).toBeInTheDocument();
    expect(sent).toBeNull();
    await userEvent.click(within(dialog).getByRole("button", { name: "declinar" }));
    await userEvent.click(within(dialog).getByRole("button", { name: "Aplicar versão 4" }));
    await waitFor(() => expect(sent).toEqual({ target_preset_version_id: newVersionId, row_version: 7, opt_ins: { automation: { choice: "declined", rule_version_id: ruleVersion } } }));
  });

  it("reloads the comparison and clears the answers when a newer version came out meanwhile", async () => {
    serve({ preset: withNewer(), diff: diffBody([{ field: "drawdown", account_value: { type: "static", amount: "1800.00" }, current_preset_value: { type: "trailing_eod", amount: "2500.00" }, new_preset_value: { type: "trailing_eod", amount: "2600.00" }, edited: true, requires_choice: true, cap_conflict: false }]) });
    server.use(http.post("*/api/v1/accounts/:id/preset-version", () => HttpResponse.json({ error: { code: "presets.version_superseded" } }, { status: 409 })));
    renderApp(<AccountPresetPanel accountId={accountId} renderOptIn={optIn} />);
    await userEvent.click(await screen.findByRole("button", { name: "Ver mudanças" }));
    const dialog = await screen.findByRole("dialog");
    await userEvent.click(await within(dialog).findByRole("radio", { name: "Manter meu valor" }));
    await userEvent.click(within(dialog).getByRole("button", { name: "Aplicar versão 4" }));
    expect(await within(dialog).findByText("Saiu uma versão mais nova enquanto você revisava. Revise de novo.")).toBeInTheDocument();
    await waitFor(() => expect(within(dialog).getByRole("radio", { name: "Manter meu valor" })).not.toBeChecked());
  });

  it("offers only the account's own platform's published values and never another platform's", async () => {
    const src = [{ url: "https://firm.example/a", accessed_at: "2026-10-18", page_title: "A" }];
    const conflict = (a: string, b: string) => ({ status: "not_confirmed", published_values: [{ value: { type: "trailing_eod", amount: a }, sources: src }, { value: { type: "static", amount: b }, sources: src }] });
    const base = version(newVersionId, "50000.00");
    const twoPlatforms = { ...base, release, rule_sets: [
      { firm_platform: "rithmic", rules: { ...base.rule_sets[0]?.rules, drawdown: conflict("2600.00", "2000.00") } },
      { firm_platform: "tradovate", rules: { ...base.rule_sets[0]?.rules, drawdown: conflict("2100.00", "1900.00") } },
    ] };
    serve({
      preset: withNewer(),
      diff: diffBody([{ field: "drawdown", account_value: { type: "static", amount: "1800.00" }, edited: false, requires_choice: true, cap_conflict: false }]),
    });
    server.use(
      http.get("*/api/v1/accounts/:id/preset", () => HttpResponse.json({ ...withNewer(), firm_platform: "tradovate" })),
      http.get("*/api/v1/preset-versions/:id", ({ params }) => HttpResponse.json(params.id === newVersionId ? twoPlatforms : version(ids.v50, "50000.00"))),
    );
    let sent: Record<string, unknown> | null = null;
    server.use(http.post("*/api/v1/accounts/:id/preset-version", async ({ request }) => { sent = (await request.json()) as Record<string, unknown>; return HttpResponse.json(state()); }));
    renderApp(<AccountPresetPanel accountId={accountId} renderOptIn={optIn} />);
    await userEvent.click(await screen.findByRole("button", { name: "Ver mudanças" }));
    const dialog = await screen.findByRole("dialog");
    expect(await within(dialog).findByRole("radio", { name: /US\$\s*2\.100,00/ })).toBeInTheDocument();
    expect(within(dialog).queryByRole("radio", { name: /2\.600,00/ })).not.toBeInTheDocument();
    await userEvent.click(within(dialog).getByRole("radio", { name: /US\$\s*1\.900,00 · Estático/ }));
    await userEvent.click(within(dialog).getByRole("button", { name: "Aplicar versão 4" }));
    await waitFor(() => expect(sent).toEqual({ target_preset_version_id: newVersionId, row_version: 7, rule_values: { drawdown: { type: "static", amount: "1900.00" } } }));
  });

  it("shows a 422 that names a field at that field", async () => {
    serve({ preset: withNewer(), diff: diffBody([{ field: "drawdown", account_value: { type: "static", amount: "1800.00" }, current_preset_value: { type: "trailing_eod", amount: "2500.00" }, new_preset_value: { type: "trailing_eod", amount: "2600.00" }, edited: true, requires_choice: true, cap_conflict: false }]) });
    server.use(http.post("*/api/v1/accounts/:id/preset-version", () => HttpResponse.json({ error: { code: "presets.choice_required", pointer: "/choices/drawdown" } }, { status: 422 })));
    renderApp(<AccountPresetPanel accountId={accountId} renderOptIn={optIn} />);
    await userEvent.click(await screen.findByRole("button", { name: "Ver mudanças" }));
    const dialog = await screen.findByRole("dialog");
    await userEvent.click(await within(dialog).findByRole("radio", { name: "Manter meu valor" }));
    await userEvent.click(within(dialog).getByRole("button", { name: "Aplicar versão 4" }));
    const group = await within(dialog).findByRole("group", { name: /Drawdown: manter seu valor/ });
    // The unmapped code falls back to the generic text that carries the code; it sits at the field, not in a banner.
    expect(group).toHaveTextContent("presets.choice_required");
    expect(within(dialog).queryByRole("alert")).not.toBeInTheDocument();
  });

  it("tells the user when the preset section fails to load and lets them retry", async () => {
    serve();
    server.use(http.get("*/api/v1/accounts/:id/preset", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    renderApp(<AccountPresetPanel accountId={accountId} renderOptIn={optIn} />);
    expect(await screen.findByText("Código: common.internal", undefined, { timeout: 6000 })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Tentar novamente" })).toBeInTheDocument();
  });
});
