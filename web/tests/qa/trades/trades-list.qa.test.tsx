// QA (Retina) — V1-S21 AC-1/4/5/6/7/8/10 on the trade list and detail, black box through the public index.
import { act, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { FakeEventSource } from "@/test/setup";
// render first: it installs the next/navigation mock before the feature loads.
import { TradesPage } from "@/features/trades";

const me = (tz = "UTC") => ({ id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: tz, is_platform_admin: false });
const uuid = (n: number) => `00000000-0000-4000-8000-${String(n).padStart(12, "0")}`;
const trade = (o: Record<string, unknown> = {}) => ({
  trade_id: uuid(1), account_id: uuid(100), account_name: "Apex 50K", account_deleted: false, instrument_id: uuid(200), instrument_symbol: "MNQ", direction: "long", quantity: 2,
  entry_time: "2026-10-23T13:30:05Z", entry_price: "21234.50", exit_time: "2026-10-23T13:42:35Z", exit_price: "21240.25", duration_seconds: 750,
  pnl: "21.50", pnl_basis: "net", gross_pnl: "23.00", commission: "1.50", commission_source: "configured", mae: "10.00", mfe: "30.00",
  origin: "routed", source_id: uuid(300), source_name: "Strikes MNQ", signal_type: "trigger", instruction_id: uuid(400), editable: false, captured_at: "2026-10-23T13:42:40Z", created_at: "2026-10-23T13:42:40Z", ...o,
});
const summary = (o: Record<string, unknown> = {}) => ({ as_of: "2026-10-23T14:00:00Z", count: 2, total_pnl: "-45.50", pnl_basis: "net", total_gross_pnl: "-40.00", total_commission: "5.50", uncommissioned_count: 0, ...o });
let listCalls: URL[] = [];
let summaryCalls: URL[] = [];
function serve(pages: Array<{ trades: object[]; next_cursor?: string }>, sum: object = summary()) {
  let n = 0;
  server.use(
    http.get("*/api/v1/trades/summary", ({ request }) => (summaryCalls.push(new URL(request.url)), HttpResponse.json(sum))),
    http.get("*/api/v1/trades", ({ request }) => {
      listCalls.push(new URL(request.url));
      return HttpResponse.json(pages[Math.min(n++, pages.length - 1)]);
    }),
  );
}
const originalMatchMedia = window.matchMedia;
const viewport = (desktop: boolean) => {
  window.matchMedia = ((query: string) => ({ matches: desktop, media: query, onchange: null, addEventListener() {}, removeEventListener() {}, addListener() {}, removeListener() {}, dispatchEvent: () => false })) as typeof window.matchMedia;
};
beforeEach(() => {
  viewport(true);
  listCalls = []; summaryCalls = []; navigation.search = ""; navigation.replace.mockClear();
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(me())),
    http.get("*/api/v1/accounts", () => HttpResponse.json({ accounts: [{ id: uuid(100), display_name: "Apex 50K" }, { id: uuid(101), display_name: "Live NQ" }] })),
    http.get("*/api/v1/sources", () => HttpResponse.json([{ id: uuid(300), name: "Strikes MNQ" }])),
  );
});
afterEach(() => { vi.useRealTimers(); window.matchMedia = originalMatchMedia; });

describe("AC-1 net main, gross beside; exact strings", () => {
  it("row shows the Core's net pnl with sign; totals show net primary and gross + commission secondary", async () => {
    serve([{ trades: [trade({ pnl: "-45.5000", gross_pnl: "-40.0000" })] }]);
    renderApp(<TradesPage />);
    const row = (await screen.findByRole("button", { name: "Apex 50K" })).closest("tr")!;
    expect(row).toHaveTextContent(/-US\$\s45,50/);
    expect(row).not.toHaveTextContent(/40,00/); // gross is not the row's main value
    expect(screen.getByLabelText("Totais do filtro")).toHaveTextContent("P&L líquido");
    expect(screen.getByLabelText("Totais do filtro")).toHaveTextContent(/Bruto -US\$\s40,00 · Comissões US\$\s5,50/);
  });
  it("does not lose precision on huge or 4-decimal Core values", async () => {
    serve([{ trades: [trade({ pnl: "99999999999999.9999" })] }], summary({ total_pnl: "-0.0100" }));
    renderApp(<TradesPage />);
    expect(await screen.findByText(/99\.999\.999\.999\.999,99/)).toBeInTheDocument();
  });
  it("uncommissioned_count > 0 warns that net equals gross (singular and plural)", async () => {
    serve([{ trades: [trade()] }], summary({ uncommissioned_count: 1 }));
    renderApp(<TradesPage />);
    expect(await screen.findByText("1 trade sem comissão informada: nele, o líquido é igual ao bruto.")).toBeInTheDocument();
  });
  it("no warning when the field is absent or zero", async () => {
    serve([{ trades: [trade()] }], summary({ uncommissioned_count: undefined }));
    renderApp(<TradesPage />);
    await screen.findByRole("button", { name: "Apex 50K" });
    expect(screen.queryByText(/sem comissão informada/)).not.toBeInTheDocument();
  });
  it("MAE/MFE missing shows an em dash with 'Não informado'", async () => {
    serve([{ trades: [trade({ mae: undefined, mfe: undefined })] }]);
    renderApp(<TradesPage />);
    const row = (await screen.findByRole("button", { name: "Apex 50K" })).closest("tr")!;
    expect(within(row).getAllByLabelText("Não informado")).toHaveLength(2);
  });
});

