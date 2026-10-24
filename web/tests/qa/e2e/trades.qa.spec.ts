// QA (Retina) — V1-S21 browser checks against mocked Core responses: axe (WCAG 2.2 AA tags), keyboard, focus, 320 px reflow.
import { createRequire } from "node:module";
import { test as base, expect, type Page } from "@playwright/test";

// Fixtures inlined on purpose: the lint rule "tests/qa uses @/ aliases only" forbids relative imports, and there is no alias for tests/qa/e2e.

const profile = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "America/Sao_Paulo", is_platform_admin: false };

async function assertNoI18nProblems(page: Page, i18nErrors: string[]) {
  await expect(page.getByText("⟦missing:")).toHaveCount(0);
  expect(i18nErrors).toEqual([]);
}

/** Every test fails on a visible missing-translation marker or an i18n console error (spec V1-S4 §7). */
const test = base.extend<{ api: { signedOut: () => Promise<void>; signedIn: (overrides?: Partial<typeof profile>) => Promise<void> } }>({
  page: async ({ page }, use) => {
    const i18nErrors: string[] = [];
    page.on("console", (message) => {
      if (message.type() === "error" && message.text().includes("[i18n]")) i18nErrors.push(message.text());
    });
    await use(page);
    await assertNoI18nProblems(page, i18nErrors);
  },
  api: async ({ page }, use) => {
    await use({
      signedOut: () => mockMe(page, 401),
      signedIn: (overrides = {}) => mockMe(page, 200, { ...profile, ...overrides }),
    });
  },
});

async function mockMe(page: Page, status: number, body?: unknown) {
  // The event stream is opened once signed in; an empty stream keeps the page quiet.
  await page.route("**/api/v1/events", (route) => route.fulfill({ status: 200, contentType: "text/event-stream", body: ": ok\n\n" }));
  await page.route("**/api/v1/me", (route) => {
    if (status === 401) return route.fulfill({ status, json: { error: { code: "auth.unauthenticated" } } });
    if (route.request().method() === "PATCH") {
      const patch = route.request().postDataJSON() as Record<string, string>;
      body = { ...(body as object), ...patch };
    }
    return route.fulfill({ status, json: body });
  });
}


const axePath = createRequire(import.meta.url).resolve("axe-core/axe.min.js", { paths: [process.cwd() + "/node_modules/.pnpm/axe-core@4.14.0/node_modules"] });
const u = (n: number) => `00000000-0000-4000-8000-${String(n).padStart(12, "0")}`;
const trade = (n: number, o: Record<string, unknown> = {}) => ({
  trade_id: u(n), account_id: u(100), account_name: "Apex 50K", account_deleted: false, instrument_id: u(200), instrument_symbol: "MNQ", direction: n % 2 ? "long" : "short", quantity: 2,
  entry_time: "2026-10-23T13:30:05Z", entry_price: "21234.50", exit_time: "2026-10-23T13:42:35Z", exit_price: "21240.25", duration_seconds: 750,
  pnl: n % 2 ? "21.50" : "-45.50", pnl_basis: "net", gross_pnl: "23.00", commission: "1.50", commission_source: "configured", mae: "10.00", mfe: "30.00",
  origin: "routed", source_id: u(300), source_name: "Strikes MNQ", signal_type: "trigger", instruction_id: u(400), editable: false, captured_at: "2026-10-23T13:42:40Z", created_at: "2026-10-23T13:42:40Z", ...o,
});
const rows = [trade(1), trade(2, { origin: "manual", editable: true, row_version: 3, source_id: undefined, source_name: undefined, signal_type: undefined, instruction_id: undefined, captured_at: undefined, mae: undefined, mfe: undefined }), trade(3, { origin: "platform", source_name: undefined, signal_type: undefined, instruction_id: undefined })];
const summary = { as_of: "2026-10-23T14:00:00Z", count: 3, total_pnl: "-2.50", pnl_basis: "net", total_gross_pnl: "1.00", total_commission: "3.50", uncommissioned_count: 1 };
const instrument = { id: u(200), symbol: "MNQ", name: "Micro E-mini Nasdaq-100", exchange: "CME", currency: "USD", tick_size: "0.25", tick_value: "0.5", point_value: "2", price_decimals: 2 };

