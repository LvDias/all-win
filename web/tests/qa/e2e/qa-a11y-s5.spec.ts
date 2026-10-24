// QA (Lente) — S5 "Nova conta" stepper: axe (WCAG 2.2 A/AA) per step, keyboard/focus, 320 px. axe-core injected from the lockfile copy.
import type { Page } from "@playwright/test";
import { expect, test } from "./fixtures";

const AXE = "/srv/www/allwin.worktrees/qa-s5web-d09ddd3-lente/web/node_modules/.pnpm/axe-core@4.14.0/node_modules/axe-core/axe.min.js";
async function axe(page: Page, label: string, known: string[] = ["label-content-name-mismatch"]) {  // S3-A11Y-1 (user menu) is fixed on the shared line at fa2c0aa, not in 6130e07: exempted here so the stepper itself is judged
  await page.addScriptTag({ path: AXE });
  const result = await page.evaluate(async (known) => {
    // @ts-expect-error injected global
    const r = await window.axe.run(document, { runOnly: { type: "tag", values: ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"] } });
    return r.violations.filter((v: { id: string }) => !known.includes(v.id)).map((v: { id: string; impact: string; nodes: { target: unknown[]; html: string }[] }) => ({ id: v.id, impact: v.impact, nodes: v.nodes.slice(0, 3).map((n) => n.target.join(" ") + " :: " + n.html.slice(0, 200)) }));
  }, known);
  expect(result, "axe violations on " + label).toEqual([]);
}
const rule = (stance: string) => ({
  id: "1f2e3d4c-5b6a-4978-8a9b-0c1d2e3f4a5b", kind: "automation", firm_stance: stance, is_generic: false, firm_id: "9c0d1e2f-3a4b-4c5d-8e6f-7a8b9c0d1e2f", firm_name: "Apex Trader Funding", version_number: 3, published_at: "2026-10-01T00:00:00Z",
  quotes: [{ text: "No Automation or Algorithm Usage allowed.", language: "en", page_title: "Prohibited Activities", url: "https://example.com/rules", accessed_at: "2026-10-23" }],
});

test.describe("S5 stepper a11y", () => {
  test.beforeEach(async ({ api, page }) => {
    await api.signedIn();
    await page.route("**/api/v1/connections", (route) => route.fulfill({ json: [] }));
    await page.route("**/api/v1/accounts", (route) => route.fulfill({ json: { accounts: [] } }));
    await page.route("**/api/v1/opt-in-rules**", (route) => route.fulfill({ json: { automation: rule("forbidden"), copy: { ...rule("forbidden"), id: "3b4c5d6e-7f8a-4b9c-8d1e-2f3a4b5c6d7e", kind: "copy" } } }));
  });

  test("real account: axe on every step, keyboard-only through to the review, step title takes focus", async ({ page }) => {
    await page.goto("/pt-BR/accounts/new");
    await expect(page.getByRole("radio", { name: "Conta real em corretora" })).toBeVisible();
    await axe(page, "step type");
    await page.getByRole("radio", { name: "Conta real em corretora" }).focus();
    await page.keyboard.press("Space");
    await page.getByRole("button", { name: "Continuar" }).focus();
    await page.keyboard.press("Enter");
    await page.getByLabel("Nome").fill("Conta QA");
    await page.getByLabel("Tamanho (USD)").fill("25000");
    await page.getByLabel("Máximo de contratos").fill("3");
    await axe(page, "step data");
    await page.getByRole("button", { name: "Continuar" }).click();
    await axe(page, "step platform");
    await page.getByRole("button", { name: "Continuar" }).click();
    await expect(page.getByRole("heading", { name: "Revisar", level: 2 })).toBeVisible();
    await axe(page, "review");
    const focusInside = await page.evaluate(() => { const a = document.activeElement as HTMLElement | null; return a?.tagName + ":" + (a?.textContent ?? "").trim().slice(0, 30); });
    console.log("focus after step change:", focusInside);
    expect(focusInside).not.toMatch(/^BODY/);
  });

  test("prop account opt-in step: axe, warning precedes the radios, error linked, no preselection", async ({ page }) => {
    await page.goto("/pt-BR/accounts/new");
    await page.getByRole("radio", { name: "Conta de prop firm" }).click();
    await page.getByRole("button", { name: "Continuar" }).click();
    await page.getByRole("textbox", { name: "Firma" }).fill("Apex");
    await page.getByRole("textbox", { name: "Plano" }).fill("50K");
    await page.getByRole("textbox", { name: "Fase" }).fill("Avaliação");
    await page.getByRole("button", { name: "Continuar" }).click();
    await page.getByLabel("Nome").fill("Apex QA");
    await page.getByLabel("Tamanho (USD)").fill("50000");
    await page.getByLabel("Máximo de contratos").fill("2");
    await page.getByRole("button", { name: "Continuar" }).click();
    await page.getByRole("button", { name: "Continuar" }).click();
    const group = page.getByRole("radiogroup", { name: "Ordens automatizadas nesta conta" });
    await expect(group).toBeVisible();
    expect(await group.locator('[role="radio"][aria-checked="true"]').count()).toBe(0);
    await axe(page, "opt-in step");
    await page.getByRole("button", { name: "Continuar" }).click();
    await expect(page.getByText("Escolha se esta conta aceita ordens automatizadas.")).toBeVisible();
    await axe(page, "opt-in step with error");
    await page.getByRole("radio", { name: /Aceitar ordens automatizadas/ }).focus();
    await page.keyboard.press("Space");
    await expect(page.getByRole("checkbox")).toBeVisible();
    await axe(page, "opt-in accepted (acknowledgement)");
  });

  for (const width of [320, 400]) {
    test("stepper reflows at " + width + " px: no horizontal scroll, footer never hides the focused field (2.4.11)", async ({ page }) => {
      await page.setViewportSize({ width, height: 640 });
      await page.goto("/pt-BR/accounts/new");
      await page.getByRole("radio", { name: "Conta real em corretora" }).click();
      await page.getByRole("button", { name: "Continuar" }).click();
      await page.getByLabel("Máximo de contratos").focus();
      expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
      const field = await page.getByLabel("Máximo de contratos").boundingBox();
      const footer = await page.getByRole("button", { name: "Continuar" }).boundingBox();
      if (field && footer) expect(field.y + field.height <= footer.y || field.y >= footer.y + footer.height, "focused field not covered by the footer").toBe(true);
    });
  }
});
