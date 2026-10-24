import type { Page } from "@playwright/test";
import { expect, test } from "./fixtures";

type Stored = Record<string, unknown> & { id: string; display_name: string; row_version: number; lifecycle_state: string };

const rule = (kind: "automation" | "copy", id: string) => ({
  id,
  kind,
  firm_stance: "unknown",
  is_generic: true,
  version_number: 1,
  published_at: "2026-10-01T00:00:00Z",
  quotes: [],
});

/** A small in-memory Core for the browser: list, create (idempotent), lifecycle, delete, rules. */
async function mockCore(page: Page) {
  const accounts = new Map<string, Stored>();
  const createBodies: Record<string, unknown>[] = [];
  await page.route("**/api/v1/connections", (route) => route.fulfill({ json: [] }));
  await page.route("**/api/v1/firms", (route) => route.fulfill({ json: { firms: [] } }));
  await page.route("**/api/v1/opt-in-rules", (route) => route.fulfill({ json: { automation: rule("automation", "1f2e3d4c-5b6a-4978-8a9b-0c1d2e3f4a5b"), copy: rule("copy", "3b4c5d6e-7f8a-4b9c-8d1e-2f3a4b5c6d7e") } }));
  await page.route(/\/api\/v1\/accounts(\/[^/?]+)?(\/[a-z-]+)?(\?.*)?$/, async (route) => {
    const request = route.request();
    const [, id, sub] = new URL(request.url()).pathname.match(/\/api\/v1\/accounts(?:\/([^/]+))?(?:\/([a-z-]+))?$/) ?? [];
    const method = request.method();
    if (!id) {
      if (method === "GET") return route.fulfill({ json: { accounts: [...accounts.values()] } });
      const body = request.postDataJSON() as Record<string, unknown>;
      createBodies.push(body);
      const created: Stored = {
        id: crypto.randomUUID(),
        kind: body.kind,
        display_name: String(body.display_name),
        size_amount: `${body.size_amount}.00`,
        currency: "USD",
        multiplier: body.multiplier ?? 1,
        max_position_qty: body.max_position_qty,
        lifecycle_state: "active",
        opt_ins: { automation: { kind: "automation", state: "not_required" }, copy: { kind: "copy", state: "not_required" } },
        has_open_position: false,
        row_version: 1,
        created_at: "2026-10-23T12:00:00Z",
        updated_at: "2026-10-23T12:00:00Z",
        ...(body.kind === "prop" ? { firm_name: body.firm_name, plan_name: body.plan_name, phase: body.phase } : {}),
      };
      accounts.set(created.id, created);
      return route.fulfill({ status: 201, json: created });
    }
    const account = accounts.get(id);
    if (!account) return route.fulfill({ status: 404, json: { error: { code: "common.not_found" } } });
    // Accounts here have no preset: the detail page asks for it and expects a 404.
    if (sub === "preset") return route.fulfill({ status: 404, json: { error: { code: "common.not_found" } } });
    if (sub === "archive-impact") return route.fulfill({ json: { has_open_position: false, routes: [], follower_links: [] } });
    if (sub === "lifecycle") {
      const next = { ...account, lifecycle_state: (request.postDataJSON() as { state: string }).state, row_version: account.row_version + 1 };
      accounts.set(id, next);
      return route.fulfill({ json: next });
    }
    if (method === "DELETE") {
      accounts.delete(id);
      return route.fulfill({ status: 204 });
    }
    return route.fulfill({ json: account });
  });
  return { accounts, createBodies };
}

