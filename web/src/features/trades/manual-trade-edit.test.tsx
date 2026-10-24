import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { ManualTradeEdit } from "./manual-trade-edit";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const tradeId = "00000000-0000-4000-8000-0000000000a1";
const accountId = "00000000-0000-4000-8000-0000000000e1";
const instrumentId = "00000000-0000-4000-8000-0000000000b1";
const instrument = { id: instrumentId, symbol: "MNQ", name: "Micro", exchange: "CME", currency: "USD", tick_size: "0.25", tick_value: "0.5", point_value: "2", price_decimals: 2 };

const manual = (overrides: Record<string, unknown> = {}) => ({
  trade_id: tradeId,
  account_id: accountId,
  account_name: "Apex 50K",
  account_deleted: false,
  instrument_id: instrumentId,
  instrument_symbol: "MNQ",
  direction: "long",
  quantity: 2,
  entry_time: "2026-10-23T13:30:05Z",
  entry_price: "21234.50",
  exit_time: "2026-10-23T13:42:35Z",
  exit_price: "21240.25",
  duration_seconds: 750,
  pnl: "21.50",
  pnl_basis: "net",
  gross_pnl: "23.00",
  commission: "1.50",
  commission_source: "configured",
  origin: "manual",
  editable: true,
  row_version: 3,
  created_at: "2026-10-23T13:50:00Z",
  ...overrides,
});

beforeEach(() => {
  navigation.push.mockClear();
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(user)),
    http.get("*/api/v1/instruments", () => HttpResponse.json({ instruments: [instrument] })),
    http.get("*/api/v1/trade-preview", () => HttpResponse.json({ duration_seconds: 750, pnl: "21.50", pnl_basis: "net", gross_pnl: "23.00", commission: "1.50", commission_source: "configured" })),
  );
});

function serve(trade: object) {
  server.use(http.get(`*/api/v1/trades/${tradeId}`, () => HttpResponse.json(trade)));
}

describe("ManualTradeEdit", () => {
  it("prefills every field from the stored trade, in the display zone, and leaves an estimated commission empty", async () => {
    serve(manual());
    renderApp(<ManualTradeEdit tradeId={tradeId} />);
    expect(await screen.findByLabelText("Quantidade")).toHaveValue("2");
    expect(screen.getByRole("radio", { name: "Compra" })).toBeChecked();
    expect(document.getElementById("me-entry-date")).toHaveValue("2026-10-23");
    expect(document.getElementById("me-entry-time")).toHaveValue("13:30:05");
    expect(screen.getByLabelText("Entrada, preço")).toHaveValue("21234.50");
    expect(screen.getByLabelText("Comissão total, em dólares (opcional)")).toHaveValue("");
    expect(screen.getByText("Trades manuais não alteram limites nem bloqueios.")).toBeInTheDocument();
  });

  it("saves an untouched form with the same values and the stored row_version", async () => {
    serve(manual());
    const bodies: unknown[] = [];
    server.use(http.put(`*/api/v1/trades/${tradeId}`, async ({ request }) => {
      bodies.push(await request.json());
      return HttpResponse.json(manual({ row_version: 4 }));
    }));
    renderApp(<ManualTradeEdit tradeId={tradeId} />);
    await userEvent.setup().click(await screen.findByRole("button", { name: "Salvar alterações" }));
    await waitFor(() => expect(bodies).toHaveLength(1));
    expect(bodies[0]).toEqual({ account_id: accountId, instrument_id: instrumentId, direction: "long", quantity: 2, entry_time: "2026-10-23T13:30:05Z", entry_price: "21234.5", exit_time: "2026-10-23T13:42:35Z", exit_price: "21240.25", row_version: 3 });
    await waitFor(() => expect(navigation.push).toHaveBeenCalledWith("/pt-BR/trades"));
  });

  it("keeps a Core excursion such as 1.125 savable while untouched (L2)", async () => {
    serve(manual({ mae: "1.125", commission_source: "manual", commission: "2.375" }));
    const bodies: Record<string, unknown>[] = [];
    server.use(http.put(`*/api/v1/trades/${tradeId}`, async ({ request }) => {
      bodies.push((await request.json()) as Record<string, unknown>);
      return HttpResponse.json(manual());
    }));
    renderApp(<ManualTradeEdit tradeId={tradeId} />);
    await userEvent.setup().click(await screen.findByRole("button", { name: "Salvar alterações" }));
    await waitFor(() => expect(bodies).toHaveLength(1));
    expect(bodies[0]).toMatchObject({ mae: "1.125", commission: "2.375" });
  });

  it("Reload after a version conflict refetches the trade, so the next save uses the new row_version", async () => {
    let reads = 0;
    server.use(http.get(`*/api/v1/trades/${tradeId}`, () => HttpResponse.json(++reads > 1 ? manual({ row_version: 5 }) : manual())));
    const versions: unknown[] = [];
    server.use(http.put(`*/api/v1/trades/${tradeId}`, async ({ request }) => {
      const body = (await request.json()) as { row_version: number };
      versions.push(body.row_version);
      return body.row_version === 3 ? HttpResponse.json({ error: { code: "common.version_conflict" } }, { status: 409 }) : HttpResponse.json(manual({ row_version: 6 }));
    }));
    renderApp(<ManualTradeEdit tradeId={tradeId} />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Salvar alterações" }));
    expect(await screen.findByText("Este trade mudou em outro lugar.")).toBeInTheDocument();
    await person.click(screen.getByRole("button", { name: "Recarregar" }));
    await waitFor(() => expect(reads).toBe(2));
    await person.click(await screen.findByRole("button", { name: "Salvar alterações" }));
    await waitFor(() => expect(versions).toEqual([3, 5]));
  });

  it("shows no form for a captured trade", async () => {
    serve(manual({ origin: "routed", editable: false, row_version: undefined, source_id: "00000000-0000-4000-8000-0000000000c1", source_name: "Strikes", signal_type: "trigger", instruction_id: "00000000-0000-4000-8000-0000000000d1", captured_at: "2026-10-23T13:42:40Z" }));
    renderApp(<ManualTradeEdit tradeId={tradeId} />);
    expect(await screen.findByText("Trade capturado: não pode ser alterado.")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Salvar alterações" })).not.toBeInTheDocument();
  });

  it("asks before discarding unsaved changes on Cancel", async () => {
    serve(manual());
    renderApp(<ManualTradeEdit tradeId={tradeId} />);
    const person = userEvent.setup();
    expect(await screen.findByRole("link", { name: "Cancelar" })).toBeInTheDocument();
    await person.type(screen.getByLabelText("Quantidade"), "1");
    await person.click(screen.getByRole("button", { name: "Cancelar" }));
    expect(await screen.findByRole("alertdialog")).toHaveTextContent("Descartar alterações?");
    await person.click(screen.getByRole("button", { name: "Continuar editando" }));
    expect(navigation.push).not.toHaveBeenCalled();
  });

  it("shows not-found for another user's trade", async () => {
    server.use(http.get(`*/api/v1/trades/${tradeId}`, () => HttpResponse.json({ error: { code: "common.not_found" } }, { status: 404 })));
    renderApp(<ManualTradeEdit tradeId={tradeId} />);
    expect(await screen.findByText("Não encontrado.")).toBeInTheDocument();
  });
});
