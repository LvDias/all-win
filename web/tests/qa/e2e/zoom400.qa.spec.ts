// QA (Retina) — WCAG 1.4.10 Reflow at 400 % zoom: 1280 px at 400 % is 320 CSS px wide (height 1024 / 4 = 256).
// Checks: no horizontal (2-D) page scroll, nothing clipped off the right edge, and every key control still reachable by scrolling vertically.
// Mocked Core responses (page.route), self-contained on purpose so the file does not depend on app-private fixtures.
import { expect, test, type Locator, type Page } from "@playwright/test";

const u = (n: number) => `00000000-0000-4000-8000-${String(n).padStart(12, "0")}`;
const id = u(1);
const me = { id: u(9), email: "trader@example.com", locale: "pt-BR", time_zone: "America/Sao_Paulo", is_platform_admin: false };

test.use({ viewport: { width: 320, height: 256 } });

async function signedIn(page: Page) {
  await page.route("**/api/v1/events", (r) => r.fulfill({ status: 200, contentType: "text/event-stream", body: ": ok\n\n" }));
  await page.route("**/api/v1/me", (r) => r.fulfill({ json: me }));
}
const noHorizontalScroll = (page: Page) => page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth + 1);
/** Elements whose right edge is past the viewport and that are not inside a scroll container made for it (tables are allowed to scroll inside). */
const clippedRight = (page: Page) =>
  page.evaluate(() => {
    const limit = document.documentElement.clientWidth + 1;
    const bad: string[] = [];
    for (const el of Array.from(document.querySelectorAll<HTMLElement>("body *"))) {
      const r = el.getBoundingClientRect();
      if (r.width === 0 || r.height === 0 || getComputedStyle(el).visibility === "hidden") continue;
      if (r.right <= limit) continue;
      let p: HTMLElement | null = el.parentElement;
      let contained = false;
      while (p && p !== document.body) {
        const o = getComputedStyle(p).overflowX;
        if (o === "auto" || o === "scroll" || o === "hidden" || o === "clip") { contained = true; break; }
        p = p.parentElement;
      }
      if (!contained && (el.textContent ?? "").trim() !== "") bad.push(`${el.tagName.toLowerCase()}:${(el.textContent ?? "").trim().slice(0, 30)}`);
    }
    return bad.slice(0, 5);
  });
async function reflowOk(page: Page) {
  expect(await noHorizontalScroll(page)).toBe(true);
  expect(await clippedRight(page)).toEqual([]);
}
async function reachable(target: Locator) {
  await target.scrollIntoViewIfNeeded();
  await expect(target).toBeInViewport();
}

test("negative control: the reflow helpers do detect a 600 px wide page", async ({ page }) => {
  await page.setContent('<body><div style="width:600px;height:20px">wide content that does not fit</div></body>');
  expect(await noHorizontalScroll(page)).toBe(false);
  expect((await clippedRight(page)).length).toBeGreaterThan(0);
});

test.describe("S13 at 400 % zoom", () => {
  const risk = {
    account_id: id, as_of: "2026-10-23T14:31:05Z", stale: false, state: "warning",
    trading_day: { start: "2026-10-22T21:00:00Z", end: "2026-10-23T21:00:00Z", next_reset: "2026-10-23T21:00:00Z", source: "firm", firm_reset: { local_time: "17:00", time_zone: "America/New_York" } },
    meters: [{ kind: "daily_loss", state: "warning", limit: "1000.00", used: "820.00", percent: 82 }, { kind: "drawdown", state: "normal", limit: "2500.00", used: "300.00", percent: 12, drawdown_type: "trailing_eod", floor: "47700.00" }],
    unset_rules: ["consistency"], untracked_rules: [], enforcement_enabled: false, limits_pending: false, pending_changes: [], pnl_basis: "net", commission_source: "none",
    blocked: { until: "2026-10-23T21:00:00Z", cause: "daily_profit_target" },
    evaluation: { start: "2026-10-01", profit_target: "6000.00", earned_before: "0.00", achieved: "4800.00", remaining: "1200.00", percent: 80, state: "in_progress", as_of: "2026-10-23T14:31:05Z" },
  };
  const rules = { account_id: id, enforcement_enabled: false, version: 4, rules: { drawdown: { type: "trailing_eod", amount: "2500.00" }, daily_loss_limit: { kind: "hard", amount: "1000.00" }, daily_profit_target: { amount: "600.00" }, profit_target_on_reach: "notify" } };
  const account = { id, kind: "prop", display_name: "Apex 50K", firm_name: "Apex", plan_name: "50K", phase: "evaluation", preset_version_id: u(2), firm_platform: "rithmic", size_amount: "50000", currency: "USD", multiplier: 2, max_position_qty: 5, lifecycle_state: "active", opt_ins: { automation: { kind: "automation", state: "declined" }, copy: { kind: "copy", state: "declined" } }, has_open_position: false, row_version: 7, created_at: "2026-10-01T12:00:00Z", updated_at: "2026-10-01T12:00:00Z" };
  test.beforeEach(async ({ page }) => {
    await signedIn(page);
    await page.route(`**/api/v1/accounts/${id}/risk`, (r) => r.fulfill({ json: risk }));
    await page.route(`**/api/v1/accounts/${id}/risk-rules`, (r) => r.fulfill({ json: rules }));
    await page.route(`**/api/v1/accounts/${id}`, (r) => r.fulfill({ json: account }));
  });
  test("risk tab: meters, block, evaluation card and actions all reachable, no 2-D scroll", async ({ page }) => {
    await page.goto(`/pt-BR/accounts/${id}/risk`);
    await expect(page.getByRole("meter", { name: "Perda diária" })).toBeAttached();
    await reflowOk(page);
    await reachable(page.getByRole("meter", { name: "Perda diária" }));
    await reachable(page.getByRole("progressbar", { name: "Meta da avaliação" }));
    await reachable(page.getByRole("button", { name: "Ajustar contagem" }));
    await reachable(page.getByRole("button", { name: /O que acontece quando/ }));
    await reflowOk(page);
  });
  test("rules form: every field, the radio group and the save button reachable; sticky footer does not cover the focused field", async ({ page }) => {
    await page.goto(`/pt-BR/accounts/${id}/risk/edit`);
    const loss = page.getByLabel(/Limite de perda diária/);
    await expect(loss).toBeAttached();
    await reflowOk(page);
    for (const field of [loss, page.getByLabel(/Drawdown, em dólares/), page.getByLabel(/Valor da meta/)]) await reachable(field);
    await reachable(page.getByRole("radio", { name: /Só avisar/ }));
    await reachable(page.getByRole("button", { name: "Salvar regras" }));
    await loss.focus();
    const box = await loss.boundingBox();
    const footer = await page.getByRole("button", { name: "Salvar regras" }).boundingBox();
    expect(box && footer && !(box.y < footer.y + footer.height && box.y + box.height > footer.y)).toBe(true); // 2.4.11 focus not obscured
    await reflowOk(page);
  });
  test("restart-evaluation dialog fits and its buttons are reachable", async ({ page }) => {
    await page.goto(`/pt-BR/accounts/${id}/risk`);
    await page.getByRole("button", { name: "Ajustar contagem" }).click();
    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();
    await expect(dialog.getByLabel("Início da contagem")).toBeInViewport();
    await reachable(dialog.getByRole("button", { name: "Cancelar" }));
    await reflowOk(page);
  });
});

