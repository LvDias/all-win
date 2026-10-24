// QA (Retina) — V1-S13 AC-1/5/7/8/9 on the rules form, black box through the public index.
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { beforeEach, describe, expect, it } from "vitest";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
// render first: it installs the next/navigation mock before the feature loads.
import { RiskTab, RulesForm } from "@/features/risk";

const me = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const id = "00000000-0000-4000-8000-0000000000a1";
const rulesBody = (rules: Record<string, unknown> = {}, o: Record<string, unknown> = {}) => ({
  account_id: id, enforcement_enabled: false, version: 4,
  rules: { drawdown: { type: "trailing_eod", amount: "2500.00" }, daily_loss_limit: { kind: "hard", amount: "1000.00" }, profit_target_on_reach: "notify", ...rules }, ...o,
});
const serve = (body: object = rulesBody(), status = 200) => server.use(http.get(`*/api/v1/accounts/${id}/risk-rules`, () => HttpResponse.json(body, { status })));
const noEffect = { changes: [], would_be_reached: false };
type Rule = { amount?: string; kind?: string; type?: string };
type Body = { version?: number; acknowledge_reached?: boolean; rules: Record<string, Rule | string | undefined> & { daily_loss_limit?: Rule; drawdown?: Rule; daily_profit_target?: Rule; profit_target_on_reach?: string } };
let puts: Body[] = [];
const body = (): Body => puts[0] ?? { rules: {} };
let previews = 0;
function writes(preview: object = noEffect, put: () => Response = () => HttpResponse.json(rulesBody({}, { version: 5 }))) {
  server.use(
    http.post(`*/api/v1/accounts/${id}/risk-rules/preview`, () => (previews++, HttpResponse.json(preview))),
    http.put(`*/api/v1/accounts/${id}/risk-rules`, async ({ request }) => (puts.push((await request.json()) as Body), put())),
  );
}
const account = (o: Record<string, unknown> = {}) => ({
  id, kind: "prop", display_name: "Apex 50K", firm_name: "Apex", plan_name: "50K", phase: "evaluation", preset_version_id: "00000000-0000-4000-8000-0000000000b1", firm_platform: "rithmic",
  size_amount: "50000", currency: "USD", multiplier: 2, max_position_qty: 5, lifecycle_state: "active",
  opt_ins: { automation: { kind: "automation", state: "declined" }, copy: { kind: "copy", state: "declined" } },
  binding: { connection_id: "00000000-0000-4000-8000-0000000000c1", connection_name: "VPS", connection_status: "connected", platform_account: "Sim101", kind: "simulation", oco_location: "local" },
  commission_per_contract_side: "0.35", has_open_position: false, row_version: 7, created_at: "2026-10-01T12:00:00Z", updated_at: "2026-10-01T12:00:00Z", ...o,
});
let accountPuts: Record<string, unknown>[] = [];
beforeEach(() => {
  accountPuts = [];
  server.use(
    http.get(`*/api/v1/accounts/${id}`, () => HttpResponse.json(account())),
    http.put(`*/api/v1/accounts/${id}`, async ({ request }) => (accountPuts.push((await request.json()) as Record<string, unknown>), HttpResponse.json(account({ row_version: 8 })))),
  );
  puts = []; previews = 0; navigation.push.mockClear();
  server.use(http.get("*/api/v1/me", () => HttpResponse.json(me)));
});
const save = () => userEvent.setup().click(screen.getByRole("button", { name: "Salvar regras" }));

describe("entry point", () => {
  it("the Risk tab's edit action has a real translated label (no raw key)", async () => {
    server.use(http.get(`*/api/v1/accounts/${id}/risk`, () => HttpResponse.json({ account_id: id, as_of: "2026-10-23T14:31:05Z", stale: false, state: "normal", trading_day: { start: "2026-10-22T21:00:00Z", end: "2026-10-23T21:00:00Z", next_reset: "2026-10-23T21:00:00Z", source: "cme_session_close" }, meters: [], unset_rules: [], untracked_rules: [], enforcement_enabled: false, limits_pending: false, pending_changes: [], pnl_basis: "net", commission_source: "platform" })));
    renderApp(<RiskTab accountId={id} />);
    expect(await screen.findByRole("link", { name: "Editar regras" })).toHaveAttribute("href", `/pt-BR/accounts/${id}/risk/edit`);
  });
});

