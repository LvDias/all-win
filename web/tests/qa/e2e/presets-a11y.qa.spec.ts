// QA (Retina) — V1-S16 browser checks (mocked Core): axe (WCAG 2.2 AA tags), keyboard/focus, 320 px and 400 % reflow on the catalog, the firm detail, the account preset panel + version dialog and the "Nova conta" picker.
import { createRequire } from "node:module";
import { expect, test, type Page } from "@playwright/test";

const axePath = createRequire(import.meta.url).resolve("axe-core/axe.min.js", { paths: [process.cwd() + "/node_modules/.pnpm/axe-core@4.14.0/node_modules"] });
const u = (n: number) => `00000000-0000-4000-8000-${String(n).padStart(12, "0")}`;
const me = { id: u(9), email: "t@example.com", locale: "pt-BR", time_zone: "America/Sao_Paulo", is_platform_admin: false };
const acc = u(500), cur = u(20), nxt = u(99);
const rel = (n: number) => ({ version_number: n, published_at: "2026-10-22T10:00:00Z", verified_on: "2026-10-21" });
const src = { url: "https://firm.example/rules", accessed_at: "2026-10-18", page_title: "Rules" };
const verified = (value: unknown) => ({ status: "verified", value, sources: [src] });
const quote = (text: string) => ({ text, language: "en", page_title: "Rules", url: "https://firm.example/rules", accessed_at: "2026-10-18" });
const summary = { id: u(1), slug: "apex", name: "Apex", plan_names: ["Plan A", "Plan B", "Plan C"], stances: { automation: "allowed", copy: "not_confirmed", vps: "allowed", personal_device: "unknown" }, not_confirmed_count: 2, release: rel(3) };
const firmDetail = {
  summary,
  stances: { automation: [{ id: u(30), kind: "automation", firm_stance: "allowed", is_generic: false, firm_id: u(1), firm_name: "Apex", version_number: 3, published_at: rel(3).published_at, quotes: [quote("Automated trading is permitted.")] }], copy: [{ id: u(31), kind: "copy", firm_stance: "not_confirmed", is_generic: false, firm_id: u(1), firm_name: "Apex", version_number: 3, published_at: rel(3).published_at, quotes: [quote("Copy A."), quote("Copy B.")] }], vps: { value: "allowed", quotes: [quote("Servers are fine.")] }, personal_device: { value: "unknown", quotes: [], research: { searched_on: "2026-10-18", sources: [src] } } },
  platforms: [{ code: "rithmic", label: "Rithmic" }],
  plans: [{ plan_id: u(10), name: "Plan A", phases: [{ phase: "evaluation", sizes: [{ preset_id: u(40), preset_version_id: cur, size_amount: "50000.00", size_label: "50K", platform_specific: false, not_confirmed_count: 0 }] }] }],
  excluded_plans: ["Legacy"],
};
const versionBody = (id: string, n: number, drawdown: unknown) => ({ id, preset_id: u(40), firm_id: u(1), firm_name: "Apex", plan_id: u(10), plan_name: "Plan A", phase: "evaluation", size_amount: "50000.00", release: rel(n), not_confirmed_count: 0, rule_sets: [{ rules: { max_position_qty: verified({ contracts: 60 }), drawdown, daily_loss_limit: verified({ kind: "none" }), trading_day_reset: verified({ local_time: "17:00", time_zone: "America/Chicago", published_text: "5:00 PM CT" }), evaluation_profit_target: verified({ amount: "3000.00" }) } }] });
const conflicting = { status: "not_confirmed", published_values: [{ value: { type: "trailing_eod", amount: "2600.00" }, sources: [src] }, { value: { type: "static", amount: "2000.00" }, sources: [{ ...src, url: "https://firm.example/faq", page_title: "FAQ" }] }] };
const account = { id: acc, kind: "prop", display_name: "Apex 50K", firm_name: "Apex", plan_name: "Plan A", phase: "evaluation", size_amount: "50000.00", currency: "USD", multiplier: 1, max_position_qty: 60, lifecycle_state: "active", has_open_position: false, opt_ins: { automation: { kind: "automation", state: "not_required" }, copy: { kind: "copy", state: "not_required" } }, row_version: 7, created_at: "2026-10-23T12:00:00Z", updated_at: "2026-10-23T12:00:00Z" };

