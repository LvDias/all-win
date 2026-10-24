import type { Page } from "@playwright/test";
import { expect, test } from "./fixtures";

const accountId = "00000000-0000-4000-8000-0000000000aa";
const foreignId = "00000000-0000-4000-8000-0000000000ff";
const oldVersion = "00000000-0000-4000-8000-000000000020";
const newVersion = "00000000-0000-4000-8000-000000000099";
const ruleVersion = "00000000-0000-4000-8000-0000000000bb";
const release3 = { version_number: 3, published_at: "2026-10-20T12:00:00Z", verified_on: "2026-10-18" };
const release4 = { version_number: 4, published_at: "2026-10-22T10:00:00Z", verified_on: "2026-10-21" };
const source = { url: "https://firm.example/rules", accessed_at: "2026-10-18", page_title: "Rules" };

const account = {
  id: accountId, kind: "prop", display_name: "Apex 50K", firm_name: "Apex", plan_name: "Plan A", phase: "evaluation",
  size_amount: "50000.00", currency: "USD", multiplier: 1, max_position_qty: 60, lifecycle_state: "active", has_open_position: false,
  opt_ins: { automation: { kind: "automation", state: "not_required" }, copy: { kind: "copy", state: "not_required" } },
  row_version: 7, created_at: "2026-10-23T12:00:00Z", updated_at: "2026-10-23T12:00:00Z",
};

const rules = (drawdown: unknown) => ({
  max_position_qty: { status: "verified", value: { contracts: 60 }, sources: [source] },
  drawdown,
  daily_loss_limit: { status: "verified", value: { kind: "none" }, sources: [source] },
  trading_day_reset: { status: "verified", value: { local_time: "17:00", time_zone: "America/Chicago", published_text: "5:00 PM CT" }, sources: [source] },
});
const presetVersion = (id: string, release: unknown, drawdown: unknown) => ({
  id, preset_id: "00000000-0000-4000-8000-000000000040", firm_id: "00000000-0000-4000-8000-000000000001", firm_name: "Apex",
  plan_id: "00000000-0000-4000-8000-000000000010", plan_name: "Plan A", phase: "evaluation", size_amount: "50000.00", release, not_confirmed_count: 0,
  rule_sets: [{ rules: rules(drawdown) }],
});
const drawdown = (amount: string) => ({ status: "verified", value: { type: "trailing_eod", amount }, sources: [source] });
const conflicting = { status: "not_confirmed", published_values: [
  { value: { type: "trailing_eod", amount: "2600.00" }, sources: [source] },
  { value: { type: "static", amount: "2000.00" }, sources: [{ url: "https://firm.example/faq", accessed_at: "2026-10-17", page_title: "FAQ" }] },
] };

type Mock = { applyBodies: Record<string, unknown>[]; requests: string[] };

