import type { Page } from "@playwright/test";
import { expect, test } from "./fixtures";

const keyId = "AbCdEfGhIjKl";
const secret = "x".repeat(43);
const webhookUrl = `https://allwin.example/ingress/v1/tradingview/awk_${keyId}_${secret}`;
const source = { id: "00000000-0000-4000-8000-0000000000a1", name: "Strikes MNQ", source_type: "tradingview", is_automated: true, status: "active", key_id: keyId, masked_url: `https://allwin.example/ingress/v1/tradingview/awk_${keyId}_…`, created_at: "2026-10-20T12:00:00Z" };
const connection = { id: "00000000-0000-4000-8000-0000000000c1", name: "VPS Chicago", status: "awaiting_pairing", status_changed_at: "2026-10-23T11:57:00Z", created_at: "2026-10-20T12:00:00Z" };
const security = { totp_enabled: false, recovery_codes_remaining: 0, session: {} };

async function mockSecurity(page: Page) {
  await page.route("**/api/v1/me/security", (route) => route.fulfill({ json: security }));
}

test.describe("public pages", () => {
  test("invite: no-referrer, fragment removed at once, token only in the body", async ({ page, api }) => {
    await api.signedOut();
    const bodies: unknown[] = [];
    await page.route("**/api/v1/auth/invitations/lookup", (route) => {
      bodies.push(route.request().postDataJSON());
      return route.fulfill({ json: { email: "new@example.com", locale: "pt-BR", expires_at: "2026-10-30T12:00:00Z" } });
    });
    const response = await page.goto("/pt-BR/invite#t=SECRET-TOKEN");
    expect(response?.headers()["referrer-policy"]).toBe("no-referrer");
    await expect(page.getByLabel("E-mail")).toHaveValue("new@example.com");
    expect(page.url()).not.toContain("SECRET-TOKEN");
    expect(await page.evaluate(() => window.location.hash)).toBe("");
    expect(bodies).toEqual([{ token: "SECRET-TOKEN" }]);
  });

  test("invite without a token shows the generic invalid message", async ({ page, api }) => {
    await api.signedOut();
    await page.goto("/en/invite");
    await expect(page.getByText("This invitation isn't valid.")).toBeVisible();
  });

  test("reset link: no-referrer and the token leaves the address bar", async ({ page, api }) => {
    await api.signedOut();
    const response = await page.goto("/en/reset-password#t=RESET-TOKEN");
    expect(response?.headers()["referrer-policy"]).toBe("no-referrer");
    await expect(page.getByLabel("New password")).toBeVisible();
    expect(page.url()).not.toContain("RESET-TOKEN");
  });

  test("login: keyboard-only sign-in lands on the app home", async ({ page, api }) => {
    await api.signedOut();
    await page.route("**/api/v1/auth/login", async (route) => {
      await api.signedIn();
      return route.fulfill({ json: { state: "authenticated", user: { id: "1", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false } } });
    });
    await mockSecurity(page);
    await page.goto("/pt-BR/login");
    await page.getByLabel("E-mail").fill("trader@example.com");
    await page.keyboard.press("Tab");
    await page.getByLabel("Senha", { exact: true }).fill("a long passphrase here");
    await page.keyboard.press("Enter");
    await expect(page).toHaveURL(/\/pt-BR$/);
    await expect(page.getByRole("navigation", { name: "Navegação principal" }).or(page.getByRole("button", { name: "Abrir menu" }))).toBeVisible();
  });

  test("login: wrong credentials show one message and keep the e-mail", async ({ page, api }) => {
    await api.signedOut();
    await page.route("**/api/v1/auth/login", (route) => route.fulfill({ status: 401, json: { error: { code: "auth.invalid_credentials" } } }));
    await page.goto("/pt-BR/login");
    await page.getByLabel("E-mail").fill("trader@example.com");
    await page.getByLabel("Senha", { exact: true }).fill("wrong password");
    await page.getByRole("button", { name: "Entrar" }).click();
    await expect(page.getByText("E-mail ou senha incorretos.")).toBeVisible();
    await expect(page.getByLabel("E-mail")).toHaveValue("trader@example.com");
  });

  test("an app page without a session redirects to login keeping returnTo", async ({ page, api }) => {
    await api.signedOut();
    await page.goto("/pt-BR/connections/sources");
    await expect(page).toHaveURL(/\/pt-BR\/login\?returnTo=%2Fpt-BR%2Fconnections%2Fsources/);
  });
});

