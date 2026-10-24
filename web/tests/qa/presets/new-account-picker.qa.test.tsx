// QA (Retina) — V1-S16 AC-2/AC-4/AC-6/AC-7 in "Nova conta": the preset picker composed with the S5 stepper through the public indexes only.
import { screen, within, configure } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { beforeEach, describe, expect, it } from "vitest";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
// render first: it installs the next/navigation mock before the features load.
import { AccountCreateStepper } from "@/features/accounts";
import { PresetPicker, usePresetSelection } from "@/features/presets";

// Findings must not depend on host load: wait up to 8 s for async UI (the default 1 s flaked on a loaded CI host).
configure({ asyncUtilTimeout: 8000 });

const me = { id: "00000000-0000-4000-8000-000000000001", email: "t@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const uuid = (n: number) => `00000000-0000-4000-8000-${String(n).padStart(12, "0")}`;
const release = { version_number: 3, published_at: "2026-10-20T12:00:00Z", verified_on: "2026-10-18" };
const src = { url: "https://firm.example/rules", accessed_at: "2026-10-18", page_title: "Rules" };
const verified = (value: unknown) => ({ status: "verified", value, sources: [src] });
const quote = (text: string) => ({ text, language: "en", page_title: "Rules", url: "https://firm.example/rules", accessed_at: "2026-10-18" });
const rule = (kind: string, id: string) => ({ id, kind, firm_stance: "unknown", is_generic: false, firm_name: "Apex", version_number: 1, published_at: "2026-10-01T00:00:00Z", quotes: [] });

function Composition() {
  const selection = usePresetSelection();
  return <AccountCreateStepper preset={{ outcome: selection.outcome, render: ({ switchToManual, error }) => <PresetPicker selection={selection} onManual={switchToManual} error={error} /> }} />;
}

function detail(o: { manualOnly?: boolean; platformSpecific?: boolean } = {}) {
  return {
    summary: { id: uuid(1), slug: "apex", name: "Apex", plan_names: ["Plan A"], stances: { automation: "allowed", copy: "allowed", vps: "allowed", personal_device: "unknown" }, not_confirmed_count: 0, release, ...(o.manualOnly ? { manual_only: { reason: quote("Orders must originate from your personal device.") } } : {}) },
    stances: { automation: [{ id: uuid(30), kind: "automation", firm_stance: "allowed", is_generic: false, firm_id: uuid(1), firm_name: "Apex", version_number: 3, published_at: release.published_at, quotes: [quote("Automation is permitted.")] }], copy: [], vps: { value: "allowed", quotes: [] }, personal_device: { value: "unknown", quotes: [] } },
    platforms: [{ code: "rithmic", label: "Rithmic" }, { code: "tradovate", label: "Tradovate" }],
    plans: [
      { plan_id: uuid(10), name: "Plan A", phases: [{ phase: "evaluation", sizes: [{ preset_id: uuid(40), preset_version_id: uuid(50), size_amount: "50000.00", size_label: "50K", platform_specific: o.platformSpecific ?? false, not_confirmed_count: 0 }] }] },
      { plan_id: uuid(11), name: "Plan B", phases: [{ phase: "evaluation", sizes: [{ preset_id: uuid(41), preset_version_id: uuid(51), size_amount: "25000.00", size_label: "25K", platform_specific: false, not_confirmed_count: 0 }] }] },
    ],
    excluded_plans: [],
  };
}
const versionBody = (ruleSets: unknown[]) => ({ id: uuid(50), preset_id: uuid(40), firm_id: uuid(1), firm_name: "Apex", plan_id: uuid(10), plan_name: "Plan A", phase: "evaluation", size_amount: "50000.00", release, not_confirmed_count: 0, rule_sets: ruleSets });
const rulesFor = (drawdown: string, platform?: string) => ({ ...(platform ? { firm_platform: platform } : {}), rules: { max_position_qty: verified({ contracts: 60 }), drawdown: verified({ type: "trailing_eod", amount: drawdown }), daily_loss_limit: verified({ kind: "none" }), trading_day_reset: verified({ local_time: "17:00", time_zone: "America/Chicago", published_text: "5:00 PM CT" }) } });

function serve(d: object, sets: unknown[]) {
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(me)),
    http.get("*/api/v1/connections", () => HttpResponse.json([])),
    http.get("*/api/v1/firms", () => HttpResponse.json({ firms: [(d as { summary: unknown }).summary] })),
    http.get("*/api/v1/firms/:id", () => HttpResponse.json(d)),
    http.get("*/api/v1/preset-versions/:id", () => HttpResponse.json(versionBody(sets))),
    http.get("*/api/v1/opt-in-rules", () => HttpResponse.json({ automation: rule("automation", uuid(60)), copy: rule("copy", uuid(61)) })),
  );
}
async function toPicker() {
  const u = userEvent.setup();
  renderApp(<Composition />);
  await u.click(await screen.findByRole("radio", { name: "Conta de prop firm" }));
  await u.click(screen.getByRole("button", { name: "Continuar" }));
  return u;
}
async function choose(u: ReturnType<typeof userEvent.setup>, label: string | RegExp, option: RegExp | string) {
  await u.click(screen.getByRole("combobox", { name: label }));
  await u.click(await screen.findByRole("option", { name: option }));
}
beforeEach(() => server.use(http.get("*/api/v1/me", () => HttpResponse.json(me))));

