import { fireEvent, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { ManualTradeFlow } from "./manual-trade-flow";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const accountId = "00000000-0000-4000-8000-0000000000a1";
const instrumentId = "00000000-0000-4000-8000-0000000000b1";
const instrument = { id: instrumentId, symbol: "MNQ", name: "Micro E-mini Nasdaq-100", exchange: "CME", currency: "USD", tick_size: "0.25", tick_value: "0.5", point_value: "2", price_decimals: 2 };
const previewBody = { duration_seconds: 750, pnl: "21.50", pnl_basis: "net", gross_pnl: "23.00", commission: "1.50", commission_source: "configured" };

beforeEach(() => {
  navigation.push.mockClear();
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(user)),
    http.get("*/api/v1/accounts", () => HttpResponse.json({ accounts: [{ id: accountId, display_name: "Apex 50K" }] })),
    http.get("*/api/v1/instruments", () => HttpResponse.json({ instruments: [instrument] })),
    http.get("*/api/v1/trade-preview", () => HttpResponse.json(previewBody)),
  );
});

async function fillTrade(person: ReturnType<typeof userEvent.setup>, quantity = "2") {
  await person.click(await screen.findByLabelText("Conta"));
  await person.click(await screen.findByRole("option", { name: "Apex 50K" }));
  await person.click(screen.getByRole("radio", { name: "Compra" }));
  await person.type(screen.getByLabelText("Quantidade"), quantity);
  await person.click(screen.getByRole("button", { name: "Continuar" }));
}
function setTimes(entryPrice = "21234.50", exitPrice = "21240.25", exitTime = "13:42:35") {
  fireEvent.change(document.getElementById("mt-entry-date") as HTMLElement, { target: { value: "2026-10-23" } });
  fireEvent.change(document.getElementById("mt-entry-time") as HTMLElement, { target: { value: "13:30:05" } });
  fireEvent.change(screen.getByLabelText("Entrada, preço"), { target: { value: entryPrice } });
  fireEvent.change(document.getElementById("mt-exit-date") as HTMLElement, { target: { value: "2026-10-23" } });
  fireEvent.change(document.getElementById("mt-exit-time") as HTMLElement, { target: { value: exitTime } });
  fireEvent.change(screen.getByLabelText("Saída, preço"), { target: { value: exitPrice } });
}