test.describe("S21 at 400 % zoom", () => {
  const trade = (n: number, o: Record<string, unknown> = {}) => ({
    trade_id: u(100 + n), account_id: u(5), account_name: "Apex 50K", account_deleted: false, instrument_id: u(6), instrument_symbol: "MNQ", direction: "long", quantity: 2,
    entry_time: "2026-10-23T13:30:05Z", entry_price: "21234.50", exit_time: "2026-10-23T13:42:35Z", exit_price: "21240.25", duration_seconds: 750, pnl: "21.50", pnl_basis: "net", gross_pnl: "23.00",
    commission: "1.50", commission_source: "configured", mae: "10.00", mfe: "30.00", origin: "routed", source_id: u(7), source_name: "Strikes MNQ", signal_type: "trigger", instruction_id: u(8), editable: false, captured_at: "2026-10-23T13:42:40Z", created_at: "2026-10-23T13:42:40Z", ...o,
  });
  const rows = [trade(1), trade(2, { origin: "manual", editable: true, row_version: 3, captured_at: undefined, source_name: undefined, signal_type: undefined, instruction_id: undefined })];
  test.beforeEach(async ({ page }) => {
    await signedIn(page);
    await page.route("**/api/v1/trades/summary*", (r) => r.fulfill({ json: { as_of: "2026-10-23T14:00:00Z", count: 2, total_pnl: "-2.50", pnl_basis: "net", total_gross_pnl: "1.00", total_commission: "3.50", uncommissioned_count: 1 } }));
    await page.route(/\/api\/v1\/trades(\?.*)?$/, (r) => r.fulfill({ json: { trades: rows } }));
    await page.route(new RegExp(`/api/v1/trades/${rows[0]!.trade_id}$`), (r) => r.fulfill({ json: rows[0] }));
    await page.route("**/api/v1/accounts", (r) => r.fulfill({ json: { accounts: [{ id: u(5), display_name: "Apex 50K" }] } }));
    await page.route("**/api/v1/sources*", (r) => r.fulfill({ json: [{ id: u(7), name: "Strikes MNQ" }] }));
    await page.route("**/api/v1/instruments", (r) => r.fulfill({ json: { instruments: [{ id: u(6), symbol: "MNQ", name: "Micro E-mini Nasdaq-100", exchange: "CME", currency: "USD", tick_size: "0.25", tick_value: "0.5", point_value: "2", price_decimals: 2 }] } }));
    await page.route("**/api/v1/trade-preview*", (r) => r.fulfill({ json: { duration_seconds: 750, pnl: "21.50", pnl_basis: "net", gross_pnl: "23.00", commission: "1.50", commission_source: "configured" } }));
  });
  test("list: totals, filters, rows (cards) reachable with no 2-D scroll", async ({ page }) => {
    await page.goto("/pt-BR/trades");
    await expect(page.getByRole("button", { name: /Apex 50K/ }).first()).toBeAttached();
    await reflowOk(page);
    await reachable(page.getByLabel("Totais do filtro"));
    await reachable(page.getByRole("button", { name: "30 dias" }));
    await reachable(page.getByRole("button", { name: /Apex 50K/ }).first());
    await reachable(page.getByRole("link", { name: "Registrar trade" }));
    await reflowOk(page);
  });
  test("detail dialog fits: content scrolls inside, close/actions reachable", async ({ page }) => {
    await page.goto("/pt-BR/trades");
    await page.getByRole("button", { name: /Apex 50K/ }).first().click();
    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();
    await reflowOk(page);
    await reachable(dialog.getByText("Trade capturado: não pode ser alterado."));
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
  });
  test("manual entry steps: fields, Continuar and Calculado block reachable on every step", async ({ page }) => {
    await page.goto("/pt-BR/trades/new");
    await expect(page.getByRole("heading", { level: 2, name: "Trade" })).toBeAttached();
    await reflowOk(page);
    await reachable(page.getByLabel("Quantidade"));
    await reachable(page.getByRole("button", { name: "Continuar" }));
  });
});
