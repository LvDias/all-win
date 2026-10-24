import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { apexDetail, ids, unconfirmedDrawdown, version } from "@/features/presets/testing/fixtures";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
import { NewAccountWithPresets } from "./new-account-with-presets";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "t@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const rule = (kind: "automation" | "copy", id: string) => ({ id, kind, firm_stance: "unknown", is_generic: false, firm_name: "Apex", version_number: 1, published_at: "2026-10-01T00:00:00Z", quotes: [] });
const automationRule = "1f2e3d4c-5b6a-4978-8a9b-0c1d2e3f4a5b";

function serve() {
  const created: { body: Record<string, unknown>; key: string | null }[] = [];
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(user)),
    http.get("*/api/v1/connections", () => HttpResponse.json([])),
    http.get("*/api/v1/firms", () => HttpResponse.json({ firms: [apexDetail.summary] })),
    http.get("*/api/v1/firms/:id", () => HttpResponse.json(apexDetail)),
    http.get("*/api/v1/preset-versions/:id", () => HttpResponse.json(version(ids.v50, "50000.00", { drawdown: unconfirmedDrawdown }))),
    http.get("*/api/v1/opt-in-rules", ({ request }) => {
      expect(new URL(request.url).searchParams.get("preset_version_id")).toBe(ids.v50);
      return HttpResponse.json({ automation: rule("automation", automationRule), copy: rule("copy", "3b4c5d6e-7f8a-4b9c-8d1e-2f3a4b5c6d7e") });
    }),
    http.post("*/api/v1/accounts", async ({ request }) => {
      created.push({ body: (await request.json()) as Record<string, unknown>, key: request.headers.get("Idempotency-Key") });
      return HttpResponse.json({ id: "00000000-0000-4000-8000-0000000000aa" }, { status: 201 });
    }),
  );
  return created;
}

async function choose(label: string | RegExp, option: RegExp | string) {
  await userEvent.click(screen.getByRole("combobox", { name: label }));
  await userEvent.click(await screen.findByRole("option", { name: option }));
}

describe("Nova conta with a firm preset", () => {
  it("picks a preset, forces the required not-confirmed value to be resolved, and creates the account from the preset", async () => {
    const created = serve();
    renderApp(<NewAccountWithPresets />);
    await userEvent.click(await screen.findByRole("radio", { name: "Conta de prop firm" }));
    await userEvent.click(screen.getByRole("button", { name: "Continuar" }));

    await choose(/^Firma/, /Apex/);
    await choose("Plano", "Plan A");
    await choose("Fase", "Avaliação");
    await choose("Tamanho", "50K");
    // The new version's drawdown is not confirmed: required, nothing preselected.
    const group = await screen.findByRole("group", { name: /Drawdown/ });
    for (const radio of within(group).getAllByRole("radio")) expect(radio).not.toBeChecked();
    await userEvent.click(screen.getByRole("button", { name: "Continuar" }));
    expect((await screen.findAllByText("Resolva este valor para salvar.")).length).toBeGreaterThan(0);
    expect(created).toHaveLength(0);

    await userEvent.click(within(group).getByRole("radio", { name: /Estático/ }));
    await userEvent.click(screen.getByRole("button", { name: "Continuar" }));

    // Data step: the size comes from the preset, the maximum position is prefilled with the preset's.
    expect(await screen.findByText(/Vem do preset\./)).toBeInTheDocument();
    expect(screen.queryByLabelText("Tamanho (USD)")).not.toBeInTheDocument();
    expect(screen.getByLabelText("Máximo de contratos")).toHaveValue("60");
    await userEvent.type(screen.getByLabelText("Nome", { exact: true }), "Apex 1");
    await userEvent.clear(screen.getByLabelText("Máximo de contratos"));
    await userEvent.type(screen.getByLabelText("Máximo de contratos"), "70");
    await userEvent.click(screen.getByRole("button", { name: "Continuar" }));
    expect(await screen.findByText("Não pode passar do máximo do preset.")).toBeInTheDocument();
    await userEvent.clear(screen.getByLabelText("Máximo de contratos"));
    await userEvent.type(screen.getByLabelText("Máximo de contratos"), "2");
    await userEvent.click(screen.getByRole("button", { name: "Continuar" }));
    await userEvent.click(await screen.findByRole("button", { name: "Continuar" }));
    await userEvent.click(await screen.findByRole("radio", { name: /Não aceitar ordens automatizadas/ }));
    await userEvent.click(screen.getByRole("button", { name: "Continuar" }));
    await userEvent.click(await screen.findByRole("button", { name: "Salvar conta" }));

    await waitFor(() => expect(created).toHaveLength(1));
    const { body } = created[0] ?? { body: {} };
    expect(body).toMatchObject({
      kind: "prop",
      display_name: "Apex 1",
      preset_version_id: ids.v50,
      max_position_qty: 2,
      rule_values: { drawdown: { type: "static", amount: "2000.00" } },
      opt_ins: { automation: { choice: "declined", rule_version_id: automationRule } },
    });
    for (const manual of ["firm_name", "plan_name", "phase", "size_amount", "profile_size"]) expect(body).not.toHaveProperty(manual);
  });

  it("'Cadastrar sem preset' switches to the free-text firm, plan and phase and back", async () => {
    serve();
    renderApp(<NewAccountWithPresets />);
    await userEvent.click(await screen.findByRole("radio", { name: "Conta de prop firm" }));
    await userEvent.click(screen.getByRole("button", { name: "Continuar" }));
    await userEvent.click(await screen.findByRole("button", { name: "Cadastrar sem preset" }));
    expect(await screen.findByRole("textbox", { name: "Firma" })).toBeInTheDocument();
    await userEvent.click(screen.getByRole("button", { name: "Escolher um preset da firma" }));
    expect(await screen.findByRole("combobox", { name: /^Firma/ })).toBeInTheDocument();
  });

  it("keeps 'Cadastrar sem preset' available when the firm catalog fails to load", async () => {
    serve();
    server.use(http.get("*/api/v1/firms", () => HttpResponse.json({ code: "internal", status: 500 }, { status: 500 })));
    renderApp(<NewAccountWithPresets />);
    await userEvent.click(await screen.findByRole("radio", { name: "Conta de prop firm" }));
    await userEvent.click(screen.getByRole("button", { name: "Continuar" }));
    expect(await screen.findByRole("button", { name: "Tentar novamente" })).toBeInTheDocument();
    await userEvent.click(screen.getByRole("button", { name: "Cadastrar sem preset" }));
    expect(await screen.findByRole("textbox", { name: "Firma" })).toBeInTheDocument();
  });
});
