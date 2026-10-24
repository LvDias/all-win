// QA (Retina) — V1-S21 AC-3/5/7/8/11 on manual entry and edit, black box through the public index.
import { fireEvent, screen, waitFor, within, configure } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { beforeEach, describe, expect, it } from "vitest";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
// render first: it installs the next/navigation mock before the feature loads.
import { ManualTradeEdit, ManualTradeFlow } from "@/features/trades";

// Findings must not depend on host load: wait up to 8 s for async UI (the default 1 s flaked on a loaded CI host).
configure({ asyncUtilTimeout: 8000 });

const me = (tz = "UTC") => ({ id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: tz, is_platform_admin: false });
const acc = "00000000-0000-4000-8000-0000000000a1";
const acc2 = "00000000-0000-4000-8000-0000000000a2";
const ins = "00000000-0000-4000-8000-0000000000b1";
const tid = "00000000-0000-4000-8000-0000000000e1";
const instrument = { id: ins, symbol: "MNQ", name: "Micro E-mini Nasdaq-100", exchange: "CME", currency: "USD", tick_size: "0.25", tick_value: "0.5", point_value: "2", price_decimals: 2 };
const preview = (o: Record<string, unknown> = {}) => ({ duration_seconds: 750, pnl: "21.5000", pnl_basis: "net", gross_pnl: "23.0000", commission: "1.5000", commission_source: "configured", ...o });
let previews: URL[] = [];
let posts: { body: Record<string, unknown>; key: string | null }[] = [];

beforeEach(() => {
  previews = []; posts = []; navigation.push.mockClear();
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(me())),
    http.get("*/api/v1/accounts", () => HttpResponse.json({ accounts: [{ id: acc, display_name: "Apex 50K" }, { id: acc2, display_name: "Live NQ" }] })),
    http.get("*/api/v1/instruments", () => HttpResponse.json({ instruments: [instrument] })),
    http.get("*/api/v1/trade-preview", ({ request }) => (previews.push(new URL(request.url)), HttpResponse.json(preview()))),
  );
});
const post = (status = 201, body: object = { trade_id: "x" }) =>
  server.use(http.post("*/api/v1/trades", async ({ request }) => (posts.push({ body: (await request.json()) as Record<string, unknown>, key: request.headers.get("Idempotency-Key") }), HttpResponse.json(body, { status }))));

async function step1(u: ReturnType<typeof userEvent.setup>, account = "Apex 50K", qty = "2") {
  await u.click(await screen.findByLabelText("Conta"));
  await u.click(await screen.findByRole("option", { name: account }));
  await u.click(screen.getByRole("radio", { name: "Compra" }));
  await u.type(screen.getByLabelText("Quantidade"), qty);
  await u.click(screen.getByRole("button", { name: "Continuar" }));
  await screen.findByRole("heading", { level: 2, name: "Entrada e saída" });
}
function times(o: { ed?: string; et?: string; ep?: string; xd?: string; xt?: string; xp?: string } = {}) {
  const v = { ed: "2026-10-23", et: "13:30:05", ep: "21234.50", xd: "2026-10-23", xt: "13:42:35", xp: "21240.25", ...o };
  fireEvent.change(document.getElementById("mt-entry-date") as HTMLElement, { target: { value: v.ed } });
  fireEvent.change(document.getElementById("mt-entry-time") as HTMLElement, { target: { value: v.et } });
  fireEvent.change(screen.getByLabelText("Entrada, preço"), { target: { value: v.ep } });
  fireEvent.change(document.getElementById("mt-exit-date") as HTMLElement, { target: { value: v.xd } });
  fireEvent.change(document.getElementById("mt-exit-time") as HTMLElement, { target: { value: v.xt } });
  fireEvent.change(screen.getByLabelText("Saída, preço"), { target: { value: v.xp } });
}
async function toReview(u: ReturnType<typeof userEvent.setup>) {
  await u.click(screen.getByRole("button", { name: "Continuar" }));
  await u.click(await screen.findByRole("button", { name: "Pular etapa" }));
  await screen.findByText(/Origem: Manual/);
}

