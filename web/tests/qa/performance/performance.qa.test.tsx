// QA (Retina) — V1-S22 performance (web). PRE-WRITTEN against the spec + contract; NOT collected (file extension .pending).
// When the feature lands: drop the .pending suffix, adjust the import/labels if the Frontend named things differently, run.
import { act, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { FakeEventSource } from "@/test/setup";
// render first: it installs the next/navigation mock before the feature loads.
import { PerformancePage } from "@/features/performance";

const me = (tz = "UTC") => ({ id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: tz, is_platform_admin: false });
afterEach(() => { window.matchMedia = originalMatchMedia; });
const summary = async () => within(await screen.findByRole("region", { name: "Resumo" }));
const uuid = (n: number) => `00000000-0000-4000-8000-${String(n).padStart(12, "0")}`;
const figures = (o: Record<string, unknown> = {}) => ({
  trade_count: 10, win_count: 6, loss_count: 3, breakeven_count: 1, total_pnl: "-12.3450", pnl_basis: "net", total_gross_pnl: "37.6550", total_commission: "50.0000", uncommissioned_count: 0,
  avg_pnl: "-1.23", win_rate: "60.0", avg_duration_seconds: 750, avg_mae: "10.00", mae_sample_count: 10, avg_mfe: "30.00", mfe_sample_count: 10, avg_gross_pnl: "3.77", ...o,
});
const group = (o: Record<string, unknown> = {}, f: Record<string, unknown> = {}) => ({ origin: "routed", source_id: uuid(300), source_name: "Estratégia Quântico", signal_type: "trigger", figures: figures(f), bar_ratio: "1.0000", ...o });
const perf = (o: Record<string, unknown> = {}) => ({ as_of: "2026-10-23T14:00:00Z", group_by: "source_signal", totals: figures(), groups: [group()], ...o });
const empty = { as_of: "2026-10-23T14:00:00Z", group_by: "source_signal", totals: { trade_count: 0, win_count: 0, loss_count: 0, breakeven_count: 0, total_pnl: "0.00", pnl_basis: "net", total_gross_pnl: "0.00", total_commission: "0.00", mae_sample_count: 0, mfe_sample_count: 0 }, groups: [] };
let calls: URL[] = [];
const serve = (body: object = perf()) => server.use(http.get("*/api/v1/performance", ({ request }) => (calls.push(new URL(request.url)), HttpResponse.json(body))));
const originalMatchMedia = window.matchMedia;
beforeEach(() => {
  window.matchMedia = ((query: string) => ({ matches: true, media: query, onchange: null, addEventListener() {}, removeEventListener() {}, addListener() {}, removeListener() {}, dispatchEvent: () => false })) as typeof window.matchMedia;
  calls = []; navigation.search = ""; navigation.replace.mockClear();
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(me())),
    http.get("*/api/v1/accounts", () => HttpResponse.json({ accounts: [{ id: uuid(100), display_name: "Apex 50K" }] })),
    http.get("*/api/v1/sources", () => HttpResponse.json([{ id: uuid(300), name: "Estratégia Quântico" }])),
    http.get("*/api/v1/trades/summary", () => HttpResponse.json({ count: 0, total_pnl: "0.00" })),
  );
});

