// QA (Retina) — V1-S16 catalog and firm detail (AC-1 stances, AC-4 source+date, AC-5/6 not confirmed shown not chosen, AC-9 excluded plans, AC-12 commission), black box.
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { beforeEach, describe, expect, it } from "vitest";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
// render first: it installs the next/navigation mock before the feature loads.
import { FirmDetailPage, PresetsCatalog } from "@/features/presets";

const me = { id: "00000000-0000-4000-8000-000000000001", email: "t@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const uuid = (n: number) => `00000000-0000-4000-8000-${String(n).padStart(12, "0")}`;
const release = { version_number: 3, published_at: "2026-10-20T12:00:00Z", verified_on: "2026-10-18" };
const quote = (text = "Automated trading is permitted.", url = "https://firm.example/rules") => ({ text, language: "en", page_title: "Rules", url, accessed_at: "2026-10-18" });
const summary = (name: string, o: Record<string, unknown> = {}) => ({ id: uuid(name.length), slug: name.toLowerCase(), name, plan_names: ["Plan A", "Plan B", "Plan C"], stances: { automation: "allowed", copy: "not_confirmed", vps: "allowed", personal_device: "unknown" }, not_confirmed_count: 2, release, ...o });

describe("catalog (S16-01)", () => {
  beforeEach(() => server.use(http.get("*/api/v1/me", () => HttpResponse.json(me))));
  const firms = [summary("Topstep", { plan_names: ["Combine"], manual_only: { reason: quote("Orders must originate from your device.") }, not_confirmed_count: 0 }), summary("Apex"), summary("Bulenox", { plan_names: ["Master"], not_confirmed_count: 0 })];
  it("sorted A to Z, manual-only badge, '— ' when nothing is unconfirmed, '+N' plans, verification date", async () => {
    server.use(http.get("*/api/v1/firms", () => HttpResponse.json({ firms })));
    renderApp(<PresetsCatalog />);
    const rows = await screen.findAllByRole("row");
    const names = rows.slice(1).map((r) => within(r).getAllByRole("link")[0]?.textContent);
    expect(names).toEqual(["Apex", "Bulenox", "Topstep"]);
    expect(screen.getAllByText("Só acompanhamento manual").length).toBeGreaterThan(0);
    expect(screen.getAllByText(/18\/10\/2026/).length).toBeGreaterThan(0);
    expect(rows[1]).toHaveTextContent(/\+1/); // 3 plans: first two + 1
  });
  it("search matches the firm AND plan names; no match offers 'Limpar busca'", async () => {
    server.use(http.get("*/api/v1/firms", () => HttpResponse.json({ firms })));
    renderApp(<PresetsCatalog />);
    const u = userEvent.setup();
    const box = await screen.findByRole("searchbox");
    await u.type(box, "combine");
    await waitFor(() => expect(screen.getAllByRole("row")).toHaveLength(2));
    await u.clear(box); await u.type(box, "zzz");
    expect(await screen.findByText("Nenhuma firma encontrada.")).toBeInTheDocument();
    await u.click(screen.getByRole("button", { name: "Limpar busca" }));
    expect(box).toHaveValue("");
  });
  it("error keeps retry; an empty catalog says presets are not available yet", async () => {
    server.use(http.get("*/api/v1/firms", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    const first = renderApp(<PresetsCatalog />);
    expect(await screen.findByRole("button", { name: /tentar/i })).toBeInTheDocument();
    first.unmount();
    server.use(http.get("*/api/v1/firms", () => HttpResponse.json({ firms: [] })));
    renderApp(<PresetsCatalog />);
    expect(await screen.findByText("Os presets ainda não estão disponíveis.")).toBeInTheDocument();
  });
});

describe("firm detail (S16-02)", () => {
  const stance = (kind: string, firm_stance: string, quotes = [quote()]) => ({ id: uuid(30 + kind.length), kind, firm_stance, is_generic: false, firm_id: uuid(1), firm_name: "Apex", version_number: 3, published_at: release.published_at, quotes });
  const detail = (o: Record<string, unknown> = {}) => ({
    summary: summary("Apex"),
    stances: { automation: [stance("automation", "allowed")], copy: [stance("copy", "not_confirmed", [quote(), quote("Copy trading is not allowed.", "https://firm.example/faq")])], vps: { value: "allowed", quotes: [quote()] }, personal_device: { value: "unknown", quotes: [], research: { searched_on: "2026-10-18", sources: [] } } },
    platforms: [],
    plans: [{ plan_id: uuid(10), name: "Plan A", phases: [{ phase: "evaluation", sizes: [{ preset_id: uuid(40), preset_version_id: uuid(50), size_amount: "50000.00", size_label: "50K", platform_specific: false, not_confirmed_count: 0 }] }] }],
    excluded_plans: ["Legacy"],
    ...o,
  });
  const verified = (value: unknown) => ({ status: "verified", value, sources: [{ url: "https://firm.example/r", accessed_at: "2026-10-18", page_title: "Rules" }] });
  const presetVersion = (rules: Record<string, unknown> = {}) => ({ id: uuid(50), preset_id: uuid(40), firm_id: uuid(1), firm_name: "Apex", plan_id: uuid(10), plan_name: "Plan A", phase: "evaluation", size_amount: "50000.00", release, not_confirmed_count: 0, rule_sets: [{ rules: { max_position_qty: verified({ contracts: 60 }), drawdown: verified({ type: "trailing_eod", amount: "2500.00" }), trading_day_reset: verified({ local_time: "17:00", time_zone: "America/Chicago", published_text: "5:00 PM CT" }), evaluation_profit_target: verified({ amount: "3000.00" }), ...rules } }] });
  function serve(d: object = detail(), rules: Record<string, unknown> = {}) {
    server.use(
      http.get("*/api/v1/me", () => HttpResponse.json(me)),
      http.get("*/api/v1/firms/:id", () => HttpResponse.json(d)),
      http.get("*/api/v1/preset-versions/:id", () => HttpResponse.json(presetVersion(rules))),
      http.get("*/api/v1/firms/:id/releases", () => HttpResponse.json({ firm_id: uuid(1), releases: [
        { version_number: 2, published_at: "2026-09-01T00:00:00Z", verified_on: "2026-08-30", changes: [], vps: { value: "allowed", quotes: [] }, personal_device: { value: "unknown", quotes: [] }, rule_versions: [] },
        { version_number: 3, published_at: "2026-10-20T00:00:00Z", verified_on: "2026-10-18", changes: [{ kind: "stance_text_changed", stance: "automation", material: true, reason: { "pt-BR": "Nova exigência.", en: "New requirement." } }], vps: { value: "allowed", quotes: [] }, personal_device: { value: "unknown", quotes: [] }, rule_versions: [] },
      ] })),
    );
  }
  it("a quote link is only ever an https href (a javascript: URL in preset data never reaches href)", async () => {
    serve(detail({ stances: { ...detail().stances, vps: { value: "allowed", quotes: [quote("Servers are fine.", "javascript:alert(1)")] } } }));
    renderApp(<FirmDetailPage firmId={uuid(1)} />);
    await screen.findByRole("heading", { level: 1, name: "Apex" });
    const hrefs = Array.from(document.querySelectorAll("a[href]")).map((a) => a.getAttribute("href") ?? "");
    expect(hrefs.filter((h) => /^javascript:/i.test(h))).toEqual([]);
    expect(Array.from(document.querySelectorAll("blockquote")).map((b) => b.getAttribute("cite") ?? "").filter((c) => /^javascript:/i.test(c))).toEqual([]);
  });
  it("not_confirmed stance lists every conflicting quote; unknown says where and when we looked", async () => {
    serve(); renderApp(<FirmDetailPage firmId={uuid(1)} />);
    const copy = await screen.findByRole("region", { name: "Replicação" });
    expect(within(copy).getByText("As páginas oficiais divergem:")).toBeInTheDocument();
    expect(within(copy).getByText("Copy trading is not allowed.")).toBeInTheDocument();
    expect(within(screen.getByRole("region", { name: "Dispositivo pessoal" })).getByText(/Pesquisamos as páginas oficiais em 18\/10\/2026/)).toBeInTheDocument();
  });
  it("manual-only firm: explained on the detail with the verbatim reason", async () => {
    serve(detail({ summary: summary("Apex", { manual_only: { reason: quote("Orders must originate from your personal device.") } }) }));
    renderApp(<FirmDetailPage firmId={uuid(1)} />);
    expect(await screen.findByText("Orders must originate from your personal device.")).toBeInTheDocument();
    expect(screen.getAllByText("Só acompanhamento manual").length).toBeGreaterThan(0);
  });
  it("404 is 'not found' with no retry", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(me)), http.get("*/api/v1/firms/:id", () => HttpResponse.json({ error: { code: "common.not_found" } }, { status: 404 })));
    renderApp(<FirmDetailPage firmId={uuid(1)} />);
    expect(await screen.findByText(/não encontrad/i)).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /tentar/i })).not.toBeInTheDocument();
  });
  it("plans tab: exact money, evaluation target column only for evaluation, excluded plans from the Core", async () => {
    serve(); renderApp(<FirmDetailPage firmId={uuid(1)} />);
    await userEvent.setup().click(await screen.findByRole("tab", { name: "Planos e regras" }));
    const row = (await screen.findByRole("rowheader", { name: "50K" })).closest("tr")!;
    expect(await within(row).findByText(/US\$\s2\.500,00 · Trailing fim do dia/)).toBeInTheDocument();
    expect(row).toHaveTextContent(/US\$\s3\.000,00/);
    expect(screen.getByRole("columnheader", { name: "Meta da avaliação" })).toBeInTheDocument();
    expect(screen.getByText(/Legacy/)).toBeInTheDocument();
  });
  it("AC-12 spec 4.2: the 'firm does not publish the commission' note is shown only when it is true (no size publishes a commission)", async () => {
    serve(detail(), { commission_per_contract_side: verified({ amount: "1.2500" }) });
    renderApp(<FirmDetailPage firmId={uuid(1)} />);
    await userEvent.setup().click(await screen.findByRole("tab", { name: "Planos e regras" }));
    const row = (await screen.findByRole("rowheader", { name: "50K" })).closest("tr")!;
    await within(row).findByText(/1,25/);
    expect(screen.queryByText(/A firma não publica a comissão/)).not.toBeInTheDocument();
  });
  it("AC-9 spec 4.2: excluded plans note carries the link 'Cadastrar conta sem preset'", async () => {
    serve(); renderApp(<FirmDetailPage firmId={uuid(1)} />);
    await userEvent.setup().click(await screen.findByRole("tab", { name: "Planos e regras" }));
    await screen.findByRole("rowheader", { name: "50K" });
    expect(screen.getByRole("link", { name: "Cadastrar conta sem preset" })).toHaveAttribute("href", expect.stringContaining("/accounts/new"));
  });
  it("spec 4.2: reset shows both zones (firm zone and the user's) in a tooltip/title", async () => {
    serve(detail(), {}); renderApp(<FirmDetailPage firmId={uuid(1)} />);
    await userEvent.setup().click(await screen.findByRole("tab", { name: "Planos e regras" }));
    const row = (await screen.findByRole("rowheader", { name: "50K" })).closest("tr")!;
    await within(row).findByText(/5:00 PM CT/);
    expect(row.innerHTML).toMatch(/GMT|UTC/);
  });
  it("versions tab: newest first, material change badge and the reason in the user's language", async () => {
    serve(); renderApp(<FirmDetailPage firmId={uuid(1)} />);
    await userEvent.setup().click(await screen.findByRole("tab", { name: "Versões" }));
    const items = await screen.findAllByRole("heading", { level: 3 });
    expect(items.map((h) => h.textContent)).toEqual(["Versão 3", "Versão 2"]);
    expect(screen.getByText("Mudança material")).toBeInTheDocument();
    expect(screen.getByText(/Nova exigência\./)).toBeInTheDocument();
  });
  it("spec 4.3: each version offers 'Ver textos desta versão' (compare what an account was shown)", async () => {
    serve(); renderApp(<FirmDetailPage firmId={uuid(1)} />);
    await userEvent.setup().click(await screen.findByRole("tab", { name: "Versões" }));
    await screen.findAllByRole("heading", { level: 3 });
    expect(screen.getAllByText("Ver textos desta versão").length).toBeGreaterThan(0);
  });
});