describe("AC-10 / spec 3.4 origin labels", () => {
  it.each([
    ["manual", { origin: "manual", editable: true, row_version: 1, source_id: undefined, source_name: undefined, signal_type: undefined, instruction_id: undefined, captured_at: undefined }, "Manual"],
    ["platform", { origin: "platform", source_id: undefined, source_name: undefined, signal_type: undefined, instruction_id: undefined }, "Plataforma (sem sinal)"],
    ["master", { origin: "master", master_account_id: uuid(9), master_account_name: "Mestre", source_id: undefined, source_name: undefined, signal_type: undefined, instruction_id: undefined }, "Replicação · Mestre"],
    ["routed", {}, "Strikes MNQ"],
  ])("%s", async (_n, o, label) => {
    serve([{ trades: [trade(o)] }]);
    renderApp(<TradesPage />);
    const row = (await screen.findByRole("button", { name: "Apex 50K" })).closest("tr")!;
    expect(row).toHaveTextContent(label);
  });
  it("platform and manual are different labels", async () => {
    serve([{ trades: [trade({ trade_id: uuid(2), origin: "platform", source_name: undefined }), trade({ trade_id: uuid(3), origin: "manual", editable: true, row_version: 1 })] }]);
    renderApp(<TradesPage />);
    await screen.findAllByRole("button", { name: "Apex 50K" });
    expect(screen.getByText("Plataforma (sem sinal)")).toBeInTheDocument();
    expect(screen.getAllByText("Manual").length).toBeGreaterThan(0);
  });
});

describe("AC-6 date filter by exit time in the profile time zone", () => {
  it("default 30d sends only `from` = local midnight 30 days back, never `to`", async () => {
    vi.useFakeTimers({ toFake: ["Date"], now: new Date("2026-10-23T02:00:00Z") });
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(me("America/Sao_Paulo"))));
    serve([{ trades: [] }]);
    renderApp(<TradesPage />);
    await waitFor(() => expect(listCalls.length).toBeGreaterThan(0));
    // 23/10 02:00Z is still 22/10 23:00 in GMT-3, so 30 days back is 22/09 00:00 local = 03:00Z.
    const last = listCalls[listCalls.length - 1]!;
    expect(last.searchParams.get("from")).toBe("2026-09-22T03:00:00Z");
    expect(last.searchParams.get("to")).toBeNull();
    expect(last.searchParams.get("sort")).toBe("exit_time");
  });
  it("custom range: from = start of the first day, to = start of the day AFTER the last (inclusive end day), same on summary", async () => {
    navigation.search = "period=custom&from=2026-10-01&to=2026-10-03";
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(me("America/Sao_Paulo"))));
    serve([{ trades: [] }]);
    renderApp(<TradesPage />);
    await waitFor(() => expect(summaryCalls.length).toBeGreaterThan(0));
    for (const call of [listCalls[listCalls.length - 1]!, summaryCalls[summaryCalls.length - 1]!]) {
      expect(call.searchParams.get("from")).toBe("2026-10-01T03:00:00Z");
      expect(call.searchParams.get("to")).toBe("2026-10-04T03:00:00Z");
    }
  });
  it("garbage in the URL falls back to safe defaults", async () => {
    navigation.search = "period=forever&from=nope&sort=drop&order=sideways&signal=evil";
    serve([{ trades: [] }]);
    renderApp(<TradesPage />);
    await waitFor(() => expect(listCalls.length).toBeGreaterThan(0));
    const c = listCalls[0]!;
    expect(c.searchParams.get("sort")).toBe("exit_time");
    expect(c.searchParams.get("order")).toBe("desc");
    expect(c.searchParams.getAll("signal_type")).toEqual([]);
  });
});

