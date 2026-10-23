import { fireEvent, screen, waitFor } from "@testing-library/react";
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

const account = (overrides: Record<string, unknown> = {}) => ({
  id: accountId,
  kind: "prop",
  display_name: "Apex 50K",
  firm_name: "Apex",
  plan_name: "50K",
  phase: "evaluation",
  preset_version_id: "00000000-0000-4000-8000-0000000000b1",
  firm_platform: "rithmic",
  size_amount: "50000",
  currency: "USD",
  multiplier: 2,
  max_position_qty: 5,
  lifecycle_state: "active",
  opt_ins: { automation: { kind: "automation", state: "declined" }, copy: { kind: "copy", state: "declined" } },
  binding: { connection_id: "00000000-0000-4000-8000-0000000000c1", connection_name: "VPS", connection_status: "connected", platform_account: "Sim101", kind: "simulation", oco_location: "local" },
  commission_per_contract_side: "0.35",
  has_open_position: false,
  row_version: 7,
  created_at: "2026-10-01T12:00:00Z",
  updated_at: "2026-10-01T12:00:00Z",
  ...overrides,
});

function serveAccount(body: object = account()) {
  server.use(http.get(`*/api/v1/accounts/${accountId}`, () => HttpResponse.json(body)));
}

function serve(body: object = rules()) {
  server.use(http.get(`*/api/v1/accounts/${accountId}/risk-rules`, () => HttpResponse.json(body)));
}
const preview = (body: object = { changes: [], would_be_reached: false }) => http.post(`*/api/v1/accounts/${accountId}/risk-rules/preview`, () => HttpResponse.json(body));