/** A small Core for the browser: the account, its preset state, the diff, apply (optionally behind step-up) and the rule version. */
async function mockCore(page: Page, options: { stepUp?: boolean; optInChange?: boolean } = {}): Promise<Mock> {
  const mock: Mock = { applyBodies: [], requests: [] };
  await page.route("**/api/v1/connections", (route) => route.fulfill({ json: [] }));
  await page.route("**/api/v1/accounts/**", async (route) => {
    const request = route.request();
    const path = new URL(request.url()).pathname;
    mock.requests.push(`${request.method()} ${path}`);
    if (path === `/api/v1/accounts/${foreignId}` || path.startsWith(`/api/v1/accounts/${foreignId}/`)) return route.fulfill({ status: 404, json: { error: { code: "common.not_found" } } });
    if (path.endsWith("/preset-diff"))
      return route.fulfill({ json: {
        account_id: accountId, from_preset_version_id: oldVersion, to_preset_version_id: newVersion, firm_platform_required: false,
        opt_in_changes: { automation: options.optInChange ? ruleVersion : null, copy: null },
        rows: [{ field: "drawdown", account_value: { type: "static", amount: "1800.00" }, current_preset_value: { type: "trailing_eod", amount: "2500.00" }, edited: false, requires_choice: true, cap_conflict: false }],
      } });
    if (path.endsWith("/preset-version")) {
      const body = request.postDataJSON() as Record<string, unknown>;
      mock.applyBodies.push(body);
      const accepting = Object.values((body.opt_ins ?? {}) as Record<string, { choice: string }>).some((entry) => entry.choice === "accepted");
      if (options.stepUp && accepting && !body.step_up) return route.fulfill({ status: 403, json: { error: { code: "auth.step_up_required" } } });
      return route.fulfill({ json: { account_id: accountId, preset_version_id: newVersion, release: release4, row_version: 8, fields: [] } });
    }
    if (path.endsWith("/preset"))
      return route.fulfill({ json: { account_id: accountId, preset_version_id: oldVersion, release: release3, row_version: 7, fields: [
        { field: "drawdown", origin: "edited", account_value: { type: "static", amount: "1800.00" }, preset_value: { type: "trailing_eod", amount: "2500.00" } },
      ], newer_version: { preset_version_id: newVersion, release: release4, suspends: false } } });
    return route.fulfill({ json: account });
  });
  await page.route("**/api/v1/preset-versions/*", (route) => {
    const id = new URL(route.request().url()).pathname.split("/").pop();
    return route.fulfill({ json: id === newVersion ? presetVersion(newVersion, release4, conflicting) : presetVersion(oldVersion, release3, drawdown("2500.00")) });
  });
  await page.route(`**/api/v1/rule-versions/${ruleVersion}`, (route) => route.fulfill({ json: {
    id: ruleVersion, kind: "automation", firm_stance: "conditional", is_generic: false, firm_name: "Apex", version_number: 4, published_at: "2026-10-22T10:00:00Z",
    quotes: [{ text: "Automation only under conditions.", language: "en", page_title: "Rules", url: "https://firm.example/rules", accessed_at: "2026-10-21" }],
  } }));
  await page.route("**/api/v1/auth/step-up", (route) => route.fulfill({ status: 202, json: { challenge_id: "challenge-9", expires_at: "2099-01-01T00:00:00Z", resend_after_s: 60 } }));
  return mock;
}

async function openDialog(page: Page) {
  await page.goto(`/pt-BR/accounts/${accountId}`);
  await expect(page.getByText("Há uma nova versão do preset da Apex")).toBeVisible();
  await page.getByRole("button", { name: "Ver mudanças" }).click();
  const dialog = page.getByRole("dialog", { name: /Mudanças da versão 4/ });
  await expect(dialog).toBeVisible();
  return dialog;
}

test("shows the edited value, the new-version notice, and applies a version choosing among conflicting values", async ({ page, api }) => {
  await api.signedIn();
  const mock = await mockCore(page);
  await page.goto(`/pt-BR/accounts/${accountId}`);
  await expect(page.getByRole("button", { name: /Drawdown: editado/ })).toBeVisible();
  await page.getByRole("button", { name: "Ver mudanças" }).click();
  const dialog = page.getByRole("dialog", { name: /Mudanças da versão 4/ });
  await expect(dialog).toBeVisible();
  for (const radio of await dialog.getByRole("radio").all()) await expect(radio).not.toBeChecked();

  await dialog.getByRole("button", { name: "Aplicar versão 4" }).click();
  await expect(dialog.getByText("Resolva este valor para aplicar.")).toBeVisible();
  expect(mock.applyBodies).toHaveLength(0);

  await dialog.getByRole("radio", { name: /Estático/ }).click();
  await dialog.getByRole("button", { name: "Aplicar versão 4" }).click();
  await expect(dialog).toBeHidden();
  expect(mock.applyBodies).toEqual([{ target_preset_version_id: newVersion, row_version: 7, rule_values: { drawdown: { type: "static", amount: "2000.00" } } }]);
});

test("the dialog works by keyboard and closes back to its trigger", async ({ page, api }) => {
  await api.signedIn();
  await mockCore(page);
  await page.goto(`/pt-BR/accounts/${accountId}`);
  const trigger = page.getByRole("button", { name: "Ver mudanças" });
  await trigger.focus();
  await page.keyboard.press("Enter");
  const dialog = page.getByRole("dialog", { name: /Mudanças da versão 4/ });
  await expect(dialog).toBeVisible();
  await page.keyboard.press("Escape");
  await expect(dialog).toBeHidden();
  await expect(trigger).toBeFocused();
});

