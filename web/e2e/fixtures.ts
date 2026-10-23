import { test as base, expect, type Page } from "@playwright/test";

export const profile = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "America/Sao_Paulo", is_platform_admin: false };

/** Every test fails on a visible missing-translation marker or an i18n console error (spec V1-S4 §7). */
export const test = base.extend<{ api: { signedOut: () => Promise<void>; signedIn: (overrides?: Partial<typeof profile>) => Promise<void> } }>({
  page: async ({ page }, use) => {
    const i18nErrors: string[] = [];
    page.on("console", (message) => {
      if (message.type() === "error" && message.text().includes("[i18n]")) i18nErrors.push(message.text());
    });
    await use(page);
    await expect(page.getByText("⟦missing:")).toHaveCount(0);
    expect(i18nErrors).toEqual([]);
  },
  api: async ({ page }, use) => {
    await use({
      signedOut: () => mockMe(page, 401),
      signedIn: (overrides = {}) => mockMe(page, 200, { ...profile, ...overrides }),
    });
  },
});

async function mockMe(page: Page, status: number, body?: unknown) {
  await page.route("**/api/v1/me", (route) => {
    if (status === 401) return route.fulfill({ status, json: { error: { code: "auth.unauthenticated" } } });
    if (route.request().method() === "PATCH") {
      const patch = route.request().postDataJSON() as Record<string, string>;
      body = { ...(body as object), ...patch };
    }
    return route.fulfill({ status, json: body });
  });
}

export { expect };
