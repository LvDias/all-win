// QA (Retina) — V1-S13 AC-2/3/4/11 and V1-S13b AC-1/2/3/5 on the Risk tab, black box through the public index.
import { act, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { beforeEach, describe, expect, it } from "vitest";
import { renderApp } from "@/test/render";
// render first: it installs the next/navigation mock before the feature loads.
import { RiskTab } from "@/features/risk";
import { server } from "@/test/server";
import { FakeEventSource } from "@/test/setup";

const me = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const id = "00000000-0000-4000-8000-0000000000a1";
const base = (o: Record<string, unknown> = {}) => ({
  account_id: id, as_of: "2026-10-23T14:31:05Z", stale: false, state: "normal",
  trading_day: { start: "2026-10-22T21:00:00Z", end: "2026-10-23T21:00:00Z", next_reset: "2026-10-23T21:00:00Z", source: "cme_session_close" },
  meters: [{ kind: "daily_loss", state: "normal", limit: "1000.00", used: "100.00", percent: 10 }],
  unset_rules: [], untracked_rules: [], enforcement_enabled: false, limits_pending: false, pending_changes: [], pnl_basis: "net", commission_source: "platform", ...o,
});
const evalOf = (o: Record<string, unknown> = {}) => ({ start: "2026-10-01", profit_target: "6000.00", earned_before: "0.00", achieved: "4800.00", remaining: "1200.00", percent: 80, state: "in_progress", as_of: "2026-10-23T14:31:05Z", ...o });
const serve = (body: object, status = 200) => server.use(http.get(`*/api/v1/accounts/${id}/risk`, () => HttpResponse.json(body, { status })));
const emit = async (data: Record<string, unknown>) => {
  await waitFor(() => expect(FakeEventSource.latest()).toBeDefined());
  act(() => FakeEventSource.latest()?.emit("risk.state_changed", { type: "risk.state_changed", at: "2026-10-23T14:32:00Z", account_id: id, stale: false, ...data }));
};

beforeEach(() => server.use(http.get("*/api/v1/me", () => HttpResponse.json(me))));

describe("S13 AC-2 / money exactness", () => {
  it("renders 4-decimal and large Core strings without float drift", async () => {
    serve(base({ meters: [{ kind: "daily_loss", state: "normal", limit: "99999999999999.9999", used: "0.1000", percent: 0 }] }));
    renderApp(<RiskTab accountId={id} />);
    const m = await screen.findByRole("meter", { name: "Perda diária" });
    expect(m.getAttribute("aria-valuetext")).toContain("99.999.999.999.999,9999");
  });
  it("shows consistency figures truncated, never rounded up toward the limit (F§6.2)", async () => {
    serve(base({ meters: [{ kind: "consistency", state: "normal", limit: "30", used: "29.9", percent: 99 }] }));
    renderApp(<RiskTab accountId={id} />);
    const m = await screen.findByRole("meter", { name: "Regra de consistência" });
    expect(m.getAttribute("aria-valuetext")).not.toMatch(/\b30 %.*30 %/);
    expect(m.getAttribute("aria-valuetext")).toMatch(/29[,.]9/);
  });
  it("never shows an in-range percent as 80 while the Core says normal (meter echoes Core percent)", async () => {
    serve(base({ meters: [{ kind: "daily_loss", state: "normal", limit: "1000.00", used: "799.99", percent: 79 }] }));
    renderApp(<RiskTab accountId={id} />);
    expect((await screen.findByRole("meter", { name: "Perda diária" })).getAttribute("aria-valuetext")).toContain("79 %");
  });
});

describe("S13 AC-3 states and live announcements", () => {
  it.each([["warning", "Alerta (80 %)"], ["reached", "Limite atingido"]])("%s shows a banner with text, not color alone", async (state, text) => {
    serve(base({ meters: [{ kind: "daily_loss", state, limit: "1000.00", used: "850.00", percent: 85 }] }));
    renderApp(<RiskTab accountId={id} />);
    await screen.findByRole("meter", { name: "Perda diária" });
    expect(screen.getAllByText(text).length).toBeGreaterThan(0);
  });
  it("does not announce stale/rules_changed/evaluation_progress events, only state changes", async () => {
    serve(base());
    renderApp(<RiskTab accountId={id} />);
    await screen.findByRole("meter", { name: "Perda diária" });
    for (const reason of ["stale", "rules_changed", "evaluation_progress", "trading_day_reset", "blocked", "unblocked"]) await emit({ state: "warning", reason });
    expect(screen.getAllByRole("status").every((n) => n.textContent === "")).toBe(true);
  });
  it("announces once per state change via a polite region", async () => {
    serve(base());
    renderApp(<RiskTab accountId={id} />);
    await screen.findByRole("meter", { name: "Perda diária" });
    await emit({ state: "warning", reason: "state" });
    const live = screen.getAllByRole("status").filter((n) => n.textContent);
    expect(live).toHaveLength(1);
    expect(live[0]?.textContent).toBe("Alerta: 80 % de um limite.");
  });
  it("shows the profit-target block as success tone with the reason in text and no early unlock", async () => {
    serve(base({ meters: [{ kind: "daily_profit_target", state: "target_reached", limit: "600.00", used: "640.00", percent: 106 }], blocked: { until: "2026-10-23T21:00:00Z", cause: "daily_profit_target" } }));
    renderApp(<RiskTab accountId={id} />);
    const alert = await screen.findByRole("alert");
    expect(alert).toHaveTextContent(/motivo: meta de lucro diária atingida/);
    expect(alert).toHaveTextContent(/Não há desbloqueio antecipado/);
    expect(screen.getByRole("meter", { name: "Meta de lucro diária" })).toBeInTheDocument();
  });
  it("loss-limit block is not success toned", async () => {
    serve(base({ blocked: { until: "2026-10-23T21:00:00Z", cause: "daily_loss" } }));
    renderApp(<RiskTab accountId={id} />);
    expect(await screen.findByRole("alert")).toHaveTextContent(/perda diária/i);
  });
  it("dims meters and flags stale data (no_recent_data too)", async () => {
    serve(base({ stale: true, stale_reason: "no_recent_data" }));
    renderApp(<RiskTab accountId={id} />);
    expect(await screen.findByText("Sem dados recentes")).toBeInTheDocument();
  });
  it("404 (another user's account) is not-found without retry or the account's data", async () => {
    serve({ error: { code: "common.not_found" } }, 404);
    renderApp(<RiskTab accountId={id} />);
    expect(await screen.findByText("Não encontrado.")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /tentar/i })).not.toBeInTheDocument();
  });
  it("5xx shows an error with retry", async () => {
    serve({ error: { code: "common.internal" } }, 500);
    renderApp(<RiskTab accountId={id} />);
    expect(await screen.findByRole("button", { name: /tentar/i })).toBeInTheDocument();
  });
});

