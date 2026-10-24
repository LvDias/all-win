// QA (Retina) — V1-S3 pairing step 2 in the "move to another machine" flow (Vitral M1 d1b, L3): the title turns
// "conectada" only with a `connected` reported after the new pairing, never from the old instance's cached row.
import { renderApp } from "@/test/render";
import { act, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { FakeEventSource } from "@/test/setup";
import { ConnectionsPage } from "@/features/connections";

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
  visible_accounts: [{ platform_account: "Sim101", kind: "simulation", oco_location: "local" }],
  ...overrides,
});
const pairing = { pairing_code: "K7QM-4D2P", expires_at: new Date(Date.now() + 9 * 60_000).toISOString() };
const list = (rows: unknown[]) => server.use(http.get("*/api/v1/connections", () => HttpResponse.json(rows)));

beforeEach(() => server.use(http.get("*/api/v1/me", () => HttpResponse.json(user))));

it("move flow: the old instance's cached 'connected' row does not make the new pairing look connected", async () => {
  list([connection()]);
  server.use(http.post("*/api/v1/connections/:id/pairing-code", () => HttpResponse.json(pairing, { status: 201 })));
  renderApp(<ConnectionsPage />);
  const person = userEvent.setup();
  await person.click(await screen.findByRole("button", { name: "Ações para VPS Chicago" }));
  await person.click(await screen.findByRole("menuitem", { name: "Mover para outra máquina" }));
  await person.click(within(await screen.findByRole("alertdialog")).getByRole("button", { name: "Gerar código" }));
  await screen.findByLabelText("Código de pareamento");
  const dialog = screen.getByRole("dialog");
  await waitFor(() => expect(FakeEventSource.latest()).toBeDefined());
  const events = FakeEventSource.latest()!;

  // After the new code is used the old instance is gone; the new one has not said hello yet.
  list([connection({ status: "disconnected", visible_accounts: [] })]);
  act(() => events.emit("connection.paired", { type: "connection.paired", at: "2026-10-23T12:00:00Z", connection_id: id, instance_name: "WIN-NEW", visible_accounts: [] }));
  expect(await within(dialog).findByRole("heading", { name: "“VPS Chicago” pareada" })).toBeInTheDocument();
  await waitFor(() => expect(within(dialog).getByRole("heading", { name: "“VPS Chicago” pareada" })).toBeInTheDocument());

  list([connection({ instance_name: "WIN-NEW" })]);
  act(() => events.emit("connection.status_changed", { type: "connection.status_changed", at: "2026-10-23T12:00:05Z", connection_id: id, status: "connected" }));
  expect(await within(dialog).findByRole("heading", { name: "“VPS Chicago” conectada" })).toBeInTheDocument();
  expect(within(dialog).getByText("“VPS Chicago” agora está em outra máquina.")).toBeInTheDocument();
});
