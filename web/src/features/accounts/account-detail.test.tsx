// The navigation mock lives in render-accounts and must load before the components.
import { renderAccounts } from "./testing/render-accounts";
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { describe, expect, it, vi } from "vitest";
import { server } from "@/test/server";
import { AccountDetail } from "./account-detail";
import type { StepUpRunner } from "./lib/use-step-up";
import { ids, propAccount, realAccount, ruleVersion } from "./testing/fixtures";
import { accountHandlers, apiError } from "./testing/handlers";

const ruleHandlers = (rule = ruleVersion()) => [
  http.get("/api/v1/rule-versions/:id", ({ params }) =>
    params.id === ids.ruleAuto ? HttpResponse.json(ruleVersion({ id: ids.ruleAuto, version_number: 3 })) : HttpResponse.json(rule),
  ),
];

const pendingAccount = () =>
  propAccount({
    opt_ins: {
      automation: { kind: "automation", firm_stance: "forbidden", state: "pending_review", pending_reason: "text_changed", choice: "accepted", chosen_at: "2026-10-01T00:00:00Z", rule_version_id: ids.ruleAuto, current_rule_version_id: ids.ruleAuto2 },
      copy: { kind: "copy", state: "not_required", firm_stance: "forbidden" },
    },
  });
const v4 = ruleVersion({ id: ids.ruleAuto2, version_number: 4, change: { previous_rule_version_id: ids.ruleAuto, previous_firm_stance: "forbidden", cause: "text_changed", reason: { "pt-BR": "Texto reescrito.", en: "Text rewritten." } } });

vi.setConfig({ testTimeout: 20_000 });