beforeEach(() => {
  navigation.push.mockClear();
  server.use(http.get("*/api/v1/me", () => HttpResponse.json(user)));
  serveAccount();
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
    expect(saved).toEqual([{ version: 4, rules: { drawdown: { type: "trailing_eod", amount: "2500.00" }, daily_loss_limit: { kind: "hard", amount: "800.5" }, profit_target_on_reach: "notify" } }]);
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
    const loss = await screen.findByLabelText("Limite de perda diária, em dólares");
    await person.clear(loss);
    await person.type(loss, "100");
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
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
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("Valor da meta, em dólares"), "600");
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
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

  it("keeps a firm's \"no daily loss limit\" read-only and still saves other changes (M1)", async () => {
    serve(rules({ rules: { drawdown: { type: "static", amount: "2500.00" }, daily_loss_limit: { kind: "none" }, profit_target_on_reach: "notify" } }));
    const saved: Record<string, unknown>[] = [];
    server.use(preview(), http.put(`*/api/v1/accounts/${accountId}/risk-rules`, async ({ request }) => {
      saved.push((await request.json()) as Record<string, unknown>);
      return HttpResponse.json(rules({ version: 5 }));
    }));
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    expect(await screen.findByText("A firma informa que não há limite de perda diária.")).toBeInTheDocument();
    expect(screen.queryByLabelText("Limite de perda diária, em dólares")).not.toBeInTheDocument();
    await person.type(screen.getByLabelText("Valor da meta, em dólares"), "600");
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    await waitFor(() => expect(saved).toHaveLength(1));
    expect(saved[0]).toMatchObject({ rules: { daily_loss_limit: { kind: "none" }, daily_profit_target: { amount: "600" } } });
  });

  it("keeps a firm's \"other\" drawdown with its description when the user changes something else (M2)", async () => {
    serve(rules({ rules: { drawdown: { type: "other", amount: "3000.00", description: "Drawdown by closed balance" }, daily_loss_limit: { kind: "hard", amount: "1000.00" }, profit_target_on_reach: "notify" } }));
    const saved: Record<string, unknown>[] = [];
    server.use(preview(), http.put(`*/api/v1/accounts/${accountId}/risk-rules`, async ({ request }) => {
      saved.push((await request.json()) as Record<string, unknown>);
      return HttpResponse.json(rules({ version: 5 }));
    }));
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    expect(await screen.findByText("Regra da firma: Drawdown by closed balance")).toBeInTheDocument();
    const loss = screen.getByLabelText("Limite de perda diária, em dólares");
    await person.clear(loss);
    await person.type(loss, "900");
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    await waitFor(() => expect(saved).toHaveLength(1));
    expect(saved[0]).toMatchObject({ rules: { drawdown: { type: "other", amount: "3000.00", description: "Drawdown by closed balance" } } });
  });

  it("shows the commission prefilled and warns when there is none (H1)", async () => {
    serve();
    serveAccount(account({ commission_per_contract_side: undefined }));
    renderApp(<RulesForm accountId={accountId} />);
    expect(await screen.findByLabelText("Comissão por contrato e lado, em dólares")).toHaveValue("");
    expect(screen.getByText(/Comissão não informada\. Sem ela o resultado líquido é igual ao bruto/)).toBeInTheDocument();
  });

  it("saves a new commission through updateAccount with the stored account unchanged, binding included (H1)", async () => {
    serve();
    const rulesCalls: unknown[] = [];
    const accountBodies: Record<string, unknown>[] = [];
    server.use(http.put(`*/api/v1/accounts/${accountId}/risk-rules`, async ({ request }) => {
      rulesCalls.push(await request.json());
      return HttpResponse.json(rules());
    }), http.put(`*/api/v1/accounts/${accountId}`, async ({ request }) => {
      accountBodies.push((await request.json()) as Record<string, unknown>);
      return HttpResponse.json(account({ row_version: 8 }));
    }));
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    const fee = await screen.findByLabelText("Comissão por contrato e lado, em dólares");
    expect(fee).toHaveValue("0.35");
    await person.clear(fee);
    await person.type(fee, "0,5");
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    await waitFor(() => expect(navigation.push).toHaveBeenCalledWith(`/pt-BR/accounts/${accountId}/risk`));
    expect(rulesCalls).toHaveLength(0);
    expect(accountBodies).toEqual([
      {
        row_version: 7,
        display_name: "Apex 50K",
        max_position_qty: 5,
        multiplier: 2,
        preset_version_id: "00000000-0000-4000-8000-0000000000b1",
        firm_platform: "rithmic",
        binding: { connection_id: "00000000-0000-4000-8000-0000000000c1", platform_account: "Sim101" },
        commission_per_contract_side: "0.5",
      },
    ]);
  });

  it("clearing the commission sends null and an unchanged one is left out (H1)", async () => {
    serve();
    const accountBodies: Record<string, unknown>[] = [];
    server.use(http.put(`*/api/v1/accounts/${accountId}`, async ({ request }) => {
      accountBodies.push((await request.json()) as Record<string, unknown>);
      return HttpResponse.json(account());
    }));
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    await person.clear(await screen.findByLabelText("Comissão por contrato e lado, em dólares"));
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    await waitFor(() => expect(accountBodies).toHaveLength(1));
    expect(accountBodies[0]).toMatchObject({ commission_per_contract_side: null });
  });

  it("saves rules and commission in that order, and a commission failure after the rules says the rules were saved (H1)", async () => {
    serve();
    const order: string[] = [];
    server.use(preview(), http.put(`*/api/v1/accounts/${accountId}/risk-rules`, () => {
      order.push("rules");
      return HttpResponse.json(rules({ version: 5 }));
    }), http.put(`*/api/v1/accounts/${accountId}`, () => {
      order.push("account");
      return HttpResponse.json({ error: { code: "common.invalid_request", pointer: "/commission_per_contract_side" } }, { status: 400 });
    }));
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    const loss = await screen.findByLabelText("Limite de perda diária, em dólares");
    await person.clear(loss);
    await person.type(loss, "900");
    const fee = screen.getByLabelText("Comissão por contrato e lado, em dólares");
    await person.clear(fee);
    await person.type(fee, "1");
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    expect(await screen.findByText("As regras foram salvas, mas a comissão não. Corrija o campo e salve de novo.")).toBeInTheDocument();
    expect(order).toEqual(["rules", "account"]);
    expect(navigation.push).not.toHaveBeenCalled();
    // Retrying repeats only the commission: the rules are already saved.
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    await waitFor(() => expect(order).toEqual(["rules", "account", "account"]));
  });

  it("lets a prop account edit the reset time of its copy and shows the CME close for a real one (H1)", async () => {
    serve(rules({ rules: { daily_loss_limit: { kind: "hard", amount: "1000.00" }, profit_target_on_reach: "notify", trading_day_reset: { local_time: "17:00", time_zone: "America/New_York" } } }));
    const saved: Record<string, unknown>[] = [];
    server.use(preview(), http.put(`*/api/v1/accounts/${accountId}/risk-rules`, async ({ request }) => {
      saved.push((await request.json()) as Record<string, unknown>);
      return HttpResponse.json(rules({ version: 5 }));
    }));
    const view = renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    const time = await screen.findByLabelText("Horário do reset do dia (America/New_York)");
    expect(time).toHaveValue("17:00");
    fireEvent.change(time, { target: { value: "16:00" } });
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    await waitFor(() => expect(saved).toHaveLength(1));
    expect(saved[0]).toMatchObject({ rules: { trading_day_reset: { local_time: "16:00", time_zone: "America/New_York" } } });
    view.unmount();

    serve();
    renderApp(<RulesForm accountId={accountId} />);
    expect(await screen.findByText(/fechamento da sessão CME/)).toBeInTheDocument();
    expect(screen.queryByLabelText(/Horário do reset do dia/)).not.toBeInTheDocument();
  });

  it("focuses the first invalid field and shows the value it understood before saving (F4)", async () => {
    serve();
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    const loss = await screen.findByLabelText("Limite de perda diária, em dólares");
    await person.clear(loss);
    await person.type(loss, "1000,5");
    expect(screen.getByText("Entendido como US$ 1.000,50")).toBeInTheDocument();
    const target = screen.getByLabelText("Valor da meta, em dólares");
    await person.type(target, "abc");
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    await waitFor(() => expect(target).toHaveFocus());
    expect(target).toHaveAttribute("aria-invalid", "true");
  });

  it("refuses an ambiguous thousands separator in a money field instead of guessing (Q1)", async () => {
    serve();
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    const loss = await screen.findByLabelText("Limite de perda diária, em dólares");
    await person.clear(loss);
    await person.type(loss, "2.500");
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    expect(await screen.findByText("Digite a parte decimal explicitamente, por exemplo 2500,00 ou 2,5.")).toBeInTheDocument();
    expect(screen.queryByText("Entendido como US$ 2,50")).not.toBeInTheDocument();
  });

  it("asks before discarding unsaved changes (F5)", async () => {
    serve();
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    expect(await screen.findByRole("link", { name: "Cancelar" })).toBeInTheDocument();
    const loss = screen.getByLabelText("Limite de perda diária, em dólares");
    await person.type(loss, "1");
    await person.click(screen.getByRole("button", { name: "Cancelar" }));
    const dialog = await screen.findByRole("alertdialog");
    expect(dialog).toHaveTextContent("Descartar alterações?");
    await person.click(screen.getByRole("button", { name: "Continuar editando" }));
    expect(navigation.push).not.toHaveBeenCalled();
    await person.click(screen.getByRole("button", { name: "Cancelar" }));
    await person.click(await screen.findByRole("button", { name: "Descartar" }));
    expect(navigation.push).toHaveBeenCalledWith(`/pt-BR/accounts/${accountId}/risk`);
  });

  it("shows not-found, without retry, when the rules of another user's account answer 404 (F6)", async () => {
    server.use(http.get(`*/api/v1/accounts/${accountId}/risk-rules`, () => HttpResponse.json({ error: { code: "common.not_found" } }, { status: 404 })));
    renderApp(<RulesForm accountId={accountId} />);
    expect(await screen.findByText("Não encontrado.")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Tentar novamente" })).not.toBeInTheDocument();
  });

  it("a second rules edit after a partial failure is saved too, at the version the first save returned (N1)", async () => {
    serve();
    const rulesBodies: Record<string, unknown>[] = [];
    let accountCalls = 0;
    server.use(preview(), http.put(`*/api/v1/accounts/${accountId}/risk-rules`, async ({ request }) => {
      rulesBodies.push((await request.json()) as Record<string, unknown>);
      return HttpResponse.json(rules({ version: 4 + rulesBodies.length }));
    }), http.put(`*/api/v1/accounts/${accountId}`, () => {
      accountCalls += 1;
      return accountCalls === 1 ? HttpResponse.json({ error: { code: "common.invalid_request", pointer: "/commission_per_contract_side" } }, { status: 400 }) : HttpResponse.json(account({ row_version: 8 }));
    }));
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    const loss = await screen.findByLabelText("Limite de perda diária, em dólares");
    await person.clear(loss);
    await person.type(loss, "900");
    const fee = screen.getByLabelText("Comissão por contrato e lado, em dólares");
    await person.clear(fee);
    await person.type(fee, "1");
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    expect(await screen.findByText("As regras foram salvas, mas a comissão não. Corrija o campo e salve de novo.")).toBeInTheDocument();
    expect(navigation.push).not.toHaveBeenCalled();

    // The user keeps editing the rules before retrying the commission (focus was moved to the commission field).
    await person.click(loss);
    await person.clear(loss);
    await person.type(loss, "800");
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    await waitFor(() => expect(navigation.push).toHaveBeenCalledWith(`/pt-BR/accounts/${accountId}/risk`));
    expect(rulesBodies).toHaveLength(2);
    expect(rulesBodies[0]).toMatchObject({ version: 4, rules: { daily_loss_limit: { amount: "900" } } });
    expect(rulesBodies[1]).toMatchObject({ version: 5, rules: { daily_loss_limit: { amount: "800" } } });
  });

  it("drops the firm's drawdown description when the type is no longer 'other' (nit)", async () => {
    serve(rules({ rules: { drawdown: { type: "other", amount: "3000.00", description: "By closed balance" }, daily_loss_limit: { kind: "hard", amount: "1000.00" }, profit_target_on_reach: "notify" } }));
    const saved: Record<string, unknown>[] = [];
    server.use(preview(), http.put(`*/api/v1/accounts/${accountId}/risk-rules`, async ({ request }) => {
      saved.push((await request.json()) as Record<string, unknown>);
      return HttpResponse.json(rules({ version: 5 }));
    }));
    renderApp(<RulesForm accountId={accountId} />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("combobox", { name: /Tipo de drawdown/ }));
    await person.click(await screen.findByRole("option", { name: "Estático" }));
    await person.click(screen.getByRole("button", { name: "Salvar regras" }));
    await waitFor(() => expect(saved).toHaveLength(1));
    expect((saved[0]?.rules as { drawdown: Record<string, unknown> }).drawdown).toEqual({ type: "static", amount: "3000.00" });
  });
});
