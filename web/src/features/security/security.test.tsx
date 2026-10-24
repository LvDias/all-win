import { navigation, renderApp } from "@/test/render";
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { SecurityPage } from "./security-page";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const session = { persistent: false, idle_timeout_s: 43200, absolute_timeout_s: 86400, idle_expires_at: "2099-01-01T00:00:00Z", absolute_expires_at: "2099-01-02T00:00:00Z" };
const security = (extra: object = {}) => ({ email: "trader@example.com", channel_change_allowed_at: "2020-01-01T00:00:00Z", session, ...extra });
const devices = [
  { id: "d1", label: "Chrome no Windows", first_seen_at: "2026-10-01T10:00:00Z", last_seen_at: "2026-10-23T10:00:00Z", current: true },
  { id: "d2", label: "Safari no iPhone", first_seen_at: "2026-10-05T10:00:00Z", last_seen_at: "2026-10-20T10:00:00Z", current: false },
];

function serve(state: object = security()) {
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(user)),
    http.get("*/api/v1/me/security", () => HttpResponse.json(state)),
    http.get("*/api/v1/me/devices", () => HttpResponse.json(devices)),
    http.get("*/api/v1/sources", () => HttpResponse.json([{ status: "active" }, { status: "revoked" }])),
    http.get("*/api/v1/connections", () => HttpResponse.json([{ status: "connected" }, { status: "awaiting_pairing" }])),
  );
}

beforeEach(() => {
  navigation.replace.mockReset();
  navigation.search = "";
});