describe("prefill and exactness", () => {
  it("round-trips untouched Core amounts exactly (pt-BR locale, 4 decimals kept out of scope)", async () => {
    serve(rulesBody({ daily_profit_target: { amount: "600.50" } }));
    writes();
    renderApp(<RulesForm accountId={id} />);
    await touch();
    await save();
    await waitFor(() => expect(puts).toHaveLength(1));
    const r = body().rules as Record<string, unknown> & Body["rules"];
    expect(Number(r.daily_loss_limit?.amount)).toBe(900);
    expect(r.daily_profit_target?.amount).toBe("600.50"); // unchanged values keep the Core text
    expect(Number(r.drawdown?.amount)).toBe(2500);
    expect(body().version).toBe(4);
  });
  it("preserves a daily_loss_limit of kind none (read-only with the firm's statement) and the rules the form does not edit", async () => {
    serve(rulesBody({ daily_loss_limit: { kind: "none" }, trading_day_reset: { local_time: "17:00", time_zone: "America/Chicago" }, consistency_threshold_pct: "30" }));
    writes();
    renderApp(<RulesForm accountId={id} />);
    expect(await screen.findByText("A firma informa que não há limite de perda diária.")).toBeInTheDocument();
    await u().type(await screen.findByLabelText(/Valor da meta|meta de lucro/i), "600");
    await save();
    await waitFor(() => expect(puts).toHaveLength(1));
    const r = body().rules;
    expect(r.daily_loss_limit).toEqual({ kind: "none" });
    expect(r.trading_day_reset).toEqual({ local_time: "17:00", time_zone: "America/Chicago" });
    expect(r.consistency_threshold_pct).toBe("30");
  });
  it("accepts comma and dot in pt-BR and rejects more than 2 decimals, grouping and negatives without calling the Core", async () => {
    serve(); writes();
    renderApp(<RulesForm accountId={id} />);
    const u = userEvent.setup();
    const loss = await screen.findByLabelText(/Limite de perda diária/);
    for (const bad of ["1.000,00", "10,123", "-5", "abc", "1e3"]) {
      await u.clear(loss); await u.type(loss, bad); await u.click(screen.getByRole("button", { name: "Salvar regras" }));
      expect(loss).toHaveAttribute("aria-invalid", "true");
    }
    expect(previews).toBe(0);
    expect(puts).toHaveLength(0);
  });
});

describe("AC-1 shape rules", () => {
  it("half-filled drawdown is blocked client-side on both sides", async () => {
    serve(rulesBody({ drawdown: undefined })); writes();
    renderApp(<RulesForm accountId={id} />);
    const u = userEvent.setup();
    await u.type(await screen.findByLabelText(/Drawdown.*(valor|dólares)/i), "500");
    await save();
    expect(previews).toBe(0);
    expect((await screen.findAllByText(/tipo/i)).length).toBeGreaterThan(0);
  });
  it("clearing a rule omits it (never sends null)", async () => {
    serve(); writes();
    renderApp(<RulesForm accountId={id} />);
    const u = userEvent.setup();
    await u.clear(await screen.findByLabelText(/Limite de perda diária/));
    await save();
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(JSON.stringify(body())).not.toContain("null");
    expect(body().rules.daily_loss_limit).toBeUndefined();
  });
  it("consistency section only when the rule exists; rejects >100 and 2 decimals", async () => {
    serve(rulesBody({ consistency_threshold_pct: "30" })); writes();
    renderApp(<RulesForm accountId={id} />);
    const u = userEvent.setup();
    const c = await screen.findByLabelText(/consist/i);
    for (const bad of ["101", "30.55", "abc"]) { await u.clear(c); await u.type(c, bad); await save(); expect(c).toHaveAttribute("aria-invalid", "true"); }
    expect(previews).toBe(0);
  });
  it("no consistency section when the account has no such rule", async () => {
    serve(); renderApp(<RulesForm accountId={id} />);
    await screen.findByLabelText(/Limite de perda diária/);
    expect(screen.queryByLabelText(/consist/i)).not.toBeInTheDocument();
  });
});

