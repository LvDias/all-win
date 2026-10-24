import { act, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { FakeEventSource } from "@/test/setup";
import { TradesPage } from "./trades-page";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };

const trade = (overrides: Record<string, unknown> = {}) => ({
  trade_id: "00000000-0000-4000-8000-0000000000t1".replace("t", "a"),
  account_id: "00000000-0000-4000-8000-0000000000a1",
  account_name: "Apex 50K",
  account_deleted: false,
  instrument_id: "00000000-0000-4000-8000-0000000000b1",
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
  origin: "routed",
  source_id: "00000000-0000-4000-8000-0000000000c1",
  source_name: "Strikes MNQ",
  signal_type: "trigger",
  instruction_id: "00000000-0000-4000-8000-0000000000d1",
  editable: false,
  captured_at: "2026-10-23T13:42:40Z",
  created_at: "2026-10-23T13:42:40Z",
  ...overrides,
});
const summary = (overrides: Record<string, unknown> = {}) => ({ as_of: "2026-10-23T14:00:00Z", count: 2, total_pnl: "-45.50", pnl_basis: "net", total_gross_pnl: "-40.00", total_commission: "5.50", uncommissioned_count: 0, ...overrides });

function serve(rows: unknown[], extra: Record<string, unknown> = {}, next?: string) {
  server.use(
    http.get("*/api/v1/trades/summary", () => HttpResponse.json(summary(extra))),
    http.get("*/api/v1/trades", () => HttpResponse.json(next ? { trades: rows, next_cursor: next } : { trades: rows })),
    http.get("*/api/v1/accounts", () => HttpResponse.json({ accounts: [{ id: "00000000-0000-4000-8000-0000000000a1", display_name: "Apex 50K" }] })),
    http.get("*/api/v1/sources", () => HttpResponse.json([])),
  );
}
/** `GET /trades/:id`, leaving `/trades/summary` to its own handler. */
const detail = (body: object) => http.get("*/api/v1/trades/:id", ({ params }) => (params.id === "summary" ? undefined : HttpResponse.json(body)));
async function stream() {
  await waitFor(() => expect(FakeEventSource.latest()).toBeDefined());
  const source = FakeEventSource.latest();
  if (!source) throw new Error("no event stream");
  return source;
}

const originalMatchMedia = window.matchMedia;
const setViewport = (desktop: boolean) => {
  window.matchMedia = ((query: string) => ({ matches: desktop, media: query, onchange: null, addEventListener() {}, removeEventListener() {}, addListener() {}, removeListener() {}, dispatchEvent: () => false })) as typeof window.matchMedia;
};
afterEach(() => {
  window.matchMedia = originalMatchMedia;
});

beforeEach(() => {
  setViewport(true);
  navigation.search = "";
  server.use(http.get("*/api/v1/me", () => HttpResponse.json(user)));
});