async function mock(page: Page) {
  await page.route("**/api/v1/events", (r) => r.fulfill({ status: 200, contentType: "text/event-stream", body: ": ok\n\n" }));
  await page.route("**/api/v1/me", (r) => r.fulfill({ json: me }));
  await page.route("**/api/v1/connections", (r) => r.fulfill({ json: [] }));
  await page.route("**/api/v1/firms", (r) => r.fulfill({ json: { firms: [summary, { ...summary, id: u(2), name: "Topstep", plan_names: ["Combine"], not_confirmed_count: 0, manual_only: { reason: quote("Orders must originate from your device.") } }] } }));
  await page.route(/\/api\/v1\/firms\/[^/]+\/releases$/, (r) => r.fulfill({ json: { firm_id: u(1), releases: [{ version_number: 3, published_at: "2026-10-20T00:00:00Z", verified_on: "2026-10-18", changes: [{ kind: "stance_text_changed", stance: "automation", material: true, reason: { "pt-BR": "Nova exigência.", en: "New requirement." } }], vps: { value: "allowed", quotes: [] }, personal_device: { value: "unknown", quotes: [] }, rule_versions: [] }] } }));
  await page.route(/\/api\/v1\/firms\/[^/]+$/, (r) => r.fulfill({ json: firmDetail }));
  await page.route("**/api/v1/preset-versions/*", (r) => r.fulfill({ json: new URL(r.request().url()).pathname.endsWith(nxt) ? versionBody(nxt, 4, conflicting) : versionBody(cur, 3, verified({ type: "trailing_eod", amount: "2500.00" })) }));
  await page.route("**/api/v1/accounts/**", (r) => {
    const path = new URL(r.request().url()).pathname;
    if (path.endsWith("/preset-diff")) return r.fulfill({ json: { account_id: acc, from_preset_version_id: cur, to_preset_version_id: nxt, firm_platform_required: false, opt_in_changes: { automation: null, copy: null }, rows: [{ field: "drawdown", account_value: { type: "static", amount: "1800.00" }, current_preset_value: { type: "trailing_eod", amount: "2500.00" }, edited: false, requires_choice: true, cap_conflict: false }] } });
    if (path.endsWith("/preset")) return r.fulfill({ json: { account_id: acc, preset_version_id: cur, release: rel(3), row_version: 7, fields: [{ field: "drawdown", origin: "edited", account_value: { type: "static", amount: "1800.00" }, preset_value: { type: "trailing_eod", amount: "2500.00" } }], newer_version: { preset_version_id: nxt, release: rel(4), suspends: false } } });
    return r.fulfill({ json: account });
  });
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
/** Elements that stick out past the viewport and are not inside a container that scrolls on its own: names the culprit when the page scrolls sideways. */
const wideOffenders = (page: Page) =>
  page.evaluate(() => {
    const limit = document.documentElement.clientWidth + 1;
    const out: string[] = [];
    for (const el of Array.from(document.querySelectorAll<HTMLElement>("body *"))) {
      const r = el.getBoundingClientRect();
      if (r.width === 0 || r.right <= limit) continue;
      let p: HTMLElement | null = el.parentElement; let contained = false;
      while (p && p !== document.body) { const o = getComputedStyle(p).overflowX; if (o === "auto" || o === "scroll" || o === "hidden" || o === "clip") { contained = true; break; } p = p.parentElement; }
      if (!contained) out.push(`${el.tagName.toLowerCase()}.${(el.className || "").toString().slice(0, 60)} right=${Math.round(r.right)} w=${Math.round(r.width)}`);
    }
    return out.slice(0, 6);
  });
/** Every element wider than the viewport, with the overflow of its ancestors, to see which container really leaks. */
const widestChain = (page: Page) =>
  page.evaluate(() => {
    const limit = document.documentElement.clientWidth + 1;
    const out: string[] = [];
    for (const el of Array.from(document.querySelectorAll<HTMLElement>("body *"))) {
      const r = el.getBoundingClientRect();
      if (r.width === 0 || r.right <= limit) continue;
      const chain: string[] = []; let p: HTMLElement | null = el;
      while (p && p !== document.documentElement) { const cs = getComputedStyle(p); chain.push(`${p.tagName.toLowerCase()}[ox=${cs.overflowX},pos=${cs.position},w=${Math.round(p.getBoundingClientRect().width)}]`); p = p.parentElement; }
      out.push(`${el.tagName.toLowerCase()} right=${Math.round(r.right)} :: ${chain.slice(0, 7).join(" < ")}`);
      if (out.length >= 3) break;
    }
    return out;
  });
/** Bisect: hide each element in turn; the ones whose removal shrinks the document's scrollWidth are what makes the page scroll sideways. */
const culprits = (page: Page) =>
  page.evaluate(() => {
    const root = document.documentElement;
    const base = root.scrollWidth;
    const found: string[] = [];
    for (const el of Array.from(document.querySelectorAll<HTMLElement>("body *"))) {
      const before = el.style.display;
      el.style.display = "none";
      const now = root.scrollWidth;
      el.style.display = before;
      if (now < base - 20 && el.children.length < 3) found.push(`${el.tagName.toLowerCase()}.${(el.className || "").toString().slice(0, 70)} (${Math.round(el.getBoundingClientRect().left)}..${Math.round(el.getBoundingClientRect().right)}) -> ${now}`);
      if (found.length >= 4) break;
    }
    return found;
  });
const noHScroll = (page: Page) => page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth + 1);