describe("ManualTradeFlow", () => {
  it("says on every step that manual trades do not change limits or blocks (AC-11)", async () => {
    renderApp(<ManualTradeFlow />);
    const person = userEvent.setup();
    expect(await screen.findByText("Trades manuais não alteram limites nem bloqueios.")).toBeInTheDocument();
    await fillTrade(person);
    await screen.findByRole("heading", { level: 2, name: "Entrada e saída" });
    expect(screen.getByText("Trades manuais não alteram limites nem bloqueios.")).toBeInTheDocument();
  });

  it("starts on step 1 with nothing preselected and asks for the missing fields", async () => {
    renderApp(<ManualTradeFlow />);
    const person = userEvent.setup();
    expect(await screen.findByRole("heading", { level: 2, name: "Trade" })).toBeInTheDocument();
    expect(screen.getByRole("radio", { name: "Compra" })).not.toBeChecked();
    expect(screen.getByRole("radio", { name: "Venda" })).not.toBeChecked();
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    expect(await screen.findByText("Escolha a conta.")).toBeInTheDocument();
    expect(screen.getByText("Escolha a direção.")).toBeInTheDocument();
    expect(screen.getByText("Use um número inteiro de 1 a 10000.")).toBeInTheDocument();
  });

  it("checks the exit against the entry and the prices against the tick on step 2", async () => {
    renderApp(<ManualTradeFlow />);
    const person = userEvent.setup();
    await fillTrade(person);
    expect(await screen.findByRole("heading", { level: 2, name: "Entrada e saída" })).toBeInTheDocument();
    setTimes("21234.30", "21240.25", "13:00:00");
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    expect(await screen.findByText("A saída deve ser depois da entrada.")).toBeInTheDocument();
    expect(screen.getByText("O preço deve ser múltiplo do tick do instrumento.")).toBeInTheDocument();
  });

  it("shows the Core's calculation with net first, gross and the commission origin, and saves with one idempotency key", async () => {
    const calls: { body: unknown; key: string | null }[] = [];
    server.use(http.post("*/api/v1/trades", async ({ request }) => {
      calls.push({ body: await request.json(), key: request.headers.get("Idempotency-Key") });
      return HttpResponse.json({ trade_id: "x" }, { status: 201 });
    }));
    renderApp(<ManualTradeFlow />);
    const person = userEvent.setup();
    await fillTrade(person);
    await screen.findByRole("heading", { level: 2, name: "Entrada e saída" });
    setTimes();
    const block = await screen.findByLabelText("Calculado");
    await waitFor(() => expect(within(block).getByText("+US$ 21,50")).toBeInTheDocument());
    expect(within(block).getByText(/estimada a partir da configuração da conta/)).toBeInTheDocument();
    expect(within(block).getByText("12 min 30 s")).toBeInTheDocument();
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    await person.click(await screen.findByRole("button", { name: "Pular etapa" }));
    expect(await screen.findByText("Origem: Manual · Conta: Apex 50K")).toBeInTheDocument();
    await person.click(screen.getByRole("button", { name: "Salvar trade" }));
    await waitFor(() => expect(calls).toHaveLength(1));
    expect(calls[0]?.key).toMatch(/\S{8,}/);
    expect(calls[0]?.body).toEqual({ account_id: accountId, instrument_id: instrumentId, direction: "long", quantity: 2, entry_time: "2026-10-23T13:30:05Z", entry_price: "21234.5", exit_time: "2026-10-23T13:42:35Z", exit_price: "21240.25" });
    await waitFor(() => expect(navigation.push).toHaveBeenCalledWith("/pt-BR/trades"));
  });

  it("sends a typed commission and the excursions only when filled", async () => {
    const bodies: Record<string, unknown>[] = [];
    server.use(http.post("*/api/v1/trades", async ({ request }) => {
      bodies.push((await request.json()) as Record<string, unknown>);
      return HttpResponse.json({ trade_id: "x" }, { status: 201 });
    }));
    renderApp(<ManualTradeFlow />);
    const person = userEvent.setup();
    await fillTrade(person);
    await screen.findByRole("heading", { level: 2, name: "Entrada e saída" });
    setTimes();
    await person.type(screen.getByLabelText("Comissão total, em dólares (opcional)"), "3,5");
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    await person.type(await screen.findByLabelText("Excursão adversa máx. (MAE), em dólares"), "12");
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    await person.click(await screen.findByRole("button", { name: "Salvar trade" }));
    await waitFor(() => expect(bodies).toHaveLength(1));
    expect(bodies[0]).toMatchObject({ commission: "3.5", mae: "12" });
    expect(bodies[0]).not.toHaveProperty("mfe");
  });

  it("sends the user back to the step that owns a field the Core refused, keeping the values", async () => {
    server.use(http.post("*/api/v1/trades", () => HttpResponse.json({ error: { code: "trades.not_tick_multiple", pointer: "/exit_price" } }, { status: 422 })));
    renderApp(<ManualTradeFlow />);
    const person = userEvent.setup();
    await fillTrade(person);
    await screen.findByRole("heading", { level: 2, name: "Entrada e saída" });
    setTimes();
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    await person.click(await screen.findByRole("button", { name: "Pular etapa" }));
    await person.click(await screen.findByRole("button", { name: "Salvar trade" }));
    expect(await screen.findByRole("heading", { level: 2, name: "Entrada e saída" })).toBeInTheDocument();
    expect(screen.getByLabelText("Saída, preço")).toHaveValue("21240.25");
    expect(navigation.push).not.toHaveBeenCalled();
  });

  it("Save and add another keeps only the account and goes back to step 1 with a new key", async () => {
    const keys: (string | null)[] = [];
    server.use(http.post("*/api/v1/trades", ({ request }) => {
      keys.push(request.headers.get("Idempotency-Key"));
      return HttpResponse.json({ trade_id: "x" }, { status: 201 });
    }));
    renderApp(<ManualTradeFlow />);
    const person = userEvent.setup();
    await fillTrade(person);
    await screen.findByRole("heading", { level: 2, name: "Entrada e saída" });
    setTimes();
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    await person.click(await screen.findByRole("button", { name: "Pular etapa" }));
    await person.click(await screen.findByRole("button", { name: "Salvar e registrar outro" }));
    const heading = await screen.findByRole("heading", { level: 2, name: "Trade" });
    await waitFor(() => expect(heading).toHaveFocus());
    expect(screen.getByLabelText("Conta")).toHaveTextContent("Apex 50K");
    expect(screen.getByLabelText("Quantidade")).toHaveValue("");
    expect(screen.getByRole("radio", { name: "Compra" })).not.toBeChecked();
    expect(navigation.push).not.toHaveBeenCalled();
    expect(keys).toHaveLength(1);
  });

  it("does not start without an account", async () => {
    server.use(http.get("*/api/v1/accounts", () => HttpResponse.json({ accounts: [] })));
    renderApp(<ManualTradeFlow />);
    expect(await screen.findByText("Cadastre uma conta para registrar trades.")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Continuar" })).not.toBeInTheDocument();
  });
});
