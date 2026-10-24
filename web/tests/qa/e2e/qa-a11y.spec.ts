// QA (Lente) — S3 web: axe (WCAG 2.2 A/AA tags) + keyboard/focus checks. axe-core is injected from the lockfile's copy; no new dependency.
import type { Page } from "@playwright/test";
import { expect, test } from "./fixtures";

const AXE = "/srv/www/allwin.worktrees/qa-s3web-781631f-lente/web/node_modules/.pnpm/axe-core@4.14.0/node_modules/axe-core/axe.min.js";
const challenge = { challenge_id: "challenge-0000000001", expires_at: "2099-01-01T00:00:00Z", resend_after_s: 60 };
const security = { email: "trader@example.com", channel_change_allowed_at: "2020-01-01T00:00:00Z", session: { persistent: false, idle_timeout_s: 43200, absolute_timeout_s: 86400, idle_expires_at: "2099-01-01T00:00:00Z", absolute_expires_at: "2099-01-02T00:00:00Z" } };
const devices = [
  { id: "d1", label: "Chrome no Windows", first_seen_at: "2026-10-01T10:00:00Z", last_seen_at: "2026-10-23T10:00:00Z", current: true },
  { id: "d2", label: "Safari no iPhone", first_seen_at: "2026-10-05T10:00:00Z", last_seen_at: "2026-10-20T10:00:00Z", current: false },
];

async function axe(page: Page, label: string, known: string[] = []) {
  await page.addScriptTag({ path: AXE });
  const result = await page.evaluate(async (known) => {
    // @ts-expect-error injected global
    const r = await window.axe.run(document, { runOnly: { type: "tag", values: ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"] } });
    return r.violations.filter((v: { id: string }) => !known.includes(v.id)).map((v: { id: string; impact: string; nodes: { target: unknown[] }[] }) => ({ id: v.id, impact: v.impact, nodes: v.nodes.slice(0, 3).map((n) => n.target.join(" ") + " :: " + ((n as unknown as { html: string }).html ?? "").slice(0, 260)) }));
  }, known);
  expect(result, "axe violations on " + label).toEqual([]);
}

test.describe("S3 a11y — public pages", () => {
  test.beforeEach(async ({ api, page }) => {
    await api.signedOut();
    await page.route("**/api/v1/auth/login/challenges", (route) => route.fulfill({ status: 202, json: challenge }));
    await page.route("**/api/v1/auth/invitations/lookup", (route) => route.fulfill({ json: { email: "new@example.com", locale: "pt-BR", expires_at: "2099-01-01T00:00:00Z" } }));
    await page.route("**/api/v1/auth/challenges/delivery", (route) => route.fulfill({ json: { delivery: "sent" } }));
  });

  test("login page: axe + keyboard order + visible focus", async ({ page }) => {
    await page.goto("/pt-BR/login");
    await expect(page.getByRole("heading", { name: "Entrar" })).toBeVisible();
    await axe(page, "login");
    const order: string[] = [];
    for (let i = 0; i < 6; i++) {
      await page.keyboard.press("Tab");
      order.push(await page.evaluate(() => { const e = document.activeElement as HTMLElement; return (e.getAttribute("aria-label") || e.id || e.tagName) + ":" + (e.textContent || "").trim().slice(0, 20); }));
    }
    console.log("login tab order", JSON.stringify(order));
    await page.getByLabel(/E-mail/i).focus();
    const outline = await page.getByLabel(/E-mail/i).evaluate((e) => { const s = getComputedStyle(e); return s.outlineStyle + "|" + s.boxShadow; });
    expect(outline).not.toBe("none|none");
  });

  test("code page: axe, focus lands on the title/field, code field labelled, error announced politely", async ({ page }) => {
    await page.route("**/api/v1/auth/login/verify", (route) => route.fulfill({ status: 401, json: { error: { code: "auth.code_invalid" } } }));
    await page.goto("/pt-BR/login");
    await page.getByLabel(/E-mail/i).fill("trader@example.com");
    await page.keyboard.press("Enter");
    const code = page.getByLabel(/Código de verificação/);
    await expect(code).toBeVisible();
    await axe(page, "code page");
    await expect(code).toBeFocused(); // autofocus: keyboard users can type at once
    await code.pressSequentially("ABCD2345");
    const alert = page.locator("p[role=alert]");
    await expect(alert).toBeVisible();
    await expect(code).toHaveValue("");
    await expect(code).toBeFocused();
    await axe(page, "code page with error");
  });

  test("invite page: axe", async ({ page }) => {
    await page.goto("/pt-BR/invite#t=tok");
    await expect(page.locator("input[value=\"new@example.com\"]")).toBeVisible();
    await axe(page, "invite");
  });

  for (const width of [320, 400]) {
    test("login reflows at " + width + " px with no horizontal scroll and 2.5.8 target size", async ({ page }) => {
      await page.setViewportSize({ width, height: 700 });
      await page.goto("/pt-BR/login");
      await expect(page.getByRole("heading", { name: "Entrar" })).toBeVisible();
      expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
      for (const b of await page.getByRole("button").all()) {
        const box = await b.boundingBox();
        if (box) expect(Math.min(box.width, box.height), "button target >= 24 px").toBeGreaterThanOrEqual(24);
      }
    });
  }
});

test.describe("S3 a11y — signed in", () => {
  test.beforeEach(async ({ api, page }) => {
    await api.signedIn();
    await page.route("**/api/v1/me/security", (route) => route.fulfill({ json: security }));
    await page.route("**/api/v1/me/devices", (route) => route.fulfill({ json: devices }));
    await page.route("**/api/v1/sources", (route) => route.fulfill({ json: [] }));
    await page.route("**/api/v1/connections", (route) => route.fulfill({ json: [] }));
  });

  test("security page: axe, then 'Sair de todos' dialog traps focus, Esc closes and returns focus to the trigger", async ({ page }) => {
    await page.goto("/pt-BR/settings/security");
    const trigger = page.getByRole("button", { name: "Sair de todos os dispositivos" });
    await expect(trigger).toBeVisible();
    await axe(page, "security");
    await trigger.focus();
    await page.keyboard.press("Enter");
    const dialog = page.getByRole("alertdialog");
    await expect(dialog).toBeVisible();
    await axe(page, "sign-out-all dialog");
    for (let i = 0; i < 6; i++) {
      await page.keyboard.press("Tab");
      expect(await page.evaluate(() => !!document.activeElement?.closest('[role="alertdialog"]')), "focus stays inside the dialog").toBe(true);
    }
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await expect(trigger).toBeFocused();
  });

  test("change e-mail dialog: axe and keyboard-only open/cancel", async ({ page }) => {
    await page.goto("/pt-BR/settings/security");
    const trigger = page.getByRole("button", { name: "Alterar e-mail" });
    await trigger.focus();
    await page.keyboard.press("Enter");
    await expect(page.getByLabel("Novo e-mail")).toBeVisible();
    await axe(page, "change e-mail dialog");
    await page.keyboard.press("Escape");
    await expect(trigger).toBeFocused();
  });

  test("security page reflows at 320 px", async ({ page }) => {
    await page.setViewportSize({ width: 320, height: 700 });
    await page.goto("/pt-BR/settings/security");
    await expect(page.getByRole("button", { name: "Alterar e-mail" })).toBeVisible();
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  });
});