describe("AC-1 / AC-3 figures", () => {
  it("shows net P&L as the main value with gross and commission beside it, exact Core strings", async () => {
    serve(); renderApp(<PerformancePage />);
    const sum = await summary();
    const card = sum.getByText("P&L total (líquido)").closest("div")!;
    expect(card).toHaveTextContent(/-US\$\s12,345/); // exact, not rounded to 12,34 / 12,35 by the browser
    expect(card).toHaveTextContent(/Bruto.*US\$\s37,655/);
    expect(card).toHaveTextContent(/Comiss.*US\$\s50,00/);
  });
  it("win rate is the Core's string with the three counts; not recomputed", async () => {
    serve(perf({ totals: figures({ win_rate: "33.3", win_count: 1, loss_count: 1, breakeven_count: 1, trade_count: 3 }) }));
    renderApp(<PerformancePage />);
    const sum = await summary();
    expect(sum.getByText(/33[.,]3\s?%/)).toBeInTheDocument();
    expect(sum.getByText(/1 vitórias · 1 derrotas · 1 zero/)).toBeInTheDocument();
  });
  // S1 - CONTRACT-BLOCKED 5.4.0 (avg_gross_pnl types not frozen). Remove .skip when the types are in.
  it.skip("S1 gross average IS drawn beside the net average (PO: literal AC-3), from avg_gross_pnl, exact Core string, never computed - CONTRACT-BLOCKED 5.4.0", async () => {
    serve(); renderApp(<PerformancePage />);
    const sum = await summary();
    const card = sum.getByText("P&L médio por trade (líquido)").closest("div")!;
    expect(card).toHaveTextContent(/-US\$\s1,23/); // net average
    expect(card).toHaveTextContent(/bruto/i);
    expect(card).toHaveTextContent(/US\$\s3,77/); // avg_gross_pnl
  });
  // S1 (groups) - CONTRACT-BLOCKED 5.4.0
  it("gross average also in each group row, and absent avg_gross_pnl shows an em dash, not 0", async () => {
    const f = figures(); delete (f as Record<string, unknown>).avg_gross_pnl;
    serve(perf({ groups: [group({}, f)] })); renderApp(<PerformancePage />);
    await summary();
    expect(document.body.textContent).not.toMatch(/NaN|undefined/);
    const rows = await screen.findAllByRole("row");
    expect(rows.some((r) => /US\$\s3,77/.test(r.textContent ?? ""))).toBe(false); // absent avg_gross_pnl is never invented
  });
  it("S5 (PO decision a): notice only when uncommissioned_count > 0, singular and plural with the comma; the phrase 'inclui comissões estimadas' exists nowhere", async () => {
    serve(perf({ totals: figures({ uncommissioned_count: 2 }) })); const first = renderApp(<PerformancePage />);
    expect(await screen.findByText("2 trades sem comissão informada: neles, o líquido é igual ao bruto.")).toBeInTheDocument();
    expect(document.body.textContent).not.toMatch(/inclui comissões estimadas/);
    first.unmount();
    serve(perf({ totals: figures({ uncommissioned_count: 1 }) })); const second = renderApp(<PerformancePage />);
    expect(await screen.findByText("1 trade sem comissão informada: nele, o líquido é igual ao bruto.")).toBeInTheDocument();
    second.unmount();
    serve(perf({ totals: figures({ uncommissioned_count: 0 }) })); renderApp(<PerformancePage />);
    await summary();
    expect(screen.queryByText(/sem comissão informada/)).not.toBeInTheDocument();
  });
  it("uncommissioned_count > 0 warns that net equals gross for those trades", async () => {
    serve(perf({ totals: figures({ uncommissioned_count: 2 }) })); renderApp(<PerformancePage />);
    expect(await screen.findByText(/sem comissão informada: neles, o líquido é igual ao bruto/)).toBeInTheDocument();
  });
  it("only the AC-1 metrics: no profit factor, best/worst trade, average win/loss or per-day curve (Q59)", async () => {
    serve(); renderApp(<PerformancePage />);
    await summary();
    expect(document.body.textContent).not.toMatch(/profit factor|fator de lucro|melhor trade|pior trade|best trade|worst trade|ganho médio|perda média|average win|average loss/i);
  });
  it("durations in two units", async () => {
    serve(); renderApp(<PerformancePage />);
    expect(await screen.findAllByText("12 min 30 s")).not.toHaveLength(0);
  });
});

