import { act, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
import { FakeEventSource } from "@/test/setup";
import { ConnectionsPage } from "./connections-page";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const id = "00000000-0000-4000-8000-0000000000c1";
const connection = (overrides: Record<string, unknown> = {}) => ({
  id,
  name: "VPS Chicago",
  status: "connected",
  status_changed_at: "2026-10-23T11:57:00Z",
  created_at: "2026-10-20T12:00:00Z",
  adapter_version: "1.2.0",
  instance_id: "00000000-0000-4000-8000-0000000000d1",
  instance_name: "WIN-CHI",
  last_seen_at: "2026-10-23T11:59:00Z",
  visible_accounts: [
    { platform_account: "Sim101", kind: "simulation", oco_location: "local", account_id: "00000000-0000-4000-8000-0000000000e1" },
    { platform_account: "Playback101", kind: "playback", oco_location: "local" },
  ],
  ...overrides,
});
const pairing = { pairing_code: "K7QM-4D2P", expires_at: new Date(Date.now() + 9 * 60_000).toISOString() };

function list(rows: unknown[]) {
  server.use(http.get("*/api/v1/connections", () => HttpResponse.json(rows)));
}
/** The stream opens once /me answers; wait for it before emitting. */
async function stream() {
  await waitFor(() => expect(FakeEventSource.latest()).toBeDefined());
  const source = FakeEventSource.latest();
  if (!source) throw new Error("no event stream");
  return source;
}

beforeEach(() => server.use(http.get("*/api/v1/me", () => HttpResponse.json(user))));

describe("ConnectionsPage", () => {
  it("lists connections with status, plugin version and the accounts with their binding", async () => {
    list([connection()]);
    renderApp(<ConnectionsPage />);
    expect(await screen.findByText("VPS Chicago")).toBeInTheDocument();
    expect(screen.getByText("WIN-CHI")).toBeInTheDocument();
    expect(screen.getByText("Conectada")).toBeInTheDocument();
    expect(screen.getByText("1.2.0")).toBeInTheDocument();
    await userEvent.setup().click(screen.getByRole("button", { name: "2 contas" }));
    expect(screen.getByText("Sim101")).toBeInTheDocument();
    expect(screen.getByText("Vinculada")).toBeInTheDocument();
    expect(screen.getByText("Não vinculada")).toBeInTheDocument();
  });

  it("shows consequence wording when disconnected and for outdated plugins", async () => {
    list([connection({ status: "disconnected" }), connection({ id: "00000000-0000-4000-8000-0000000000c2", name: "Old", status: "outdated" })]);
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    await screen.findByText("VPS Chicago");
    for (const button of screen.getAllByRole("button", { name: /contas?$/ })) await person.click(button);
    expect(screen.getByText(/Ordens para as contas desta instância não serão executadas/)).toBeInTheDocument();
    expect(screen.getByText("Atualize o plugin para voltar a conectar.")).toBeInTheDocument();
  });

  it("shows the generic text when disconnect_reason is absent and the reason text when it arrives", async () => {
    list([connection({ status: "disconnected" }), connection({ id: "00000000-0000-4000-8000-0000000000c3", name: "Mac", status: "disconnected", disconnect_reason: "user" }), connection({ id: "00000000-0000-4000-8000-0000000000c4", name: "Net", status: "disconnected", disconnect_reason: "network" })]);
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    await screen.findByText("VPS Chicago");
    for (const button of screen.getAllByRole("button", { name: /contas?$/ })) await person.click(button);
    expect(screen.getAllByText("Desconectada")).toHaveLength(3);
    expect(screen.getByText("O plugin não está conectado. Ordens para as contas desta instância não serão executadas.")).toBeInTheDocument();
    expect(screen.getByText(/Desconectada pelo usuário no plugin/)).toBeInTheDocument();
    expect(screen.getByText(/O plugin perdeu a conexão/)).toBeInTheDocument();
  });

  it("shows the empty state with the three steps", async () => {
    list([]);
    renderApp(<ConnectionsPage />);
    expect(await screen.findByText("Nenhum NinjaTrader conectado")).toBeInTheDocument();
    expect(screen.getByText("Digite o código de pareamento no plugin.")).toBeInTheDocument();
  });

  it("creates a connection, shows the visible pairing code and reacts to connection.paired", async () => {
    list([]);
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/connections", async ({ request }) => { bodies.push(await request.json()); return HttpResponse.json({ connection: connection({ status: "awaiting_pairing" }), pairing }, { status: 201 }); }));
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    await person.click((await screen.findAllByRole("button", { name: "Nova conexão" }))[0] as HTMLElement);
    await person.type(await screen.findByLabelText("Nome da conexão"), "VPS Chicago");
    await person.click(screen.getByRole("button", { name: "Criar" }));
    const dialog = await screen.findByRole("dialog");
    expect(bodies).toEqual([{ name: "VPS Chicago" }]);
    expect(within(dialog).getByLabelText("Código de pareamento")).toHaveTextContent("K7QM-4D2P");
    expect(within(dialog).getByText(/Expira em \d:\d\d/)).toBeInTheDocument();
    expect(within(dialog).getByText("Aguardando o plugin…")).toBeInTheDocument();

    const events = await stream();
    list([connection({ visible_accounts: [{ platform_account: "Sim101", kind: "simulation", oco_location: "local" }] })]);
    act(() => events.emit("connection.paired", { type: "connection.paired", at: "2026-10-23T12:00:00Z", connection_id: id, instance_name: "WIN-CHI", visible_accounts: [] }));
    expect(await within(dialog).findByText(/1 conta encontrada: Sim101/)).toBeInTheDocument();
    expect(within(dialog).getByRole("heading", { name: "“VPS Chicago” conectada" })).toBeInTheDocument();
  });

  it("shows a neutral line while the paired event has no accounts, then the accounts once the list says connected", async () => {
    list([connection({ status: "awaiting_pairing", visible_accounts: [] })]);
    server.use(http.post("*/api/v1/connections", () => HttpResponse.json({ connection: connection({ status: "awaiting_pairing" }), pairing }, { status: 201 })));
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    await person.click((await screen.findAllByRole("button", { name: "Nova conexão" }))[0] as HTMLElement);
    await person.type(await screen.findByLabelText("Nome da conexão"), "VPS Chicago");
    await person.click(screen.getByRole("button", { name: "Criar" }));
    const dialog = await screen.findByRole("dialog");
    const events = await stream();

    act(() => events.emit("connection.paired", { type: "connection.paired", at: "2026-10-23T12:00:00Z", connection_id: id, instance_name: "WIN-CHI", visible_accounts: [] }));
    const status = (await within(dialog).findByText("Buscando contas…")).closest('[role="status"]');
    expect(status).not.toBeNull();
    expect(within(dialog).getByRole("heading", { name: "“VPS Chicago” pareada" })).toBeInTheDocument();
    expect(within(dialog).queryByRole("link", { name: "Vincular contas" })).not.toBeInTheDocument();
    expect(within(dialog).queryByText(/Nenhuma conta encontrada/)).not.toBeInTheDocument();

    list([connection()]);
    act(() => events.emit("connection.status_changed", { type: "connection.status_changed", at: "2026-10-23T12:00:05Z", connection_id: id, status: "connected" }));
    await waitFor(() => expect(status).toHaveTextContent("2 contas encontradas: Sim101, Playback101"));
    expect(status).toBeInTheDocument();
    expect(within(dialog).getByRole("heading", { name: "“VPS Chicago” conectada" })).toBeInTheDocument();
    expect(within(dialog).getByRole("link", { name: "Vincular contas" })).toHaveAttribute("href", "/pt-BR/accounts");
    expect(within(dialog).queryByText("Buscando contas…")).not.toBeInTheDocument();
  });

  it("ignores events of other connections", async () => {
    list([]);
    server.use(http.post("*/api/v1/connections", () => HttpResponse.json({ connection: connection({ status: "awaiting_pairing" }), pairing }, { status: 201 })));
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    await person.click((await screen.findAllByRole("button", { name: "Nova conexão" }))[0] as HTMLElement);
    await person.type(await screen.findByLabelText("Nome da conexão"), "X");
    await person.click(screen.getByRole("button", { name: "Criar" }));
    const dialog = await screen.findByRole("dialog");
    const events = await stream();
    act(() => events.emit("connection.paired", { type: "connection.paired", at: "2026-10-23T12:00:00Z", connection_id: "00000000-0000-4000-8000-0000000000ff", instance_name: "OTHER", visible_accounts: [] }));
    expect(within(dialog).getByText("Aguardando o plugin…")).toBeInTheDocument();
  });

  it.each([
    ["superseded", /Um código mais novo foi gerado/],
    ["pairing_locked", /temporariamente bloqueado/],
  ])("handles pairing_code_invalidated (%s) and offers a new code", async (reason, text) => {
    list([]);
    server.use(http.post("*/api/v1/connections", () => HttpResponse.json({ connection: connection({ status: "awaiting_pairing" }), pairing }, { status: 201 })));
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    await person.click((await screen.findAllByRole("button", { name: "Nova conexão" }))[0] as HTMLElement);
    await person.type(await screen.findByLabelText("Nome da conexão"), "X");
    await person.click(screen.getByRole("button", { name: "Criar" }));
    const dialog = await screen.findByRole("dialog");
    const events = await stream();
    act(() => events.emit("connection.pairing_code_invalidated", { type: "connection.pairing_code_invalidated", at: "2026-10-23T12:00:00Z", connection_id: id, reason }));
    expect(await within(dialog).findByText(text)).toBeInTheDocument();
    expect(within(dialog).getByLabelText("Código de pareamento")).toHaveTextContent("—");
    expect(within(dialog).getByRole("button", { name: "Gerar novo código" })).toBeInTheDocument();
  });

  it("derives expiry from expires_at and generates a new code", async () => {
    list([]);
    server.use(
      http.post("*/api/v1/connections", () => HttpResponse.json({ connection: connection({ status: "awaiting_pairing" }), pairing: { pairing_code: "AAAA-BBBB", expires_at: new Date(Date.now() - 1000).toISOString() } }, { status: 201 })),
      http.post("*/api/v1/connections/:id/pairing-code", () => HttpResponse.json(pairing, { status: 201 })),
    );
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    await person.click((await screen.findAllByRole("button", { name: "Nova conexão" }))[0] as HTMLElement);
    await person.type(await screen.findByLabelText("Nome da conexão"), "X");
    await person.click(screen.getByRole("button", { name: "Criar" }));
    const dialog = await screen.findByRole("dialog");
    expect((await within(dialog).findAllByText("O código expirou.")).length).toBeGreaterThan(0);
    await person.click(within(dialog).getByRole("button", { name: "Gerar novo código" }));
    await waitFor(() => expect(within(dialog).getByLabelText("Código de pareamento")).toHaveTextContent("K7QM-4D2P"));
  });

  it("shows the duplicate connection name inline", async () => {
    list([]);
    server.use(http.post("*/api/v1/connections", () => HttpResponse.json({ error: { code: "connections.name_taken" } }, { status: 409 })));
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    await person.click((await screen.findAllByRole("button", { name: "Nova conexão" }))[0] as HTMLElement);
    await person.type(await screen.findByLabelText("Nome da conexão"), "Dup");
    await person.click(screen.getByRole("button", { name: "Criar" }));
    expect(await screen.findByText("Você já tem uma conexão com este nome.")).toBeInTheDocument();
  });

  it("moves to another machine: confirmation first, then a new code", async () => {
    list([connection()]);
    server.use(http.post("*/api/v1/connections/:id/pairing-code", () => HttpResponse.json(pairing, { status: 201 })));
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ações para VPS Chicago" }));
    await person.click(await screen.findByRole("menuitem", { name: "Mover para outra máquina" }));
    const confirm = await screen.findByRole("alertdialog");
    expect(within(confirm).getByText(/a instância atual será desconectada/)).toBeInTheDocument();
    await person.click(within(confirm).getByRole("button", { name: "Gerar código" }));
    expect(await screen.findByLabelText("Código de pareamento")).toHaveTextContent("K7QM-4D2P");
  });

  it("unpairs with the affected accounts listed and requires acknowledging open positions", async () => {
    list([connection({ open_position_accounts: ["Sim101"] })]);
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/connections/:id/unpair", async ({ request }) => { bodies.push(await request.json()); return new HttpResponse(null, { status: 204 }); }));
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ações para VPS Chicago" }));
    await person.click(await screen.findByRole("menuitem", { name: "Desparear" }));
    const dialog = await screen.findByRole("alertdialog");
    expect(within(dialog).getByText(/Estas contas ficarão fora de sincronia.*Sim101/)).toBeInTheDocument();
    expect(within(dialog).getByText(/A posição continua aberta no NinjaTrader/)).toBeInTheDocument();
    const confirm = within(dialog).getByRole("button", { name: "Desparear" });
    expect(confirm).toBeDisabled();
    await person.click(within(dialog).getByRole("checkbox", { name: "Entendo que a posição continuará aberta" }));
    await person.click(confirm);
    await waitFor(() => expect(bodies).toEqual([{ acknowledge_open_positions: true }]));
  });

  it("keeps the dialog open and shows the position alert when the Core says positions were not acknowledged", async () => {
    let calls = 0;
    server.use(http.get("*/api/v1/connections", () => HttpResponse.json([connection(calls++ === 0 ? {} : { open_position_accounts: ["Sim101"] })])));
    server.use(http.post("*/api/v1/connections/:id/unpair", () => HttpResponse.json({ error: { code: "connections.open_positions_not_acknowledged" } }, { status: 409 })));
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ações para VPS Chicago" }));
    await person.click(await screen.findByRole("menuitem", { name: "Desparear" }));
    const dialog = await screen.findByRole("alertdialog");
    await person.click(within(dialog).getByRole("button", { name: "Desparear" }));
    expect(await within(dialog).findByText(/A posição continua aberta/)).toBeInTheDocument();
    expect(screen.getByRole("alertdialog")).toBeInTheDocument();
  });

  it("refetches the list on connection.status_changed", async () => {
    let calls = 0;
    server.use(http.get("*/api/v1/connections", () => HttpResponse.json([connection(++calls > 1 ? { status: "disconnected" } : {})])));
    renderApp(<ConnectionsPage />);
    expect(await screen.findByText("Conectada")).toBeInTheDocument();
    const events = await stream();
    act(() => events.emit("connection.status_changed", { type: "connection.status_changed", at: "2026-10-23T12:00:00Z", connection_id: id, status: "disconnected" }));
    expect(await screen.findByText("Desconectada")).toBeInTheDocument();
  });
});

describe("ConnectionsPage rename and unpaired history", () => {
  it("renames a connection", async () => {
    list([connection()]);
    const bodies: unknown[] = [];
    server.use(http.patch("*/api/v1/connections/:id", async ({ request }) => { bodies.push(await request.json()); return HttpResponse.json(connection({ name: "Home PC" })); }));
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ações para VPS Chicago" }));
    await person.click(await screen.findByRole("menuitem", { name: "Renomear" }));
    const input = await screen.findByLabelText("Nome da conexão");
    await person.clear(input);
    await person.type(input, "Home PC");
    await person.click(screen.getByRole("button", { name: "Salvar" }));
    await waitFor(() => expect(bodies).toEqual([{ name: "Home PC" }]));
  });

  it("'Mostrar despareadas' lists unpaired connections as read-only history", async () => {
    const asked: string[] = [];
    server.use(
      http.get("*/api/v1/connections", ({ request }) => {
        const include = new URL(request.url).searchParams.get("include_unpaired") === "true";
        asked.push(String(include));
        return HttpResponse.json(include ? [connection(), connection({ id: "00000000-0000-4000-8000-0000000000c9", name: "Old PC", status: "unpaired", unpaired_at: "2026-10-22T12:00:00Z" })] : [connection()]);
      }),
    );
    renderApp(<ConnectionsPage />);
    await screen.findByText("VPS Chicago");
    expect(screen.queryByText("Old PC")).not.toBeInTheDocument();
    await userEvent.setup().click(screen.getByRole("switch", { name: "Mostrar despareadas" }));
    expect(await screen.findByText("Old PC")).toBeInTheDocument();
    expect(screen.getByText("Despareada")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Ações para Old PC" })).not.toBeInTheDocument();
    expect(asked).toContain("true");
  });

  it("returns focus to 'Nova conexão' after the pairing dialog closes with Esc", async () => {
    list([]);
    server.use(http.post("*/api/v1/connections", () => HttpResponse.json({ connection: connection({ status: "awaiting_pairing" }), pairing }, { status: 201 })));
    renderApp(<ConnectionsPage />);
    const person = userEvent.setup();
    const open = (await screen.findAllByRole("button", { name: "Nova conexão" }))[0] as HTMLElement;
    await person.click(open);
    await person.type(await screen.findByLabelText("Nome da conexão"), "X");
    await person.click(screen.getByRole("button", { name: "Criar" }));
    await screen.findByLabelText("Código de pareamento");
    await person.keyboard("{Escape}");
    await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());
    await waitFor(() => expect(open).toHaveFocus());
  });
});