describe("SecurityPage", () => {
  it("shows the sign-in e-mail, session limits and no password or TOTP controls", async () => {
    serve();
    renderApp(<SecurityPage />);
    expect(await screen.findByText("trader@example.com")).toBeInTheDocument();
    expect(screen.getByText(/Sua sessão termina após 12 horas sem uso ou 1 dia depois de entrar/)).toBeInTheDocument();
    expect(screen.queryByText(/senha/i)).not.toBeInTheDocument();
    expect(screen.queryByRole("heading", { name: /duas etapas/i })).not.toBeInTheDocument();
  });

  it("disables the e-mail change during the 24 h freeze and says when it ends", async () => {
    serve(security({ channel_change_allowed_at: "2099-01-01T12:00:00Z" }));
    renderApp(<SecurityPage />);
    expect(await screen.findByRole("button", { name: "Alterar e-mail" })).toBeDisabled();
    expect(screen.getByText(/Uma nova troca fica disponível em/)).toBeInTheDocument();
  });

  it("lists devices, forgets another device after confirmation and cannot forget the current one", async () => {
    serve();
    const forgotten = vi.fn();
    server.use(http.post("*/api/v1/me/devices/:id/forget", ({ params }) => { forgotten(params.id); return new HttpResponse(null, { status: 204 }); }));
    renderApp(<SecurityPage />);
    expect(await screen.findByText("Chrome no Windows")).toBeInTheDocument();
    expect(screen.getByText("Este dispositivo")).toBeInTheDocument();
    expect(screen.getAllByRole("button", { name: /Esquecer dispositivo/ })).toHaveLength(1);
    const person = userEvent.setup();
    await person.click(screen.getByRole("button", { name: "Esquecer dispositivo Safari no iPhone" }));
    await person.click(within(await screen.findByRole("alertdialog")).getByRole("button", { name: "Esquecer dispositivo" }));
    await waitFor(() => expect(forgotten).toHaveBeenCalledWith("d2"));
  });

  it("signs out everywhere after a warning that sources and connections stay active", async () => {
    serve();
    let called = 0;
    server.use(http.post("*/api/v1/auth/logout-everywhere", () => { called += 1; return new HttpResponse(null, { status: 204 }); }));
    renderApp(<SecurityPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Sair de todos os dispositivos" }));
    const dialog = await screen.findByRole("alertdialog");
    expect(within(dialog).getByText(/não revoga as URLs de alertas nem as conexões do NinjaTrader/)).toBeInTheDocument();
    expect(await within(dialog).findByText(/1 fonte ativa/)).toBeInTheDocument();
    expect(within(dialog).getByText(/1 conexão pareada/)).toBeInTheDocument();
    await person.click(within(dialog).getByRole("button", { name: "Sair de todos" }));
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/pt-BR/login?reason=sessions_ended"));
    expect(called).toBe(1);
  });

  it("changes the e-mail with a code on the current address and one on the new address", async () => {
    serve();
    const verified: unknown[] = [];
    let open = true;
    const change = (confirmed: string[]) => ({
      id: "change-1",
      channel: "email",
      status: confirmed.length === 2 ? "completed" : "awaiting_codes",
      masked_new_value: "n***@e***.com",
      expires_at: "2099-01-01T00:00:00Z",
      codes: [
        { role: "current_value", challenge_id: "old-1", confirmed: confirmed.includes("old-1") },
        { role: "new_value", challenge_id: "new-1", confirmed: confirmed.includes("new-1") },
      ],
    });
    server.use(
      http.post("*/api/v1/me/channel-changes", () => HttpResponse.json(change([]), { status: 202 })),
      http.post("*/api/v1/auth/challenges/delivery", () => HttpResponse.json({ delivery: "sent" })),
      http.post("*/api/v1/me/channel-changes/:id/verify", async ({ request }) => {
        const body = (await request.json()) as { challenge_id: string };
        verified.push(body);
        open = body.challenge_id !== "new-1";
        return HttpResponse.json(change(body.challenge_id === "old-1" ? ["old-1"] : ["old-1", "new-1"]));
      }),
    );
    renderApp(<SecurityPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Alterar e-mail" }));
    await person.type(await screen.findByLabelText("Novo e-mail"), "new@example.com");
    await person.click(screen.getByRole("button", { name: "Enviar códigos" }));
    expect(await screen.findByText(/Digite o código enviado para trader@example.com/)).toBeInTheDocument();
    await person.type(screen.getByLabelText(/Código de verificação/), "AAAA1111");
    expect(await screen.findByText(/Digite o código enviado para n\*\*\*@e\*\*\*\.com/)).toBeInTheDocument();
    await person.type(screen.getByLabelText(/Código de verificação/), "BBBB2222");
    expect(await screen.findByText(/E-mail alterado para n\*\*\*@e\*\*\*\.com/)).toBeInTheDocument();
    expect(verified).toEqual([
      { challenge_id: "old-1", code: "AAAA1111" },
      { challenge_id: "new-1", code: "BBBB2222" },
    ]);
    expect(open).toBe(false);
  });

  it("shows the pending-change banner for a held change and cancels it from any session", async () => {
    serve(security({ pending_channel_change: { id: "change-9", channel: "email", status: "on_hold", masked_new_value: "n***@e***.com", expires_at: "2099-01-01T00:00:00Z", hold_until: "2099-01-04T00:00:00Z", codes: [] } }));
    const cancelled = vi.fn();
    server.use(http.post("*/api/v1/me/channel-changes/:id/cancel", ({ params }) => { cancelled(params.id); return new HttpResponse(null, { status: 204 }); }));
    renderApp(<SecurityPage />);
    const person = userEvent.setup();
    expect(await screen.findByText(/Troca de e-mail pendente: n\*\*\*@e\*\*\*\.com/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Alterar e-mail" })).toBeDisabled();
    await person.click(screen.getAllByRole("button", { name: "Cancelar troca" })[0] as HTMLElement);
    await person.click(within(await screen.findByRole("alertdialog")).getByRole("button", { name: "Cancelar troca" }));
    await waitFor(() => expect(cancelled).toHaveBeenCalledWith("change-9"));
  });

  it("shows the generic error state with a retry when the page cannot load", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(user)), http.get("*/api/v1/me/security", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })), http.get("*/api/v1/me/devices", () => HttpResponse.json(devices)));
    renderApp(<SecurityPage />);
    expect(await screen.findByRole("button", { name: "Tentar novamente" })).toBeInTheDocument();
  });
});
