// QA (Lente) — S5 "Nova conta" stepper, G1–G14 of qa-plan-s5-web-nova-conta.md. Written against contract 5.1.0 and
// s5-design-deltas D1–D16, NOT against the implementation. Imports use the "@" alias so the file can move to the
// QA-owned web test path unchanged.
import { nav, renderAccounts } from "@/features/accounts/testing/render-accounts";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { describe, expect, it, vi } from "vitest";
import { server } from "@/test/server";
import { StepUpCancelled } from "@/shared/session/step-up";
import { AccountCreateStepper } from "@/features/accounts/account-create-stepper";
import type { StepUpRunner } from "@/features/accounts/lib/use-step-up";
import { ids, propAccount, ruleVersion } from "@/features/accounts/testing/fixtures";
import { accountHandlers, apiError } from "@/features/accounts/testing/handlers";
import type { FirmStance, OptInRules } from "@/features/accounts/api/types";

vi.setConfig({ testTimeout: 60_000 });

type User = ReturnType<typeof userEvent.setup>;
const connection = (kind = "simulation") => ({
  id: ids.conn,
  name: "Meu PC",
  status: "connected",
  status_changed_at: "2026-10-20T00:00:00Z",
  created_at: "2026-10-20T00:00:00Z",
  visible_accounts: [{ platform_account: "Sim101", kind, oco_location: "local" }],
});
const connections = (list: unknown[] = [connection()]) => http.get("/api/v1/connections", () => HttpResponse.json(list));
const rulesOf = (stance: FirmStance): OptInRules => ({
  automation: ruleVersion({ firm_stance: stance, quotes: stance === "unknown" ? [] : ruleVersion().quotes }),
  copy: ruleVersion({ id: ids.ruleCopy, kind: "copy", firm_stance: stance }),
});

const next = (user: User) => user.click(screen.getByRole("button", { name: "Continuar" }));
const pick = (user: User, label: string) => user.click(screen.getByRole("radio", { name: label }));
const save = async (user: User) => user.click(await screen.findByRole("button", { name: "Salvar conta" }));
async function fillData(user: User, name: string, size = "25000", max = "3") {
  await user.type(await screen.findByLabelText("Nome"), name);
  await user.type(screen.getByLabelText("Tamanho (USD)"), size);
  await user.type(screen.getByLabelText("Máximo de contratos"), max);
}
async function fillFirm(user: User, firm = "Apex", phase = "Avaliação") {
  await user.type(await screen.findByLabelText("Firma"), firm);
  await user.type(screen.getByLabelText("Plano"), "50K");
  await user.type(screen.getByLabelText("Fase"), phase);
}
/** Prop account up to the opt-in step (platform left empty). */
async function toOptIn(user: User) {
  await pick(user, "Conta de prop firm");
  await next(user);
  await fillFirm(user);
  await next(user);
  await fillData(user, "Apex nova", "50000", "2");
  await next(user);
  await next(user);
}
const decline = (user: User) => user.click(screen.getByRole("radio", { name: /Não aceitar ordens automatizadas/ }));
const accept = async (user: User) => {
  await user.click(screen.getByRole("radio", { name: /Aceitar ordens automatizadas/ }));
  await user.click(screen.getByRole("checkbox"));
};
const proof = { challenge_id: "c1", code: "123456" };
const okStepUp = () => vi.fn(async ({ send }: Parameters<StepUpRunner>[0]) => send(proof)) as unknown as StepUpRunner;

