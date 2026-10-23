import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { RulesForm } from "./rules-form";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const accountId = "00000000-0000-4000-8000-0000000000a1";
const rules = (overrides: Record<string, unknown> = {}) => ({
  account_id: accountId,
  enforcement_enabled: false,
  version: 4,
  rules: { drawdown: { type: "trailing_eod", amount: "2500.00" }, daily_loss_limit: { kind: "hard", amount: "1000.00" }, profit_target_on_reach: "notify" },
  ...overrides,
});

function serve(body: object = rules()) {
  server.use(http.get(`*/api/v1/accounts/${accountId}/risk-rules`, () => HttpResponse.json(body)));
}
const preview = (body: object = { changes: [], would_be_reached: false }) => http.post(`*/api/v1/accounts/${accountId}/risk-rules/preview`, () => HttpResponse.json(body));

beforeEach(() => {
  navigation.push.mockClear();
  server.use(http.get("*/api/v1/me", () => HttpResponse.json(user)));
});

describe("RulesForm", () => {
  it("fills the form from the Core and keeps flatten-and-block unavailable while enforcement is off", async () => {
    serve();
    renderApp(<RulesForm accountId={accountId} />);
    expect(await screen.findByLabelText("Limite de perda diária, em dólares")).toHaveValue("1000.00");
    expect(screen.getByLabelText("Só avisar", { exact: false })).toBeChecked();
    const flatten = screen.getByRole("radio", { name: /Fechar posições e bloquear/ });
    expect(flatten).toHaveAttribute("aria-disabled", "true");
    expect(screen.getByText("Disponível quando a aplicação automática estiver ligada nesta conta.")).toBeInTheDocument();
    await userEvent.setup().click(flatten);
    expect(screen.getByLabelText("Só avisar", { exact: false })).toBeChecked();
  });

  it("previews, then saves with the version read and goes back to the risk tab", async () => {
    serve();
    const saved: unknown[] = [];
    server.use(preview(), http.put(`*/api/v1/accounts/${accountId}/risk-rules`, async ({ request }) => {
      saved.push(await request.json());
      return HttpResponse.json(rules({ version: 5 }));
    }));
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    const loss = await screen.findByLabelText("Limite de perda diária, em dólares");
    await person.clear(loss);
    await person.type(loss, "800,5");
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    await waitFor(() => expect(navigation.push).toHaveBeenCalledWith(`/pt-BR/accounts/${accountId}/risk`));
    expect(saved).toEqual([{ version: 4, rules: { drawdown: { type: "trailing_eod", amount: "2500" }, daily_loss_limit: { kind: "hard", amount: "800.5" }, profit_target_on_reach: "notify" } }]);
  });

  it("asks before saving when the new value is already exceeded, and acknowledges it", async () => {
    serve();
    const saved: Record<string, unknown>[] = [];
    server.use(preview({ changes: [{ kind: "daily_loss", effective: "now", effective_from: "2026-10-23T14:00:00Z" }], would_be_reached: true }), http.put(`*/api/v1/accounts/${accountId}/risk-rules`, async ({ request }) => {
      saved.push((await request.json()) as Record<string, unknown>);
      return HttpResponse.json(rules());
    }));
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Salvar regras" }));
    const dialog = await screen.findByRole("alertdialog");
    expect(saved).toHaveLength(0);
    expect(dialog).toHaveTextContent("já passou do limite hoje");
    await person.click(screen.getByRole("button", { name: "Salvar e aplicar agora" }));
    await waitFor(() => expect(saved[0]).toMatchObject({ acknowledge_reached: true, version: 4 }));
  });

  it("maps a 422 pointer to its field", async () => {
    serve(rules({ rules: { profit_target_on_reach: "notify" } }));
    server.use(http.post(`*/api/v1/accounts/${accountId}/risk-rules/preview`, () => HttpResponse.json({ error: { code: "risk.daily_loss_required", pointer: "/rules/daily_loss_limit" } }, { status: 422 })));
    renderApp(<RulesForm accountId={accountId} />);
    await userEvent.setup().click(await screen.findByRole("button", { name: "Salvar regras" }));
    expect(await screen.findByText("Informe o limite de perda diária desta conta.")).toBeInTheDocument();
    expect(screen.getByLabelText("Limite de perda diária, em dólares")).toHaveAttribute("aria-invalid", "true");
  });

  it("rejects half a drawdown on the client and asks to reload on a version conflict", async () => {
    serve();
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    await person.clear(await screen.findByLabelText("Drawdown, em dólares"));
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    expect(await screen.findByText("Informe o valor do drawdown.")).toBeInTheDocument();

    server.use(http.post(`*/api/v1/accounts/${accountId}/risk-rules/preview`, () => HttpResponse.json({ error: { code: "common.version_conflict" } }, { status: 409 })));
    await person.type(screen.getByLabelText("Drawdown, em dólares"), "2000");
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    expect(await screen.findByText("Estas regras mudaram em outro lugar.")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Recarregar" })).toBeInTheDocument();
  });
});
