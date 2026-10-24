import { act, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
import { FakeEventSource } from "@/test/setup";
import { RiskTab } from "./risk-tab";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const accountId = "00000000-0000-4000-8000-0000000000a1";

const risk = (overrides: Record<string, unknown> = {}) => ({
  account_id: accountId,
  as_of: "2026-10-23T14:31:05Z",
  stale: false,
  state: "normal",
  trading_day: { start: "2026-10-22T21:00:00Z", end: "2026-10-23T21:00:00Z", next_reset: "2026-10-23T21:00:00Z", source: "firm", firm_reset: { local_time: "17:00", time_zone: "America/New_York" } },
  meters: [
    { kind: "daily_loss", state: "warning", limit: "1000.00", used: "820.00", percent: 82 },
    { kind: "drawdown", state: "normal", limit: "2500.00", used: "300.00", percent: 12, drawdown_type: "trailing_eod", floor: "47700.00" },
  ],
  unset_rules: ["daily_profit_target"],
  untracked_rules: [],
  enforcement_enabled: false,
  limits_pending: false,
  pending_changes: [],
  pnl_basis: "net",
  commission_source: "platform",
  ...overrides,
});

const evaluation = (overrides: Record<string, unknown> = {}) => ({ start: "2026-10-01", profit_target: "6000.00", earned_before: "0.00", achieved: "4800.00", remaining: "1200.00", percent: 80, state: "in_progress", as_of: "2026-10-23T14:31:05Z", ...overrides });

function serve(body: object) {
  server.use(http.get(`*/api/v1/accounts/${accountId}/risk`, () => HttpResponse.json(body)));
}
async function stream() {
  await waitFor(() => expect(FakeEventSource.latest()).toBeDefined());
  const source = FakeEventSource.latest();
  if (!source) throw new Error("no event stream");
  return source;
}

beforeEach(() => server.use(http.get("*/api/v1/me", () => HttpResponse.json(user))));

describe("RiskTab", () => {
  it("shows the meters with exact Core figures, the net basis and the warning banner", async () => {
    serve(risk());
    renderApp(<RiskTab accountId={accountId} />);
    const loss = await screen.findByRole("meter", { name: "Perda diária" });
    expect(loss).toHaveAttribute("aria-valuenow", "82");
    expect(loss).toHaveAttribute("aria-valuetext", "Perda diária: US$\u00a0820,00 de US$\u00a01.000,00, 82 %, alerta");
    expect(screen.getByRole("meter", { name: "Drawdown (Trailing fim do dia)" })).toBeInTheDocument();
    expect(screen.getByText("Piso atual: US$ 47.700,00")).toBeInTheDocument();
    expect(screen.getByText("Valores líquidos, depois de comissões.")).toBeInTheDocument();
    expect(screen.getByText(/Meta de lucro diária: sem limite definido/)).toBeInTheDocument();
    expect(screen.getAllByText("Alerta (80 %)").length).toBeGreaterThan(0);
    expect(screen.getByText(/Reset da firma: 17:00 \(America\/New_York\)/)).toBeInTheDocument();
  });

  it("never shows stale data as current", async () => {
    serve(risk({ stale: true, stale_reason: "connection_down" }));
    renderApp(<RiskTab accountId={accountId} />);
    expect(await screen.findByText("Sem dados recentes")).toBeInTheDocument();
    expect(screen.getByText("NinjaTrader desconectado: os valores abaixo podem estar desatualizados.")).toBeInTheDocument();
  });

  it("states that a block by the profit target is the target, with no early unlock", async () => {
    serve(risk({ meters: [{ kind: "daily_profit_target", state: "target_reached", limit: "600.00", used: "640.00", percent: 106 }], unset_rules: [], blocked: { until: "2026-10-23T21:00:00Z", cause: "daily_profit_target" } }));
    renderApp(<RiskTab accountId={accountId} />);
    expect(await screen.findByText(/motivo: meta de lucro diária atingida/)).toBeInTheDocument();
    expect(screen.getAllByText(/Não há desbloqueio antecipado/).length).toBeGreaterThan(0);
  });

  it("shows the empty state when no rule exists", async () => {
    serve(risk({ meters: [], unset_rules: ["daily_loss", "drawdown", "daily_profit_target", "consistency"] }));
    renderApp(<RiskTab accountId={accountId} />);
    expect(await screen.findByText("Esta conta ainda não tem regras de risco.")).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Definir regras" })).toHaveAttribute("href", `/pt-BR/accounts/${accountId}/risk/edit`);
  });

  it("warns that commission is missing without blocking", async () => {
    serve(risk({ commission_source: "none" }));
    renderApp(<RiskTab accountId={accountId} />);
    expect(await screen.findByText(/Comissão não informada: os limites podem estar folgados demais/)).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Informar comissão" })).toHaveAttribute("href", `/pt-BR/accounts/${accountId}/risk/edit#commission`);
  });

  it("refetches and announces only a change of state from the stream", async () => {
    let calls = 0;
    server.use(http.get(`*/api/v1/accounts/${accountId}/risk`, () => HttpResponse.json(++calls > 1 ? risk({ meters: [{ kind: "daily_loss", state: "reached", limit: "1000.00", used: "1000.00", percent: 100 }] }) : risk())));
    renderApp(<RiskTab accountId={accountId} />);
    await screen.findByRole("meter", { name: "Perda diária" });
    const events = await stream();
    act(() => events.emit("risk.state_changed", { type: "risk.state_changed", at: "2026-10-23T14:32:00Z", account_id: "00000000-0000-4000-8000-0000000000ff", state: "reached", stale: false, reason: "state" }));
    expect(screen.queryByText("Limite atingido.")).not.toBeInTheDocument();
    act(() => events.emit("risk.state_changed", { type: "risk.state_changed", at: "2026-10-23T14:32:00Z", account_id: accountId, state: "reached", stale: false, reason: "state" }));
    await waitFor(() => expect(screen.getByRole("meter", { name: "Perda diária" })).toHaveAttribute("aria-valuenow", "100"));
    expect(screen.getAllByRole("status").some((node) => node.textContent === "Limite atingido.")).toBe(true);
  });

  it("shows another user's account as not found", async () => {
    server.use(http.get(`*/api/v1/accounts/${accountId}/risk`, () => HttpResponse.json({ error: { code: "common.not_found" } }, { status: 404 })));
    renderApp(<RiskTab accountId={accountId} />);
    expect(await screen.findByText("Não encontrado.")).toBeInTheDocument();
  });
});

describe("evaluation card (S13b)", () => {
  it("shows target, earned and what is left as exact strings", async () => {
    serve(risk({ evaluation: evaluation() }));
    renderApp(<RiskTab accountId={accountId} />);
    const card = (await screen.findByRole("heading", { name: "Meta da avaliação" })).closest("div[aria-labelledby], section, [data-slot]") ?? document.body;
    expect(within(document.body).getByText("US$ 6.000,00")).toBeInTheDocument();
    expect(within(document.body).getByText("US$ 4.800,00")).toBeInTheDocument();
    expect(within(document.body).getByText("US$ 1.200,00")).toBeInTheDocument();
    expect(screen.getByRole("progressbar", { name: "Meta da avaliação" })).toHaveAttribute("aria-valuenow", "80");
    expect(card).toBeTruthy();
  });

  it("has no card without an evaluation", async () => {
    serve(risk());
    renderApp(<RiskTab accountId={accountId} />);
    await screen.findByRole("meter", { name: "Perda diária" });
    expect(screen.queryByRole("heading", { name: "Meta da avaliação" })).not.toBeInTheDocument();
  });

  it("on target reached says nothing is closed or blocked, with no negative amount", async () => {
    serve(risk({ evaluation: evaluation({ state: "target_reached", achieved: "6200.00", remaining: "0.00", percent: 103, reached_at: "2026-10-22T15:00:00Z" }) }));
    renderApp(<RiskTab accountId={accountId} />);
    expect(await screen.findByText(/Nada é fechado nem bloqueado automaticamente/)).toBeInTheDocument();
    expect(screen.getByText("Meta atingida em")).toBeInTheDocument();
  });

  it("restarts the count after a confirmation with the account row_version", async () => {
    serve(risk({ evaluation: evaluation() }));
    server.use(http.get(`*/api/v1/accounts/${accountId}`, () => HttpResponse.json({ id: accountId, row_version: 7 })));
    const bodies: unknown[] = [];
    server.use(http.post(`*/api/v1/accounts/${accountId}/evaluation/restart`, async ({ request }) => {
      bodies.push(await request.json());
      return HttpResponse.json({ id: accountId, row_version: 8 });
    }));
    renderApp(<RiskTab accountId={accountId} />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ajustar contagem" }));
    const dialog = await screen.findByRole("dialog");
    await person.clear(within(dialog).getByLabelText("Início da contagem"));
    await person.type(within(dialog).getByLabelText("Início da contagem"), "2026-10-15");
    await person.type(within(dialog).getByLabelText("Já ganho antes do início (líquido)"), "250,5");
    await waitFor(() => expect(within(dialog).getByRole("button", { name: "Reiniciar contagem" })).toBeEnabled());
    await person.click(within(dialog).getByRole("button", { name: "Reiniciar contagem" }));
    await person.click(await screen.findByRole("button", { name: "Reiniciar contagem", hidden: false }).then(() => within(screen.getByRole("alertdialog")).getByRole("button", { name: "Reiniciar contagem" })));
    await waitFor(() => expect(bodies).toEqual([{ evaluation_start: "2026-10-15", evaluation_earned_before: "250.5", row_version: 7 }]));
  });

  it("asks to reload on a version conflict and keeps the values", async () => {
    serve(risk({ evaluation: evaluation() }));
    server.use(http.get(`*/api/v1/accounts/${accountId}`, () => HttpResponse.json({ id: accountId, row_version: 7 })));
    server.use(http.post(`*/api/v1/accounts/${accountId}/evaluation/restart`, () => HttpResponse.json({ error: { code: "common.version_conflict" } }, { status: 409 })));
    renderApp(<RiskTab accountId={accountId} />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ajustar contagem" }));
    const dialog = await screen.findByRole("dialog");
    await person.clear(within(dialog).getByLabelText("Início da contagem"));
    await person.type(within(dialog).getByLabelText("Início da contagem"), "2026-10-15");
    await waitFor(() => expect(within(dialog).getByRole("button", { name: "Reiniciar contagem" })).toBeEnabled());
    await person.click(within(dialog).getByRole("button", { name: "Reiniciar contagem" }));
    await person.click(within(screen.getByRole("alertdialog")).getByRole("button", { name: "Reiniciar contagem" }));
    expect(await screen.findByText("A conta mudou. Recarregue.")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Recarregar" })).toBeInTheDocument();
    expect(within(screen.getByRole("dialog")).getByLabelText("Início da contagem")).toHaveValue("2026-10-15");
  });

  it("says the commission was estimated for configured or mixed sources, and not provided only for none (M3)", async () => {
    serve(risk({ evaluation: evaluation(), commission_source: "configured" }));
    const view = renderApp(<RiskTab accountId={accountId} />);
    expect(await screen.findByText("Comissão estimada pelo mínimo configurado: o acumulado pode estar acima do real.")).toBeInTheDocument();
    expect(screen.queryByText("Comissão não informada: o acumulado pode estar acima do real.")).not.toBeInTheDocument();
    view.unmount();

    serve(risk({ evaluation: evaluation(), commission_source: "none" }));
    renderApp(<RiskTab accountId={accountId} />);
    expect(await screen.findByText("Comissão não informada: o acumulado pode estar acima do real.")).toBeInTheDocument();
  });

  it("closes the restart dialog with an Alert when the account has no evaluation target (L2)", async () => {
    serve(risk({ evaluation: evaluation() }));
    server.use(http.get(`*/api/v1/accounts/${accountId}`, () => HttpResponse.json({ id: accountId, row_version: 7 })));
    server.use(http.post(`*/api/v1/accounts/${accountId}/evaluation/restart`, () => HttpResponse.json({ error: { code: "accounts.evaluation_not_applicable" } }, { status: 422 })));
    renderApp(<RiskTab accountId={accountId} />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ajustar contagem" }));
    const dialog = await screen.findByRole("dialog");
    await person.clear(within(dialog).getByLabelText("Início da contagem"));
    await person.type(within(dialog).getByLabelText("Início da contagem"), "2026-10-15");
    await waitFor(() => expect(within(dialog).getByRole("button", { name: "Reiniciar contagem" })).toBeEnabled());
    await person.click(within(dialog).getByRole("button", { name: "Reiniciar contagem" }));
    await person.click(within(screen.getByRole("alertdialog")).getByRole("button", { name: "Reiniciar contagem" }));
    expect(await screen.findByText("Esta conta não tem meta de avaliação.")).toBeInTheDocument();
    await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());
  });

  it("renders the edit entry point with its label, not a translation error (F1)", async () => {
    serve(risk());
    renderApp(<RiskTab accountId={accountId} />);
    expect(await screen.findByRole("link", { name: "Editar regras" })).toHaveAttribute("href", `/pt-BR/accounts/${accountId}/risk/edit`);
  });

  it("ignores a state it does not know instead of announcing a raw key (n1)", async () => {
    serve(risk());
    renderApp(<RiskTab accountId={accountId} />);
    await screen.findByRole("meter", { name: "Perda diária" });
    const events = await stream();
    act(() => events.emit("risk.state_changed", { type: "risk.state_changed", at: "2026-10-23T14:32:00Z", account_id: accountId, state: "mystery", stale: false, reason: "state" }));
    expect(screen.queryByText(/announce/)).not.toBeInTheDocument();
  });

  it("sends 0 when the user clears a non-zero amount earned before (L2)", async () => {
    serve(risk({ evaluation: evaluation({ earned_before: "250.0000" }) }));
    server.use(http.get(`*/api/v1/accounts/${accountId}`, () => HttpResponse.json({ id: accountId, row_version: 7 })));
    const bodies: unknown[] = [];
    server.use(http.post(`*/api/v1/accounts/${accountId}/evaluation/restart`, async ({ request }) => {
      bodies.push(await request.json());
      return HttpResponse.json({ id: accountId, row_version: 8 });
    }));
    renderApp(<RiskTab accountId={accountId} />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ajustar contagem" }));
    const dialog = await screen.findByRole("dialog");
    await person.clear(within(dialog).getByLabelText("Já ganho antes do início (líquido)"));
    await waitFor(() => expect(within(dialog).getByRole("button", { name: "Reiniciar contagem" })).toBeEnabled());
    await person.click(within(dialog).getByRole("button", { name: "Reiniciar contagem" }));
    await person.click(within(screen.getByRole("alertdialog")).getByRole("button", { name: "Reiniciar contagem" }));
    await waitFor(() => expect(bodies).toEqual([{ evaluation_start: "2026-10-01", evaluation_earned_before: "0", row_version: 7 }]));
  });

  it("keeps the earned-before amount the Core sent when only the start changes, even if it would be ambiguous typed (Grosa low)", async () => {
    serve(risk({ evaluation: evaluation({ earned_before: "250.125" }) }));
    server.use(http.get(`*/api/v1/accounts/${accountId}`, () => HttpResponse.json({ id: accountId, row_version: 7 })));
    const bodies: unknown[] = [];
    server.use(http.post(`*/api/v1/accounts/${accountId}/evaluation/restart`, async ({ request }) => {
      bodies.push(await request.json());
      return HttpResponse.json({ id: accountId, row_version: 8 });
    }));
    renderApp(<RiskTab accountId={accountId} />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ajustar contagem" }));
    const dialog = await screen.findByRole("dialog");
    await person.clear(within(dialog).getByLabelText("Início da contagem"));
    await person.type(within(dialog).getByLabelText("Início da contagem"), "2026-10-15");
    await waitFor(() => expect(within(dialog).getByRole("button", { name: "Reiniciar contagem" })).toBeEnabled());
    await person.click(within(dialog).getByRole("button", { name: "Reiniciar contagem" }));
    await person.click(within(screen.getByRole("alertdialog")).getByRole("button", { name: "Reiniciar contagem" }));
    await waitFor(() => expect(bodies).toEqual([{ evaluation_start: "2026-10-15", evaluation_earned_before: "250.125", row_version: 7 }]));
  });
});