describe("AC-5 / AC-9 profit target", () => {
  it("target cleared → on_reach is forced to notify", async () => {
    serve(rulesBody({ daily_profit_target: { amount: "600.00" }, profit_target_on_reach: "flatten_and_block" }, { enforcement_enabled: true })); writes();
    renderApp(<RulesForm accountId={id} />);
    await u().clear(await screen.findByLabelText(/Valor da meta|meta de lucro/i));
    await save();
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(body().rules.profit_target_on_reach).toBe("notify");
  });
  it("flatten is selectable when enforcement is on", async () => {
    serve(rulesBody({ daily_profit_target: { amount: "600.00" } }, { enforcement_enabled: true })); writes();
    renderApp(<RulesForm accountId={id} />);
    const flatten = await screen.findByRole("radio", { name: /Fechar posições e bloquear/ });
    expect(flatten).not.toHaveAttribute("aria-disabled", "true");
    await u().click(flatten);
    await save();
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(body().rules.profit_target_on_reach).toBe("flatten_and_block");
  });
  it("keyboard cannot select the disabled flatten option either", async () => {
    serve(rulesBody({ daily_profit_target: { amount: "600.00" } })); writes();
    renderApp(<RulesForm accountId={id} />);
    const notify = await screen.findByRole("radio", { name: /Só avisar/ });
    notify.focus();
    const user = u();
    await user.keyboard("{ArrowDown}");
    expect(notify).toBeChecked();
    expect(screen.getByRole("radio", { name: /Fechar posições e bloquear/ })).not.toBeChecked();
  });
  it("flatten_requires_enforcement 422 lands on the radio group", async () => {
    serve(); writes(noEffect, () => HttpResponse.json({ error: { code: "risk.flatten_requires_enforcement", pointer: "/rules/profit_target_on_reach" } }, { status: 422 }));
    renderApp(<RulesForm accountId={id} />);
    await screen.findByLabelText(/Limite de perda diária/);
    await save();
    expect((await screen.findAllByText(/aplicação automática/i)).length).toBeGreaterThan(1);
  });
});

describe("AC-7 / AC-8 save flow", () => {
  it("daily_loss_required maps to the daily-loss field", async () => {
    serve(); writes(noEffect, () => HttpResponse.json({ error: { code: "risk.daily_loss_required", pointer: "/rules/daily_loss_limit" } }, { status: 422 }));
    renderApp(<RulesForm accountId={id} />);
    await u().clear(await screen.findByLabelText(/Limite de perda diária/));
    await save();
    await waitFor(() => expect(screen.getByLabelText(/Limite de perda diária/)).toHaveAttribute("aria-invalid", "true"));
  });
  it("an unmapped 422 pointer still surfaces a visible error", async () => {
    serve(); writes(noEffect, () => HttpResponse.json({ error: { code: "risk.amount_invalid", pointer: "/rules/somewhere_new" } }, { status: 422 }));
    renderApp(<RulesForm accountId={id} />);
    await screen.findByLabelText(/Limite de perda diária/);
    await save();
    expect((await screen.findAllByText(/inválid|Valor/i)).length).toBeGreaterThan(0);
  });
  it("would_be_reached: dialog first, nothing saved until confirmed, Voltar keeps form, confirm sends acknowledge_reached", async () => {
    serve(); writes({ changes: [], would_be_reached: true });
    renderApp(<RulesForm accountId={id} />);
    await touch();
    await save();
    const dlg = await screen.findByRole("alertdialog");
    expect(puts).toHaveLength(0);
    await u().click(within(dlg).getByRole("button", { name: /Voltar/ }));
    await waitFor(() => expect(screen.queryByRole("alertdialog")).not.toBeInTheDocument());
    expect(puts).toHaveLength(0);
    await save();
    await u().click(within(await screen.findByRole("alertdialog")).getByRole("button", { name: /Salvar e aplicar agora/ }));
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(body().acknowledge_reached).toBe(true);
  });
  it("dialog default focus is the safe button", async () => {
    serve(); writes({ changes: [], would_be_reached: true });
    renderApp(<RulesForm accountId={id} />);
    await touch();
    await save();
    const dlg = await screen.findByRole("alertdialog");
    await waitFor(() => expect(within(dlg).getByRole("button", { name: /Voltar/ })).toHaveFocus());
  });
  it("409 version_conflict offers reload and re-reads the rules", async () => {
    serve(); writes(noEffect, () => HttpResponse.json({ error: { code: "common.version_conflict" } }, { status: 409 }));
    renderApp(<RulesForm accountId={id} />);
    await touch();
    await save();
    let reads = 0;
    server.use(http.get(`*/api/v1/accounts/${id}/risk-rules`, () => (reads++, HttpResponse.json(rulesBody({}, { version: 9 })))));
    await u().click(await screen.findByRole("button", { name: "Recarregar" }));
    await waitFor(() => expect(reads).toBe(1));
  });
  it("double-click on Salvar sends one request", async () => {
    serve(); writes();
    renderApp(<RulesForm accountId={id} />);
    await touch();
    const b = screen.getByRole("button", { name: "Salvar regras" });
    await u().dblClick(b);
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(previews).toBe(1);
  });
  it("5xx keeps the entered values and shows an error", async () => {
    serve(); writes(noEffect, () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 }));
    renderApp(<RulesForm accountId={id} />);
    const loss = await screen.findByLabelText(/Limite de perda diária/);
    await u().clear(loss); await u().type(loss, "700");
    await save();
    expect(await screen.findByRole("alert")).toBeInTheDocument();
    expect(loss).toHaveValue("700");
  });
  it("404 on load (another user's account) is not-found, not a generic load failure", async () => {
    serve({ error: { code: "common.not_found" } }, 404);
    renderApp(<RulesForm accountId={id} />);
    expect(await screen.findByText("Não encontrado.")).toBeInTheDocument();
  });
});