test("registers a real account, pauses it and deletes it", async ({ page, api }) => {
  await api.signedIn();
  await mockCore(page);
  await page.goto("/pt-BR/accounts");
  await expect(page.getByText("Você ainda não cadastrou contas.")).toBeVisible();

  await page.getByRole("link", { name: "Nova conta" }).first().click();
  await page.getByRole("radio", { name: "Conta real em corretora" }).click();
  await page.getByRole("button", { name: "Continuar" }).click();
  await page.getByLabel("Nome").fill("Minha conta real");
  await page.getByLabel("Tamanho (USD)").fill("25000");
  await page.getByLabel("Máximo de contratos").fill("3");
  await page.getByRole("button", { name: "Continuar" }).click();
  await page.getByRole("button", { name: "Continuar" }).click();
  await expect(page.getByRole("heading", { name: "Revisar", level: 2 })).toBeVisible();
  await page.getByRole("button", { name: "Salvar conta" }).click();
  await expect(page.getByRole("heading", { name: "Minha conta real" })).toBeVisible();

  await page.goto("/pt-BR/accounts");
  await page.getByRole("button", { name: /Ações: Minha conta real/ }).click();
  await page.getByRole("menuitem", { name: "Pausar" }).click();
  await page.getByRole("button", { name: "Pausar conta" }).click();
  await expect(page.getByText("Pausada").first()).toBeVisible();

  await page.getByRole("button", { name: /Ações: Minha conta real/ }).click();
  await page.getByRole("menuitem", { name: "Excluir" }).click();
  await page.getByRole("button", { name: "Excluir conta" }).click();
  await expect(page.getByText("Você ainda não cadastrou contas.")).toBeVisible();
});

test("a prop account cannot be saved without an explicit automation choice", async ({ page, api }) => {
  await api.signedIn();
  const { createBodies } = await mockCore(page);
  await page.goto("/pt-BR/accounts/new");
  await page.getByRole("radio", { name: "Conta de prop firm" }).click();
  await page.getByRole("button", { name: "Continuar" }).click();
  await page.getByRole("button", { name: "Cadastrar sem preset" }).click();
  await page.getByRole("textbox", { name: "Firma" }).fill("Apex");
  await page.getByRole("textbox", { name: "Plano" }).fill("50K");
  await page.getByRole("textbox", { name: "Fase" }).fill("Avaliação");
  await page.getByRole("button", { name: "Continuar" }).click();
  await page.getByLabel("Nome", { exact: true }).fill("Apex 1");
  await page.getByLabel("Tamanho (USD)").fill("50000");
  await page.getByLabel("Máximo de contratos").fill("2");
  await page.getByRole("button", { name: "Continuar" }).click();
  await page.getByRole("button", { name: "Continuar" }).click();
  await expect(page.getByRole("radio", { name: /Não aceitar ordens automatizadas/ })).not.toBeChecked();
  await expect(page.getByRole("radio", { name: /Aceitar ordens automatizadas/ })).not.toBeChecked();
  await page.getByRole("button", { name: "Continuar" }).click();
  await expect(page.getByText("Escolha se esta conta aceita ordens automatizadas.").last()).toBeVisible();
  expect(createBodies).toHaveLength(0);

  await page.getByRole("radio", { name: /Não aceitar ordens automatizadas/ }).click();
  await page.getByRole("button", { name: "Continuar" }).click();
  await page.getByRole("button", { name: "Salvar conta" }).click();
  await expect(page).toHaveURL(/\/pt-BR\/accounts\/[0-9a-f-]{36}$/);
  expect(createBodies[0]).toMatchObject({ opt_ins: { automation: { choice: "declined" } } });
});

test.describe("english", () => {
  test("the form and list are translated", async ({ page, api }) => {
    await api.signedIn({ locale: "en" });
    await mockCore(page);
    await page.goto("/en/accounts");
    await expect(page.getByText("You haven't added any accounts yet.")).toBeVisible();
    await page.goto("/en/accounts/new");
    await page.getByRole("radio", { name: "Prop firm account" }).click();
    await page.getByRole("button", { name: "Continue" }).click();
    // With the catalog mocked empty the step shows the preset picker, whose Firm control is a combobox.
    await expect(page.getByRole("combobox", { name: /^Firm/ })).toBeVisible();
  });
});