test.describe("signed in", () => {
  test.beforeEach(async ({ api, page }) => {
    await api.signedIn();
    await mockSecurity(page);
  });

  test("sources: create shows the URL once, masked until revealed; the list keeps only the masked URL", async ({ page, context }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    let created = false;
    await page.route("**/api/v1/sources", (route) => {
      if (route.request().method() === "POST") {
        created = true;
        return route.fulfill({ status: 201, json: { source, webhook_url: webhookUrl } });
      }
      return route.fulfill({ json: created ? [source] : [] });
    });
    await page.goto("/pt-BR/connections/sources");
    await page.getByRole("button", { name: "Nova fonte" }).first().click();
    await page.getByLabel("Nome da fonte").fill("Strikes MNQ");
    await page.getByRole("button", { name: "Criar" }).click();
    const dialog = page.getByRole("dialog");
    await expect(dialog.getByText(/awk_AbCdEfGhIjKl_••••••••/)).toBeVisible();
    expect(await dialog.innerHTML()).not.toContain(secret);
    await dialog.getByRole("button", { name: "Revelar" }).click();
    await expect(dialog.getByText(webhookUrl)).toBeVisible();
    await dialog.getByRole("button", { name: "Copiar" }).click();
    await dialog.getByRole("button", { name: "Concluir" }).click();
    await expect(page.getByRole("dialog")).toHaveCount(0);
    await expect(page.getByText("Strikes MNQ")).toBeVisible();
    await expect(page.locator("body")).not.toContainText(secret);
  });

  test("connections: pairing code is visible and the dialog closes without a guard", async ({ page }) => {
    await page.route("**/api/v1/connections", (route) => {
      if (route.request().method() === "POST") return route.fulfill({ status: 201, json: { connection, pairing: { pairing_code: "K7QM-4D2P", expires_at: new Date(Date.now() + 600_000).toISOString() } } });
      return route.fulfill({ json: [] });
    });
    await page.goto("/pt-BR/connections/ninjatrader");
    await page.getByRole("button", { name: "Nova conexão" }).first().click();
    await page.getByLabel("Nome da conexão").fill("VPS Chicago");
    await page.getByRole("button", { name: "Criar" }).click();
    await expect(page.getByLabel("Código de pareamento")).toHaveText("K7QM-4D2P");
    await page.keyboard.press("Escape");
    await expect(page.getByRole("dialog")).toHaveCount(0);
  });

  test("session.ended on the stream sends the user to login with the mapped reason", async ({ page }) => {
    await page.route("**/api/v1/events", (route) =>
      route.fulfill({ status: 200, contentType: "text/event-stream", body: 'event: session.ended\ndata: {"type":"session.ended","at":"2026-10-23T12:00:00Z","reason":"password_changed"}\n\n' }),
    );
    await page.route("**/api/v1/sources", (route) => route.fulfill({ json: [] }));
    await page.goto("/pt-BR/connections/sources");
    await expect(page).toHaveURL(/\/pt-BR\/login\?reason=sessions_ended/);
  });

  for (const path of ["connections/sources", "connections/ninjatrader", "settings/security"]) {
    test(`no horizontal scroll at 320 px: ${path}`, async ({ page }) => {
      await page.setViewportSize({ width: 320, height: 700 });
      await page.route("**/api/v1/sources", (route) => route.fulfill({ json: [source] }));
      await page.route("**/api/v1/connections", (route) => route.fulfill({ json: [connection] }));
      await page.goto(`/pt-BR/${path}`);
      await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
      expect(await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth)).toBe(true);
    });
  }
});