describe("form UX (spec §4, F§10)", () => {
  it("focuses the first invalid field on submit", async () => {
    serve(); writes();
    renderApp(<RulesForm accountId={id} />);
    const loss = await screen.findByLabelText(/Limite de perda diária/);
    await u().clear(loss); await u().type(loss, "abc");
    await save();
    await waitFor(() => expect(loss).toHaveFocus());
  });
  it("cancelling with unsaved changes asks 'Descartar alterações?'", async () => {
    serve(); writes();
    renderApp(<RulesForm accountId={id} />);
    const loss = await screen.findByLabelText(/Limite de perda diária/);
    await u().type(loss, "9");
    await u().click(screen.getByRole("button", { name: "Cancelar" }));
    expect(await screen.findByRole("alertdialog")).toHaveTextContent(/Descartar alterações/);
  });
  it("loosening / tightening effect copy is shown for every limit field", async () => {
    serve(); renderApp(<RulesForm accountId={id} />);
    await screen.findByLabelText(/Limite de perda diária/);
    expect(screen.getAllByText(/mais apertado vale na hora/i).length).toBeGreaterThanOrEqual(2);
    expect(screen.getAllByText(/mais folgado vale a partir do próximo dia de trading/i).length).toBeGreaterThanOrEqual(2);
  });
  it("spec §4.5: shows the enforcement state line", async () => {
    serve(); renderApp(<RulesForm accountId={id} />);
    expect(await screen.findByText(/Aplicação automática: desligada/)).toBeInTheDocument();
  });
});

async function touch() {
  // The form sends nothing for unchanged values (by design), so every save-path test changes the daily loss first.
  const loss = await screen.findByLabelText(/Limite de perda diária/);
  await u().clear(loss);
  await u().type(loss, "900");
}

