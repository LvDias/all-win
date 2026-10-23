// QA (Lente) — regression for the focus-stealing error alert (inline ref re-focusing on every render). Typing after an error must keep focus in the field.
import { expect, test } from "./fixtures";

test.describe("error alert never steals focus while typing", () => {
  test("login: after a generic failure alert the user keeps typing in the e-mail field", async ({ api, page }) => {
    await api.signedOut();
    await page.route("**/api/v1/auth/login/challenges", (route) => route.fulfill({ status: 500, json: { error: { code: "common.internal" } } }));
    await page.goto("/pt-BR/login");
    const email = page.getByLabel(/E-mail/i);
    await email.fill("trader@example.com");
    await page.getByRole("button", { name: /Enviar código/i }).click();
    await expect(page.getByText(/Não foi possível/).first()).toBeVisible();
    await email.focus();
    await email.pressSequentially("xyz", { delay: 40 });
    await expect(email).toBeFocused();
    await expect(email).toHaveValue("trader@example.comxyz");
  });

  test("change e-mail dialog: after a server error, typing a new address keeps focus in the field", async ({ api, page }) => {
    await api.signedIn();
    await page.route("**/api/v1/me/security", (route) => route.fulfill({ json: { email: "trader@example.com", channel_change_allowed_at: "2020-01-01T00:00:00Z", session: { persistent: false, idle_timeout_s: 43200, absolute_timeout_s: 86400, idle_expires_at: "2099-01-01T00:00:00Z", absolute_expires_at: "2099-01-02T00:00:00Z" } } }));
    await page.route("**/api/v1/me/devices", (route) => route.fulfill({ json: [] }));
    await page.route("**/api/v1/sources", (route) => route.fulfill({ json: [] }));
    await page.route("**/api/v1/connections", (route) => route.fulfill({ json: [] }));
    await page.route("**/api/v1/me/channel-changes", (route) => route.fulfill({ status: 422, json: { error: { code: "auth.channel_unavailable" } } }));
    await page.goto("/pt-BR/settings/security");
    await page.getByRole("button", { name: "Alterar e-mail" }).click();
    const field = page.getByLabel("Novo e-mail");
    await field.fill("a@example.com");
    await page.getByRole("button", { name: "Enviar códigos" }).click();
    await expect(page.getByRole("alert").filter({ hasText: /./ }).first()).toBeVisible();
    await field.focus();
    await field.pressSequentially("bc", { delay: 40 });
    await expect(field).toBeFocused();
    await expect(field).toHaveValue("a@example.combc");
  });
});

test("source name dialog: after a server error, typing in the name keeps focus in the field", async ({ api, page }) => {
  await api.signedIn();
  await page.route("**/api/v1/sources", (route) => (route.request().method() === "POST" ? route.fulfill({ status: 500, json: { error: { code: "common.internal" } } }) : route.fulfill({ json: [] })));
  await page.route("**/api/v1/connections", (route) => route.fulfill({ json: [] }));
  await page.goto("/pt-BR/connections/sources");
  await page.getByRole("button", { name: "Nova fonte" }).first().click();
  const name = page.getByLabel("Nome da fonte");
  await name.fill("Fonte");
  await page.getByRole("button", { name: "Criar" }).click();
  await expect(page.getByRole("dialog").getByRole("alert")).toBeVisible();
  await name.focus();
  await name.pressSequentially("XYZ", { delay: 40 });
  await expect(name).toBeFocused();
  await expect(name).toHaveValue("FonteXYZ");
});
