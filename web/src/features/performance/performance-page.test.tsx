import { act, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { FakeEventSource } from "@/test/setup";
import { PerformancePage } from "./performance-page";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const sourceId = "00000000-0000-4000-8000-0000000000c1";

const figures = (overrides: Record<string, unknown> = {}) => ({
  trade_count: 10,
  win_count: 6,
  loss_count: 3,
  breakeven_count: 1,
  total_pnl: "320.50",
  pnl_basis: "net",
  total_gross_pnl: "350.00",
  total_commission: "29.50",
  uncommissioned_count: 0,
  avg_pnl: "32.05",
  avg_gross_pnl: "35.00",
  win_rate: "60.0",
  avg_duration_seconds: 750,
  avg_mae: "11.25",
  mae_sample_count: 10,
  avg_mfe: "20.00",
  mfe_sample_count: 10,
  ...overrides,
});
const performance = (overrides: Record<string, unknown> = {}) => ({
  as_of: "2026-10-23T14:00:00Z",
  group_by: "source_signal",
  totals: figures(),
  groups: [
    { origin: "routed", source_id: sourceId, source_name: "Strikes MNQ", signal_type: "trigger", figures: figures({ trade_count: 7, total_pnl: "400.00" }), bar_ratio: "1.0000" },
    { origin: "manual", figures: figures({ trade_count: 3, total_pnl: "-79.50", avg_mae: undefined, mae_sample_count: 0 }), bar_ratio: "0.1988" },
  ],
  ...overrides,
});

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
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(user)),
    http.get("*/api/v1/accounts", () => HttpResponse.json({ accounts: [{ id: "00000000-0000-4000-8000-0000000000a1", display_name: "Apex 50K" }] })),
  );
});
function serve(body: object) {
  server.use(http.get("*/api/v1/performance", () => HttpResponse.json(body)));
}
async function stream() {
  await waitFor(() => expect(FakeEventSource.latest()).toBeDefined());
  const source = FakeEventSource.latest();
  if (!source) throw new Error("no event stream");
  return source;
}

