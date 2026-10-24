// The navigation mock lives in render-accounts and must load before the components.
import { nav, renderAccounts } from "./testing/render-accounts";
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { server } from "@/test/server";
import { AccountsList, filterAccounts } from "./accounts-list";
import { propAccount, realAccount } from "./testing/fixtures";
import { accountHandlers } from "./testing/handlers";

beforeEach(() => {
  nav.search = "";
});

vi.setConfig({ testTimeout: 20_000 });

describe("AccountsList", () => {
  it("lists the accounts sorted by name with state and binding", async () => {
    server.use(...accountHandlers([realAccount({ display_name: "Zeta" }), propAccount({ display_name: "Alfa", lifecycle_state: "paused" })]).handlers);
    renderAccounts(<AccountsList />);
    const links = await screen.findAllByRole("link", { name: /Alfa|Zeta/ });
    expect(links.map((l) => l.textContent)).toEqual(["Alfa", "Zeta"]);
    expect(screen.getByText("Pausada")).toBeInTheDocument();
    expect(screen.getAllByText("Sem vínculo")).toHaveLength(2);
  });

  it("adds the page's extra column (header + one cell per account) only when the slot is given", async () => {
    server.use(...accountHandlers([realAccount({ display_name: "Zeta" }), propAccount({ display_name: "Alfa" })]).handlers);
    const { unmount } = renderAccounts(<AccountsList riskColumn={{ header: "Risco hoje", cell: (account) => <span>{`cel-${account.display_name}`}</span> }} />);
    await screen.findAllByRole("link", { name: /Alfa|Zeta/ });
    expect(screen.getAllByText(/Risco hoje/).length).toBeGreaterThan(0);
    expect(screen.getByText("cel-Alfa")).toBeInTheDocument();
    expect(screen.getByText("cel-Zeta")).toBeInTheDocument();
    unmount();
    renderAccounts(<AccountsList />);
    await screen.findAllByRole("link", { name: /Alfa|Zeta/ });
    expect(screen.queryByText(/Risco hoje/)).not.toBeInTheDocument();
  });

  it("shows the empty state with a create action", async () => {
    server.use(...accountHandlers([]).handlers);
    renderAccounts(<AccountsList />);
    expect(await screen.findByText("Você ainda não cadastrou contas.")).toBeInTheDocument();
    expect(screen.getAllByRole("link", { name: "Nova conta" }).length).toBeGreaterThan(0);
  });

  it("shows an error state with retry", async () => {
    server.use(http.get("/api/v1/accounts", () => HttpResponse.json({ error: { code: "http.500" } }, { status: 500 })));
    renderAccounts(<AccountsList />);
    expect(await screen.findByText("Não foi possível carregar suas contas.")).toBeInTheDocument();
  });

  it("filters by the pending-choice state from the URL", async () => {
    nav.search = "state=choice_pending";
    const pending = propAccount({
      display_name: "Pendente",
      opt_ins: { automation: { kind: "automation", firm_stance: "forbidden", state: "pending_review", pending_reason: "stance_changed", current_rule_version_id: "r2" }, copy: { kind: "copy", state: "not_required" } },
    });
    server.use(...accountHandlers([pending, realAccount({ display_name: "Normal" })]).handlers);
    renderAccounts(<AccountsList />);
    expect(await screen.findByText("Pendente")).toBeInTheDocument();
    expect(screen.queryByText("Normal")).not.toBeInTheDocument();
    expect(within(screen.getByRole("listitem")).getByText("Escolha pendente")).toBeInTheDocument();
  });

  it("pauses after confirmation, sending the row version", async () => {
    const { handlers, calls } = accountHandlers([realAccount({ display_name: "Conta A", row_version: 4 })]);
    server.use(...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountsList />);
    await user.click(await screen.findByRole("button", { name: /Ações: Conta A/ }));
    await user.click(await screen.findByRole("menuitem", { name: "Pausar" }));
    const dialog = await screen.findByRole("alertdialog");
    expect(within(dialog).getByText(/nem de saída \(fechar\)/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Pausar conta" }));
    await waitFor(() => expect(calls.lifecycle).toEqual([{ state: "paused", row_version: 4 }]));
    expect(await screen.findByText("Conta pausada.")).toBeInTheDocument();
  });

  it("explains instead of confirming when deleting with an open position", async () => {
    server.use(...accountHandlers([realAccount({ display_name: "Conta A", has_open_position: true })]).handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountsList />);
    await user.click(await screen.findByRole("button", { name: /Ações: Conta A/ }));
    await user.click(await screen.findByRole("menuitem", { name: "Excluir" }));
    const dialog = await screen.findByRole("alertdialog");
    expect(within(dialog).getByText(/há posição aberta/)).toBeInTheDocument();
    expect(within(dialog).queryByRole("button", { name: "Excluir conta" })).not.toBeInTheDocument();
  });

  it("deletes after confirmation and removes the row", async () => {
    const { handlers, accounts } = accountHandlers([realAccount({ display_name: "Conta A" })]);
    server.use(...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountsList />);
    await user.click(await screen.findByRole("button", { name: /Ações: Conta A/ }));
    await user.click(await screen.findByRole("menuitem", { name: "Excluir" }));
    await user.click(await screen.findByRole("button", { name: "Excluir conta" }));
    await waitFor(() => expect(accounts.size).toBe(0));
    expect(await screen.findByText("Conta excluída.")).toBeInTheDocument();
  });
});

describe("filterAccounts", () => {
  const list = [propAccount({ display_name: "Apex" }), realAccount({ display_name: "Casa", lifecycle_state: "disabled" })];
  it("filters by text, type and lifecycle", () => {
    expect(filterAccounts(list, { q: "ap", type: "all", state: "all" })).toHaveLength(1);
    expect(filterAccounts(list, { q: "", type: "real", state: "all" })[0]?.display_name).toBe("Casa");
    expect(filterAccounts(list, { q: "", type: "all", state: "disabled" })).toHaveLength(1);
  });
});