async function mock(page: Page) {
  await page.route("**/api/v1/trades/summary*", (r) => r.fulfill({ json: summary }));
  await page.route(/\/api\/v1\/trades(\?.*)?$/, (r) => r.fulfill({ json: { trades: rows } }));
  await page.route(new RegExp(`/api/v1/trades/${u(2)}$`), (r) => r.fulfill({ json: rows[1] }));
  await page.route(new RegExp(`/api/v1/trades/${u(1)}$`), (r) => r.fulfill({ json: rows[0] }));
  await page.route("**/api/v1/accounts", (r) => r.fulfill({ json: { accounts: [{ id: u(100), display_name: "Apex 50K" }] } }));
  await page.route("**/api/v1/sources*", (r) => r.fulfill({ json: [{ id: u(300), name: "Strikes MNQ" }] }));
  await page.route("**/api/v1/instruments", (r) => r.fulfill({ json: { instruments: [instrument] } }));
  await page.route("**/api/v1/trade-preview*", (r) => r.fulfill({ json: { duration_seconds: 750, pnl: "21.50", pnl_basis: "net", gross_pnl: "23.00", commission: "1.50", commission_source: "configured" } }));
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
const noHScroll = (page: Page) => page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth);

test.describe("S21 trades (mocked Core)", () => {
  test.beforeEach(async ({ page, api }) => { await api.signedIn(); await mock(page); });

  test("list: axe clean, signs in text, totals net primary", async ({ page }) => {
    await page.goto("/pt-BR/trades");
    await expect(page.getByRole("button", { name: "Apex 50K" }).first()).toBeVisible();
    await expect(page.getByLabel("Totais do filtro")).toContainText("P&L líquido");
    await expect(page.getByText("Plataforma (sem sinal)")).toBeVisible();
    expect(await axe(page)).toEqual([]);
  });

  test("list at 320 px: no horizontal page scroll and axe clean", async ({ page }) => {
    await page.setViewportSize({ width: 320, height: 800 });
    await page.goto("/pt-BR/trades");
    await expect(page.getByRole("button", { name: /Apex 50K/ }).first()).toBeVisible();
    expect(await noHScroll(page)).toBe(true);
    expect(await axe(page)).toEqual([]);
  });

  test("detail: opens from keyboard, focus on the title, Esc returns focus to the row control", async ({ page }) => {
    await page.goto("/pt-BR/trades");
    const first = page.getByRole("button", { name: "Apex 50K" }).first();
    await first.focus();
    await page.keyboard.press("Enter");
    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();
    await expect(dialog.getByRole("heading", { name: /Trade Apex 50K/ })).toBeFocused();
    expect(await axe(page)).toEqual([]);
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await expect(first).toBeFocused();
  });

  test("manual trade: delete asks first, Cancel is focused", async ({ page }) => {
    await page.goto("/pt-BR/trades");
    await page.getByRole("button", { name: "Apex 50K" }).nth(1).click();
    await page.getByRole("dialog").getByRole("button", { name: "Excluir" }).click();
    const ad = page.getByRole("alertdialog");
    await expect(ad.getByRole("button", { name: "Cancelar" })).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(ad).toBeHidden();
  });

  test("manual entry: stepper is keyboard operable, step heading focused, errors linked, axe clean at 320 px", async ({ page }) => {
    await page.setViewportSize({ width: 320, height: 800 });
    await page.goto("/pt-BR/trades/new");
    await expect(page.getByRole("heading", { level: 2, name: "Trade" })).toBeVisible();
    await page.getByRole("button", { name: "Continuar" }).click();
    await expect(page.getByText("Escolha a conta.")).toBeVisible();
    expect(await noHScroll(page)).toBe(true);
    expect(await axe(page)).toEqual([]);
    await page.getByLabel("Conta").focus();
    await page.keyboard.press("Enter");
    await page.getByRole("option", { name: "Apex 50K" }).click();
    await page.getByRole("radio", { name: "Compra" }).check();
    await page.getByLabel("Quantidade").fill("2");
    await page.getByLabel("Quantidade").press("Enter");
    await expect(page.getByRole("heading", { level: 2, name: "Entrada e saída" })).toBeFocused();
  });
});