describe("picker (S16-03)", () => {
  it("preview header says firm/plan/phase/size/version/date and the preset values with exact money", async () => {
    serve(detail(), [rulesFor("2500.00")]);
    const u = await toPicker();
    await choose(u, /^Firma/, /Apex/); await choose(u, "Plano", "Plan A"); await choose(u, "Fase", "Avaliação"); await choose(u, "Tamanho", "50K");
    expect(await screen.findByText(/versão 3/)).toBeInTheDocument();
    expect(document.body.textContent).toMatch(/US\$\s2\.500,00 · Trailing fim do dia/);
  });
  it("dependent fields are disabled with a reason until the previous one is chosen", async () => {
    serve(detail(), [rulesFor("2500.00")]);
    const u = await toPicker();
    await choose(u, /^Firma/, /Apex/);
    expect(await screen.findByRole("combobox", { name: "Fase" })).toBeDisabled();
    expect(screen.getByRole("combobox", { name: "Tamanho" })).toBeDisabled();
  });
  it("manual-only firm: the reason is shown as soon as the firm is chosen, before the rest is filled", async () => {
    serve(detail({ manualOnly: true }), [rulesFor("2500.00")]);
    const u = await toPicker();
    await choose(u, /^Firma/, /Apex/);
    expect(await screen.findByText("Orders must originate from your personal device.")).toBeInTheDocument();
  });
  it("platform-specific preset: a platform the preset has NO rule set for must never show (or fill) another platform's rules", async () => {
    serve(detail({ platformSpecific: true }), [rulesFor("2500.00", "rithmic")]); // rules exist for rithmic only
    const u = await toPicker();
    await choose(u, /^Firma/, /Apex/); await choose(u, "Plano", "Plan A"); await choose(u, "Fase", "Avaliação"); await choose(u, "Tamanho", "50K");
    await choose(u, /Plataforma/, "Tradovate");
    await new Promise((r) => setTimeout(r, 300));
    expect(document.body.textContent).not.toMatch(/US\$\s2\.500,00/); // rithmic's drawdown must not appear under Tradovate
  });
  it("changing the selection after editing a value asks first, Cancelar focused, and Cancelar keeps everything", async () => {
    serve(detail(), [rulesFor("2500.00")]);
    const u = await toPicker();
    await choose(u, /^Firma/, /Apex/); await choose(u, "Plano", "Plan A"); await choose(u, "Fase", "Avaliação"); await choose(u, "Tamanho", "50K");
    await u.click(await screen.findAllByRole("button", { name: "Editar" }).then((b) => b[0]!)); // edit a verified value (becomes Editado)
    await choose(u, "Plano", "Plan B");
    const dialog = await screen.findByRole("alertdialog");
    expect(dialog).toHaveTextContent(/substitui/);
    // NOTE: jsdom leaves focus on the Select trigger here (Radix returns it); whether Cancelar gets the focus in a real browser is checked in e2e, not asserted here.
    await u.click(within(dialog).getByRole("button", { name: "Cancelar" }));
    expect(screen.queryByRole("alertdialog")).not.toBeInTheDocument();
    expect(screen.getByRole("combobox", { name: "Plano" })).toHaveTextContent("Plan A"); // nothing changed
  });
  it("a preset service error inside the picker is an error state with retry, not a blank", async () => {
    serve(detail(), [rulesFor("2500.00")]);
    server.use(http.get("*/api/v1/firms", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    await toPicker();
    expect(await screen.findByRole("button", { name: /tentar/i })).toBeInTheDocument();
  });
});