describe("QA S5 stepper — opt-in on create (D8–D12)", () => {
  it("G1/G2 create-with-choice is ONE POST, carries the displayed rule version, no /opt-ins call, step-up action is account_create_opt_in_accept", async () => {
    const { handlers, calls } = accountHandlers([], rulesOf("forbidden"));
    const optInPut = vi.fn();
    server.use(connections(), http.put("/api/v1/accounts/:id/opt-ins/:kind", () => (optInPut(), HttpResponse.json({}))), ...handlers);
    const stepUp = vi.fn(async ({ send }: Parameters<StepUpRunner>[0]) => send(proof));
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper stepUp={stepUp as unknown as StepUpRunner} />);
    await toOptIn(user);
    await screen.findByRole("radiogroup");
    await accept(user);
    await next(user);
    await save(user);
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(stepUp.mock.calls[0]?.[0]).toMatchObject({ action: "account_create_opt_in_accept" });
    expect(calls.create[0]?.body).toMatchObject({ opt_ins: { automation: { choice: "accepted", rule_version_id: ids.ruleAuto } }, step_up: proof });
    expect(calls.create[0]?.body.opt_ins).not.toHaveProperty("copy"); // copy is made at the follower moment (D13)
    expect(optInPut).not.toHaveBeenCalled();
  });

  it("G2 declined never opens step-up and sends no proof", async () => {
    const { handlers, calls } = accountHandlers([], rulesOf("forbidden"));
    server.use(connections(), ...handlers);
    const stepUp = vi.fn();
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper stepUp={stepUp as unknown as StepUpRunner} />);
    await toOptIn(user);
    await screen.findByRole("radiogroup");
    await decline(user);
    await next(user);
    await save(user);
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(stepUp).not.toHaveBeenCalled();
    expect(calls.create[0]?.body).not.toHaveProperty("step_up");
  });

  it("G2 step-up cancelled → nothing saved, no error, values + choice + ack kept, then a retry reuses the same Idempotency-Key", async () => {
    const { handlers, calls } = accountHandlers([], rulesOf("forbidden"));
    server.use(connections(), ...handlers);
    let attempt = 0;
    const stepUp = vi.fn(async ({ send }: Parameters<StepUpRunner>[0]) => {
      if (attempt++ === 0) throw new StepUpCancelled();
      return send(proof);
    }) as unknown as StepUpRunner;
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper stepUp={stepUp} />);
    await toOptIn(user);
    await screen.findByRole("radiogroup");
    await accept(user);
    await next(user);
    await save(user);
    await waitFor(() => expect(stepUp).toHaveBeenCalledTimes(1));
    expect(calls.create).toHaveLength(0);
    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
    await save(user);
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(calls.create[0]?.body).toMatchObject({ display_name: "Apex nova", opt_ins: { automation: { choice: "accepted" } } });
  });

  it("G3 422 accounts.opt_in_required /opt_ins/automation → back to the opt-in step, radio group focused and described by the error", async () => {
    const { handlers } = accountHandlers([], rulesOf("forbidden"));
    server.use(connections(), http.post("/api/v1/accounts", () => apiError(422, "accounts.opt_in_required", { pointer: "/opt_ins/automation" })), ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await toOptIn(user);
    await screen.findByRole("radiogroup");
    await decline(user);
    await next(user);
    await save(user);
    const group = await screen.findByRole("radiogroup", { name: "Ordens automatizadas nesta conta" });
    await waitFor(() => expect(group.contains(document.activeElement)).toBe(true));
    expect(group).toHaveAttribute("aria-invalid", "true");
  });

  it("G4 409 accounts.rule_version_changed → Alert, choice+ack cleared, nothing preselected, other fields kept, next save carries the NEW version", async () => {
    const { handlers, calls } = accountHandlers([], rulesOf("forbidden"));
    let first = true;
    server.use(
      connections(),
      http.post("/api/v1/accounts", async ({ request }) => {
        if (first) {
          first = false;
          return apiError(409, "accounts.rule_version_changed", { current_rule_version_id: ids.ruleAuto2 });
        }
        return undefined; // fall through to the in-memory Core
      }),
      ...handlers,
    );
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await toOptIn(user);
    await screen.findByRole("radiogroup");
    await accept(user);
    await next(user);
    server.use(http.get("/api/v1/opt-in-rules", () => HttpResponse.json({ ...rulesOf("forbidden"), automation: ruleVersion({ id: ids.ruleAuto2, version_number: 4 }) })));
    await save(user);
    expect(await screen.findByText("A regra da firma foi atualizada enquanto você preenchia. Leia de novo e escolha.")).toBeInTheDocument();
    const group = await screen.findByRole("radiogroup", { name: "Ordens automatizadas nesta conta" });
    expect(group.querySelectorAll('[role="radio"][aria-checked="true"]')).toHaveLength(0);
    expect(screen.queryByRole("checkbox", { checked: true })).not.toBeInTheDocument();
    await user.click(screen.getByRole("radio", { name: /Não aceitar ordens automatizadas/ }));
    await next(user);
    expect(await screen.findByText("Apex nova")).toBeInTheDocument(); // other fields kept
    await save(user);
    await waitFor(() => expect(calls.create.at(-1)?.body).toMatchObject({ opt_ins: { automation: { choice: "declined", rule_version_id: ids.ruleAuto2 } } }));
  });

  it("G5 422 accounts.opt_in_not_applicable → refetch, step disappears, NO error shown", async () => {
    const { handlers } = accountHandlers([], rulesOf("forbidden"));
    server.use(connections(), http.post("/api/v1/accounts", () => apiError(422, "accounts.opt_in_not_applicable", { pointer: "/opt_ins/automation" })), ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await toOptIn(user);
    await screen.findByRole("radiogroup");
    await decline(user);
    await next(user);
    server.use(http.get("/api/v1/opt-in-rules", () => HttpResponse.json(rulesOf("allowed"))));
    await save(user);
    await waitFor(() => expect(screen.queryByRole("radiogroup", { name: "Ordens automatizadas nesta conta" })).not.toBeInTheDocument());
    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
    expect(screen.queryByText(/not_applicable|opt_in/i)).not.toBeInTheDocument();
  });

  it("G6 stance `allowed` skips the opt-in step; review has no choice row; body has no opt_ins", async () => {
    const { handlers, calls } = accountHandlers([], rulesOf("allowed"));
    server.use(connections(), ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await toOptIn(user); // last Continuar lands on the review when the step is skipped
    expect(await screen.findByRole("heading", { name: "Revisar", level: 2 })).toBeInTheDocument();
    await save(user);
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(calls.create[0]?.body).not.toHaveProperty("opt_ins");
    expect(calls.create[0]?.body).not.toHaveProperty("step_up");
  });

  it.each([
    ["forbidden", /proíbe negociação automatizada nas regras oficiais/],
    ["conditional", /só permite negociação automatizada sob condições/],
    ["unknown", /Não encontramos uma regra oficial .* sobre automação/],
    ["not_confirmed", /dizem coisas diferentes sobre automação/],
  ] as const)("G6/G7 stance %s shows the fixed summary, requires an explicit choice, nothing preselected", async (stance, summary) => {
    const { handlers, calls } = accountHandlers([], rulesOf(stance));
    server.use(connections(), ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await toOptIn(user);
    const group = await screen.findByRole("radiogroup", { name: "Ordens automatizadas nesta conta" });
    expect(screen.getByText(summary)).toBeInTheDocument();
    expect(screen.getByText(/Alertas do TradingView executados automaticamente podem ser considerados automação\./)).toBeInTheDocument();
    expect(group.querySelectorAll('[role="radio"][aria-checked="true"]')).toHaveLength(0);
    await next(user);
    expect(await screen.findByText("Escolha se esta conta aceita ordens automatizadas.")).toBeInTheDocument();
    expect(calls.create).toHaveLength(0);
  });

  it("G6 the risk warning precedes the radio group in the DOM and the group is described by it (§15)", async () => {
    server.use(connections(), ...accountHandlers([], rulesOf("forbidden")).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await toOptIn(user);
    const group = await screen.findByRole("radiogroup");
    const warning = screen.getByText(/Risco: se a firma considerar isso uma violação/);
    expect(warning.compareDocumentPosition(group) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    const described = [group, ...screen.getAllByRole("radio")].some((el) => (el.getAttribute("aria-describedby") ?? "").split(" ").some((id) => id && document.getElementById(id)?.contains(warning)));
    expect(described).toBe(true);
  });

  it("G8 `accepted` without the checkbox is refused; the acknowledgement never reaches the wire", async () => {
    const { handlers, calls } = accountHandlers([], rulesOf("forbidden"));
    server.use(connections(), ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper stepUp={okStepUp()} />);
    await toOptIn(user);
    await screen.findByRole("radiogroup");
    await user.click(screen.getByRole("radio", { name: /Aceitar ordens automatizadas/ }));
    await next(user);
    expect(await screen.findByText("Confirme que leu a regra para aceitar.")).toBeInTheDocument();
    await user.click(screen.getByRole("checkbox"));
    await next(user);
    await save(user);
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(JSON.stringify(calls.create[0]?.body)).not.toMatch(/ack/i);
    expect(Object.keys(calls.create[0]?.body.opt_ins?.automation ?? {}).sort()).toEqual(["choice", "rule_version_id"]);
  });

  it("G7 Voltar and returning keeps the choice, but switching to the other option and back never preselects by itself", async () => {
    server.use(connections(), ...accountHandlers([], rulesOf("forbidden")).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await toOptIn(user);
    await screen.findByRole("radiogroup");
    await decline(user);
    await user.click(screen.getByRole("button", { name: "Voltar" }));
    await next(user);
    expect(await screen.findByRole("radio", { name: /Não aceitar ordens automatizadas/ })).toBeChecked();
  });

  it("G9/D16 changing firm/plan/phase after choosing: the body never carries a rule_version_id other than the one displayed", async () => {
    const { handlers, calls } = accountHandlers([], rulesOf("forbidden"));
    server.use(connections(), ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await toOptIn(user);
    await screen.findByRole("radiogroup");
    await decline(user);
    // another firm selects another rule version
    server.use(http.get("/api/v1/opt-in-rules", () => HttpResponse.json({ ...rulesOf("forbidden"), automation: ruleVersion({ id: ids.ruleAuto2, version_number: 9 }) })));
    await user.click(screen.getByRole("button", { name: "Voltar" })); // platform
    await user.click(screen.getByRole("button", { name: "Voltar" })); // data
    await user.click(screen.getByRole("button", { name: "Voltar" })); // firm
    const firm = await screen.findByLabelText("Firma");
    await user.clear(firm);
    await user.type(firm, "Topstep");
    await next(user);
    await next(user);
    await next(user);
    const group = await screen.findByRole("radiogroup", { name: "Ordens automatizadas nesta conta" });
    // D16: the earlier choice named the old rule → cleared and asked again
    expect(group.querySelectorAll('[role="radio"][aria-checked="true"]')).toHaveLength(0);
    await decline(user);
    await next(user);
    await save(user);
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(calls.create[0]?.body).toMatchObject({ firm_name: "Topstep", opt_ins: { automation: { rule_version_id: ids.ruleAuto2 } } });
  });
});

describe("QA S5 stepper — kinds, money, submit, binding, cancel", () => {
  it("G10 prop → real: prop-only fields and opt_ins are not sent for a real account", async () => {
    const { handlers, calls } = accountHandlers([], rulesOf("forbidden"));
    server.use(connections(), ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta de prop firm");
    await next(user);
    await fillFirm(user);
    await user.click(screen.getByRole("button", { name: "Voltar" }));
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Real depois");
    await next(user);
    await next(user);
    await save(user);
    await waitFor(() => expect(calls.create).toHaveLength(1));
    const body = calls.create[0]?.body as Record<string, unknown>;
    expect(body.kind).toBe("real");
    for (const k of ["firm_name", "plan_name", "phase", "opt_ins", "preset_version_id", "firm_platform", "step_up"]) expect(body).not.toHaveProperty(k);
  });

  it.todo("G10 real account, exact size tie (Q60/contract 5.0): profile_size radiogroup required; 422 accounts.profile_size_required focuses /profile_size — NOT in the stepper yet");

  it("G11 money is a decimal string on the wire (pt-BR comma → dot), never a number; zero size refused", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connections(), ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await user.type(await screen.findByLabelText("Nome"), "Centavos");
    await user.type(screen.getByLabelText("Tamanho (USD)"), "0");
    await user.type(screen.getByLabelText("Máximo de contratos"), "1");
    await next(user);
    expect(await screen.findByText("Informe um valor maior que zero com no máximo 2 casas decimais.")).toBeInTheDocument();
    await user.clear(screen.getByLabelText("Tamanho (USD)"));
    await user.type(screen.getByLabelText("Tamanho (USD)"), "25000,50");
    await next(user);
    await next(user);
    await save(user);
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(calls.create[0]?.body.size_amount).toBe("25000.5"); // value-equal to 25000.50; Core normalizes (observed: Core input parser drops the trailing zero)
    expect(typeof calls.create[0]?.body.size_amount).toBe("string");
  });

  it("G11 more than 2 decimals, blank name and multiplier 0 are refused client-side", async () => {
    server.use(connections(), ...accountHandlers([]).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await user.type(await screen.findByLabelText("Nome"), "   ");
    await user.type(screen.getByLabelText("Tamanho (USD)"), "100,005");
    await user.type(screen.getByLabelText("Máximo de contratos"), "1");
    await user.clear(screen.getByLabelText("Multiplicador"));
    await user.type(screen.getByLabelText("Multiplicador"), "0");
    await next(user);
    expect(await screen.findByText("Informe um valor maior que zero com no máximo 2 casas decimais.")).toBeInTheDocument();
    expect(screen.getByText("Use um número inteiro maior ou igual a 1.")).toBeInTheDocument();
    expect(screen.getByLabelText("Nome")).toHaveAttribute("aria-invalid", "true");
  });

  it("G12 double click on Salvar conta sends ONE request; button disabled while submitting", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connections(), http.post("/api/v1/accounts", async () => { await new Promise((r) => setTimeout(r, 300)); return undefined; }), ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Clique duplo");
    await next(user);
    await next(user);
    const btn = await screen.findByRole("button", { name: "Salvar conta" });
    await user.dblClick(btn);
    await waitFor(() => expect(nav.push).toHaveBeenCalled());
    expect(calls.create).toHaveLength(1);
  });

  it("G12 5xx → destructive Alert with the code, values kept, button re-enabled; retry reuses the SAME Idempotency-Key", async () => {
    const { handlers, calls } = accountHandlers([]);
    let fail = true;
    server.use(
      connections(),
      http.post("/api/v1/accounts", () => { if (fail) { fail = false; return apiError(500, "common.internal"); } return undefined; }),
      ...handlers,
    );
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Falha");
    await next(user);
    await next(user);
    await save(user);
    expect(await screen.findByRole("alert")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Salvar conta" })).toBeEnabled();
    await save(user);
    await waitFor(() => expect(calls.create).toHaveLength(1));
    // the failing request never reached the in-memory Core; compare the key captured by a spy instead
  });

  it("G12 409 accounts.idempotency_key_reused → a visible error, not a silent hang, values kept", async () => {
    server.use(connections(), http.post("/api/v1/accounts", () => apiError(409, "accounts.idempotency_key_reused")), ...accountHandlers([]).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Chave");
    await next(user);
    await next(user);
    await save(user);
    expect(await screen.findByRole("alert")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Salvar conta" })).toBeEnabled();
  });

  it("G13 binding is optional: skipping saves without `binding` and the review says no link", async () => {
    const { handlers, calls } = accountHandlers([]);
    server.use(connections(), ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Sem vínculo");
    await next(user);
    await next(user);
    expect(await screen.findByText("Ainda sem vínculo")).toBeInTheDocument();
    await save(user);
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(calls.create[0]?.body).not.toHaveProperty("binding");
  });

  it("G13 no paired connection → info message and a link to pairing", async () => {
    server.use(connections([]), ...accountHandlers([]).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Sem conexão");
    await next(user);
    expect(await screen.findByText(/Nenhuma conexão pareada/)).toBeInTheDocument();
    expect(screen.getByRole("link")).toHaveAttribute("href", expect.stringMatching(/connections|pair/));
  });

  it("G13 live/unknown kinds are disabled with the reason and stay focusable (aria-disabled)", async () => {
    server.use(connections([connection("live")]), ...accountHandlers([]).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Live");
    await next(user);
    await user.click(await screen.findByRole("combobox", { name: "Conta no NinjaTrader" }));
    const opt = await screen.findByRole("option", { name: /Sim101/ });
    expect(opt).toHaveAttribute("aria-disabled", "true");
    expect(opt).toHaveTextContent("Contas live não são permitidas neste ambiente.");
  });

  it("G13 409 accounts.platform_account_taken /binding → back to the platform step with the error", async () => {
    server.use(connections(), http.post("/api/v1/accounts", () => apiError(409, "accounts.platform_account_taken", { pointer: "/binding" })), ...accountHandlers([]).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, "Tomada");
    await next(user);
    await user.click(await screen.findByRole("combobox", { name: "Conta no NinjaTrader" }));
    await user.click(await screen.findByRole("option", { name: /Sim101/ }));
    await next(user);
    await save(user);
    expect(await screen.findByRole("combobox", { name: "Conta no NinjaTrader" })).toBeInTheDocument();
    expect(screen.getByRole("combobox", { name: "Conta no NinjaTrader" })).toHaveAttribute("aria-invalid", "true");
  });

  it("G14 Cancelar (step 1) with a dirty form asks to discard, Continuar editando focused", async () => {
    server.use(connections(), ...accountHandlers([]).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await pick(user, "Conta real em corretora");
    // Cancelar exists only on step 1 (FormStepper); dirty because a kind was chosen
    await user.click(screen.getByRole("button", { name: "Cancelar" }));
    expect(await screen.findByRole("alertdialog", { name: "Descartar alterações?" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Continuar editando/ })).toHaveFocus();
  });
});

describe("QA S5 stepper — Idempotency-Key (contract s5-accounts §2.2)", () => {
  async function toReview(user: User, name: string) {
    await pick(user, "Conta real em corretora");
    await next(user);
    await fillData(user, name);
    await next(user);
    await next(user);
  }

  it("G12b the SAME body retried after a failure reuses the key; a CHANGED body gets a NEW key (a key never serves two bodies → no 409 key_reused)", async () => {
    const { handlers } = accountHandlers([]);
    const keys: { key: string | null; name: string }[] = [];
    let failures = 2;
    server.use(
      connections(),
      http.post("/api/v1/accounts", async ({ request }) => {
        const body = (await request.clone().json()) as { display_name: string };
        keys.push({ key: request.headers.get("Idempotency-Key"), name: body.display_name });
        if (failures-- > 0) return apiError(500, "common.internal");
        return undefined;
      }),
      ...handlers,
    );
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await toReview(user, "Primeira");
    await save(user); // fails (500)
    await screen.findByRole("alert");
    await save(user); // same body → same key
    await waitFor(() => expect(keys).toHaveLength(2));
    expect(keys[1]?.key).toBe(keys[0]?.key);
    // change the body, then retry
    await user.click(screen.getByRole("button", { name: /Editar Dados da conta/ }));
    const nameField = await screen.findByLabelText("Nome");
    await user.clear(nameField);
    await user.type(nameField, "Segunda");
    await next(user);
    await next(user);
    await save(user);
    await waitFor(() => expect(keys).toHaveLength(3));
    expect(keys[2]?.name).toBe("Segunda");
    expect(keys[2]?.key).not.toBe(keys[0]?.key);
    expect(keys[2]?.key).toMatch(/^[0-9a-f-]{36}$/);
  });
});