describe("preview (X-3: the Core does the arithmetic)", () => {
  it("is a GET with UTC instants converted from the profile time zone and no commission unless typed", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(me("America/Sao_Paulo"))));
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u);
    times();
    await waitFor(() => expect(previews.length).toBeGreaterThan(0));
    const p = previews[previews.length - 1]!;
    expect(p.searchParams.get("entry_time")).toBe("2026-10-23T16:30:05Z"); // 13:30:05 GMT-3
    expect(p.searchParams.get("exit_time")).toBe("2026-10-23T16:42:35Z");
    expect(p.searchParams.get("entry_price")).toBe("21234.5");
    expect(p.searchParams.get("quantity")).toBe("2");
    expect(p.searchParams.get("instrument_id")).toBe(ins);
    expect(p.searchParams.has("commission")).toBe(false);
  });
  it("typing a commission re-runs the preview with it; clearing returns to the Core's estimate", async () => {
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times();
    await waitFor(() => expect(previews.length).toBeGreaterThan(0));
    await u.type(screen.getByLabelText(/Comissão total/), "3,25");
    await waitFor(() => expect(previews[previews.length - 1]!.searchParams.get("commission")).toBe("3.25"));
    await u.clear(screen.getByLabelText(/Comissão total/));
    await waitFor(() => expect(previews[previews.length - 1]!.searchParams.has("commission")).toBe(false));
  });
  it("shows the Core strings untouched (4 decimals rounded only by formatting), net before gross", async () => {
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times();
    const block = await screen.findByLabelText("Calculado");
    await waitFor(() => expect(within(block).getByText(/\+US\$\s21,50/)).toBeInTheDocument());
    const text = block.textContent ?? "";
    expect(text.indexOf("P&L líquido")).toBeLessThan(text.indexOf("P&L bruto"));
    expect(text).toMatch(/Comissão.*US\$\s1,50.*estimada/);
  });
  it("commission_source none: warning that commission is zero and net equals gross", async () => {
    server.use(http.get("*/api/v1/trade-preview", () => HttpResponse.json(preview({ commission: "0.00", commission_source: "none", pnl: "23.00" }))));
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times();
    expect(await screen.findByText(/Comissões não informadas: a comissão é zero e o líquido é igual ao bruto/)).toBeInTheDocument();
  });
  it("preview failure still lets the user continue and save", async () => {
    server.use(http.get("*/api/v1/trade-preview", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    post();
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times();
    expect(await screen.findByText("Não foi possível calcular agora. Você ainda pode salvar.")).toBeInTheDocument();
    await toReview(u);
    await u.click(screen.getByRole("button", { name: "Salvar trade" }));
    await waitFor(() => expect(posts).toHaveLength(1));
  });
});

describe("validation", () => {
  it("rejects a time in the future, exit == entry, and an off-tick price; does not call the preview", async () => {
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u);
    times({ xd: "2999-01-01", xt: "00:00:00", xp: "21240.10" });
    await u.click(screen.getByRole("button", { name: "Continuar" }));
    expect(await screen.findByText("A data e hora não podem estar no futuro.")).toBeInTheDocument();
    expect(screen.getByText("O preço deve ser múltiplo do tick do instrumento.")).toBeInTheDocument();
    times({ xt: "13:30:05", xp: "21234.50" });
    await u.click(screen.getByRole("button", { name: "Continuar" }));
    expect(await screen.findByText("A saída deve ser depois da entrada.")).toBeInTheDocument();
  });
  it("commission and MAE/MFE: more than 2 decimals, negatives and text are rejected", async () => {
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times();
    for (const bad of ["1,234", "-5", "abc"]) {
      const f = screen.getByLabelText(/Comissão total/);
      await u.clear(f); await u.type(f, bad);
      await u.click(screen.getByRole("button", { name: "Continuar" }));
      expect(await screen.findByText(/Use um valor em dólares|Digite a parte decimal/)).toBeInTheDocument();
    }
  });
  it("quantity bounds 1..10000", async () => {
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await u.type(await screen.findByLabelText("Quantidade"), "10001");
    await u.click(screen.getByRole("button", { name: "Continuar" }));
    expect(await screen.findByText("Use um número inteiro de 1 a 10000.")).toBeInTheDocument();
  });
  it("no accounts: the flow does not start", async () => {
    server.use(http.get("*/api/v1/accounts", () => HttpResponse.json({ accounts: [] })));
    renderApp(<ManualTradeFlow />);
    expect(await screen.findByText("Cadastre uma conta para registrar trades.")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Continuar" })).not.toBeInTheDocument();
  });
});