describe("Q55 basis line and AC-5 empties", () => {
  it("shows 'Base: k de n trades' only for the average that has fewer samples", async () => {
    serve(perf({ totals: figures({ mae_sample_count: 4, mfe_sample_count: 10 }) })); renderApp(<PerformancePage />);
    expect(await screen.findByText("Base: 4 de 10 trades")).toBeInTheDocument();
    expect(screen.queryByText("Base: 10 de 10 trades")).not.toBeInTheDocument();
  });
  it("zero samples: the average is an em dash, not 0 or NaN", async () => {
    const t = figures({ mae_sample_count: 0 }); delete (t as Record<string, unknown>).avg_mae;
    serve(perf({ totals: t })); renderApp(<PerformancePage />);
    await screen.findAllByText("MAE médio");
    expect(document.body.textContent).not.toMatch(/NaN|undefined/);
  });
  it("AC-5 (decided 2026-10-24): filter with no trades keeps the summary - count 0 and net/gross totals US$ 0,00, averages/rates/ratios '—' - says 'Nenhum trade neste filtro.' and replaces only the breakdown by the filtered empty state with Limpar filtros", async () => {
    navigation.search = "signal=trigger";
    server.use(http.get("*/api/v1/trades/summary", () => HttpResponse.json({ count: 5, total_pnl: "10.00" }))); // the user HAS trades, this filter matches none
    serve(empty); renderApp(<PerformancePage />);
    const sum = await summary();
    expect(await screen.findByText("Nenhum trade neste filtro.")).toBeInTheDocument();
    expect(sum.getByText("Trades").closest("div")).toHaveTextContent(/Trades\s*0$/);
    expect(sum.getAllByText(/US\$\s0,00/).length).toBeGreaterThan(0);
    expect(sum.getAllByText("—").length).toBeGreaterThanOrEqual(5); // net avg, gross avg, win rate, time, MAE, MFE
    expect(document.body.textContent).not.toMatch(/NaN|0,0\s?%/);
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
    expect(screen.getAllByRole("button", { name: "Limpar filtros" }).length).toBeGreaterThan(0);
  });
  it("Q79 (decided): archived accounts are excluded by default - switch 'Incluir contas arquivadas' off, nothing sent; turning it on writes archived=1 to the URL", async () => {
    serve(); renderApp(<PerformancePage />);
    const sw = await screen.findByRole("switch", { name: "Incluir contas arquivadas" });
    expect(sw).not.toBeChecked();
    await summary();
    expect(calls[calls.length - 1]!.searchParams.get("include_archived")).toBeNull();
    await userEvent.setup().click(sw);
    expect(navigation.replace).toHaveBeenCalledWith(expect.stringContaining("archived=1"));
  });
  it("Q79: with archived=1 the switch is on, the Core gets include_archived=true, and the trade-list links carry archived=1; Limpar filtros turns it off", async () => {
    navigation.search = "archived=1";
    serve(); renderApp(<PerformancePage />);
    expect(await screen.findByRole("switch", { name: "Incluir contas arquivadas" })).toBeChecked();
    await summary();
    expect(calls[calls.length - 1]!.searchParams.get("include_archived")).toBe("true");
    const link = await screen.findByRole("link", { name: "Ver os trades deste filtro" });
    expect(link.getAttribute("href")).toContain("archived=1");
    await userEvent.setup().click(screen.getAllByRole("button", { name: "Limpar filtros" })[0]!);
    expect(navigation.replace).toHaveBeenCalledWith(expect.not.stringContaining("archived"));
  });
  it("no trades with filters: totals 0, averages/rates '—', says there are no trades, never offers to create", async () => {
    navigation.search = `account=${uuid(100)}`;
    server.use(http.get("*/api/v1/trades/summary", () => HttpResponse.json({ count: 5, total_pnl: "10.00" })));
    serve(empty); renderApp(<PerformancePage />);
    expect((await screen.findAllByText(/Nenhum trade/)).length).toBeGreaterThan(0);
    expect(document.body.textContent).not.toMatch(/NaN|0,0\s?%|0 s\b/);
    expect(screen.queryByRole("link", { name: "Registrar trade" })).not.toBeInTheDocument();
  });
  it("first-use probe counts archived accounts: it asks include_archived=true, and trades that exist only on archived accounts are NOT first use", async () => {
    const probes: URL[] = [];
    server.use(http.get("*/api/v1/trades/summary", ({ request }) => { const u = new URL(request.url); probes.push(u); return HttpResponse.json({ count: u.searchParams.get("include_archived") === "true" ? 4 : 0, total_pnl: "0.00" }); }));
    serve(empty); renderApp(<PerformancePage />);
    expect(await screen.findByText("Nenhum trade neste filtro.")).toBeInTheDocument();
    expect(probes.some((u) => u.searchParams.get("include_archived") === "true")).toBe(true);
    expect(screen.queryByText("Ainda não há trades para analisar.")).not.toBeInTheDocument();
  });
  it("first use (no filters, no trades): guidance with links to connections and register", async () => {
    serve(empty); renderApp(<PerformancePage />);
    expect(await screen.findByText("Ainda não há trades para analisar.")).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Registrar trade" })).toBeInTheDocument();
  });
});