describe("S13 AC-11 commission notices", () => {
  it.each(["platform", "manual"])("%s: no commission warning", async (commission_source) => {
    serve(base({ commission_source }));
    renderApp(<RiskTab accountId={id} />);
    await screen.findByRole("meter", { name: "Perda diária" });
    expect(screen.queryByText(/Comissão não informada/)).not.toBeInTheDocument();
  });
  it("pending lowered commission is stated with the value and time", async () => {
    serve(base({ pending_commission_change: { effective_from: "2026-10-24T21:00:00Z", new_commission_per_contract_side: "1.2500", change: "commission_floor_lowered" } }));
    renderApp(<RiskTab accountId={id} />);
    expect(await screen.findByText(/US\$\s1,25/)).toBeInTheDocument();
  });
  it("pending removal without a value does not print a value", async () => {
    serve(base({ pending_commission_change: { effective_from: "2026-10-24T21:00:00Z", new_commission_per_contract_side: "0", change: "commission_floor_removed" } }));
    renderApp(<RiskTab accountId={id} />);
    expect(await screen.findByText(/removida/i)).toBeInTheDocument();
  });
  it("manual trades line is present", async () => {
    serve(base());
    renderApp(<RiskTab accountId={id} />);
    expect(await screen.findByText("Operações lançadas à mão não entram no risco desta conta.")).toBeInTheDocument();
  });
});