describe("save semantics (AC-3, idempotency)", () => {
  it("MAE/MFE and commission are omitted when blank, sent as exact strings when typed", async () => {
    post();
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times();
    await u.type(screen.getByLabelText(/Comissão total/), "4,5");
    await u.click(screen.getByRole("button", { name: "Continuar" }));
    await u.type(await screen.findByLabelText(/MAE/), "12,50");
    await u.click(screen.getByRole("button", { name: "Continuar" }));
    await screen.findByText(/Origem: Manual/);
    await u.click(screen.getByRole("button", { name: "Salvar trade" }));
    await waitFor(() => expect(posts).toHaveLength(1));
    expect(posts[0]!.body).toMatchObject({ commission: "4.5", mae: "12.5", direction: "long", quantity: 2 });
    expect(posts[0]!.body).not.toHaveProperty("mfe");
    expect(posts[0]!.body).not.toHaveProperty("origin");
    expect(posts[0]!.body).not.toHaveProperty("pnl");
  });
  it("a refused save keeps every value and a retry reuses the same Idempotency-Key", async () => {
    server.use(http.post("*/api/v1/trades", async ({ request }) => (posts.push({ body: (await request.json()) as Record<string, unknown>, key: request.headers.get("Idempotency-Key") }), HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 }))));
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times(); await toReview(u);
    await u.click(screen.getByRole("button", { name: "Salvar trade" }));
    await waitFor(() => expect(posts).toHaveLength(1));
    await u.click(await screen.findByRole("button", { name: "Salvar trade" }));
    await waitFor(() => expect(posts).toHaveLength(2));
    expect(posts[1]!.key).toBe(posts[0]!.key);
    expect(navigation.push).not.toHaveBeenCalled();
  });
  it("two clicks in the same tick reuse one Idempotency-Key (the Core can never store two trades)", async () => {
    post();
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times(); await toReview(u);
    const save = screen.getByRole("button", { name: "Salvar trade" });
    fireEvent.click(save); fireEvent.click(save);
    await waitFor(() => expect(posts.length).toBeGreaterThan(0));
    await new Promise((r) => setTimeout(r, 300));
    expect(new Set(posts.map((p) => p.key)).size).toBe(1);
  });
  it("two clicks in the same tick send ONE request (submit guard must not depend on async state)", async () => {
    post();
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times(); await toReview(u);
    const save = screen.getByRole("button", { name: "Salvar trade" });
    fireEvent.click(save); fireEvent.click(save);
    await waitFor(() => expect(posts.length).toBeGreaterThan(0));
    await new Promise((r) => setTimeout(r, 300));
    expect(posts).toHaveLength(1);
  });
  it("422 on a step-2 field sends the user back to step 2 with the error on the field", async () => {
    post(422, { error: { code: "trades.not_tick_multiple", pointer: "/exit_price" } });
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times(); await toReview(u);
    await u.click(screen.getByRole("button", { name: "Salvar trade" }));
    expect(await screen.findByRole("heading", { level: 2, name: "Entrada e saída" })).toBeInTheDocument();
    await waitFor(() => expect(screen.getByLabelText("Saída, preço")).toHaveAttribute("aria-invalid", "true"));
  });
  it("Salvar e registrar outro: new key, only the account is kept, back on step 1, nothing navigates", async () => {
    post();
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times(); await toReview(u);
    await u.click(screen.getByRole("button", { name: "Salvar e registrar outro" }));
    await waitFor(() => expect(posts).toHaveLength(1));
    expect(await screen.findByRole("heading", { level: 2, name: "Trade" })).toBeInTheDocument();
    expect(navigation.push).not.toHaveBeenCalled();
    expect(screen.getByLabelText("Conta")).toHaveTextContent("Apex 50K");
    expect(screen.getByLabelText("Quantidade")).toHaveValue("");
    expect(screen.getByRole("radio", { name: "Compra" })).not.toBeChecked();
    await step1(userEvent.setup(), "Apex 50K", "1"); times(); await toReview(userEvent.setup());
    await userEvent.setup().click(screen.getByRole("button", { name: "Salvar trade" }));
    await waitFor(() => expect(posts).toHaveLength(2));
    expect(posts[1]!.key).not.toBe(posts[0]!.key);
  });
  it("cancel with typed data asks before discarding", async () => {
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await u.type(await screen.findByLabelText("Quantidade"), "3");
    await u.click(screen.getByRole("button", { name: "Cancelar" }));
    expect(await screen.findByRole("alertdialog")).toHaveTextContent("Descartar alterações?");
    expect(navigation.push).not.toHaveBeenCalled();
  });
  it("the note that manual trades never change limits or blocks is on screen (AC-11)", async () => {
    renderApp(<ManualTradeFlow />);
    expect(await screen.findByText("Trades manuais não alteram limites nem bloqueios.")).toBeInTheDocument();
  });
  it("saving a manual trade never calls a risk endpoint", async () => {
    let risk = 0;
    server.use(http.all("*/api/v1/accounts/*/risk*", () => (risk++, HttpResponse.json({}))));
    post();
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times(); await toReview(u);
    await u.click(screen.getByRole("button", { name: "Salvar trade" }));
    await waitFor(() => expect(posts).toHaveLength(1));
    expect(risk).toBe(0);
  });
});

