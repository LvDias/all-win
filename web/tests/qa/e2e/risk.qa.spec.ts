// QA (Retina) — V1-S13 browser checks against mocked Core responses: keyboard, focus, 320 px reflow, axe (WCAG 2.2 AA tags).
import { createRequire } from "node:module";
import type { Page } from "@playwright/test";
import { expect, test } from "../../../e2e/fixtures";

const axePath = createRequire(import.meta.url).resolve("axe-core/axe.min.js", { paths: [process.cwd() + "/node_modules/.pnpm/axe-core@4.14.0/node_modules"] });
const id = "00000000-0000-4000-8000-0000000000a1";
const risk = {
  account_id: id, as_of: "2026-10-23T14:31:05Z", stale: false, state: "warning",
  trading_day: { start: "2026-10-22T21:00:00Z", end: "2026-10-23T21:00:00Z", next_reset: "2026-10-23T21:00:00Z", source: "firm", firm_reset: { local_time: "17:00", time_zone: "America/New_York" } },
  meters: [
    { kind: "daily_loss", state: "warning", limit: "1000.00", used: "820.00", percent: 82 },
    { kind: "drawdown", state: "normal", limit: "2500.00", used: "300.00", percent: 12, drawdown_type: "trailing_eod", floor: "47700.00" },
    { kind: "daily_profit_target", state: "normal", limit: "600.00", used: "100.00", percent: 16 },
  ],
  unset_rules: ["consistency"], untracked_rules: [], enforcement_enabled: false, limits_pending: false, pending_changes: [], pnl_basis: "net", commission_source: "none",
  evaluation: { start: "2026-10-01", profit_target: "6000.00", earned_before: "0.00", achieved: "4800.00", remaining: "1200.00", percent: 80, state: "in_progress", as_of: "2026-10-23T14:31:05Z" },
};
const rules = { account_id: id, enforcement_enabled: false, version: 4, rules: { drawdown: { type: "trailing_eod", amount: "2500.00" }, daily_loss_limit: { kind: "hard", amount: "1000.00" }, daily_profit_target: { amount: "600.00" }, profit_target_on_reach: "notify" } };
const account = { id, kind: "prop", display_name: "Apex 50K", firm_name: "Apex", plan_name: "50K", phase: "evaluation", preset_version_id: "00000000-0000-4000-8000-0000000000b1", firm_platform: "rithmic", size_amount: "50000", currency: "USD", multiplier: 2, max_position_qty: 5, lifecycle_state: "active", opt_ins: { automation: { kind: "automation", state: "declined" }, copy: { kind: "copy", state: "declined" } }, has_open_position: false, row_version: 7, created_at: "2026-10-01T12:00:00Z", updated_at: "2026-10-01T12:00:00Z" };

async function mock(page: Page, puts: unknown[] = []) {
  await page.route(`**/api/v1/accounts/${id}/risk`, (r) => r.fulfill({ json: risk }));
  await page.route(`**/api/v1/accounts/${id}/risk-rules/preview`, (r) => r.fulfill({ json: { changes: [], would_be_reached: false } }));
  await page.route(`**/api/v1/accounts/${id}/risk-rules`, (r) => {
    if (r.request().method() === "PUT") { puts.push(r.request().postDataJSON()); return r.fulfill({ json: { ...rules, version: 5 } }); }
    return r.fulfill({ json: rules });
  });
  await page.route(`**/api/v1/accounts/${id}`, (r) => r.fulfill({ json: account }));
}
async function axe(page: Page) {
  await page.addScriptTag({ path: axePath });
  return page.evaluate(async () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const result = await (window as any).axe.run(document, { runOnly: { type: "tag", values: ["wcag2a", "wcag2aa", "wcag21aa", "wcag22aa"] } });
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    return result.violations.map((v: any) => `${v.id}: ${v.nodes.length} (${v.nodes[0]?.target?.join(" ")})`);
  });
}

