import type { Page } from "@playwright/test";
import { expect, test } from "./fixtures";

const keyId = "AbCdEfGhIjKl";
const secret = "x".repeat(43);
const webhookUrl = `https://allwin.example/ingress/v1/tradingview/awk_${keyId}_${secret}`;
const source = { id: "00000000-0000-4000-8000-0000000000a1", name: "Strikes MNQ", source_type: "tradingview", is_automated: true, status: "active", key_id: keyId, masked_url: `https://allwin.example/ingress/v1/tradingview/awk_${keyId}_…`, created_at: "2026-10-20T12:00:00Z" };
const connection = { id: "00000000-0000-4000-8000-0000000000c1", name: "VPS Chicago", status: "awaiting_pairing", status_changed_at: "2026-10-23T11:57:00Z", created_at: "2026-10-20T12:00:00Z" };
const security = {
  email: "trader@example.com",
  channel_change_allowed_at: "2020-01-01T00:00:00Z",
  session: { persistent: false, idle_timeout_s: 43200, absolute_timeout_s: 86400, idle_expires_at: "2099-01-01T00:00:00Z", absolute_expires_at: "2099-01-02T00:00:00Z" },
};
const challenge = { challenge_id: "challenge-1", expires_at: "2099-01-01T00:00:00Z", resend_after_s: 60 };

async function mockSecurity(page: Page) {
  await page.route("**/api/v1/me/security", (route) => route.fulfill({ json: security }));
  await page.route("**/api/v1/me/devices", (route) => route.fulfill({ json: [] }));
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

  test("login: keyboard-only sign-in by e-mail code lands on the app home, without any password field", async ({ page, api }) => {
    await api.signedOut();
    await page.route("**/api/v1/auth/login/challenges", (route) => route.fulfill({ status: 202, json: challenge }));
    const verified: unknown[] = [];
    await page.route("**/api/v1/auth/login/verify", async (route) => {
      verified.push(route.request().postDataJSON());
      await api.signedIn();
      return route.fulfill({ json: { user: { id: "1", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false }, new_device: false } });
    });
    await mockSecurity(page);
    await page.goto("/pt-BR/login");
    await expect(page.getByLabel(/senha/i)).toHaveCount(0);
    await page.getByLabel("E-mail").fill("trader@example.com");
    await page.keyboard.press("Enter");
    await expect(page).toHaveURL(/\/pt-BR\/login\/code$/);
    expect(page.url()).not.toContain("trader");
    await page.getByLabel(/Código de verificação/).fill("ABCD2345");
    await expect(page).toHaveURL(/\/pt-BR$/);
    expect(verified).toEqual([{ challenge_id: "challenge-1", code: "ABCD2345", remember_me: false }]);
    await expect(page.getByRole("navigation", { name: "Navegação principal" }).or(page.getByRole("button", { name: "Abrir menu" }))).toBeVisible();
  });

  test("login: a wrong code shows one message and clears the field", async ({ page, api }) => {
    await api.signedOut();
    await page.route("**/api/v1/auth/login/challenges", (route) => route.fulfill({ status: 202, json: challenge }));
    await page.route("**/api/v1/auth/login/verify", (route) => route.fulfill({ status: 401, json: { error: { code: "auth.code_invalid" } } }));
    await page.goto("/pt-BR/login");
    await page.getByLabel("E-mail").fill("trader@example.com");
    await page.getByRole("button", { name: "Enviar código" }).click();
    const field = page.getByLabel(/Código de verificação/);
    await field.fill("WRONG123");
    await expect(page.getByText("Código inválido ou expirado. Confira ou peça um novo.")).toBeVisible();
    await expect(field).toHaveValue("");
  });

  test("the code page without a pending login goes back to the e-mail page", async ({ page, api }) => {
    await api.signedOut();
    await page.goto("/pt-BR/login/code");
    await expect(page).toHaveURL(/\/pt-BR\/login$/);
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
    // Once the Core ends the session, /me answers 401 too (as the real server does).
    let ended = false;
    await page.route("**/api/v1/me", (route) => (ended ? route.fulfill({ status: 401, json: { error: { code: "auth.unauthenticated" } } }) : route.fallback()));
    await page.route("**/api/v1/events", (route) => {
      ended = true;
      return route.fulfill({ status: 200, contentType: "text/event-stream", body: 'event: session.ended\ndata: {"type":"session.ended","at":"2026-10-23T12:00:00Z","reason":"signed_out_everywhere"}\n\n' });
    });
    await page.route("**/api/v1/sources", (route) => route.fulfill({ json: [] }));
    await page.goto("/pt-BR/connections/sources");
    await expect(page).toHaveURL(/\/pt-BR\/login\?reason=sessions_ended/);
    await expect(page.getByRole("heading", { level: 1, name: "Entrar" })).toBeVisible();
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

  test("Esc returns focus to the control that opened the dialog (WCAG 2.4.3)", async ({ page }) => {
    await page.route("**/api/v1/sources", (route) => route.fulfill({ json: [source] }));
    await page.route("**/api/v1/connections", (route) =>
      route.request().method() === "POST"
        ? route.fulfill({ status: 201, json: { connection, pairing: { pairing_code: "K7QM-4D2P", expires_at: new Date(Date.now() + 600_000).toISOString() } } })
        : route.fulfill({ json: [] }),
    );

    await page.goto("/pt-BR/connections/sources");
    const create = page.getByRole("button", { name: "Nova fonte" });
    await create.focus();
    await page.keyboard.press("Enter");
    await expect(page.getByRole("dialog")).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(create).toBeFocused();

    const kebab = page.getByRole("button", { name: "Ações para Strikes MNQ" });
    await kebab.focus();
    await page.keyboard.press("Enter");
    await page.getByRole("menuitem", { name: "Revogar" }).click();
    await expect(page.getByRole("alertdialog")).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(kebab).toBeFocused();

    await page.goto("/pt-BR/connections/ninjatrader");
    const pair = page.getByRole("button", { name: "Nova conexão" }).first();
    await pair.focus();
    await page.keyboard.press("Enter");
    await page.getByLabel("Nome da conexão").fill("VPS");
    await page.keyboard.press("Enter");
    await expect(page.getByLabel("Código de pareamento")).toHaveText("K7QM-4D2P");
    await page.keyboard.press("Escape");
    await expect(pair).toBeFocused();

    await page.goto("/pt-BR/settings/security");
    const change = page.getByRole("button", { name: "Alterar e-mail" });
    await change.focus();
    await page.keyboard.press("Enter");
    await expect(page.getByRole("dialog")).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(change).toBeFocused();
  });
});
