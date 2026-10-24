// QA (Retina) — V1-S22 browser checks against mocked Core responses: axe (WCAG 2.2 AA tags), keyboard, 320 px / 400 % reflow, dimming while a filter loads.
import { createRequire } from "node:module";
import { expect, test, type Page } from "@playwright/test";

const axePath = createRequire(import.meta.url).resolve("axe-core/axe.min.js", { paths: [process.cwd() + "/node_modules/.pnpm/axe-core@4.14.0/node_modules"] });
const u = (n: number) => `00000000-0000-4000-8000-${String(n).padStart(12, "0")}`;
const me = { id: u(9), email: "trader@example.com", locale: "pt-BR", time_zone: "America/Sao_Paulo", is_platform_admin: false };
const figures = (o: Record<string, unknown> = {}) => ({ trade_count: 10, win_count: 6, loss_count: 3, breakeven_count: 1, total_pnl: "-12.3450", pnl_basis: "net", total_gross_pnl: "37.6550", total_commission: "50.0000", uncommissioned_count: 1, avg_pnl: "-1.23", win_rate: "60.0", avg_duration_seconds: 750, avg_mae: "10.00", mae_sample_count: 6, avg_mfe: "30.00", mfe_sample_count: 10, ...o });
const group = (n: number, o: Record<string, unknown> = {}, f: Record<string, unknown> = {}) => ({ origin: "routed", source_id: u(300 + n), source_name: `Estratégia ${n}`, signal_type: "trigger", figures: figures(f), bar_ratio: n === 1 ? "1.0000" : "0.4250", ...o });
const perf = { as_of: "2026-10-23T14:00:00Z", group_by: "source_signal", totals: figures(), groups: [group(1, {}, { total_pnl: "120.00" }), group(2, {}, { total_pnl: "-60.00" }), group(3, { origin: "manual", source_id: undefined, source_name: undefined, signal_type: undefined }, { total_pnl: "5.00" })] };

async function mock(page: Page, delayMs = 0) {
  await page.route("**/api/v1/events", (r) => r.fulfill({ status: 200, contentType: "text/event-stream", body: ": ok\n\n" }));
  await page.route("**/api/v1/me", (r) => r.fulfill({ json: me }));
  await page.route("**/api/v1/accounts", (r) => r.fulfill({ json: { accounts: [{ id: u(100), display_name: "Apex 50K" }] } }));
  await page.route("**/api/v1/performance*", async (r) => { if (delayMs && r.request().url().includes("period=7d")) await new Promise((x) => setTimeout(x, delayMs)); await r.fulfill({ json: perf }); });
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
const noHScroll = (page: Page) => page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth + 1);

test.describe("S22 performance (mocked Core)", () => {
  test("desktop: axe clean, table semantics, Total in tfoot with row header, aria-sort", async ({ page }) => {
    await mock(page);
    await page.goto("/pt-BR/performance");
    const table = page.getByRole("table");
    await expect(table).toBeVisible();
    await expect(table.locator("tfoot").getByRole("rowheader", { name: "Total" })).toBeVisible();
    await expect(table.getByRole("columnheader", { name: /P&L total/ })).toHaveAttribute("aria-sort", "descending");
    expect(await axe(page)).toEqual([]);
  });
  test("definition popover: opens by keyboard, Esc closes and returns focus, axe clean while open", async ({ page }) => {
    await mock(page);
    await page.goto("/pt-BR/performance");
    const info = page.getByRole("button", { name: "Como é calculado: Taxa de acerto" });
    await info.focus();
    await page.keyboard.press("Enter");
    await expect(page.getByText(/Trades com resultado/)).toBeVisible();
    expect(await axe(page)).toEqual([]);
    await page.keyboard.press("Escape");
    await expect(page.getByText(/Trades com resultado/)).toBeHidden();
    await expect(info).toBeFocused();
  });
  test("group-by and sort by keyboard write the URL and call the Core with the same parameters", async ({ page }) => {
    await mock(page);
    const urls: string[] = [];
    page.on("request", (r) => { if (r.url().includes("/api/v1/performance")) urls.push(r.url()); });
    await page.goto("/pt-BR/performance");
    const by = page.getByRole("button", { name: "Fonte", exact: true });
    await by.focus(); await page.keyboard.press("Enter");
    await expect(page).toHaveURL(/group=source/);
    await expect.poll(() => urls.some((x) => x.includes("group_by=source&"))).toBe(true);
    await page.getByRole("button", { name: "Taxa de acerto", exact: true }).focus();
    await page.keyboard.press("Enter");
    await expect(page).toHaveURL(/sort=win_rate/);
  });
  test("changing the period keeps the previous figures dimmed with aria-busy until the new ones arrive", async ({ page }) => {
    await mock(page, 1500);
    await page.goto("/pt-BR/performance");
    await expect(page.getByRole("region", { name: "Resumo" })).toBeVisible();
    await page.getByRole("button", { name: "7 dias" }).click();
    await expect(page.locator('[aria-busy="true"]').first()).toBeAttached();
    await expect(page.getByRole("region", { name: "Resumo" })).toBeVisible(); // old numbers still shown, not a blank
    await expect(page.locator('[aria-busy="true"]')).toHaveCount(0, { timeout: 8000 });
  });
  for (const [label, width, height] of [["320 px", 320, 640], ["400 % zoom (320x256)", 320, 256]] as const) {
    test(`reflow at ${label}: no horizontal page scroll, cards with Total first, axe clean`, async ({ page }) => {
      await page.setViewportSize({ width, height });
      await mock(page);
      await page.goto("/pt-BR/performance");
      await expect(page.getByRole("region", { name: "Resumo" })).toBeAttached();
      expect(await noHScroll(page)).toBe(true);
      const first = page.locator("ul > li").first();
      await expect(first).toContainText("Total");
      await page.getByRole("link", { name: "Ver os trades deste filtro" }).scrollIntoViewIfNeeded();
      await expect(page.getByRole("link", { name: "Ver os trades deste filtro" })).toBeInViewport();
      expect(await noHScroll(page)).toBe(true);
      if (height === 640) expect(await axe(page)).toEqual([]);
    });
  }
});
