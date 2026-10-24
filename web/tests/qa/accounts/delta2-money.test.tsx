// QA (Lente) — S5 delta 2: shared money parser hardening. pt-BR "25.000" is ambiguous (thousands vs decimal) and is refused with its own message.
import { renderAccounts, accountHandlers } from "@/../tests/qa/shared/accounts";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { describe, expect, it, vi } from "vitest";
import { AccountCreateStepper } from "@/features/accounts";
import { server } from "@/test/server";

vi.setConfig({ testTimeout: 60_000 });

type User = ReturnType<typeof userEvent.setup>;
const noConnections = http.get("/api/v1/connections", () => HttpResponse.json([]));
const next = (user: User) => user.click(screen.getByRole("button", { name: "Continuar" }));

async function typeSize(user: User, value: string) {
  await user.click(screen.getByRole("radio", { name: "Conta real em corretora" }));
  await next(user);
  await user.type(await screen.findByLabelText("Nome"), "Dinheiro");
  await user.type(screen.getByLabelText("Tamanho (USD)"), value);
  await user.type(screen.getByLabelText("Máximo de contratos"), "1");
  await next(user);
}

describe("QA S5 delta 2 — money input", () => {
  it.each(["25.000", "25.5.0", "1,000.00", "abc", "0,001"])("G11c %s is refused, nothing is sent", async (value) => {
    const { handlers, calls } = accountHandlers([]);
    server.use(noConnections, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await typeSize(user, value);
    expect(await screen.findByText(value === "25.000" ? /Digite a parte decimal explicitamente/ : /Informe um valor|Digite a parte decimal/)).toBeInTheDocument();
    expect(calls.create).toHaveLength(0);
  });

  it.each([
    ["25000", "25000"],
    ["25000,50", "25000.5"],
    ["25000.50", "25000.5"],
  ])("G11d %s is accepted, the review shows the parsed value and the wire carries %s", async (typed, wire) => {
    const { handlers, calls } = accountHandlers([]);
    server.use(noConnections, ...handlers);
    const user = userEvent.setup();
    renderAccounts(<AccountCreateStepper />);
    await typeSize(user, typed);
    await next(user);
    expect(await screen.findByText(/US\$\s?25\.000/)).toBeInTheDocument();
    await user.click(await screen.findByRole("button", { name: "Salvar conta" }));
    await waitFor(() => expect(calls.create).toHaveLength(1));
    expect(calls.create[0]?.body.size_amount).toBe(wire);
  });
});