describe("AccountDetail", () => {
  it("shows the same not-found state for an account that is not the user's", async () => {
    server.use(...accountHandlers([]).handlers);
    renderAccounts(<AccountDetail accountId="99999999-9999-4999-8999-999999999999" />);
    expect(await screen.findByText("Conta não encontrada.")).toBeInTheDocument();
  });

  it("encodes a hostile route id instead of rewriting the request path", async () => {
    const paths: string[] = [];
    server.use(
      http.get("/api/v1/*", ({ request }) => {
        paths.push(new URL(request.url).pathname);
        return undefined;
      }),
      ...accountHandlers([]).handlers,
    );
    renderAccounts(<AccountDetail accountId="..%2F..%2Fme" />);
    await screen.findByText("Conta não encontrada.");
    expect(paths.some((p) => p.startsWith("/api/v1/accounts/") && p.includes("%252F"))).toBe(true);
    expect(paths).not.toContain("/api/v1/me/me");
  });

  it("renders the alert and risk slots from the page, and nothing when they are empty", async () => {
    server.use(...ruleHandlers(), ...accountHandlers([propAccount()]).handlers);
    const { unmount } = renderAccounts(<AccountDetail accountId={ids.acc1} alertSlot={<p>alerta de risco</p>} riskSection={<p>resumo de risco</p>} />);
    expect(await screen.findByText("alerta de risco")).toBeInTheDocument();
    expect(screen.getByText("resumo de risco")).toBeInTheDocument();
    unmount();
    const { container } = renderAccounts(<AccountDetail accountId={ids.acc1} alertSlot={null} riskSection={false} />);
    await screen.findByRole("heading", { name: /./, level: 1 });
    expect(screen.queryByText("alerta de risco")).not.toBeInTheDocument();
    // No empty wrapper for an absent alert: nothing between the header and the first card.
    expect(container.querySelectorAll(":scope > div > div:empty")).toHaveLength(0);
  });

  it("shows no opt-in card for a real account", async () => {
    server.use(...accountHandlers([realAccount()]).handlers);
    renderAccounts(<AccountDetail accountId={ids.acc2} />);
    expect(await screen.findByRole("heading", { name: "Conta real" })).toBeInTheDocument();
    expect(screen.queryByText("Automação")).not.toBeInTheDocument();
  });

  it("shows the recorded automation choice with the rule version", async () => {
    server.use(...ruleHandlers(), ...accountHandlers([propAccount()]).handlers);
    renderAccounts(<AccountDetail accountId={ids.acc1} />);
    expect(await screen.findByText("Sem automação")).toBeInTheDocument();
    expect(await screen.findByText(/regra da Apex Trader Funding versão 3/)).toBeInTheDocument();
  });

  it("shows the suspension alert with its cause and resolves it through the review dialog", async () => {
    const acc = pendingAccount();
    const { handlers } = accountHandlers([acc]);
    const puts: unknown[] = [];
    server.use(
      http.put("/api/v1/accounts/:id/opt-ins/:kind", async ({ request }) => {
        puts.push(await request.json());
        return HttpResponse.json({ kind: "automation", firm_stance: "forbidden", state: "chosen", choice: "accepted", chosen_at: "2026-10-23T00:00:00Z", rule_version_id: ids.ruleAuto2, current_rule_version_id: ids.ruleAuto2 });
      }),
      ...ruleHandlers(v4),
      ...handlers,
    );
    const stepUp = vi.fn(async ({ send }: Parameters<StepUpRunner>[0]) => send({ challenge_id: "c", code: "1" })) as unknown as StepUpRunner;
    const user = userEvent.setup();
    renderAccounts(<AccountDetail accountId={acc.id} stepUp={stepUp} />);
    expect(await screen.findByText(/Ordens automatizadas estão suspensas nesta conta até você revisar/)).toBeInTheDocument();
    expect(screen.getByText(/Motivo informado: Texto reescrito\./)).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Revisar agora" }));
    const dialog = await screen.findByRole("dialog");
    expect(within(dialog).getByText("Sua escolha anterior: Aceitar")).toBeInTheDocument();
    expect(within(dialog).getAllByRole("radio").every((r) => r.getAttribute("aria-checked") === "false")).toBe(true);
    await user.click(within(dialog).getByRole("radio", { name: /Aceitar ordens automatizadas/ }));
    await user.click(within(dialog).getByRole("checkbox"));
    await user.click(within(dialog).getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(puts).toEqual([{ choice: "accepted", rule_version_id: ids.ruleAuto2, step_up: { challenge_id: "c", code: "1" } }]));
    expect(stepUp).toHaveBeenCalledTimes(1);
  });

  it("firm text is rendered as plain text, never as HTML", async () => {
    const hostile = ruleVersion({ quotes: [{ text: "<img src=x onerror=alert(1)> no automation", language: "en", page_title: "Rules", url: "https://example.com", accessed_at: "2026-10-23" }] });
    const acc = pendingAccount();
    server.use(...ruleHandlers(ruleVersion({ ...hostile, id: ids.ruleAuto2 })), ...accountHandlers([acc]).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountDetail accountId={acc.id} />);
    await user.click(await screen.findByRole("button", { name: "Revisar agora" }));
    const dialog = await screen.findByRole("dialog");
    expect(await within(dialog).findByText(/<img src=x onerror=alert\(1\)> no automation/)).toBeInTheDocument();
    expect(dialog.querySelector("img")).toBeNull();
  });

  it("translates the stance names in the what-changed line", async () => {
    const acc = pendingAccount();
    const { handlers } = accountHandlers([acc]);
    const changed = ruleVersion({ id: ids.ruleAuto2, version_number: 4, firm_stance: "not_confirmed", change: { previous_rule_version_id: ids.ruleAuto, previous_firm_stance: "unknown", cause: "stance_changed" } });
    server.use(...ruleHandlers(changed), ...handlers);
    renderAccounts(<AccountDetail accountId={acc.id} />);
    expect(await screen.findByText(/passou de desconhecido para não confirmado/)).toBeInTheDocument();
  });

  it("a stale rule (409) in the review dialog refreshes the account so the new rule version loads", async () => {
    const acc = pendingAccount();
    const { handlers } = accountHandlers([acc]);
    let gets = 0;
    server.use(
      http.get("/api/v1/accounts/:id", () => {
        gets += 1;
        return HttpResponse.json(gets === 1 ? acc : { ...acc, opt_ins: { ...acc.opt_ins, automation: { ...acc.opt_ins.automation, current_rule_version_id: ids.ruleAuto2 } } });
      }),
      http.put("/api/v1/accounts/:id/opt-ins/:kind", () => apiError(409, "accounts.rule_version_changed", { current_rule_version_id: ids.ruleAuto2 })),
      ...ruleHandlers(v4),
      ...handlers,
    );
    const user = userEvent.setup();
    renderAccounts(<AccountDetail accountId={acc.id} />);
    await user.click(await screen.findByRole("button", { name: "Revisar agora" }));
    const dialog = await screen.findByRole("dialog");
    await user.click(await within(dialog).findByRole("radio", { name: /Não aceitar ordens automatizadas/ }));
    await user.click(within(dialog).getByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(gets).toBeGreaterThanOrEqual(2));
    expect(await within(dialog).findByText(/foi atualizada enquanto você preenchia/)).toBeInTheDocument();
  });
});