describe("TradesPage", () => {
  it("shows the net P&L as the main value with the gross and the commission beside it, from the Core", async () => {
    serve([trade(), trade({ trade_id: "00000000-0000-4000-8000-0000000000a2", pnl: "-67.00", direction: "short" })]);
    renderApp(<TradesPage />);
    const totals = await screen.findByLabelText("Totais do filtro");
    await waitFor(() => expect(within(totals).getByText("-US$ 45,50")).toBeInTheDocument());
    expect(within(totals).getByText(/Bruto -US\$ 40,00 · Comissões US\$ 5,50/)).toBeInTheDocument();
    expect(await screen.findByText("+US$ 21,50")).toBeInTheDocument();
    expect(screen.getAllByText("Apex 50K").length).toBeGreaterThan(0);
    expect(screen.getAllByText("12 min 30 s").length).toBeGreaterThan(0);
  });

  it("warns that net equals gross for trades without commission data", async () => {
    serve([trade({ commission_source: "none" })], { uncommissioned_count: 3 });
    renderApp(<TradesPage />);
    expect(await screen.findByText("3 trades sem comissão informada: neles, o líquido é igual ao bruto.")).toBeInTheDocument();
  });

  it("labels the origin: source and signal, manual, platform without signal", async () => {
    serve([
      trade(),
      trade({ trade_id: "00000000-0000-4000-8000-0000000000a2", origin: "manual", editable: true, row_version: 1, source_id: undefined, source_name: undefined, signal_type: undefined, instruction_id: undefined, captured_at: undefined }),
      trade({ trade_id: "00000000-0000-4000-8000-0000000000a3", origin: "platform", source_id: undefined, source_name: undefined, signal_type: undefined, instruction_id: undefined }),
    ]);
    renderApp(<TradesPage />);
    expect(await screen.findByText("Strikes MNQ")).toBeInTheDocument();
    expect(screen.getByText("Gatilhos")).toBeInTheDocument();
    expect(screen.getByText("Manual")).toBeInTheDocument();
    expect(screen.getByText("Plataforma (sem sinal)")).toBeInTheDocument();
  });

  it("shows — for a missing MAE/MFE, never a number", async () => {
    serve([trade()]);
    renderApp(<TradesPage />);
    await screen.findByText("Strikes MNQ");
    expect(screen.getAllByLabelText("Não informado").length).toBe(2);
  });

  it("shows the first-use empty state, and the filtered one only when a filter is active", async () => {
    serve([]);
    const view = renderApp(<TradesPage />);
    expect(await screen.findByText("Nenhum trade ainda.")).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Ver conexões" })).toBeInTheDocument();
    view.unmount();
    navigation.search = "account=00000000-0000-4000-8000-0000000000a1";
    serve([]);
    renderApp(<TradesPage />);
    expect(await screen.findByText("Nenhum trade com estes filtros.")).toBeInTheDocument();
  });

  it("hides archived accounts by default and asks the Core for them only when the switch is on", async () => {
    const urls: string[] = [];
    serve([trade()]);
    server.use(
      http.get("*/api/v1/trades/summary", ({ request }) => {
        urls.push(request.url);
        return HttpResponse.json(summary());
      }),
      http.get("*/api/v1/trades", ({ request }) => {
        urls.push(request.url);
        return HttpResponse.json({ trades: [trade()] });
      }),
    );
    renderApp(<TradesPage />);
    const toggle = await screen.findByRole("switch", { name: "Incluir contas arquivadas" });
    expect(toggle).not.toBeChecked();
    expect(urls.length).toBeGreaterThan(0);
    expect(urls.every((url) => !url.includes("include_archived"))).toBe(true);
    await userEvent.setup().click(toggle);
    expect(navigation.replace).toHaveBeenCalledWith(expect.stringContaining("archived=1"));
  });

  it("sends include_archived=true on the list and the summary when the URL asks for archived accounts", async () => {
    navigation.search = "archived=1";
    const urls: string[] = [];
    serve([trade()]);
    server.use(
      http.get("*/api/v1/trades/summary", ({ request }) => {
        urls.push(request.url);
        return HttpResponse.json(summary());
      }),
      http.get("*/api/v1/trades", ({ request }) => {
        urls.push(request.url);
        return HttpResponse.json({ trades: [trade()] });
      }),
    );
    renderApp(<TradesPage />);
    expect(await screen.findByRole("switch", { name: "Incluir contas arquivadas" })).toBeChecked();
    await waitFor(() => expect(urls.length).toBeGreaterThanOrEqual(2));
    expect(urls.every((url) => url.includes("include_archived=true"))).toBe(true);
  });

  it("sorts by the clicked column through the URL and exposes aria-sort", async () => {
    serve([trade()]);
    renderApp(<TradesPage />);
    const pnl = await screen.findByRole("button", { name: "P&L líquido" });
    expect(pnl.closest("th")).toHaveAttribute("aria-sort", "none");
    await userEvent.setup().click(pnl);
    expect(navigation.replace).toHaveBeenCalledWith(expect.stringContaining("sort=pnl"));
  });

  it("loads the next page by cursor and shows the count of the whole set", async () => {
    const cursors: (string | null)[] = [];
    server.use(
      http.get("*/api/v1/trades/summary", () => HttpResponse.json(summary({ count: 2 }))),
      http.get("*/api/v1/trades", ({ request }) => {
        const cursor = new URL(request.url).searchParams.get("cursor");
        cursors.push(cursor);
        return HttpResponse.json(cursor ? { trades: [trade({ trade_id: "00000000-0000-4000-8000-0000000000a9", account_name: "Second" })] } : { trades: [trade()], next_cursor: "abc" });
      }),
      http.get("*/api/v1/accounts", () => HttpResponse.json({ accounts: [] })),
      http.get("*/api/v1/sources", () => HttpResponse.json([])),
    );
    renderApp(<TradesPage />);
    expect(await screen.findByText("1 de 2")).toBeInTheDocument();
    await userEvent.setup().click(screen.getByRole("button", { name: "Carregar mais" }));
    expect(await screen.findByText("Second")).toBeInTheDocument();
    expect(cursors).toEqual([null, "abc"]);
  });

  it("does not insert a captured trade under the cursor: it raises a banner and shows it on request", async () => {
    let calls = 0;
    server.use(
      http.get("*/api/v1/trades/summary", () => HttpResponse.json(summary())),
      http.get("*/api/v1/trades", () => HttpResponse.json({ trades: ++calls > 1 ? [trade(), trade({ trade_id: "00000000-0000-4000-8000-0000000000a5", account_name: "Newest" })] : [trade()] })),
      http.get("*/api/v1/accounts", () => HttpResponse.json({ accounts: [] })),
      http.get("*/api/v1/sources", () => HttpResponse.json([])),
    );
    renderApp(<TradesPage />);
    await screen.findByText("Strikes MNQ");
    const events = await stream();
    act(() => events.emit("trade.captured", { type: "trade.captured", at: "2026-10-23T14:01:00Z", trade_id: "00000000-0000-4000-8000-0000000000a5", account_id: "00000000-0000-4000-8000-0000000000a1" }));
    expect(await screen.findByText("1 novo trade")).toBeInTheDocument();
    expect(screen.queryByText("Newest")).not.toBeInTheDocument();
    await userEvent.setup().click(screen.getByRole("button", { name: "Mostrar" }));
    expect(await screen.findByText("Newest")).toBeInTheDocument();
  });

  it("opens the detail with net first, gross, commission and its origin; a manual trade offers edit and delete", async () => {
    serve([trade({ origin: "manual", editable: true, row_version: 3, source_id: undefined, source_name: undefined, signal_type: undefined, instruction_id: undefined, captured_at: undefined, commission_source: "manual" })]);
    server.use(detail(trade({ origin: "manual", editable: true, row_version: 3, source_id: undefined, source_name: undefined, signal_type: undefined, instruction_id: undefined, captured_at: undefined, commission_source: "manual" })));
    renderApp(<TradesPage />);
    await userEvent.setup().click(await screen.findByRole("button", { name: "Apex 50K" }));
    const dialog = await screen.findByRole("dialog");
    expect(within(dialog).getByText("P&L líquido")).toBeInTheDocument();
    expect(within(dialog).getByText("digitada por você")).toBeInTheDocument();
    expect(within(dialog).getByRole("link", { name: "Editar" })).toHaveAttribute("href", "/pt-BR/trades/00000000-0000-4000-8000-0000000000a1/edit");
    expect(within(dialog).getByRole("button", { name: "Excluir" })).toBeInTheDocument();
  });

  it("a captured trade has neither edit nor delete", async () => {
    serve([trade()]);
    server.use(detail(trade()));
    renderApp(<TradesPage />);
    await userEvent.setup().click(await screen.findByRole("button", { name: "Apex 50K" }));
    const dialog = await screen.findByRole("dialog");
    expect(within(dialog).getByText("Trade capturado: não pode ser alterado.")).toBeInTheDocument();
    expect(within(dialog).queryByRole("button", { name: "Excluir" })).not.toBeInTheDocument();
  });

  it("deletes a manual trade with its row_version after a confirmation", async () => {
    const manual = trade({ origin: "manual", editable: true, row_version: 3, source_id: undefined, source_name: undefined, signal_type: undefined, instruction_id: undefined, captured_at: undefined, commission_source: "manual" });
    serve([manual]);
    server.use(detail(manual));
    const deleted: string[] = [];
    server.use(http.delete("*/api/v1/trades/:id", ({ request }) => {
      deleted.push(new URL(request.url).search);
      return new HttpResponse(null, { status: 204 });
    }));
    renderApp(<TradesPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Apex 50K" }));
    await person.click(within(await screen.findByRole("dialog")).getByRole("button", { name: "Excluir" }));
    await person.click(within(await screen.findByRole("alertdialog")).getByRole("button", { name: "Excluir trade" }));
    await waitFor(() => expect(deleted).toEqual(["?row_version=3"]));
  });

  it("below md every trade is a card with the same data in a definition list", async () => {
    setViewport(false);
    serve([trade()]);
    renderApp(<TradesPage />);
    expect(await screen.findByRole("button", { name: /Apex 50K/ })).toBeInTheDocument();
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
    expect(screen.getByText("12 min 30 s")).toBeInTheDocument();
    expect(screen.getByText("+US$ 21,50")).toBeInTheDocument();
  });
});