test("accepting an opt-in asks for a step-up code and sends the same body once more with the proof", async ({ page, api }) => {
  await api.signedIn();
  const mock = await mockCore(page, { stepUp: true, optInChange: true });
  const dialog = await openDialog(page);
  await dialog.getByRole("radio", { name: /Estático/ }).click();
  await dialog.getByRole("radio", { name: /Aceitar ordens automatizadas/ }).click();
  await dialog.getByRole("checkbox").check();
  await dialog.getByRole("button", { name: "Aplicar versão 4" }).click();
  await page.getByLabel(/Código de verificação/).fill("ABCD2345");
  await expect(dialog).toBeHidden();
  expect(mock.applyBodies).toHaveLength(2);
  const [probe, proved] = mock.applyBodies;
  expect(probe).not.toHaveProperty("step_up");
  expect(proved).toMatchObject({ ...probe, step_up: { challenge_id: "challenge-9", code: "ABCD2345" } });
  expect(probe).toMatchObject({ opt_ins: { automation: { choice: "accepted", rule_version_id: ruleVersion } } });
});

test("declining an opt-in needs no step-up", async ({ page, api }) => {
  await api.signedIn();
  const mock = await mockCore(page, { stepUp: true, optInChange: true });
  const dialog = await openDialog(page);
  await dialog.getByRole("radio", { name: /Estático/ }).click();
  await dialog.getByRole("radio", { name: /Não aceitar ordens automatizadas/ }).click();
  await dialog.getByRole("button", { name: "Aplicar versão 4" }).click();
  await expect(dialog).toBeHidden();
  expect(mock.applyBodies).toHaveLength(1);
  expect(mock.applyBodies[0]).not.toHaveProperty("step_up");
});

test("another user's account is not found and its preset is never requested; a hostile firm id never reaches the API", async ({ page, api }) => {
  await api.signedIn();
  const mock = await mockCore(page);
  await page.goto(`/pt-BR/accounts/${foreignId}`);
  await expect(page.getByText("Conta não encontrada.")).toBeVisible();
  await expect(page.getByText("Preset da firma")).toHaveCount(0);
  expect(mock.requests.filter((entry) => entry.includes("/preset"))).toEqual([]);

  const firmRequests: string[] = [];
  await page.route("**/api/v1/firms/**", (route) => { firmRequests.push(route.request().url()); return route.fulfill({ status: 404, json: { error: { code: "common.not_found" } } }); });
  await page.goto("/pt-BR/presets/..%2Fadmin");
  await expect(page.getByRole("heading", { name: "Presets de firmas" })).toHaveCount(0);
  await expect(page.getByText("This page could not be found.")).toBeVisible();
  expect(firmRequests).toEqual([]);
});

test.describe("english", () => {
  test("the preset panel and dialog are translated", async ({ page, api }) => {
    await api.signedIn({ locale: "en" });
    await mockCore(page);
    await page.goto(`/en/accounts/${accountId}`);
    await expect(page.getByText("There is a new version of the Apex preset")).toBeVisible();
    await page.getByRole("button", { name: "See changes" }).click();
    await expect(page.getByRole("dialog", { name: /Changes in version 4/ })).toBeVisible();
  });
});