test.describe("S16 screens (mocked Core)", () => {
  test.beforeEach(async ({ page }) => { await mock(page); });

  test("nav shows 'Presets' (no missing-translation marker) and the catalog is axe clean", async ({ page }) => {
    await page.goto("/pt-BR/presets");
    await expect(page.getByRole("heading", { level: 1, name: /Presets/ })).toBeVisible();
    // On narrow screens (mobile project) the side menu is a drawer behind the "Abrir menu" button: open it first, as a user would.
    const menuButton = page.getByRole("button", { name: "Abrir menu" });
    if (await menuButton.isVisible()) await menuButton.click();
    await expect(page.getByRole("link", { name: /^Presets/ }).first()).toBeVisible();
    await expect(page.getByText("⟦missing:")).toHaveCount(0);
    expect(await axe(page)).toEqual([]);
  });
  for (const [label, width, height] of [["320 px", 320, 640], ["400 % zoom", 320, 256]] as const) {
    test(`catalog reflows at ${label}: cards, no horizontal page scroll`, async ({ page }) => {
      await page.setViewportSize({ width, height });
      await page.goto("/pt-BR/presets");
      await expect(page.getByRole("link", { name: "Apex" }).first()).toBeAttached();
      expect(await noHScroll(page)).toBe(true);
      if (height === 640) expect(await axe(page)).toEqual([]);
    });
  }
  test("firm detail: each tab is axe clean (stances, plans and rules, versions)", async ({ page }) => {
    await page.goto(`/pt-BR/presets/${u(1)}`);
    await expect(page.getByRole("heading", { level: 1, name: "Apex" })).toBeVisible();
    expect(await axe(page)).toEqual([]);
    await page.getByRole("tab", { name: "Planos e regras" }).click();
    await expect(page.getByRole("rowheader", { name: "50K" })).toBeVisible();
    expect(await axe(page)).toEqual([]);
    await page.getByRole("tab", { name: "Versões" }).click();
    await expect(page.getByText("Mudança material")).toBeVisible();
    expect(await axe(page)).toEqual([]);
  });
  test("firm detail at 400 % zoom: no 2-D page scroll (the rules table scrolls inside its container)", async ({ page }) => {
    await page.setViewportSize({ width: 320, height: 256 });
    await page.goto(`/pt-BR/presets/${u(1)}`);
    await expect(page.getByRole("heading", { level: 1, name: "Apex" })).toBeAttached();
    await page.getByRole("tab", { name: "Planos e regras" }).click();
    await expect(page.getByRole("rowheader", { name: "50K" })).toBeAttached();
    const hs = await noHScroll(page);
    expect(hs, `page scrolls sideways; offenders: ${JSON.stringify(await wideOffenders(page))} culprits: ${JSON.stringify(await culprits(page))} scrollWidth=${await page.evaluate(() => document.documentElement.scrollWidth)}`).toBe(true);
  });
  test("account detail: preset panel and version dialog are axe clean, Esc returns focus to 'Ver mudanças', 320 px and 400 % have no horizontal page scroll", async ({ page }) => {
    await page.goto(`/pt-BR/accounts/${acc}`);
    await expect(page.getByText("Há uma nova versão do preset da Apex")).toBeVisible();
    expect(await axe(page)).toEqual([]);
    const trigger = page.getByRole("button", { name: "Ver mudanças" });
    await trigger.focus(); await page.keyboard.press("Enter");
    const dialog = page.getByRole("dialog", { name: /Mudanças da versão 4/ });
    await expect(dialog).toBeVisible();
    await expect(dialog.getByRole("radio").first()).toBeVisible();
    for (const radio of await dialog.getByRole("radio").all()) await expect(radio).not.toBeChecked(); // nothing preselected
    expect(await axe(page)).toEqual([]);
    for (const [w, h] of [[320, 640], [320, 256]] as const) { await page.setViewportSize({ width: w, height: h }); expect(await noHScroll(page)).toBe(true); }
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await expect(trigger).toBeFocused();
  });
  test("Nova conta picker: cascade is axe clean and 'Cadastrar sem preset' is reachable at 400 % zoom", async ({ page }) => {
    await page.route("**/api/v1/opt-in-rules**", (r) => r.fulfill({ json: { automation: { id: u(60), kind: "automation", firm_stance: "unknown", is_generic: false, firm_name: "Apex", version_number: 1, published_at: "2026-10-01T00:00:00Z", quotes: [] }, copy: { id: u(61), kind: "copy", firm_stance: "unknown", is_generic: false, firm_name: "Apex", version_number: 1, published_at: "2026-10-01T00:00:00Z", quotes: [] } } }));
    await page.goto("/pt-BR/accounts/new");
    await page.getByRole("radio", { name: "Conta de prop firm" }).click();
    await page.getByRole("button", { name: "Continuar" }).click();
    await expect(page.getByRole("combobox", { name: /^Firma/ })).toBeVisible();
    expect(await axe(page)).toEqual([]);
    await page.setViewportSize({ width: 320, height: 256 });
    const manual = page.getByRole("button", { name: "Cadastrar sem preset" });
    await manual.scrollIntoViewIfNeeded();
    await expect(manual).toBeInViewport();
    expect(await noHScroll(page)).toBe(true);
  });
});
