// QA (Retina) — V1-S16 edit page with the preset picker (modes keep / pick another / free text), Q41 attach notice, opt-in rules of the account's preset, and the S13-2 slots in the S5 list/detail. Public indexes only.
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { beforeEach, describe, expect, it } from "vitest";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
// render first: it installs the next/navigation mock before the features load.
import { AccountDetail, AccountFormPage, AccountsList } from "@/features/accounts";
import { PresetPicker, usePresetSelection } from "@/features/presets";

const me = { id: "00000000-0000-4000-8000-000000000001", email: "t@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const uuid = (n: number) => `00000000-0000-4000-8000-${String(n).padStart(12, "0")}`;
const acc = uuid(500), presetA = uuid(50), presetB = uuid(51), ruleAuto = uuid(60), ruleCopy = uuid(61);
const release = { version_number: 3, published_at: "2026-10-20T12:00:00Z", verified_on: "2026-10-18" };
const src = { url: "https://firm.example/rules", accessed_at: "2026-10-18", page_title: "Rules" };
const verified = (value: unknown) => ({ status: "verified", value, sources: [src] });
const optIn = (kind: string) => ({ kind, state: "not_required" });
const account = (o: Record<string, unknown> = {}) => ({
  id: acc, kind: "prop", display_name: "Apex 50K", size_amount: "50000.00", currency: "USD", multiplier: 1, max_position_qty: 5, lifecycle_state: "active",
  firm_name: "Apex", plan_name: "Plan A", phase: "evaluation", preset_version_id: presetA, firm_platform: "rithmic",
  opt_ins: { automation: optIn("automation"), copy: optIn("copy") }, has_open_position: false, row_version: 7, created_at: "2026-10-01T12:00:00Z", updated_at: "2026-10-01T12:00:00Z", ...o,
});
const rule = (kind: string, id: string) => ({ id, kind, firm_stance: "allowed", is_generic: false, firm_name: "Apex", version_number: 1, published_at: "2026-10-01T00:00:00Z", quotes: [] });
const firmSummary = { id: uuid(1), slug: "apex", name: "Apex", plan_names: ["Plan A", "Plan B"], stances: { automation: "allowed", copy: "allowed", vps: "allowed", personal_device: "unknown" }, not_confirmed_count: 0, release };
const firmDetail = {
  summary: firmSummary,
  stances: { automation: [], copy: [], vps: { value: "allowed", quotes: [] }, personal_device: { value: "unknown", quotes: [] } },
  platforms: [], excluded_plans: [],
  plans: [
    { plan_id: uuid(10), name: "Plan A", phases: [{ phase: "evaluation", sizes: [{ preset_id: uuid(40), preset_version_id: presetA, size_amount: "50000.00", size_label: "50K", platform_specific: false, not_confirmed_count: 0 }] }] },
    { plan_id: uuid(11), name: "Plan B", phases: [{ phase: "evaluation", sizes: [{ preset_id: uuid(41), preset_version_id: presetB, size_amount: "25000.00", size_label: "25K", platform_specific: false, not_confirmed_count: 0 }] }] },
  ],
};
const versionBody = (id: string, size: string) => ({ id, preset_id: uuid(40), firm_id: uuid(1), firm_name: "Apex", plan_id: uuid(10), plan_name: "Plan", phase: "evaluation", size_amount: size, release, not_confirmed_count: 0, rule_sets: [{ rules: { max_position_qty: verified({ contracts: 60 }), drawdown: verified({ type: "trailing_eod", amount: "2500.00" }), daily_loss_limit: verified({ kind: "none" }), trading_day_reset: verified({ local_time: "17:00", time_zone: "America/Chicago", published_text: "5:00 PM CT" }) } }] });

function Edit() {
  const selection = usePresetSelection();
  return <AccountFormPage accountId={acc} preset={{ outcome: selection.outcome, render: ({ switchToManual, error }) => <PresetPicker selection={selection} onManual={switchToManual} error={error} /> }} />;
}
let puts: Record<string, unknown>[] = [];
let ruleQueries: URL[] = [];
function serve(a: object = account()) {
  puts = []; ruleQueries = [];
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(me)),
    http.get("*/api/v1/connections", () => HttpResponse.json([])),
    http.get("*/api/v1/accounts/:id", () => HttpResponse.json(a)),
    http.put("*/api/v1/accounts/:id", async ({ request }) => (puts.push((await request.json()) as Record<string, unknown>), HttpResponse.json(account({ row_version: 8 })))),
    http.get("*/api/v1/opt-in-rules", ({ request }) => (ruleQueries.push(new URL(request.url)), HttpResponse.json({ automation: rule("automation", ruleAuto), copy: rule("copy", ruleCopy) }))),
    http.get("*/api/v1/firms", () => HttpResponse.json({ firms: [firmSummary] })),
    http.get("*/api/v1/firms/:id", () => HttpResponse.json(firmDetail)),
    http.get("*/api/v1/preset-versions/:id", ({ params }) => HttpResponse.json(versionBody(String(params.id), params.id === presetB ? "25000.00" : "50000.00"))),
  );
}
async function choose(u: ReturnType<typeof userEvent.setup>, label: string | RegExp, option: RegExp | string) {
  await u.click(screen.getByRole("combobox", { name: label }));
  await u.click(await screen.findByRole("option", { name: option }));
}
beforeEach(() => server.use(http.get("*/api/v1/me", () => HttpResponse.json(me))));

