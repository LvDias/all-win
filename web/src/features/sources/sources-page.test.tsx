import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
import { SourcesPage } from "./sources-page";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const secretUrl = "https://allwin.example/ingress/v1/tradingview/awk_AbCdEfGhIjKl_" + "x".repeat(43);
const source = (overrides: Record<string, unknown> = {}) => ({
  id: "00000000-0000-4000-8000-0000000000a1",
  name: "Strikes MNQ",
  source_type: "tradingview",
  is_automated: true,
  status: "active",
  key_id: "AbCdEfGhIjKl",
  masked_url: "https://allwin.example/ingress/v1/tradingview/awk_AbCdEfGhIjKl_…",
  created_at: "2026-10-20T12:00:00Z",
  ...overrides,
});

function list(rows: unknown[]) {
  server.use(http.get("*/api/v1/sources", ({ request }) => HttpResponse.json(new URL(request.url).searchParams.get("include_revoked") === "true" ? rows : (rows as Array<{ status: string }>).filter((row) => row.status === "active"))));
}

beforeEach(() => server.use(http.get("*/api/v1/me", () => HttpResponse.json(user))));

describe("SourcesPage", () => {
  it("lists sources with the masked URL only and the automation badge", async () => {
    list([source()]);
    const { container } = renderApp(<SourcesPage />);
    expect(await screen.findByText("Strikes MNQ")).toBeInTheDocument();
    expect(screen.getByText("Automatizada")).toBeInTheDocument();
    expect(screen.getByLabelText("URL secreta, oculta, identificador AbCdEfGhIjKl")).toBeInTheDocument();
    expect(container.innerHTML).not.toContain("x".repeat(43));
    expect(screen.getByText("Nunca")).toBeInTheDocument();
  });

  it("shows the empty state and loading and error states", async () => {
    list([]);
    renderApp(<SourcesPage />);
    expect(await screen.findByText("Nenhuma fonte de sinal")).toBeInTheDocument();
  });

  it("shows an error state with the code and a retry", async () => {
    server.use(http.get("*/api/v1/sources", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    renderApp(<SourcesPage />);
    expect(await screen.findByText("Erro inesperado no servidor. Tente de novo.")).toBeInTheDocument();
    expect(screen.getByText("Código: common.internal")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Tentar novamente" })).toBeInTheDocument();
  });

  it("hides revoked sources until asked", async () => {
    list([source(), source({ id: "00000000-0000-4000-8000-0000000000a2", name: "Old", status: "revoked", revoked_at: "2026-10-21T12:00:00Z" })]);
    renderApp(<SourcesPage />);
    await screen.findByText("Strikes MNQ");
    expect(screen.queryByText("Old")).not.toBeInTheDocument();
    await userEvent.setup().click(screen.getByRole("switch", { name: "Mostrar revogadas" }));
    expect(await screen.findByText("Old")).toBeInTheDocument();
    const card = screen.getByText("Old").closest("li") as HTMLElement;
    expect(within(card).queryByRole("button", { name: /Ações para/ })).not.toBeInTheDocument();
  });

  it("creates a source and shows the URL once, masked until revealed", async () => {
    list([]);
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/sources", async ({ request }) => { bodies.push(await request.json()); return HttpResponse.json({ source: source(), webhook_url: secretUrl }, { status: 201 }); }));
    renderApp(<SourcesPage />);
    const person = userEvent.setup();
    await person.click((await screen.findAllByRole("button", { name: "Nova fonte" }))[0] as HTMLElement);
    await person.type(await screen.findByLabelText("Nome da fonte"), "  Strikes MNQ ");
    await person.click(screen.getByRole("button", { name: "Criar" }));
    const dialog = await screen.findByRole("dialog");
    expect(bodies).toEqual([{ name: "Strikes MNQ", source_type: "tradingview" }]);
    expect(within(dialog).getByText(/awk_AbCdEfGhIjKl_••••••••/)).toBeInTheDocument();
    expect(dialog.innerHTML).not.toContain("x".repeat(43));
    await person.click(within(dialog).getByRole("button", { name: "Revelar" }));
    expect(dialog.innerHTML).toContain("x".repeat(43));
  });

  it("asks before closing without having copied, and not after copying", async () => {
    list([]);
    server.use(http.post("*/api/v1/sources", () => HttpResponse.json({ source: source(), webhook_url: secretUrl }, { status: 201 })));
    renderApp(<SourcesPage />);
    const person = userEvent.setup();
    await person.click((await screen.findAllByRole("button", { name: "Nova fonte" }))[0] as HTMLElement);
    await person.type(await screen.findByLabelText("Nome da fonte"), "S");
    await person.click(screen.getByRole("button", { name: "Criar" }));
    const dialog = await screen.findByRole("dialog");
    await person.click(within(dialog).getByRole("button", { name: "Concluir" }));
    expect(await screen.findByText(/Você ainda não copiou a URL/)).toBeInTheDocument();
    await person.click(screen.getByRole("button", { name: "Voltar e copiar" }));
    await person.click(within(dialog).getByRole("button", { name: "Copiar" }));
    await person.click(within(dialog).getByRole("button", { name: "Concluir" }));
    await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());
    expect(screen.queryByText(/Você ainda não copiou/)).not.toBeInTheDocument();
  });

  it("shows the duplicate name inline and creates nothing", async () => {
    list([]);
    server.use(http.post("*/api/v1/sources", () => HttpResponse.json({ error: { code: "sources.name_taken" } }, { status: 409 })));
    renderApp(<SourcesPage />);
    const person = userEvent.setup();
    await person.click((await screen.findAllByRole("button", { name: "Nova fonte" }))[0] as HTMLElement);
    await person.type(await screen.findByLabelText("Nome da fonte"), "Dup");
    await person.click(screen.getByRole("button", { name: "Criar" }));
    expect(await screen.findByText("Você já tem uma fonte com este nome.")).toBeInTheDocument();
  });

  it("says the outcome is unknown after a network failure and offers a refresh", async () => {
    list([]);
    server.use(http.post("*/api/v1/sources", () => HttpResponse.error()));
    renderApp(<SourcesPage />);
    const person = userEvent.setup();
    await person.click((await screen.findAllByRole("button", { name: "Nova fonte" }))[0] as HTMLElement);
    await person.type(await screen.findByLabelText("Nome da fonte"), "Net");
    await person.click(screen.getByRole("button", { name: "Criar" }));
    expect(await screen.findByText(/Não sabemos se a fonte foi criada/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Atualizar lista" })).toBeInTheDocument();
  });

  it("rotates after confirmation, retries once after step-up and shows the new URL", async () => {
    list([source()]);
    let attempts = 0;
    server.use(
      http.get("*/api/v1/me/security", () => HttpResponse.json({ totp_enabled: false, recovery_codes_remaining: 0, session: {} })),
      http.post("*/api/v1/auth/step-up", () => new HttpResponse(null, { status: 204 })),
      http.post("*/api/v1/sources/:id/rotate", () => (++attempts === 1 ? HttpResponse.json({ error: { code: "auth.step_up_required" } }, { status: 403 }) : HttpResponse.json({ source: source(), webhook_url: secretUrl }))),
    );
    renderApp(<SourcesPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ações para Strikes MNQ" }));
    await person.click(await screen.findByRole("menuitem", { name: "Gerar nova URL" }));
    const confirm = await screen.findByRole("alertdialog");
    expect(within(confirm).getByText(/para de funcionar imediatamente/)).toBeInTheDocument();
    await person.click(within(confirm).getByRole("button", { name: "Gerar nova URL" }));
    const stepUp = await screen.findByLabelText("Senha");
    await person.type(stepUp, "pw");
    await person.click(screen.getByRole("button", { name: "Confirmar" }));
    expect(await screen.findByText("Copie a nova URL")).toBeInTheDocument();
    expect(attempts).toBe(2);
  });

  it("revokes after confirmation", async () => {
    list([source()]);
    const revoked = vi.fn();
    server.use(http.post("*/api/v1/sources/:id/revoke", () => { revoked(); return new HttpResponse(null, { status: 204 }); }));
    renderApp(<SourcesPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ações para Strikes MNQ" }));
    await person.click(await screen.findByRole("menuitem", { name: "Revogar" }));
    await person.click(within(await screen.findByRole("alertdialog")).getByRole("button", { name: "Revogar fonte" }));
    await waitFor(() => expect(revoked).toHaveBeenCalledOnce());
  });

  it("tells the user a stale source was already revoked and refreshes", async () => {
    list([source()]);
    server.use(http.patch("*/api/v1/sources/:id", () => HttpResponse.json({ error: { code: "sources.revoked" } }, { status: 409 })));
    renderApp(<SourcesPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ações para Strikes MNQ" }));
    await person.click(await screen.findByRole("menuitem", { name: "Renomear" }));
    const input = await screen.findByLabelText("Nome da fonte");
    await person.clear(input);
    await person.type(input, "New");
    await person.click(screen.getByRole("button", { name: "Salvar" }));
    expect(await screen.findByText("Esta fonte já foi revogada.")).toBeInTheDocument();
  });
});