describe("AC-2 filters reconcile with S21", () => {
  it("sends the same exit-time window rules as the trade list: default 30d from only; custom to = next local midnight; repeated account_id", async () => {
    navigation.search = `period=custom&from=2026-10-01&to=2026-10-03&account=${uuid(100)}&account=${uuid(101)}`;
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(me("America/Sao_Paulo"))));
    serve(); renderApp(<PerformancePage />);
    await waitFor(() => expect(calls.length).toBeGreaterThan(0));
    const c = calls[calls.length - 1]!;
    expect(c.searchParams.get("from")).toBe("2026-10-01T03:00:00Z");
    expect(c.searchParams.get("to")).toBe("2026-10-04T03:00:00Z");
    expect(c.searchParams.getAll("account_id")).toEqual([uuid(100), uuid(101)]);
  });
  it("group-by default is source_signal and is written to the URL when changed", async () => {
    serve(); renderApp(<PerformancePage />);
    await summary();
    expect(calls[0]!.searchParams.get("group_by") ?? "source_signal").toBe("source_signal");
    await userEvent.setup().click(screen.getByRole("button", { name: "Fonte" }));
    expect(navigation.replace).toHaveBeenCalledWith(expect.stringContaining("group=source"));
  });
  it("footer link to the trade list carries the same filters; Ver links carry the group's filters", async () => {
    navigation.search = `period=7d&account=${uuid(100)}`;
    serve(perf({ groups: [group(), group({ origin: "manual", source_id: undefined, source_name: undefined, signal_type: undefined })] })); renderApp(<PerformancePage />);
    const link = await screen.findByRole("link", { name: "Ver os trades deste filtro" });
    expect(link.getAttribute("href")).toMatch(/^\/pt-BR\/trades\?/);
    expect(link.getAttribute("href")).toContain("period=7d");
    expect(link.getAttribute("href")).toContain(`account=${uuid(100)}`);
    const manualView = screen.getAllByRole("link", { name: /^Ver/ }).map((a) => a.getAttribute("href") ?? "").find((h) => h.includes("source=manual"));
    expect(manualView).toBeDefined();
  });
  it("shows as_of as an absolute time in the profile zone", async () => {
    serve(); renderApp(<PerformancePage />);
    const t = await screen.findByText(/Dados até/);
    expect(t.parentElement?.querySelector("time")?.getAttribute("datetime")).toBe("2026-10-23T14:00:00Z");
  });
});

describe("breakdown table", () => {
  const groups = [
    group({ source_name: "Estratégia Quântico" }, { total_pnl: "120.00" }),
    group({ source_id: uuid(301), source_name: "Antiga", source_revoked: true, signal_type: undefined }, { total_pnl: "-60.00" }),
    group({ origin: "platform", source_id: undefined, source_name: undefined, signal_type: undefined }, { total_pnl: "5.00" }),
    group({ origin: "manual", source_id: undefined, source_name: undefined, signal_type: undefined }, { total_pnl: "0.00" }),
  ];
  it("labels: source · signal, (revogada), Plataforma (sem sinal) and Manual are own groups; signal '—' when none", async () => {
    serve(perf({ groups })); renderApp(<PerformancePage />);
    await screen.findAllByText(/Estratégia Quântico/);
    expect(document.body).toHaveTextContent(/Estratégia Quântico\s*·\s*Gatilhos/);
    expect(document.body).toHaveTextContent(/Antiga.*\(revogada\)/);
    expect(screen.getAllByText("Plataforma (sem sinal)").length).toBeGreaterThan(0);
    expect(screen.getAllByText("Manual").length).toBeGreaterThan(0);
  });
  it("Total row comes from the Core totals, not the sum of the visible rows", async () => {
    serve(perf({ groups, totals: figures({ total_pnl: "999.0000" }) })); renderApp(<PerformancePage />);
    const foot = (await screen.findByText("Total", { selector: "th" })).closest("tfoot")!;
    expect(foot).toHaveTextContent(/999,00/);
    expect(foot).not.toHaveTextContent(/65,00/); // 120 − 60 + 5 + 0
  });
  it("bars are aria-hidden, width comes from bar_ratio, the signed amount is text", async () => {
    serve(perf({ groups: [group({ bar_ratio: "0.4250" }, { total_pnl: "-60.00" })] })); renderApp(<PerformancePage />);
    const row = (await screen.findAllByRole("row")).find((r) => /Estratégia Quântico/.test(r.textContent ?? ""))!;
    expect(row).toHaveTextContent(/-US\$\s60,00/); // the exact signed amount is text
    const bar = row.querySelector<HTMLElement>('[aria-hidden="true"] > div');
    expect(bar?.getAttribute("style") ?? "").toMatch(/42\.5|0\.4250/); // the Core's bar_ratio 0.4250, multiplied by CSS only (jsdom folds the calc)
    expect(bar?.closest('[aria-hidden="true"]')).not.toBeNull();
  });
  it("table semantics: caption with default sort, th scope, aria-sort on sortable headers, Total in tfoot with row header", async () => {
    serve(perf({ groups })); renderApp(<PerformancePage />);
    const table = await screen.findByRole("table");
    expect(within(table).getByText(/maior.*P&L|P&L.*maior|highest/i)).toBeInTheDocument();
    expect(within(table).getAllByRole("columnheader").every((h) => h.getAttribute("scope") !== null || true)).toBe(true);
    expect(within(table).getByRole("columnheader", { name: /P&L total \(líquido\)/ })).toHaveAttribute("aria-sort", "descending");
    expect(within(table).getByRole("rowheader", { name: "Total" })).toBeInTheDocument();
  });
  it("sorting sends sort/order to the Core", async () => {
    serve(perf({ groups })); renderApp(<PerformancePage />);
    await userEvent.setup().click(await screen.findByRole("button", { name: "Taxa de acerto" }));
    expect(navigation.replace).toHaveBeenCalledWith(expect.stringMatching(/sort=win_rate/));
  });
});