describe("edit page modes", () => {
  it("keep: renaming sends the account's preset_version_id + firm_platform and NOTHING manual (no firm/plan/phase/size/rule_values); the opt-in rules asked are the preset's", async () => {
    serve(); renderApp(<Edit />);
    const u = userEvent.setup();
    const name = await screen.findByLabelText("Nome");
    await u.clear(name); await u.type(name, "Renomeada");
    await u.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(puts[0]).toMatchObject({ row_version: 7, display_name: "Renomeada", preset_version_id: presetA, firm_platform: "rithmic" });
    for (const manual of ["firm_name", "plan_name", "phase", "size_amount", "rule_values"]) expect(puts[0]).not.toHaveProperty(manual);
    expect(ruleQueries.some((q) => q.searchParams.get("preset_version_id") === presetA)).toBe(true); // not the generic rules
  });
  it("pick another: a different selection sends the NEW preset_version_id and no manual fields", async () => {
    serve(); renderApp(<Edit />);
    const u = userEvent.setup();
    await u.click(await screen.findByRole("button", { name: "Trocar o preset" }));
    await choose(u, /^Firma/, /Apex/); await choose(u, "Plano", "Plan B"); await choose(u, "Fase", "Avaliação"); await choose(u, "Tamanho", "25K");
    await u.click(await screen.findByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(puts[0]).toMatchObject({ preset_version_id: presetB });
    expect(puts[0]).not.toHaveProperty("firm_name");
  });
  it("cancelling the change returns to the account's own preset and size (nothing from the half-picked preset leaks into the body)", async () => {
    serve(); renderApp(<Edit />);
    const u = userEvent.setup();
    await u.click(await screen.findByRole("button", { name: "Trocar o preset" }));
    await choose(u, /^Firma/, /Apex/); await choose(u, "Plano", "Plan B"); await choose(u, "Fase", "Avaliação"); await choose(u, "Tamanho", "25K");
    await u.click(screen.getByRole("button", { name: "Manter como está" }));
    expect(await screen.findByText(/Preset atual: Apex · Plan A · evaluation/)).toBeInTheDocument();
    await u.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(puts[0]).toMatchObject({ preset_version_id: presetA });
    expect(JSON.stringify(puts[0])).not.toContain(presetB);
  });
  it("a selection that is not complete cannot be saved: the 'preset required' message is shown and nothing is sent", async () => {
    serve(); renderApp(<Edit />);
    const u = userEvent.setup();
    await u.click(await screen.findByRole("button", { name: "Trocar o preset" }));
    await u.click(screen.getByRole("button", { name: "Salvar conta" }));
    expect(await screen.findByText("Escolha um preset e resolva os valores obrigatórios.")).toBeInTheDocument();
    expect(puts).toHaveLength(0);
  });
  it("free text: 'Cadastrar sem preset' sends the manual fields and no preset keys", async () => {
    serve(); renderApp(<Edit />);
    const u = userEvent.setup();
    await u.click(await screen.findByRole("button", { name: "Cadastrar sem preset" }));
    const firm = await screen.findByLabelText("Firma");
    await u.clear(firm); await u.type(firm, "Outra firma");
    expect(firm).toHaveValue("Outra firma");
    await u.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(puts[0]).toMatchObject({ firm_name: "Outra firma" });
    expect(puts[0]).not.toHaveProperty("preset_version_id");
  });
  it("free text from the start (account without a preset): typing in Firma sticks and is sent", async () => {
    serve(account({ preset_version_id: undefined, firm_platform: undefined })); renderApp(<Edit />);
    const u = userEvent.setup();
    const firm = await screen.findByLabelText("Firma");
    await u.clear(firm); await u.type(firm, "Outra firma");
    expect(firm).toHaveValue("Outra firma");
    await u.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(puts).toHaveLength(1));
    expect(puts[0]).toMatchObject({ firm_name: "Outra firma" });
  });
  it("free text shows ONE Firma, ONE Plano and ONE Fase field (no duplicated inputs/ids), for an account without a preset and after 'Cadastrar sem preset'", async () => {
    serve(account({ preset_version_id: undefined, firm_platform: undefined })); const first = renderApp(<Edit />);
    await screen.findByLabelText("Nome");
    for (const name of ["firmName", "planName", "phase"]) expect(document.querySelectorAll(`input[name="${name}"]`)).toHaveLength(1);
    const ids = Array.from(document.querySelectorAll("[id]")).map((n) => n.id).filter((id) => id !== "");
    expect(new Set(ids).size).toBe(ids.length); // duplicate ids break label association (WCAG 4.1.1/1.3.1)
    first.unmount();
    serve(); renderApp(<Edit />);
    await userEvent.setup().click(await screen.findByRole("button", { name: "Cadastrar sem preset" }));
    await screen.findByLabelText("Firma");
    for (const name of ["firmName", "planName", "phase"]) expect(document.querySelectorAll(`input[name="${name}"]`)).toHaveLength(1);
  });
  it("Q41: attaching a preset to an account without one shows the final notice; an account that already had one does not", async () => {
    serve(account({ preset_version_id: undefined, firm_platform: undefined })); const first = renderApp(<Edit />);
    const u = userEvent.setup();
    await u.click(await screen.findByRole("button", { name: "Escolher um preset da firma" }));
    expect(await screen.findByText("Ao anexar o preset, as ordens automatizadas desta conta ficam suspensas até você revisar e escolher.")).toBeInTheDocument();
    first.unmount();
    serve(); renderApp(<Edit />);
    await u.click(await screen.findByRole("button", { name: "Trocar o preset" }));
    await screen.findByRole("combobox", { name: /^Firma/ });
    expect(screen.queryByText(/ficam suspensas até você revisar/)).not.toBeInTheDocument();
  });
  it("a real account has no preset picker at all", async () => {
    serve(account({ kind: "real", firm_name: undefined, plan_name: undefined, phase: undefined, preset_version_id: undefined, firm_platform: undefined }));
    renderApp(<Edit />);
    await screen.findByLabelText("Nome");
    expect(screen.queryByRole("button", { name: /preset/i })).not.toBeInTheDocument();
  });
});

