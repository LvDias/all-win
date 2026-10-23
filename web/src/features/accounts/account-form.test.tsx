// The navigation mock lives in render-accounts and must load before the components.
import { nav, renderAccounts } from "./testing/render-accounts";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { describe, expect, it, vi } from "vitest";
import { server } from "@/test/server";
import { AccountFormPage } from "./account-form";
import { ids, propAccount, ruleVersion } from "./testing/fixtures";
import { accountHandlers, apiError } from "./testing/handlers";
import type { StepUpRunner } from "./lib/use-step-up";

const connection = {
  id: ids.conn,
  name: "Meu PC",
  status: "connected",
  status_changed_at: "2026-10-20T00:00:00Z",
  created_at: "2026-10-20T00:00:00Z",
  visible_accounts: [
    { platform_account: "Sim101", kind: "simulation", oco_location: "local" },
    { platform_account: "Sim102", kind: "simulation", oco_location: "local", account_id: ids.acc1 },
  ],
};
const connectionsHandler = http.get("/api/v1/connections", () => HttpResponse.json([connection]));

async function chooseKind(user: ReturnType<typeof userEvent.setup>, label: string) {
  await user.click(await screen.findByRole("radio", { name: label }));
}

async function fillCommon(user: ReturnType<typeof userEvent.setup>, name: string) {
  await user.type(screen.getByLabelText("Nome"), name);
  await user.type(screen.getByLabelText("Tamanho (USD)"), "25000");
  await user.type(screen.getByLabelText("Máximo de contratos"), "3");
}

vi.setConfig({ testTimeout: 20_000 });

describe("AccountFormPage (create)", () => {
  it("creates a real account with one idempotency key and no opt-in", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage />);
    await chooseKind(user, "Conta real em corretora");
    await fillCommon(user, "Minha conta");
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.create).toHaveLength(1));
    const { body, key } = calls.create[0] ?? { body: null, key: null };
    expect(body).toEqual({ kind: "real", display_name: "Minha conta", size_amount: "25000", max_position_qty: 3, multiplier: 1 });
    expect(key).toMatch(/^[0-9a-f-]{36}$/);
    await waitFor(() => expect(nav.push).toHaveBeenCalled());
  });

  it("refuses a fractional multiplier and zero max position without calling the API", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage />);
    await chooseKind(user, "Conta real em corretora");
    await user.type(screen.getByLabelText("Nome"), "X");
    await user.type(screen.getByLabelText("Tamanho (USD)"), "1000");
    await user.type(screen.getByLabelText("Máximo de contratos"), "0");
    await user.clear(screen.getByLabelText("Multiplicador"));
    await user.type(screen.getByLabelText("Multiplicador"), "1.5");
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    expect(await screen.findByText("Use um número inteiro maior ou igual a 1.")).toBeInTheDocument();
    expect(screen.getByText("Informe um número inteiro de contratos, 1 ou mais.")).toBeInTheDocument();
    expect(calls.create).toHaveLength(0);
  });

  it("shows the name-taken error from the Core inline", async () => {
    const { handlers } = accountHandlers([propAccount({ display_name: "Duplicada" })]);
    server.use(connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage />);
    await chooseKind(user, "Conta real em corretora");
    await fillCommon(user, "duplicada");
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    expect(await screen.findByText("Já existe uma conta com este nome.")).toBeInTheDocument();
  });

  it("offers bound platform accounts as disabled with the reason", async () => {
    const { handlers } = accountHandlers([propAccount({ binding: { connection_id: ids.conn, connection_name: "Meu PC", connection_status: "connected", platform_account: "Sim102", kind: "simulation", oco_location: "local" } })]);
    server.use(connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage />);
    await chooseKind(user, "Conta real em corretora");
    await user.click(await screen.findByRole("combobox", { name: "Conta no NinjaTrader" }));
    const taken = await screen.findByRole("option", { name: /Sim102.*Já vinculada a Apex 50K 1/ });
    expect(taken).toHaveAttribute("aria-disabled", "true");
  });

  async function fillProp(user: ReturnType<typeof userEvent.setup>) {
    await chooseKind(user, "Conta de prop firm");
    await user.type(screen.getByLabelText("Nome"), "Apex nova");
    await user.type(screen.getByLabelText("Firma"), "Apex");
    await user.type(screen.getByLabelText("Plano"), "50K");
    await user.type(screen.getByLabelText("Fase"), "Avaliação");
    await user.type(screen.getByLabelText("Tamanho (USD)"), "50000");
    await user.type(screen.getByLabelText("Máximo de contratos"), "2");
  }

  it("requires an explicit opt-in choice with nothing preselected", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage />);
    await fillProp(user);
    const group = await screen.findByRole("radiogroup", { name: "Ordens automatizadas nesta conta" });
    expect(group.querySelectorAll('[role="radio"][aria-checked="true"]')).toHaveLength(0);
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    expect(await screen.findByText("Escolha se esta conta aceita ordens automatizadas.")).toBeInTheDocument();
    expect(calls.create).toHaveLength(0);
  });

  it("sends a declined choice with the rule version seen, without step-up", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connectionsHandler, ...handlers);
    const stepUp = vi.fn();
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage stepUp={stepUp as unknown as StepUpRunner} />);
    await fillProp(user);
    await user.click(await screen.findByRole("radio", { name: /Não aceitar ordens automatizadas/ }));
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(calls.create[0]?.body).toMatchObject({ kind: "prop", firm_name: "Apex", opt_ins: { automation: { choice: "declined", rule_version_id: ids.ruleAuto } } });
    expect(calls.create[0]?.body).not.toHaveProperty("step_up");
    expect(stepUp).not.toHaveBeenCalled();
  });

  it("accepting needs the acknowledgement and goes through step-up once", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connectionsHandler, ...handlers);
    const stepUp = vi.fn(async ({ send }: Parameters<StepUpRunner>[0]) => send({ challenge_id: "c1", code: "123456" })) as unknown as StepUpRunner;
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage stepUp={stepUp} />);
    await fillProp(user);
    await user.click(await screen.findByRole("radio", { name: /Aceitar ordens automatizadas/ }));
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    expect(await screen.findByText("Confirme que leu a regra para aceitar.")).toBeInTheDocument();
    expect(calls.create).toHaveLength(0);
    await user.click(screen.getByRole("checkbox"));
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(stepUp).toHaveBeenCalledTimes(1);
    expect(calls.create[0]?.body).toMatchObject({ opt_ins: { automation: { choice: "accepted" } }, step_up: { challenge_id: "c1", code: "123456" } });
  });

  it("cancelling the step-up saves nothing and keeps the choice", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connectionsHandler, ...handlers);
    const stepUp = vi.fn(async () => {
      throw new DOMException("cancelled", "AbortError");
    }) as unknown as StepUpRunner;
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage stepUp={stepUp} />);
    await fillProp(user);
    await user.click(await screen.findByRole("radio", { name: /Aceitar ordens automatizadas/ }));
    await user.click(screen.getByRole("checkbox"));
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(stepUp).toHaveBeenCalled());
    expect(calls.create).toHaveLength(0);
    expect(screen.getByRole("radio", { name: /Aceitar ordens automatizadas/ })).toBeChecked();
  });

  it("clears the choice and warns when the rule changed under it (409)", async () => {
    const { handlers } = accountHandlers([]);
    server.use(
      http.post("/api/v1/accounts", () => apiError(409, "accounts.rule_version_changed", { current_rule_version_id: ids.ruleAuto2 })),
      connectionsHandler,
      ...handlers,
    );
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage />);
    await fillProp(user);
    await user.click(await screen.findByRole("radio", { name: /Não aceitar ordens automatizadas/ }));
    server.use(http.get("/api/v1/opt-in-rules", () => HttpResponse.json({ automation: ruleVersion({ id: ids.ruleAuto2, version_number: 4 }), copy: ruleVersion({ id: ids.ruleCopy, kind: "copy" }) })));
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    expect(await screen.findByText("A regra da firma foi atualizada enquanto você preenchia. Leia de novo e escolha.")).toBeInTheDocument();
    expect(screen.getByRole("radio", { name: /Não aceitar ordens automatizadas/ })).not.toBeChecked();
    expect(screen.getByLabelText("Nome")).toHaveValue("Apex nova");
  });
});

