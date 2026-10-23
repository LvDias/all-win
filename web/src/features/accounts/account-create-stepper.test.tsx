// The navigation mock lives in render-accounts and must load before the components.
import { nav, renderAccounts } from "./testing/render-accounts";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { describe, expect, it, vi } from "vitest";
import { ApiError } from "@/shared/api/client";
import { server } from "@/test/server";
import { StepUpCancelled } from "@/shared/session/step-up";
import { AccountCreateStepper } from "./account-create-stepper";
import type { StepUpRunner } from "./lib/use-step-up";
import { ids, propAccount, ruleVersion } from "./testing/fixtures";
import { accountHandlers, apiError } from "./testing/handlers";

vi.setConfig({ testTimeout: 30_000 });

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
// Keys requested from the catalog owner (profile size on a tie); remove with the `extra` argument once merged.
const profileSizeMessages = {
  accounts: { form: { profileSize: { label: "Perfil inicial", help: "Escolha o tamanho padrão de onde copiar o perfil." } } },
  errors: { accounts: { profile_size_required: "Escolha o tamanho padrão de onde copiar o perfil." } },
};
type User = ReturnType<typeof userEvent.setup>;

const next = (user: User) => user.click(screen.getByRole("button", { name: "Continuar" }));
const pick = (user: User, label: string) => user.click(screen.getByRole("radio", { name: label }));

async function fillData(user: User, name: string, size = "25000", max = "3") {
  await user.type(await screen.findByLabelText("Nome"), name);
  await user.type(screen.getByLabelText("Tamanho (USD)"), size);
  await user.type(screen.getByLabelText("Máximo de contratos"), max);
}

async function fillFirm(user: User) {
  await user.type(await screen.findByLabelText("Firma"), "Apex");
  await user.type(screen.getByLabelText("Plano"), "50K");
  await user.type(screen.getByLabelText("Fase"), "Avaliação");
}

/** Walks a prop account up to the opt-in step. */
async function toOptIn(user: User) {
  await pick(user, "Conta de prop firm");
  await next(user);
  await fillFirm(user);
  await next(user);
  await fillData(user, "Apex nova", "50000", "2");
  await next(user);
  await next(user); // platform is optional and left empty
}