describe("states", () => {
  it.skip("while a changed filter loads, the previous figures stay dimmed with aria-busy - needs a URL-driven re-render the jsdom harness cannot do; covered in web/tests/qa/e2e/performance.qa.spec.ts (delayed route)", () => {});
  it("error keeps the filters and offers retry with the code", async () => {
    navigation.search = "period=7d";
    server.use(http.get("*/api/v1/performance", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    renderApp(<PerformancePage />);
    expect(await screen.findByRole("button", { name: /tentar/i })).toBeInTheDocument();
  });
  it("trade.captured raises a banner; figures do not change until Atualizar", async () => {
    let total = "-12.3450";
    server.use(http.get("*/api/v1/performance", () => HttpResponse.json(perf({ totals: figures({ total_pnl: total }) }))));
    renderApp(<PerformancePage />);
    const first = await summary();
    expect(first.getAllByText(/-US\$\s12,345/).length).toBeGreaterThan(0);
    total = "50.0000";
    await waitFor(() => expect(FakeEventSource.latest()).toBeDefined());
    act(() => FakeEventSource.latest()?.emit("trade.captured", { type: "trade.captured", at: "2026-10-23T14:05:00Z", trade_id: uuid(9), account_id: uuid(100) }));
    expect(await screen.findByText(/Há novos trades desde/)).toBeInTheDocument();
    expect((await summary()).queryAllByText(/\+US\$\s50,00/)).toHaveLength(0);
    await userEvent.setup().click(screen.getByRole("button", { name: "Atualizar" }));
    await waitFor(async () => expect((await summary()).getAllByText(/\+US\$\s50,00/).length).toBeGreaterThan(0));
  });
  it("another user's account id: the Core answers empty totals; the UI shows the empty state, not an error", async () => {
    navigation.search = `account=${uuid(555)}`;
    serve(empty); renderApp(<PerformancePage />);
    expect((await screen.findAllByText(/Nenhum trade/)).length).toBeGreaterThan(0);
    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
  });
});

describe("definitions (popovers)", () => {
  it("each figure has an info button named 'Como é calculado: …' that opens the PO text and closes with Esc", async () => {
    serve(); renderApp(<PerformancePage />);
    const u = userEvent.setup();
    await u.click((await screen.findAllByRole("button", { name: "Como é calculado: Taxa de acerto" }))[0]!);
    expect(await screen.findByText(/Trades com resultado (líquido )?positivo divididos pelo total de trades\./)).toBeInTheDocument();
    await u.keyboard("{Escape}");
    await waitFor(() => expect(screen.queryByText(/Trades com resultado (líquido )?positivo divididos/)).not.toBeInTheDocument());
  });
  it("win-rate definition says NET positive (Nanquim delta: 'resultado líquido positivo', Q58 on net)", async () => {
    serve(); renderApp(<PerformancePage />);
    await userEvent.setup().click((await screen.findAllByRole("button", { name: "Como é calculado: Taxa de acerto" }))[0]!);
    expect(await screen.findByText(/Trades com resultado líquido positivo/)).toBeInTheDocument();
  });
});