describe("AccountFormPage (edit)", () => {
  it("sends the row version and no opt_ins when the rule version did not change", async () => {
    const acc = propAccount({ row_version: 7, opt_ins: { automation: { kind: "automation", firm_stance: "unknown", state: "chosen", choice: "declined", chosen_at: "2026-10-21T10:00:00Z", rule_version_id: ids.ruleAuto, current_rule_version_id: ids.ruleAuto }, copy: { kind: "copy", state: "not_required" } } });
    const { handlers, calls } = accountHandlers([acc]);
    server.use(connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage accountId={acc.id} />);
    const name = await screen.findByLabelText("Nome");
    await user.clear(name);
    await user.type(name, "Renomeada");
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.update).toHaveLength(1));
    expect(calls.update[0]).toMatchObject({ row_version: 7, display_name: "Renomeada" });
    expect(calls.update[0]).not.toHaveProperty("opt_ins");
  });

  it("shows the reload alert on a version conflict and keeps typed values", async () => {
    const acc = propAccount({ row_version: 7 });
    const { handlers } = accountHandlers([acc]);
    server.use(http.put("/api/v1/accounts/:id", () => apiError(409, "common.version_conflict")), connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage accountId={acc.id} />);
    const name = await screen.findByLabelText("Nome");
    await user.clear(name);
    await user.type(name, "Digitado");
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    expect(await screen.findByText(/foi alterada em outro lugar/)).toBeInTheDocument();
    expect(screen.getByLabelText("Nome")).toHaveValue("Digitado");
  });

  it("shows not found for an account that is not the user's", async () => {
    server.use(connectionsHandler, ...accountHandlers([]).handlers);
    renderAccounts(<AccountFormPage accountId="99999999-9999-4999-8999-999999999999" />);
    expect(await screen.findByText("Conta não encontrada.")).toBeInTheDocument();
  });
});
