// The navigation mock lives in render-accounts and must load before the components.
import { renderAccounts } from "./testing/render-accounts";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { describe, expect, it, vi } from "vitest";
import { ApiError } from "@/shared/api/client";
import { server } from "@/test/server";
import { AccountFormPage } from "./account-form";
import { ids, propAccount } from "./testing/fixtures";
import { accountHandlers, apiError } from "./testing/handlers";
import type { StepUpRunner } from "./lib/use-step-up";
import type { PresetOutcome, PresetSlot } from "./api/preset-slot";
import { ruleVersion } from "./testing/fixtures";

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

vi.setConfig({ testTimeout: 20_000 });

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

  it("keeps a preset account on its preset: sends preset_version_id and firm_platform, never the manual fields or rule_values", async () => {
    const presetVersion = "00000000-0000-4000-8000-0000000000c1";
    const acc = propAccount({ row_version: 7, preset_version_id: presetVersion, firm_platform: "rithmic", firm_name: "Apex", plan_name: "Plan A", phase: "evaluation", size_amount: "50000.00" });
    const { handlers, calls } = accountHandlers([acc]);
    server.use(connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage accountId={acc.id} />);
    const name = await screen.findByLabelText("Nome");
    expect(screen.queryByLabelText("Firma")).not.toBeInTheDocument();
    expect(screen.queryByLabelText("Tamanho (USD)")).not.toBeInTheDocument();
    await user.clear(name);
    await user.type(name, "Renomeada");
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.update).toHaveLength(1));
    expect(calls.update[0]).toMatchObject({ row_version: 7, display_name: "Renomeada", preset_version_id: presetVersion, firm_platform: "rithmic" });
    for (const forbidden of ["firm_name", "plan_name", "phase", "size_amount", "rule_values"]) expect(calls.update[0]).not.toHaveProperty(forbidden);
  });

  it("shows a 422 about a field the edit page does not render in the banner (never a silent save)", async () => {
    const acc = propAccount({ row_version: 7 });
    const { handlers } = accountHandlers([acc]);
    server.use(http.put("/api/v1/accounts/:id", () => apiError(422, "accounts.profile_size_required", { pointer: "/profile_size" })), connectionsHandler, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage accountId={acc.id} />);
    await screen.findByLabelText("Nome");
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    expect(await screen.findByRole("alert")).toBeInTheDocument();
  });

  describe("firm preset on the edit page", () => {
    const current = "00000000-0000-4000-8000-0000000000c1";
    const other = "00000000-0000-4000-8000-0000000000c2";
    const outcome = (over: Partial<PresetOutcome> = {}): PresetOutcome => ({ presetVersionId: other, firmPlatform: null, label: "Apex · Plan A · Avaliação · 50K", sizeAmount: "50000.00", maxPositionCap: 60, maxPositionDefault: 60, ruleValues: { drawdown: { type: "static", amount: "2000.00" } }, valid: true, ...over });
    // A stand-in for the S16 picker: renders the chosen outcome's label and lets the test switch to free text.
    const slot = (o: PresetOutcome | null): PresetSlot => ({ outcome: o, render: ({ switchToManual, error }) => <div><span>picker aberto</span><button type="button" onClick={switchToManual}>sem preset (picker)</button>{error ? <p>{error}</p> : null}</div> });

    it("asks the opt-in rules of the account's own preset, not the generic text", async () => {
      const acc = propAccount({ preset_version_id: current, firm_platform: "rithmic" });
      const { handlers } = accountHandlers([acc]);
      const asked: string[] = [];
      server.use(http.get("/api/v1/opt-in-rules", ({ request }) => { asked.push(new URL(request.url).search); return HttpResponse.json({ automation: ruleVersion({ id: ids.ruleAuto }), copy: ruleVersion({ id: ids.ruleCopy, kind: "copy" }) }); }), connectionsHandler, ...handlers);
      renderAccounts(<AccountFormPage accountId={acc.id} preset={slot(null)} />);
      await screen.findByLabelText("Nome");
      await waitFor(() => expect(asked.length).toBeGreaterThan(0));
      expect(asked[0]).toContain(`preset_version_id=${current}`);
      expect(asked[0]).toContain("firm_platform=rithmic");
    });

    it("attaches a preset to a free-text account: sends the preset and its rule values, no manual fields, and warns first", async () => {
      const acc = propAccount({ row_version: 4, firm_name: "Apex", plan_name: "50K", phase: "Avaliação" });
      const { handlers, calls } = accountHandlers([acc]);
      server.use(connectionsHandler, ...handlers);
      const user = userEvent.setup();
      renderAccounts(<AccountFormPage accountId={acc.id} preset={slot(outcome())} />);
      await user.click(await screen.findByRole("button", { name: "Escolher um preset da firma" }));
      expect(screen.getByText(/Anexar um preset coloca esta conta em revisão/)).toBeInTheDocument();
      expect(screen.getByText("picker aberto")).toBeInTheDocument();
      await user.click(screen.getByRole("button", { name: "Salvar conta" }));
      await waitFor(() => expect(calls.update).toHaveLength(1));
      expect(calls.update[0]).toMatchObject({ row_version: 4, preset_version_id: other, rule_values: { drawdown: { type: "static", amount: "2000.00" } } });
      for (const manual of ["firm_name", "plan_name", "phase", "size_amount"]) expect(calls.update[0]).not.toHaveProperty(manual);
    });

    it("does not send rule_values when the picked preset is the account's own selection", async () => {
      const acc = propAccount({ row_version: 4, preset_version_id: current, firm_platform: "rithmic" });
      const { handlers, calls } = accountHandlers([acc]);
      server.use(connectionsHandler, ...handlers);
      const user = userEvent.setup();
      renderAccounts(<AccountFormPage accountId={acc.id} preset={slot(outcome({ presetVersionId: current, firmPlatform: "rithmic" }))} />);
      await user.click(await screen.findByRole("button", { name: "Trocar o preset" }));
      await user.click(screen.getByRole("button", { name: "Salvar conta" }));
      await waitFor(() => expect(calls.update).toHaveLength(1));
      expect(calls.update[0]).toMatchObject({ preset_version_id: current, firm_platform: "rithmic" });
      expect(calls.update[0]).not.toHaveProperty("rule_values");
    });

    it("blocks the save while the picked preset is not valid (a required value is unresolved)", async () => {
      const acc = propAccount({ preset_version_id: current });
      const { handlers, calls } = accountHandlers([acc]);
      server.use(connectionsHandler, ...handlers);
      const user = userEvent.setup();
      renderAccounts(<AccountFormPage accountId={acc.id} preset={slot(outcome({ valid: false }))} />);
      await user.click(await screen.findByRole("button", { name: "Trocar o preset" }));
      await user.click(screen.getByRole("button", { name: "Salvar conta" }));
      expect(await screen.findByText("Escolha um preset e resolva os valores obrigatórios.")).toBeInTheDocument();
      expect(calls.update).toHaveLength(0);
    });

    it("goes back to free text: sends the manual fields and no preset", async () => {
      const acc = propAccount({ row_version: 4, preset_version_id: current, firm_platform: "rithmic", firm_name: "Apex", plan_name: "Plan A", phase: "evaluation", size_amount: "50000.00" });
      const { handlers, calls } = accountHandlers([acc]);
      server.use(connectionsHandler, ...handlers);
      const user = userEvent.setup();
      renderAccounts(<AccountFormPage accountId={acc.id} preset={slot(null)} />);
      await user.click(await screen.findByRole("button", { name: "Cadastrar sem preset" }));
      await user.click(screen.getByRole("button", { name: "Salvar conta" }));
      await waitFor(() => expect(calls.update).toHaveLength(1));
      expect(calls.update[0]).toMatchObject({ firm_name: "Apex", plan_name: "Plan A", phase: "evaluation", size_amount: "50000" });
      for (const preset of ["preset_version_id", "firm_platform", "rule_values"]) expect(calls.update[0]).not.toHaveProperty(preset);
    });
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

  it("asks for the opt-in only when this edit changes the rule version, and sends the version seen", async () => {
    const acc = propAccount({ row_version: 3 });
    const { handlers, calls } = accountHandlers([acc]);
    const newRule = ruleVersion({ id: ids.ruleAuto2, version_number: 4 });
    server.use(
      http.get("/api/v1/opt-in-rules", () => HttpResponse.json({ automation: newRule, copy: ruleVersion({ id: ids.ruleCopy, kind: "copy" }) })),
      connectionsHandler,
      ...handlers,
    );
    const stepUp = vi.fn(async ({ send }: Parameters<StepUpRunner>[0]) => send({ challenge_id: "c1", code: "123456" })) as unknown as StepUpRunner;
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage accountId={acc.id} stepUp={stepUp} />);
    await screen.findByLabelText("Nome");
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
    expect(await screen.findByText("Escolha se esta conta aceita ordens automatizadas.")).toBeInTheDocument();
    expect(calls.update).toHaveLength(0);
    await user.click(screen.getByRole("radio", { name: /Aceitar ordens automatizadas/ }));
    await user.click(screen.getByRole("checkbox"));
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.update).toHaveLength(1));
    expect(stepUp).toHaveBeenCalledTimes(1);
    expect(calls.update[0]).toMatchObject({ row_version: 3, opt_ins: { automation: { choice: "accepted", rule_version_id: ids.ruleAuto2 } }, step_up: { challenge_id: "c1", code: "123456" } });
  });

  it("a binding change on an automated account goes through the step-up and saves once with the proof", async () => {
    const acc = propAccount({ row_version: 4, opt_ins: { automation: { kind: "automation", firm_stance: "unknown", state: "chosen", choice: "accepted", chosen_at: "2026-10-21T10:00:00Z", rule_version_id: ids.ruleAuto, current_rule_version_id: ids.ruleAuto }, copy: { kind: "copy", state: "not_required", firm_stance: "unknown" } } });
    const { handlers } = accountHandlers([acc]);
    const puts: { step_up?: unknown; binding?: unknown }[] = [];
    server.use(
      http.put("/api/v1/accounts/:id", async ({ request }) => {
        const body = (await request.clone().json()) as { step_up?: unknown; binding?: unknown };
        puts.push(body);
        return body.step_up ? undefined : apiError(403, "auth.step_up_required");
      }),
      connectionsHandler,
      ...handlers,
    );
    // Mirrors the shared runner: probe without proof, then one request with the proof.
    const stepUp = vi.fn(async ({ send }: Parameters<StepUpRunner>[0]) => {
      try {
        return await send();
      } catch (error) {
        if (error instanceof ApiError && error.code === "auth.step_up_required") return send({ challenge_id: "c9", code: "654321" });
        throw error;
      }
    }) as unknown as StepUpRunner;
    const user = userEvent.setup();
    renderAccounts(<AccountFormPage accountId={acc.id} stepUp={stepUp} />);
    await screen.findByLabelText("Nome");
    await user.click(await screen.findByRole("combobox", { name: "Conta no NinjaTrader" }));
    await user.click(await screen.findByRole("option", { name: /Sim101/ }));
    await user.click(screen.getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(puts).toHaveLength(2));
    expect(stepUp).toHaveBeenCalledTimes(1);
    expect(stepUp).toHaveBeenCalledWith(expect.objectContaining({ action: "account_binding_change", target: acc.id }));
    expect(puts[0]?.step_up).toBeUndefined();
    expect(puts[1]).toMatchObject({ binding: { platform_account: "Sim101" }, step_up: { challenge_id: "c9", code: "654321" } });
  });
});