describe("S13-2 slots in the S5 screens (no risk logic, just places)", () => {
  it("list: the risk column exists only when the page supplies it, header and one cell per account; absent = no column", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(me)), http.get("*/api/v1/accounts", () => HttpResponse.json({ accounts: [account()] })), http.get("*/api/v1/connections", () => HttpResponse.json([])));
    const original = window.matchMedia;
    try {
    window.matchMedia = ((query: string) => ({ matches: true, media: query, onchange: null, addEventListener() {}, removeEventListener() {}, addListener() {}, removeListener() {}, dispatchEvent: () => false })) as typeof window.matchMedia;
    const first = renderApp(<AccountsList />);
    await screen.findAllByText("Apex 50K");
    expect(screen.queryByRole("columnheader", { name: "Risco hoje" })).not.toBeInTheDocument();
    first.unmount();
    renderApp(<AccountsList riskColumn={{ header: "Risco hoje", cell: (a) => <span>{`slot:${a.id.slice(-3)}`}</span> }} />);
    expect(await screen.findByRole("columnheader", { name: "Risco hoje" })).toBeInTheDocument();
    expect(screen.getAllByText("slot:500").length).toBeGreaterThan(0);
    } finally {
      window.matchMedia = original; // restored even when an assertion fails
    }
  });
  it("detail: alertSlot sits right under the header, riskSection after 'Dados da conta'; an empty/false slot renders no wrapper", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(me)), http.get("*/api/v1/accounts/:id", () => HttpResponse.json(account())), http.get("*/api/v1/connections", () => HttpResponse.json([])), http.get("*/api/v1/opt-in-rules", () => HttpResponse.json({ automation: rule("automation", ruleAuto), copy: rule("copy", ruleCopy) })));
    const first = renderApp(<AccountDetail accountId={acc} alertSlot={<p>ALERT-SLOT</p>} riskSection={<section aria-label="risk-slot">RISK-SLOT</section>} />);
    const alertEl = await screen.findByText("ALERT-SLOT");
    const risk = screen.getByLabelText("risk-slot");
    const data = (await screen.findAllByText("Dados")).find((n) => n.closest("[class*=card], div") !== null)!;
    expect(alertEl.compareDocumentPosition(data) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy(); // alert before the data card
    expect(data.compareDocumentPosition(risk) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy(); // risk card after it
    first.unmount();
    const { container } = renderApp(<AccountDetail accountId={acc} alertSlot={false} riskSection={null} />);
    await screen.findByText("Posição aberta");
    expect(container.textContent).not.toMatch(/ALERT-SLOT|RISK-SLOT/);
    // "no wrapper": an absent slot leaves no empty element behind (an empty div with no text, children or attributes)
    const empties = Array.from(container.querySelectorAll("div")).filter((d) => d.childElementCount === 0 && (d.textContent ?? "").trim() === "" && d.attributes.length === 0);
    expect(empties).toHaveLength(0);
  });
});