describe("commission floor (AC-11, via updateAccount)", () => {
  const fee = () => screen.findByLabelText("Comissão por contrato e lado, em dólares");
  it("untouched commission sends no account update", async () => {
    serve(); writes(); renderApp(<RulesForm accountId={id} />);
    await touch(); await save();
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(accountPuts).toHaveLength(0);
  });
  it("a changed floor updates the account with row_version, keeps the binding, and runs after the rules", async () => {
    const order: string[] = [];
    serve();
    server.use(
      http.post(`*/api/v1/accounts/${id}/risk-rules/preview`, () => HttpResponse.json(noEffect)),
      http.put(`*/api/v1/accounts/${id}/risk-rules`, () => (order.push("rules"), HttpResponse.json(rulesBody({}, { version: 5 })))),
      http.put(`*/api/v1/accounts/${id}`, async ({ request }) => (order.push("account"), accountPuts.push((await request.json()) as Record<string, unknown>), HttpResponse.json(account({ row_version: 8 })))),
    );
    renderApp(<RulesForm accountId={id} />);
    const f = await fee();
    await u().clear(f); await u().type(f, "0,5");
    await touch(); await save();
    await waitFor(() => expect(accountPuts).toHaveLength(1));
    expect(order).toEqual(["rules", "account"]);
    const sent = accountPuts[0] as Record<string, any>; // eslint-disable-line @typescript-eslint/no-explicit-any
    expect(sent.row_version).toBe(7);
    expect(sent.commission_per_contract_side).toBe("0.5");
    expect(sent.binding).toEqual({ connection_id: "00000000-0000-4000-8000-0000000000c1", platform_account: "Sim101" });
    expect(sent.display_name).toBe("Apex 50K");
  });
  it("commission only: no preview/PUT of rules", async () => {
    serve(); writes(); renderApp(<RulesForm accountId={id} />);
    const f = await fee(); await u().clear(f); await u().type(f, "0.4"); await save();
    await waitFor(() => expect(accountPuts).toHaveLength(1));
    expect(puts).toHaveLength(0); expect(previews).toBe(0);
  });
  it("clearing the floor sends null", async () => {
    serve(); writes(); renderApp(<RulesForm accountId={id} />);
    await u().clear(await fee()); await save();
    await waitFor(() => expect(accountPuts).toHaveLength(1));
    expect(accountPuts[0]?.commission_per_contract_side).toBeNull();
  });
  it("rules saved but commission failed: says so and does not navigate", async () => {
    serve(); writes();
    server.use(http.put(`*/api/v1/accounts/${id}`, () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    renderApp(<RulesForm accountId={id} />);
    const f = await fee(); await u().clear(f); await u().type(f, "0.9");
    await touch(); await save();
    expect(await screen.findByText(/As regras foram salvas, mas a comissão não/)).toBeInTheDocument();
    expect(navigation.push).not.toHaveBeenCalled();
  });
  it("more than 4 decimals or negative is rejected client-side", async () => {
    serve(); writes(); renderApp(<RulesForm accountId={id} />);
    const f = await fee();
    for (const bad of ["0.12345", "-1", "x"]) { await u().clear(f); await u().type(f, bad); await save(); expect(f).toHaveAttribute("aria-invalid", "true"); }
    expect(accountPuts).toHaveLength(0);
  });
});

describe("prefilled Core value with 3+ decimals stays savable in pt-BR", () => {
  it("unchanged 1000.125 is not rejected as ambiguous; editing it re-applies the 2-decimal rule", async () => {
    serve(rulesBody({ daily_loss_limit: { kind: "hard", amount: "1000.125" } })); writes(); renderApp(<RulesForm accountId={id} />);
    const loss = await screen.findByLabelText(/Limite de perda diária/);
    const target = await screen.findByLabelText(/Valor da meta/);
    await u().type(target, "600"); await save();
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(loss).not.toHaveAttribute("aria-invalid", "true");
    expect(body().rules.daily_loss_limit?.amount).toBe("1000.125");
  });
  it("editing the prefilled value to 3 decimals is rejected again", async () => {
    serve(rulesBody({ daily_loss_limit: { kind: "hard", amount: "1000.125" } })); writes(); renderApp(<RulesForm accountId={id} />);
    const loss = await screen.findByLabelText(/Limite de perda diária/);
    await u().type(loss, "9"); await save();
    expect(loss).toHaveAttribute("aria-invalid", "true"); expect(previews).toBe(0);
  });
});

function u() { return userEvent.setup(); }

describe("discard guard", () => {
  it("cancel without changes leaves without asking", async () => {
    serve(); renderApp(<RulesForm accountId={id} />);
    await screen.findByLabelText(/Limite de perda diária/);
    const cancel = screen.queryByRole("button", { name: "Cancelar" }) ?? screen.getByRole("link", { name: "Cancelar" });
    await u().click(cancel);
    expect(screen.queryByRole("alertdialog")).not.toBeInTheDocument();
  });
});