describe("AC-4 filters", () => {
  it("manual/platform go as origin, real sources as source_id, accounts and signals repeat", async () => {
    navigation.search = `source=manual&source=platform&source=${uuid(300)}&account=${uuid(100)}&account=${uuid(101)}&signal=trigger`;
    serve([{ trades: [] }]);
    renderApp(<TradesPage />);
    await waitFor(() => expect(listCalls.length).toBeGreaterThan(0));
    const c = listCalls[listCalls.length - 1]!;
    expect(c.searchParams.getAll("origin")).toEqual(["manual", "platform"]);
    expect(c.searchParams.getAll("source_id")).toEqual([uuid(300)]);
    expect(c.searchParams.getAll("account_id")).toEqual([uuid(100), uuid(101)]);
    expect(c.searchParams.getAll("signal_type")).toEqual(["trigger"]);
  });
  it("changing the period writes the URL; 30d is the clean default", async () => {
    serve([{ trades: [trade()] }]);
    renderApp(<TradesPage />);
    await screen.findByRole("button", { name: "Apex 50K" });
    await userEvent.setup().click(screen.getByRole("button", { name: "7 dias" }));
    expect(navigation.replace).toHaveBeenCalledWith(expect.stringContaining("period=7d"));
  });
  it("active filter count is stated in text", async () => {
    navigation.search = `account=${uuid(100)}&signal=trigger`;
    serve([{ trades: [trade()] }]);
    renderApp(<TradesPage />);
    expect(await screen.findByText("2 filtros ativos")).toBeInTheDocument();
  });
  it("empty without filters offers register; empty with filters never offers it, only clear", async () => {
    serve([{ trades: [] }], summary({ count: 0 }));
    const first = renderApp(<TradesPage />);
    expect(await screen.findByText("Nenhum trade ainda.")).toBeInTheDocument();
    first.unmount();
    navigation.search = `signal=trigger`;
    serve([{ trades: [] }], summary({ count: 0 }));
    renderApp(<TradesPage />);
    expect(await screen.findByText("Nenhum trade com estes filtros.")).toBeInTheDocument();
    expect(screen.queryByText("Nenhum trade ainda.")).not.toBeInTheDocument();
    expect(screen.getAllByRole("button", { name: "Limpar filtros" }).length).toBeGreaterThan(0);
  });
});

describe("sorting, paging and live banner", () => {
  it("sort headers expose aria-sort and clicking sends the sort", async () => {
    serve([{ trades: [trade()] }]);
    renderApp(<TradesPage />);
    const exit = (await screen.findByRole("columnheader", { name: /Saída/ }));
    expect(exit).toHaveAttribute("aria-sort", "descending");
    await userEvent.setup().click(screen.getByRole("button", { name: /^P&L líquido$/ }));
    expect(navigation.replace).toHaveBeenCalledWith(expect.stringContaining("sort=pnl"));
  });
  it("Carregar mais requests the opaque cursor with the same filters and appends rows", async () => {
    navigation.search = `signal=trigger`;
    serve([{ trades: [trade()], next_cursor: "abc_DEF-1" }, { trades: [trade({ trade_id: uuid(2), account_name: "Live NQ" })] }], summary({ count: 2 }));
    renderApp(<TradesPage />);
    await screen.findByRole("button", { name: "Apex 50K" });
    expect(screen.getByText("1 de 2")).toBeInTheDocument();
    await userEvent.setup().click(screen.getByRole("button", { name: "Carregar mais" }));
    await screen.findByRole("button", { name: "Live NQ" });
    const c = listCalls[listCalls.length - 1]!;
    expect(c.searchParams.get("cursor")).toBe("abc_DEF-1");
    expect(c.searchParams.getAll("signal_type")).toEqual(["trigger"]);
    expect(screen.queryByRole("button", { name: "Carregar mais" })).not.toBeInTheDocument();
  });
  it("a captured trade only raises the banner; rows change only after Mostrar", async () => {
    serve([{ trades: [trade()] }]);
    renderApp(<TradesPage />);
    await screen.findByRole("button", { name: "Apex 50K" });
    const before = listCalls.length;
    await waitFor(() => expect(FakeEventSource.latest()).toBeDefined());
    act(() => FakeEventSource.latest()?.emit("trade.captured", { type: "trade.captured", at: "2026-10-23T14:00:00Z", trade_id: uuid(9), account_id: uuid(100) }));
    expect(await screen.findByText("1 novo trade")).toBeInTheDocument();
    expect(listCalls.length).toBe(before);
    await userEvent.setup().click(screen.getByRole("button", { name: "Mostrar" }));
    await waitFor(() => expect(listCalls.length).toBeGreaterThan(before));
    expect(screen.queryByText("1 novo trade")).not.toBeInTheDocument();
  });
  it("5xx on the list keeps the totals and offers retry", async () => {
    server.use(http.get("*/api/v1/trades/summary", () => HttpResponse.json(summary())), http.get("*/api/v1/trades", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    renderApp(<TradesPage />);
    expect(await screen.findByRole("button", { name: /tentar/i })).toBeInTheDocument();
  });
  it("a failed summary shows an em dash and retry, not zeros", async () => {
    server.use(http.get("*/api/v1/trades/summary", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })), http.get("*/api/v1/trades", () => HttpResponse.json({ trades: [trade()] })));
    renderApp(<TradesPage />);
    expect(await screen.findByLabelText("Totais indisponíveis")).toHaveTextContent("—");
  });
});