test("Nova conta: picks a firm preset, must resolve the not-confirmed value, and creates the account without manual firm fields", async ({ page, api }) => {
  // The longest flow (four dropdowns, six steps): on a loaded host it needs more than the default 30 s.
  test.slow();
  await api.signedIn();
  const firmId = "00000000-0000-4000-8000-000000000001";
  const planId = "00000000-0000-4000-8000-000000000010";
  const ruleId = "1f2e3d4c-5b6a-4978-8a9b-0c1d2e3f4a5b";
  const created: Record<string, unknown>[] = [];
  const summary = { id: firmId, slug: "apex", name: "Apex", plan_names: ["Plan A"], stances: { automation: "unknown", copy: "unknown", vps: "unknown", personal_device: "unknown" }, not_confirmed_count: 1, release: release4 };
  await page.route("**/api/v1/connections", (route) => route.fulfill({ json: [] }));
  await page.route("**/api/v1/firms", (route) => route.fulfill({ json: { firms: [summary] } }));
  await page.route(`**/api/v1/firms/${firmId}`, (route) => route.fulfill({ json: {
    summary, platforms: [], excluded_plans: [],
    stances: { automation: [], copy: [], vps: { value: "unknown", quotes: [] }, personal_device: { value: "unknown", quotes: [] } },
    plans: [{ plan_id: planId, name: "Plan A", phases: [{ phase: "evaluation", sizes: [{ preset_id: "00000000-0000-4000-8000-000000000040", preset_version_id: newVersion, size_amount: "50000.00", size_label: "50K", platform_specific: false, not_confirmed_count: 1 }] }] }],
  } }));
  await page.route("**/api/v1/preset-versions/*", (route) => route.fulfill({ json: presetVersion(newVersion, release4, conflicting) }));
  await page.route("**/api/v1/opt-in-rules*", (route) => route.fulfill({ json: {
    automation: { id: ruleId, kind: "automation", firm_stance: "unknown", is_generic: false, firm_name: "Apex", version_number: 1, published_at: "2026-10-01T00:00:00Z", quotes: [] },
    copy: { id: "3b4c5d6e-7f8a-4b9c-8d1e-2f3a4b5c6d7e", kind: "copy", firm_stance: "unknown", is_generic: false, firm_name: "Apex", version_number: 1, published_at: "2026-10-01T00:00:00Z", quotes: [] },
  } }));
  await page.route(/\/api\/v1\/accounts(\?.*)?$/, async (route) => {
    if (route.request().method() === "POST") {
      created.push(route.request().postDataJSON() as Record<string, unknown>);
      return route.fulfill({ status: 201, json: { ...account, id: accountId } });
    }
    return route.fulfill({ json: { accounts: [] } });
  });
  await page.route(`**/api/v1/accounts/${accountId}`, (route) => route.fulfill({ json: account }));
  await page.route(`**/api/v1/accounts/${accountId}/preset`, (route) => route.fulfill({ status: 404, json: { error: { code: "common.not_found" } } }));

  await page.goto("/pt-BR/accounts/new");
  await page.getByRole("radio", { name: "Conta de prop firm" }).click();
  await page.getByRole("button", { name: "Continuar" }).click();
  await page.getByRole("combobox", { name: /^Firma/ }).click();
  await page.getByRole("option", { name: /Apex/ }).click();
  await page.getByRole("combobox", { name: "Plano" }).click();
  await page.getByRole("option", { name: "Plan A" }).click();
  await page.getByRole("combobox", { name: "Fase" }).click();
  await page.getByRole("option", { name: "Avaliação" }).click();
  await page.getByRole("combobox", { name: "Tamanho" }).click();
  await page.getByRole("option", { name: "50K" }).click();

  await page.getByRole("button", { name: "Continuar" }).click();
  await expect(page.getByText("Resolva este valor para salvar.").first()).toBeVisible();
  expect(created).toHaveLength(0);
  await page.getByRole("radio", { name: /Estático/ }).click();
  await page.getByRole("button", { name: "Continuar" }).click();
  await page.getByLabel("Nome", { exact: true }).fill("Apex 1");
  await page.getByRole("button", { name: "Continuar" }).click();
  await page.getByRole("button", { name: "Continuar" }).click();
  await page.getByRole("radio", { name: /Não aceitar ordens automatizadas/ }).click();
  await page.getByRole("button", { name: "Continuar" }).click();
  await page.getByRole("button", { name: "Salvar conta" }).click();
  // The redirect follows the create and a page render: give it room when the host is loaded.
  await expect(page).toHaveURL(new RegExp(`/pt-BR/accounts/${accountId}$`), { timeout: 20_000 });
  expect(created[0]).toMatchObject({ kind: "prop", preset_version_id: newVersion, rule_values: { drawdown: { type: "static", amount: "2000.00" } }, opt_ins: { automation: { choice: "declined" } } });
  for (const manual of ["firm_name", "plan_name", "phase", "size_amount"]) expect(created[0]).not.toHaveProperty(manual);
});