describe("AccountCreateStepper", () => {
  it("real account: Tipo → Dados → Conta na plataforma → Revisar, one idempotency key, no opt-in", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    expect(screen.queryByLabelText("Firma")).not.toBeInTheDocument();
    await fillData(user, "Minha conta");
    await next(user);
    await next(user);
    expect(await screen.findByRole("heading", { name: "Revisar", level: 2 })).toBeInTheDocument();
    expect(screen.queryByText("Ordens automatizadas")).not.toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.create).toHaveLength(1));
    const { body, key } = calls.create[0] ?? { body: null, key: null };
    expect(body).toEqual({ kind: "real", display_name: "Minha conta", size_amount: "25000", max_position_qty: 3, multiplier: 1 });
    expect(key).toMatch(/^[0-9a-f-]{36}$/);
    await waitFor(() => expect(nav.push).toHaveBeenCalled());
  });

  it("Continuar validates only the current step and Voltar keeps typed data", async () => {
    server.use(connectionsHandler, ...accountHandlers([]).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await next(user);
    expect(await screen.findByText("Obrigatório.")).toBeInTheDocument();
    await pick(user, "Conta real em corretora");
    await next(user);
    await user.type(await screen.findByLabelText("Nome"), "Mantida");
    await user.type(screen.getByLabelText("Máximo de contratos"), "0");
    await next(user);
    expect(screen.getByText("Informe um número inteiro de contratos, 1 ou mais.")).toBeInTheDocument();
    expect(screen.getByText("Informe um valor maior que zero com no máximo 2 casas decimais.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Voltar" }));
    await user.click(screen.getByRole("button", { name: "Continuar" }));
    expect(await screen.findByLabelText("Nome")).toHaveValue("Mantida");
  });

  it("refuses a fractional multiplier", async () => {
    server.use(connectionsHandler, ...accountHandlers([]).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "X", "1000", "1");
    await user.clear(screen.getByLabelText("Multiplicador"));
    await user.type(screen.getByLabelText("Multiplicador"), "1.5");
    await next(user);
    expect(await screen.findByText("Use um número inteiro maior ou igual a 1.")).toBeInTheDocument();
  });

  it("a 422/409 with a pointer returns to the step that owns the field", async () => {
    const { handlers } = accountHandlers([propAccount({ display_name: "Duplicada" })]);
    server.use(connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "duplicada");
    await next(user);
    await next(user);
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
    expect(await screen.findByText("Já existe uma conta com este nome.")).toBeInTheDocument();
    expect(screen.getByLabelText("Nome")).toHaveValue("duplicada");
  });

  it("offers bound platform accounts as disabled with the reason", async () => {
    const { handlers } = accountHandlers([propAccount({ binding: { connection_id: ids.conn, connection_name: "Meu PC", connection_status: "connected", platform_account: "Sim102", kind: "simulation", oco_location: "local" } })]);
    server.use(connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Nova");
    await next(user);
    await user.click(await screen.findByRole("combobox", { name: "Conta no NinjaTrader" }));
    expect(await screen.findByRole("option", { name: /Sim102.*Já vinculada a Apex 50K 1/ })).toHaveAttribute("aria-disabled", "true");
  });

  it("prop: the opt-in step requires an explicit choice with nothing preselected", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await toOptIn(user);
    const group = await screen.findByRole("radiogroup", { name: "Ordens automatizadas nesta conta" });
    expect(group.querySelectorAll('[role="radio"][aria-checked="true"]')).toHaveLength(0);
    await next(user);
    expect(await screen.findByText("Escolha se esta conta aceita ordens automatizadas.")).toBeInTheDocument();
    expect(calls.create).toHaveLength(0);
    await waitFor(() => expect(group).toContainElement(document.activeElement as HTMLElement));
    expect(group).toHaveAttribute("aria-invalid", "true");
  });

  it("a server 422 opt_in_required returns to the opt-in step and focuses the radio group", async () => {
    const { handlers } = accountHandlers([]);
    server.use(http.post("/api/v1/accounts", () => apiError(422, "accounts.opt_in_required", { pointer: "/opt_ins/automation" })), connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await toOptIn(user);
    await user.click(await screen.findByRole("radio", { name: /Não aceitar ordens automatizadas/ }));
    await next(user);
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
    const group = await screen.findByRole("radiogroup", { name: "Ordens automatizadas nesta conta" });
    await waitFor(() => expect(group).toContainElement(document.activeElement as HTMLElement));
    expect(group).toHaveAttribute("aria-invalid", "true");
  });

  it("disables live and unknown platform accounts with the reason, and links to pairing when nothing is paired", async () => {
    const { handlers } = accountHandlers([]);
    const live = { ...connection, visible_accounts: [{ platform_account: "Live1", kind: "live", oco_location: "local" }, { platform_account: "Sim101", kind: "simulation", oco_location: "local" }] };
    server.use(http.get("/api/v1/connections", () => HttpResponse.json([live])), ...handlers);
    const user = userEvent.setup();
    const view = renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Nova");
    await next(user);
    await user.click(await screen.findByRole("combobox", { name: "Conta no NinjaTrader" }));
    expect(await screen.findByRole("option", { name: /Live1.*Contas live não são permitidas neste ambiente/ })).toHaveAttribute("aria-disabled", "true");
    expect(screen.getByRole("option", { name: /Sim101/ })).not.toHaveAttribute("aria-disabled", "true");
    view.unmount();
    server.use(http.get("/api/v1/connections", () => HttpResponse.json([])));
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Outra");
    await next(user);
    expect(await screen.findByRole("link", { name: "Parear o NinjaTrader" })).toHaveAttribute("href", "/pt-BR/connections/ninjatrader");
  });

  it("sends a declined choice with the rule version seen, without step-up", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connectionsHandler, ...handlers);
    const stepUp = vi.fn();
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper stepUp={stepUp as unknown as StepUpRunner} />);
    await toOptIn(user);
    await user.click(await screen.findByRole("radio", { name: /Não aceitar ordens automatizadas/ }));
    await next(user);
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
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
    renderAccounts(<AccountCreateStepper stepUp={stepUp} />);
    await toOptIn(user);
    await user.click(await screen.findByRole("radio", { name: /Aceitar ordens automatizadas/ }));
    await next(user);
    expect(await screen.findByText("Confirme que leu a regra para aceitar.")).toBeInTheDocument();
    await user.click(screen.getByRole("checkbox"));
    await next(user);
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(stepUp).toHaveBeenCalledTimes(1);
    expect(calls.create[0]?.body).toMatchObject({ opt_ins: { automation: { choice: "accepted" } }, step_up: { challenge_id: "c1", code: "123456" } });
  });

  it("cancelling the step-up saves nothing, shows no error and keeps the review", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connectionsHandler, ...handlers);
    const stepUp = vi.fn(async () => {
      throw new StepUpCancelled();
    }) as unknown as StepUpRunner;
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper stepUp={stepUp} />);
    await toOptIn(user);
    await user.click(await screen.findByRole("radio", { name: /Aceitar ordens automatizadas/ }));
    await user.click(screen.getByRole("checkbox"));
    await next(user);
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(stepUp).toHaveBeenCalled());
    expect(calls.create).toHaveLength(0);
    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Salvar conta" })).toBeEnabled();
  });

  it("a stale rule (409) clears the choice, warns, and sends the user back to the opt-in step", async () => {
    const { handlers } = accountHandlers([]);
    server.use(
      http.post("/api/v1/accounts", () => apiError(409, "accounts.rule_version_changed", { current_rule_version_id: ids.ruleAuto2 })),
      connectionsHandler,
      ...handlers,
    );
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await toOptIn(user);
    await user.click(await screen.findByRole("radio", { name: /Não aceitar ordens automatizadas/ }));
    await next(user);
    server.use(http.get("/api/v1/opt-in-rules", () => HttpResponse.json({ automation: ruleVersion({ id: ids.ruleAuto2, version_number: 4 }), copy: ruleVersion({ id: ids.ruleCopy, kind: "copy" }) })));
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
    expect(await screen.findByText("A regra da firma foi atualizada enquanto você preenchia. Leia de novo e escolha.")).toBeInTheDocument();
    expect(await screen.findByRole("radio", { name: /Não aceitar ordens automatizadas/ })).not.toBeChecked();
  });

  it("a corrected body after a failure goes out with a new idempotency key", async () => {
    const { handlers, calls } = accountHandlers([propAccount({ display_name: "Taken" })]);
    server.use(connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Taken");
    await next(user);
    await next(user);
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.create).toHaveLength(1));
    const name = await screen.findByLabelText("Nome");
    await user.clear(name);
    await user.type(name, "Another name");
    await next(user);
    await next(user);
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.create).toHaveLength(2));
    expect(calls.create[1]?.key).not.toBe(calls.create[0]?.key);
    await waitFor(() => expect(nav.push).toHaveBeenCalled());
  });

  it("the step-up probe and the proof request carry the same Idempotency-Key", async () => {
    const { handlers, calls } = accountHandlers([]);
    const keys: (string | null)[] = [];
    server.use(
      http.post("/api/v1/accounts", async ({ request }) => {
        keys.push(request.headers.get("Idempotency-Key"));
        const body = (await request.clone().json()) as { step_up?: unknown };
        return body.step_up ? undefined : apiError(403, "auth.step_up_required");
      }),
      connectionsHandler,
      ...handlers,
    );
    const stepUp = vi.fn(async ({ send }: Parameters<StepUpRunner>[0]) => {
      try {
        return await send();
      } catch (error) {
        if (error instanceof ApiError && error.code === "auth.step_up_required") return send({ challenge_id: "c1", code: "123456" });
        throw error;
      }
    }) as unknown as StepUpRunner;
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper stepUp={stepUp} />);
    await toOptIn(user);
    await user.click(await screen.findByRole("radio", { name: /Aceitar ordens automatizadas/ }));
    await user.click(screen.getByRole("checkbox"));
    await next(user);
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(keys).toHaveLength(2);
    expect(keys[0]).toBe(keys[1]);
  });

  it("an exact-tie size asks which standard size to copy the profile from, then sends profile_size", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(
      http.post("/api/v1/accounts", async ({ request }) => {
        const body = (await request.clone().json()) as { profile_size?: string };
        if (!body.profile_size) return apiError(422, "accounts.profile_size_required", { pointer: "/profile_size" });
        return undefined; // falls through to the in-memory Core
      }),
      http.get("/api/v1/instruments", () => HttpResponse.json({ instruments: [{ id: ids.conn }] })),
      http.get("/api/v1/profile-sizes/nearest", () => HttpResponse.json({ size_amount: "75000.00", candidates: [{ size_amount: "50000.00", has_default: true }, { size_amount: "100000.00", has_default: false }] })),
      connectionsHandler,
      ...handlers,
    );
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />, "pt-BR", profileSizeMessages);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Between", "75000");
    await next(user);
    await next(user);
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
    expect(await screen.findByText("Perfil inicial")).toBeInTheDocument();
    await user.click(screen.getByRole("radio", { name: /100\.000/ }));
    await next(user);
    await next(user);
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.create.at(-1)?.body).toMatchObject({ profile_size: "100000.00" }));
  });

  it("the review lists each group with an Editar action", async () => {
    server.use(connectionsHandler, ...accountHandlers([]).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Resumo");
    await next(user);
    await next(user);
    expect(await screen.findByText("US$ 25.000,00")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /Editar Dados da conta/ }));
    expect(await screen.findByLabelText("Nome")).toHaveValue("Resumo");
  });
});