describe("edit (Q56)", () => {
  const manual = (o: Record<string, unknown> = {}) => ({
    trade_id: tid, account_id: acc, account_name: "Apex 50K", account_deleted: false, instrument_id: ins, instrument_symbol: "MNQ", direction: "long", quantity: 2,
    entry_time: "2026-10-23T13:30:05Z", entry_price: "21234.50", exit_time: "2026-10-23T13:42:35Z", exit_price: "21240.25", duration_seconds: 750, pnl: "21.50", pnl_basis: "net", gross_pnl: "23.00",
    commission: "1.50", commission_source: "manual", mae: "10.00", origin: "manual", editable: true, row_version: 6, created_at: "2026-10-23T13:50:00Z", ...o,
  });
  it("prefills, shows Core exact strings, PUTs the full replacement with row_version, no origin/pnl", async () => {
    const puts: Record<string, unknown>[] = [];
    server.use(http.get(`*/api/v1/trades/${tid}`, () => HttpResponse.json(manual())), http.put(`*/api/v1/trades/${tid}`, async ({ request }) => (puts.push((await request.json()) as Record<string, unknown>), HttpResponse.json(manual({ row_version: 7 })))));
    renderApp(<ManualTradeEdit tradeId={tid} />);
    const u = userEvent.setup();
    const qty = await screen.findByLabelText("Quantidade");
    expect(qty).toHaveValue("2");
    await u.clear(qty); await u.type(qty, "3");
    await u.click(screen.getByRole("button", { name: "Salvar alterações" }));
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(puts[0]).toMatchObject({ row_version: 6, quantity: 3, account_id: acc, mae: "10.00" });
    expect(puts[0]).not.toHaveProperty("pnl");
    expect(puts[0]).not.toHaveProperty("origin");
    expect(puts[0]).not.toHaveProperty("mfe");
  });
  it("409 then Recarregar refetches the trade and the next PUT carries the new row_version", async () => {
    let version = 6; const puts: Record<string, unknown>[] = [];
    server.use(http.get(`*/api/v1/trades/${tid}`, () => HttpResponse.json(manual({ row_version: version }))), http.put(`*/api/v1/trades/${tid}`, async ({ request }) => {
      const b = (await request.json()) as Record<string, unknown>; puts.push(b);
      return b.row_version === 6 ? HttpResponse.json({ error: { code: "common.version_conflict" } }, { status: 409 }) : HttpResponse.json(manual({ row_version: 8 }));
    }));
    renderApp(<ManualTradeEdit tradeId={tid} />);
    const u = userEvent.setup();
    const qty = await screen.findByLabelText("Quantidade");
    await u.clear(qty); await u.type(qty, "4");
    await u.click(screen.getByRole("button", { name: "Salvar alterações" }));
    const reload = await screen.findByRole("button", { name: "Recarregar" });
    version = 7; // the trade changed elsewhere before the user reloads
    await u.click(reload);
    const q2 = await screen.findByLabelText("Quantidade");
    await u.clear(q2); await u.type(q2, "4");
    await u.click(screen.getByRole("button", { name: "Salvar alterações" }));
    await waitFor(() => expect(puts.length).toBeGreaterThan(1), { timeout: 5000 });
    expect(puts[puts.length - 1]?.row_version).toBe(7);
  });
  it("409 trades.idempotency_key_reused on create shows guidance to check the list", async () => {
    server.use(http.post("*/api/v1/trades", () => HttpResponse.json({ error: { code: "trades.idempotency_key_reused" } }, { status: 409 })));
    renderApp(<ManualTradeFlow />);
    const u = userEvent.setup();
    await step1(u); times(); await toReview(u);
    await u.click(screen.getByRole("button", { name: "Salvar trade" }));
    expect(await screen.findByText(/Confira a lista de trades/)).toBeInTheDocument();
  });
  it("a captured (non-editable) trade cannot be edited, even by URL", async () => {
    server.use(http.get(`*/api/v1/trades/${tid}`, () => HttpResponse.json({ ...manual(), origin: "routed", editable: false, row_version: undefined, captured_at: "2026-10-23T13:42:40Z" })));
    renderApp(<ManualTradeEdit tradeId={tid} />);
    expect(await screen.findByText("Trade capturado: não pode ser alterado.")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Salvar alterações" })).not.toBeInTheDocument();
  });
  it("another user's trade (404) is not found without retry", async () => {
    server.use(http.get(`*/api/v1/trades/${tid}`, () => HttpResponse.json({ error: { code: "common.not_found" } }, { status: 404 })));
    renderApp(<ManualTradeEdit tradeId={tid} />);
    expect(await screen.findByText("Não encontrado.")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /tentar/i })).not.toBeInTheDocument();
  });
  it("409 version conflict offers reload and keeps typed values", async () => {
    server.use(http.get(`*/api/v1/trades/${tid}`, () => HttpResponse.json(manual())), http.put(`*/api/v1/trades/${tid}`, () => HttpResponse.json({ error: { code: "common.version_conflict" } }, { status: 409 })));
    renderApp(<ManualTradeEdit tradeId={tid} />);
    const u = userEvent.setup();
    const qty = await screen.findByLabelText("Quantidade");
    await u.clear(qty); await u.type(qty, "5");
    await u.click(screen.getByRole("button", { name: "Salvar alterações" }));
    expect(await screen.findByText("Este trade mudou em outro lugar.")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Recarregar" })).toBeInTheDocument();
    expect(qty).toHaveValue("5");
  });
});
