import { assertNoI18nProblems, expect, test } from "./fixtures";

test.describe("locale resolution", () => {
  test.use({ locale: "en-GB" });

  test("first visit follows the browser language", async ({ page, api }) => {
    await api.signedOut();
    await page.goto("/");
    await expect(page).toHaveURL(/\/en$/);
    await expect(page.locator("html")).toHaveAttribute("lang", "en");
  });

  test("keeps path and query on unprefixed links", async ({ page, api }) => {
    await api.signedOut();
    await page.goto("/settings/preferences?x=1");
    await expect(page).toHaveURL(/\/en\/settings\/preferences\?x=1$/);
  });
});

test.describe("unsupported browser language", () => {
  test.use({ locale: "fr-FR" });

  test("falls back to pt-BR", async ({ page, api }) => {
    await api.signedOut();
    await page.goto("/");
    await expect(page).toHaveURL(/\/pt-BR$/);
    await expect(page.locator("html")).toHaveAttribute("lang", "pt-BR");
  });
});

test("the NEXT_LOCALE cookie wins over the browser", async ({ page, api, context, baseURL }) => {
  await api.signedOut();
  await context.addCookies([{ name: "NEXT_LOCALE", value: "en", url: baseURL }]);
  await page.goto("/");
  await expect(page).toHaveURL(/\/en$/);
});

test("signed-out visitor switches language: URL, lang, heading focus and cookie", async ({ page, api, context }) => {
  await api.signedOut();
  await page.goto("/pt-BR");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("All Win");
  await page.getByRole("button", { name: /Idioma: Português/ }).click();
  await page.getByRole("menuitemradio", { name: "English" }).click();
  await expect(page).toHaveURL(/\/en$/);
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
  await expect(page.getByRole("link", { name: "Preferences" })).toBeVisible();
  await expect(page.getByRole("heading", { level: 1 })).toBeFocused();
  expect((await context.cookies()).find((c) => c.name === "NEXT_LOCALE")?.value).toBe("en");
});

test("signed-in user: the profile language wins over the URL prefix", async ({ page, api }) => {
  await api.signedIn({ locale: "pt-BR" });
  await page.goto("/en/settings/preferences");
  await expect(page).toHaveURL(/\/pt-BR\/settings\/preferences$/);
  await expect(page.getByRole("heading", { level: 1, name: "Preferências" })).toBeVisible();
});

test("the proxy redirects a session cookie holder to the persisted language", async ({ page }) => {
  // `__Host-` cookies cannot be stored over http, so the header is sent as the browser would on https.
  const get = (path: string) => page.request.get(path, { maxRedirects: 0, headers: { cookie: "NEXT_LOCALE=en; __Host-aw_session=x" } });
  const redirected = await get("/pt-BR/settings/preferences?x=1");
  expect(redirected.status()).toBe(307);
  expect(redirected.headers()["location"]).toMatch(/\/en\/settings\/preferences\?x=1$/);
  // Invite links keep their embedded locale (the page itself arrives in S3, so only the redirect is asserted).
  expect((await get("/pt-BR/invite")).status()).not.toBe(307);
});

test("preferences: switching language saves it in the profile", async ({ page, api }) => {
  await api.signedIn({ locale: "pt-BR" });
  await page.goto("/pt-BR/settings/preferences");
  await expect(page.getByText("Também usado pelo plugin do NinjaTrader a partir da próxima conexão.")).toBeVisible();
  const patch = page.waitForRequest((request) => request.method() === "PATCH" && request.url().endsWith("/api/v1/me"));
  await page.getByLabel("Idioma da interface").click();
  await page.getByRole("option", { name: "English" }).click();
  expect((await patch).postDataJSON()).toEqual({ locale: "en" });
  await expect(page).toHaveURL(/\/en\/settings\/preferences$/);
  await expect(page.getByLabel("Interface language")).toBeVisible();
});

test("preferences: time zone search and save", async ({ page, api }) => {
  await api.signedIn({ locale: "en" });
  await page.goto("/en/settings/preferences");
  await page.getByRole("combobox", { name: "Time zone" }).click();
  await page.getByRole("combobox", { name: "Search time zone" }).fill("chicago");
  await page.getByRole("option", { name: /Chicago/ }).click();
  await expect(page.getByText("Preference saved.")).toBeVisible();
});

test("no horizontal scroll at 320 px", async ({ page, api }) => {
  await api.signedIn({ locale: "pt-BR" });
  await page.setViewportSize({ width: 320, height: 640 });
  await page.goto("/pt-BR/settings/preferences");
  await expect(page.getByLabel("Idioma da interface")).toBeVisible();
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(overflow).toBeLessThanOrEqual(0);
});

test("the missing-translation gate detects a marker and an i18n console error", async ({ page }) => {
  await page.setContent("<p>⟦missing: sources.create.title⟧</p>");
  await expect(assertNoI18nProblems(page, [])).rejects.toThrow();
  await page.setContent("<p>ok</p>");
  await expect(assertNoI18nProblems(page, ["[i18n] missing message: a.b"])).rejects.toThrow();
  await assertNoI18nProblems(page, []);
});