describe("S13b evaluation card", () => {
  it("AC-3 spec: commission estimated from the configured minimum says so (not 'not provided')", async () => {
    serve(base({ evaluation: evalOf(), commission_source: "configured" }));
    renderApp(<RiskTab accountId={id} />);
    await screen.findByRole("heading", { name: "Meta da avaliação" });
    expect(screen.getByText(/Comissão estimada pelo mínimo configurado/)).toBeInTheDocument();
  });
  it("platform/manual source: no evaluation commission note", async () => {
    serve(base({ evaluation: evalOf(), commission_source: "platform" }));
    renderApp(<RiskTab accountId={id} />);
    await screen.findByRole("heading", { name: "Meta da avaliação" });
    expect(screen.queryByText(/Comissão/)).not.toBeInTheDocument();
  });
  it("reached: success notice, no remaining amount, no negative number", async () => {
    serve(base({ evaluation: evalOf({ state: "target_reached", achieved: "6200.00", remaining: "0.00", percent: 103, reached_at: "2026-10-22T15:00:00Z" }) }));
    renderApp(<RiskTab accountId={id} />);
    await screen.findByRole("heading", { name: "Meta da avaliação" });
    expect(screen.queryByText("Falta")).not.toBeInTheDocument();
    expect(screen.getByRole("progressbar", { name: "Meta da avaliação" })).toHaveAttribute("aria-valuenow", "100");
    expect(document.body.textContent).not.toMatch(/-US\$|US\$\s-/);
    expect(screen.getByText(/Nada é fechado nem bloqueado automaticamente/)).toBeInTheDocument();
    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
  });
  it("count line mentions earned-before only when non-zero (textual compare, '0.0000' too)", async () => {
    serve(base({ evaluation: evalOf({ earned_before: "0.0000" }) }));
    renderApp(<RiskTab accountId={id} />);
    await screen.findByRole("heading", { name: "Meta da avaliação" });
    expect(document.body.textContent).not.toMatch(/já ganhos antes/);
  });
  it("count line includes earned-before and the manual-trades exclusion (AC-2/AC-4)", async () => {
    serve(base({ evaluation: evalOf({ earned_before: "250.5000" }) }));
    renderApp(<RiskTab accountId={id} />);
    await screen.findByRole("heading", { name: "Meta da avaliação" });
    expect(document.body.textContent).toMatch(/250,50/);
    expect(document.body.textContent).toMatch(/lançadas à mão não contam/);
  });
  it("start date shows the calendar day regardless of the user's time zone", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json({ ...me, time_zone: "America/Sao_Paulo" })));
    serve(base({ evaluation: evalOf({ start: "2026-10-01" }) }));
    renderApp(<RiskTab accountId={id} />);
    await screen.findByRole("heading", { name: "Meta da avaliação" });
    expect(document.body.textContent).toMatch(/01\/10\/2026/);
  });
  it("restart: dialog is not submittable until changed and sends nothing on Voltar", async () => {
    serve(base({ evaluation: evalOf() }));
    server.use(http.get(`*/api/v1/accounts/${id}`, () => HttpResponse.json({ id, row_version: 3 })));
    let posted = 0;
    server.use(http.post(`*/api/v1/accounts/${id}/evaluation/restart`, () => (posted++, HttpResponse.json({ id, row_version: 4 }))));
    renderApp(<RiskTab accountId={id} />);
    const u = userEvent.setup();
    await u.click(await screen.findByRole("button", { name: "Ajustar contagem" }));
    const dlg = await screen.findByRole("dialog");
    expect(within(dlg).getByRole("button", { name: "Reiniciar contagem" })).toBeDisabled();
    await u.type(within(dlg).getByLabelText("Já ganho antes do início (líquido)"), "10");
    await u.click(within(dlg).getByRole("button", { name: "Reiniciar contagem" }));
    await u.click(within(await screen.findByRole("alertdialog")).getByRole("button", { name: /Voltar/ }));
    expect(posted).toBe(0);
  });
  it("restart: 422 evaluation_not_applicable closes the dialog; start pointer maps to the field", async () => {
    serve(base({ evaluation: evalOf() }));
    server.use(http.get(`*/api/v1/accounts/${id}`, () => HttpResponse.json({ id, row_version: 3 })));
    server.use(http.post(`*/api/v1/accounts/${id}/evaluation/restart`, () => HttpResponse.json({ error: { code: "accounts.evaluation_start_invalid", pointer: "/evaluation_start" } }, { status: 422 })));
    renderApp(<RiskTab accountId={id} />);
    const u = userEvent.setup();
    await u.click(await screen.findByRole("button", { name: "Ajustar contagem" }));
    const dlg = await screen.findByRole("dialog");
    await u.type(within(dlg).getByLabelText("Já ganho antes do início (líquido)"), "10");
    await u.click(within(dlg).getByRole("button", { name: "Reiniciar contagem" }));
    await u.click(within(await screen.findByRole("alertdialog")).getByRole("button", { name: "Reiniciar contagem" }));
    const field = await within(await screen.findByRole("dialog")).findByLabelText("Início da contagem");
    expect(field).toHaveAttribute("aria-invalid", "true");
  });
});