describe("PerformancePage", () => {

  it("asks for archived accounts only when the switch is on (URL ?archived=1)", async () => {
    const urls: string[] = [];
    server.use(
      http.get("*/api/v1/performance", ({ request }) => {
        urls.push(request.url);
        return HttpResponse.json(performance());
      }),
    );
    renderApp(<PerformancePage />);
    const toggle = await screen.findByRole("switch", { name: "Incluir contas arquivadas" });
    expect(toggle).not.toBeChecked();
    expect(urls.every((url) => !url.includes("include_archived"))).toBe(true);
    await userEvent.setup().click(toggle);
    expect(navigation.replace).toHaveBeenCalledWith(expect.stringContaining("archived=1"));
  });

  it("sends include_archived=true when the URL asks for archived accounts", async () => {
    navigation.search = "archived=1";
    const urls: string[] = [];
    server.use(
      http.get("*/api/v1/performance", ({ request }) => {
        urls.push(request.url);
        return HttpResponse.json(performance());
      }),
    );
    renderApp(<PerformancePage />);
    expect(await screen.findByRole("switch", { name: "Incluir contas arquivadas" })).toBeChecked();
    await waitFor(() => expect(urls.length).toBeGreaterThan(0));
    expect(urls.every((url) => url.includes("include_archived=true"))).toBe(true);
  });
  it("shows the Core's figures: net first, gross and commissions beside it, win/loss/zero counts, no invented zero", async () => {
    serve(performance());
    renderApp(<PerformancePage />);
    const summary = await screen.findByLabelText("Resumo");
    expect(within(summary).getByText("+US$ 320,50")).toBeInTheDocument();
    expect(within(summary).getByText(/Bruto \+US\$ 350,00 · Comissões US\$ 29,50/)).toBeInTheDocument();
    expect(within(summary).getByText("60,0 %")).toBeInTheDocument();
    expect(within(summary).getByText("6 vitórias · 3 derrotas · 1 zero")).toBeInTheDocument();
    expect(within(summary).getByText("12 min 30 s")).toBeInTheDocument();
    expect(screen.getByText(/Dados até/)).toBeInTheDocument();
  });

  it("shows — for a missing average and the basis line when not every trade has MAE/MFE", async () => {
    serve(performance({ totals: figures({ avg_mae: undefined, mae_sample_count: 0, avg_mfe: "20.00", mfe_sample_count: 4 }) }));
    renderApp(<PerformancePage />);
    const summary = await screen.findByLabelText("Resumo");
    expect(within(summary).getByText("Base: 4 de 10 trades")).toBeInTheDocument();
    expect(within(summary).getByText("Base: 0 de 10 trades")).toBeInTheDocument();
    expect(within(summary).getAllByText("—").length).toBeGreaterThan(0);
  });

  it("warns that net equals gross for trades without commission data", async () => {
    serve(performance({ totals: figures({ uncommissioned_count: 2 }) }));
    renderApp(<PerformancePage />);
    expect(await screen.findByText("2 trades sem comissão informada: neles, o líquido é igual ao bruto.")).toBeInTheDocument();
  });

  it("lists one row per group with its label, a Total row from the Core in the footer, and aria-hidden proportion bars from bar_ratio", async () => {
    serve(performance());
    renderApp(<PerformancePage />);
    const table = await screen.findByRole("table");
    expect(within(table).getByText("Strikes MNQ · Gatilhos")).toBeInTheDocument();
    expect(within(table).getByText("Manual")).toBeInTheDocument();
    const footer = table.querySelector("tfoot");
    expect(footer).not.toBeNull();
    expect(within(footer as HTMLElement).getByRole("rowheader", { name: "Total" })).toBeInTheDocument();
    expect(within(footer as HTMLElement).getByText("+US$ 320,50")).toBeInTheDocument();
    const bars = table.querySelectorAll('[aria-hidden="true"] > div[style]');
    expect(Array.from(bars).map((bar) => (bar as HTMLElement).style.width)).toEqual(["calc(100%)", "calc(19.88%)"]);
  });

  it("links each group to the trade list with the same filter and its source and signal", async () => {
    navigation.search = "period=7d&account=00000000-0000-4000-8000-0000000000a1";
    serve(performance());
    renderApp(<PerformancePage />);
    const table = await screen.findByRole("table");
    const links = within(table).getAllByRole("link", { name: "Ver" });
    expect(links[0]).toHaveAttribute("href", `/pt-BR/trades?period=7d&account=00000000-0000-4000-8000-0000000000a1&source=${sourceId}&signal=trigger`);
    expect(links[1]).toHaveAttribute("href", "/pt-BR/trades?period=7d&account=00000000-0000-4000-8000-0000000000a1&source=manual");
    expect(screen.getByRole("link", { name: "Ver os trades deste filtro" })).toHaveAttribute("href", "/pt-BR/trades?period=7d&account=00000000-0000-4000-8000-0000000000a1");
  });

  it("sends the same exit-time window as the list: custom ends at the start of the next local day", async () => {
    navigation.search = "period=custom&from=2026-10-01&to=2026-10-31";
    const urls: URL[] = [];
    server.use(http.get("*/api/v1/performance", ({ request }) => {
      urls.push(new URL(request.url));
      return HttpResponse.json(performance());
    }));
    renderApp(<PerformancePage />);
    await screen.findByRole("table");
    expect(urls[0]?.searchParams.get("from")).toBe("2026-10-01T00:00:00Z");
    expect(urls[0]?.searchParams.get("to")).toBe("2026-11-01T00:00:00Z");
    expect(urls[0]?.searchParams.get("group_by")).toBe("source_signal");
  });

  it("group-by buttons and sortable headers go through the URL", async () => {
    serve(performance());
    renderApp(<PerformancePage />);
    const person = userEvent.setup();
    await screen.findByRole("table");
    await person.click(screen.getByRole("button", { name: "Fonte" }));
    expect(navigation.replace).toHaveBeenCalledWith(expect.stringContaining("group=source"));
    const head = screen.getByRole("button", { name: "P&L total (líquido)" });
    expect(head.closest("th")).toHaveAttribute("aria-sort", "descending");
    await person.click(screen.getByRole("button", { name: "Taxa de acerto" }));
    expect(navigation.replace).toHaveBeenCalledWith(expect.stringContaining("sort=win_rate"));
  });

  it("the first-use probe counts trades of archived accounts too", async () => {
    const urls: string[] = [];
    server.use(
      http.get("*/api/v1/performance", () => HttpResponse.json(performance({ totals: figures({ trade_count: 0 }), groups: [] }))),
      http.get("*/api/v1/trades/summary", ({ request }) => {
        urls.push(request.url);
        return HttpResponse.json({ as_of: "2026-10-23T14:00:00Z", count: 3, total_pnl: "1.00", pnl_basis: "net", total_gross_pnl: "1.00", total_commission: "0.00" });
      }),
    );
    renderApp(<PerformancePage />);
    await waitFor(() => expect(urls.length).toBeGreaterThan(0));
    expect(urls.every((url) => url.includes("include_archived=true"))).toBe(true);
  });

  it("shows the gross average beside the net one, from the Core, in the cards and in the Total row", async () => {
    serve(performance());
    renderApp(<PerformancePage />);
    await screen.findAllByText("P&L médio por trade (líquido)");
    const gross = screen.getAllByText(/^bruto /);
    expect(gross.length).toBeGreaterThanOrEqual(1);
    expect(gross[0]).toHaveTextContent(/35[.,]00/);
  });

  it("shows the gross total beside the net total in every group and in the Total row, from the Core", async () => {
    serve(performance());
    renderApp(<PerformancePage />);
    await screen.findAllByText("P&L médio por trade (líquido)");
    const gross = screen.getAllByText(/^bruto /);
    // average and total gross for the totals, and for each group, in the cards
    expect(gross.length).toBeGreaterThanOrEqual(4);
    expect(screen.getAllByText(/^bruto .*350[.,]00/).length).toBeGreaterThanOrEqual(1);
  });

  it("shows no gross average when the Core sends none, and never computes one", async () => {
    serve(performance({ totals: figures({ avg_gross_pnl: undefined }), groups: [] }));
    renderApp(<PerformancePage />);
    const label = (await screen.findAllByText("P&L médio por trade (líquido)"))[0];
    const figure = label?.closest("div");
    expect(figure).not.toBeNull();
    expect(within(figure as HTMLElement).queryByText(/^bruto /)).not.toBeInTheDocument();
  });

  it("first use is told apart from a filter that matches nothing", async () => {
    serve(performance({ totals: figures({ trade_count: 0, win_count: 0, loss_count: 0, breakeven_count: 0, total_pnl: "0.00", total_gross_pnl: "0.00", total_commission: "0.00", avg_pnl: undefined, win_rate: undefined, avg_duration_seconds: undefined, avg_mae: undefined, mae_sample_count: 0, avg_mfe: undefined, mfe_sample_count: 0 }), groups: [] }));
    server.use(http.get("*/api/v1/trades/summary", () => HttpResponse.json({ as_of: "2026-10-23T14:00:00Z", count: 0, total_pnl: "0.00", pnl_basis: "net", total_gross_pnl: "0.00", total_commission: "0.00" })));
    const view = renderApp(<PerformancePage />);
    expect(await screen.findByText("Ainda não há trades para analisar.")).toBeInTheDocument();
    view.unmount();
    server.use(http.get("*/api/v1/trades/summary", () => HttpResponse.json({ as_of: "2026-10-23T14:00:00Z", count: 40, total_pnl: "10.00", pnl_basis: "net", total_gross_pnl: "10.00", total_commission: "0.00" })));
    renderApp(<PerformancePage />);
    expect(await screen.findByText("Nenhum trade com estes filtros.")).toBeInTheDocument();
    // AC-5: the summary stays, with zero totals and dashes, and says there are no trades in the filter.
    const summary = screen.getByLabelText("Resumo");
    expect(within(summary).getAllByText("US$ 0,00", { exact: false }).length).toBeGreaterThan(0);
    expect(screen.getByText("Nenhum trade neste filtro.")).toBeInTheDocument();
    expect(within(summary).queryByText(/vitórias/)).not.toBeInTheDocument();
    expect(within(summary).getAllByText("—").length).toBeGreaterThanOrEqual(3);
  });

  it("a captured trade only raises a banner; the figures change when the user refreshes", async () => {
    let calls = 0;
    server.use(http.get("*/api/v1/performance", () => HttpResponse.json(++calls > 1 ? performance({ totals: figures({ trade_count: 11, total_pnl: "999.00" }) }) : performance())));
    renderApp(<PerformancePage />);
    await screen.findByRole("table");
    const events = await stream();
    act(() => events.emit("trade.captured", { type: "trade.captured", at: "2026-10-23T14:05:00Z", trade_id: "00000000-0000-4000-8000-0000000000a9", account_id: "00000000-0000-4000-8000-0000000000a1" }));
    expect(await screen.findByText(/Há novos trades desde/)).toBeInTheDocument();
    expect(screen.queryByText("+US$ 999,00")).not.toBeInTheDocument();
    await userEvent.setup().click(screen.getByRole("button", { name: "Atualizar" }));
    await waitFor(() => expect(screen.getAllByText("+US$ 999,00").length).toBeGreaterThan(0));
  });

  it("below md every group is a card, the Total card first", async () => {
    setViewport(false);
    serve(performance());
    renderApp(<PerformancePage />);
    await screen.findByText("Strikes MNQ · Gatilhos");
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
    const cards = screen.getAllByRole("listitem");
    expect(within(cards[0] as HTMLElement).getByText("Total")).toBeInTheDocument();
  });

  it("shows an error with retry and keeps the filters", async () => {
    server.use(http.get("*/api/v1/performance", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    renderApp(<PerformancePage />);
    expect(await screen.findByText("Não foi possível carregar o desempenho.")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Tentar novamente" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "30 dias" })).toHaveAttribute("aria-pressed", "true");
  });

  it("keeps every dt and dd one level under the dl, as a description list requires (S2)", async () => {
    serve(performance());
    renderApp(<PerformancePage />);
    const summary = await screen.findByLabelText("Resumo");
    const list = summary.querySelector("dl") as HTMLElement;
    for (const term of Array.from(list.querySelectorAll("dt, dd"))) {
      const parent = term.parentElement as HTMLElement;
      expect(parent === list || parent.parentElement === list).toBe(true);
      expect(parent.tagName).toBe("DIV");
    }
    expect(list.querySelectorAll("dt")).toHaveLength(7);
  });

  it("defines the win rate on the NET result (S4)", async () => {
    serve(performance());
    renderApp(<PerformancePage />);
    await userEvent.setup().click(await screen.findByRole("button", { name: "Como é calculado: Taxa de acerto" }));
    expect(await screen.findByText(/resultado líquido positivo/)).toBeInTheDocument();
  });

  it("states the active sort in the table caption and follows a change (S6)", async () => {
    navigation.search = "sort=win_rate&order=asc";
    serve(performance());
    renderApp(<PerformancePage />);
    const table = await screen.findByRole("table");
    expect(table.querySelector("caption")).toHaveTextContent("ordenado por taxa de acerto (do menor ao maior)");
  });

  it("warns with the decided wording, singular and plural, only when trades have no commission data", async () => {
    serve(performance({ totals: figures({ uncommissioned_count: 1 }) }));
    const view = renderApp(<PerformancePage />);
    expect(await screen.findByText("1 trade sem comissão informada: nele, o líquido é igual ao bruto.")).toBeInTheDocument();
    expect(screen.queryByText(/inclui comissões estimadas/)).not.toBeInTheDocument();
    view.unmount();
    serve(performance({ totals: figures({ uncommissioned_count: 3 }) }));
    const second = renderApp(<PerformancePage />);
    expect(await screen.findByText("3 trades sem comissão informada: neles, o líquido é igual ao bruto.")).toBeInTheDocument();
    second.unmount();
    serve(performance({ totals: figures({ uncommissioned_count: 0 }) }));
    renderApp(<PerformancePage />);
    await screen.findByRole("table");
    expect(screen.queryByText(/sem comissão informada/)).not.toBeInTheDocument();
  });

  it("uses the English wording, singular and plural", async () => {
    serve(performance({ totals: figures({ uncommissioned_count: 1 }) }));
    const view = renderApp(<PerformancePage />, "en");
    expect(await screen.findByText("1 trade has no commission entered: for it, net equals gross.")).toBeInTheDocument();
    view.unmount();
    serve(performance({ totals: figures({ uncommissioned_count: 2 }) }));
    renderApp(<PerformancePage />, "en");
    expect(await screen.findByText("2 trades have no commission entered: for those, net equals gross.")).toBeInTheDocument();
  });
});