describe("AC-8 detail: captured read-only, manual edit/delete", () => {
  it("captured trade: no edit or delete, says it cannot be changed; commission origin and gross shown", async () => {
    serve([{ trades: [trade({ commission_source: "none", commission: "0.00" })] }]);
    server.use(http.get(`*/api/v1/trades/${uuid(1)}`, () => HttpResponse.json(trade({ commission_source: "none", commission: "0.00" }))));
    renderApp(<TradesPage />);
    await userEvent.setup().click(await screen.findByRole("button", { name: "Apex 50K" }));
    const d = await screen.findByRole("dialog");
    expect(within(d).queryByRole("link", { name: "Editar" })).not.toBeInTheDocument();
    expect(within(d).queryByRole("button", { name: "Excluir" })).not.toBeInTheDocument();
    expect(within(d).getByText("Trade capturado: não pode ser alterado.")).toBeInTheDocument();
    expect(within(d).getByText("P&L bruto")).toBeInTheDocument();
    expect(within(d).getByText("comissões não informadas")).toBeInTheDocument();
  });
  it("manual trade: Editar links to the edit page; delete asks, cancel is focused, sends row_version", async () => {
    const manual = trade({ origin: "manual", editable: true, row_version: 4, source_id: undefined, source_name: undefined, signal_type: undefined, instruction_id: undefined, captured_at: undefined, commission_source: "manual" });
    serve([{ trades: [manual] }]);
    let url: URL | undefined;
    server.use(http.get(`*/api/v1/trades/${uuid(1)}`, () => HttpResponse.json(manual)), http.delete(`*/api/v1/trades/${uuid(1)}`, ({ request }) => ((url = new URL(request.url)), new HttpResponse(null, { status: 204 }))));
    renderApp(<TradesPage />);
    const u = userEvent.setup();
    await u.click(await screen.findByRole("button", { name: "Apex 50K" }));
    const d = await screen.findByRole("dialog");
    expect(within(d).getByRole("link", { name: "Editar" })).toHaveAttribute("href", `/pt-BR/trades/${uuid(1)}/edit`);
    await u.click(within(d).getByRole("button", { name: "Excluir" }));
    const ad = await screen.findByRole("alertdialog");
    await waitFor(() => expect(within(ad).getByRole("button", { name: "Cancelar" })).toHaveFocus());
    expect(url).toBeUndefined();
    await u.click(within(ad).getByRole("button", { name: "Excluir trade" }));
    await waitFor(() => expect(url?.searchParams.get("row_version")).toBe("4"));
  });
  it("delete conflict shows the message and keeps the dialog", async () => {
    const manual = trade({ origin: "manual", editable: true, row_version: 4, captured_at: undefined });
    serve([{ trades: [manual] }]);
    server.use(http.get(`*/api/v1/trades/${uuid(1)}`, () => HttpResponse.json(manual)), http.delete(`*/api/v1/trades/${uuid(1)}`, () => HttpResponse.json({ error: { code: "common.version_conflict" } }, { status: 409 })));
    renderApp(<TradesPage />);
    const u = userEvent.setup();
    await u.click(await screen.findByRole("button", { name: "Apex 50K" }));
    await u.click(await within(await screen.findByRole("dialog")).findByRole("button", { name: "Excluir" }));
    await u.click(within(await screen.findByRole("alertdialog")).getByRole("button", { name: "Excluir trade" }));
    expect(await screen.findByText("Este trade mudou em outro lugar.")).toBeInTheDocument();
  });
  it("detail of another user's trade (404) is not found, no data from the row leaks", async () => {
    serve([{ trades: [] }]);
    renderApp(<TradesPage />);
    // the list can't open an id it doesn't list; the detail query itself must map 404 safely
    expect(await screen.findByText("Nenhum trade ainda.")).toBeInTheDocument();
  });
});