test.describe("S13 risk (mocked Core)", () => {
  test.beforeEach(async ({ page, api }) => { await api.signedIn(); await mock(page); });

  test("risk tab renders meters and has no axe violations", async ({ page }) => {
    await page.goto(`/pt-BR/accounts/${id}/risk`);
    await expect(page.getByRole("meter", { name: "Perda diária" })).toHaveAttribute("aria-valuenow", "82");
    await expect(page.getByRole("progressbar", { name: "Meta da avaliação" })).toBeVisible();
    expect(await axe(page)).toEqual([]);
  });

  test("risk tab does not scroll horizontally at 320 px", async ({ page }) => {
    await page.setViewportSize({ width: 320, height: 800 });
    await page.goto(`/pt-BR/accounts/${id}/risk`);
    await expect(page.getByRole("meter", { name: "Perda diária" })).toBeVisible();
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth)).toBe(true);
  });

  test("what-happens collapsible is keyboard operable and its content is in the DOM", async ({ page }) => {
    await page.goto(`/pt-BR/accounts/${id}/risk`);
    const trigger = page.getByRole("button", { name: /O que acontece quando/ });
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
    await trigger.focus(); await page.keyboard.press("Enter");
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    await expect(page.getByText(/Saídas continuam permitidas/)).toBeVisible();
  });

  test("edit form: axe clean, keyboard reaches disabled flatten but cannot select it, first invalid field gets focus", async ({ page }) => {
    await page.goto(`/pt-BR/accounts/${id}/risk/edit`);
    const loss = page.getByLabel(/Limite de perda diária/);
    await expect(loss).toBeVisible();
    expect(await axe(page)).toEqual([]);
    const flatten = page.getByRole("radio", { name: /Fechar posições e bloquear/ });
    await expect(flatten).toHaveAttribute("aria-disabled", "true");
    await page.getByRole("radio", { name: /Só avisar/ }).focus();
    await page.keyboard.press("ArrowDown");
    await expect(page.getByRole("radio", { name: /Só avisar/ })).toBeChecked();
    await loss.fill("abc");
    await page.getByRole("button", { name: "Salvar regras" }).click();
    await expect(loss).toBeFocused();
    await expect(loss).toHaveAttribute("aria-invalid", "true");
  });

  test("edit form saves by keyboard (Enter) and returns to the risk tab", async ({ page }) => {
    const puts: unknown[] = [];
    await mock(page, puts);
    await page.goto(`/pt-BR/accounts/${id}/risk/edit`);
    const loss = page.getByLabel(/Limite de perda diária/);
    await loss.fill("900,5");
    await loss.press("Enter");
    await expect(page).toHaveURL(new RegExp(`/accounts/${id}/risk$`));
    expect(puts).toHaveLength(1);
    expect(JSON.stringify(puts[0])).toContain('"900.5"');
  });

  test("edit form reflows at 320 px; discard dialog traps focus and Esc closes it", async ({ page }) => {
    await page.setViewportSize({ width: 320, height: 800 });
    await page.goto(`/pt-BR/accounts/${id}/risk/edit`);
    const loss = page.getByLabel(/Limite de perda diária/);
    await loss.fill("1");
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth)).toBe(true);
    await page.getByRole("button", { name: "Cancelar" }).click();
    const dialog = page.getByRole("alertdialog");
    await expect(dialog).toBeVisible();
    await page.keyboard.press("Tab"); await page.keyboard.press("Tab"); await page.keyboard.press("Tab");
    expect(await page.evaluate(() => !!document.activeElement?.closest('[role="alertdialog"]'))).toBe(true);
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await expect(loss).toHaveValue("1");
  });

  test("restart-evaluation dialog is keyboard operable and axe clean", async ({ page }) => {
    await page.goto(`/pt-BR/accounts/${id}/risk`);
    await page.getByRole("button", { name: "Ajustar contagem" }).click();
    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();
    await expect(page.getByLabel("Início da contagem")).toBeFocused();
    expect(await axe(page)).toEqual([]);
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await expect(page.getByRole("button", { name: "Ajustar contagem" })).toBeFocused();
  });
});
