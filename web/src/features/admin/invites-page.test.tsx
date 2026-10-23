import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
import { InvitesPage } from "./invites-page";

const admin = { id: "00000000-0000-4000-8000-000000000001", email: "admin@example.com", locale: "pt-BR", time_zone: "America/Sao_Paulo", is_platform_admin: true };
const invitation = { id: "00000000-0000-4000-8000-0000000000a1", email: "new@example.com", status: "pending", created_at: "2026-10-23T12:00:00Z", expires_at: "2026-10-30T12:00:00Z" };
const inviteUrl = "https://allwin.example/pt-BR/invite#t=SECRET-TOKEN-VALUE";

function signIn(overrides: Partial<typeof admin> = {}) {
  server.use(http.get("*/api/v1/me", () => HttpResponse.json({ ...admin, ...overrides })));
}

describe("InvitesPage", () => {
  it("shows the generic not-found screen to a non-admin and never calls the admin API", async () => {
    signIn({ is_platform_admin: false });
    const listed = vi.fn();
    server.use(http.get("*/api/v1/admin/invitations", () => { listed(); return HttpResponse.json([]); }));
    renderApp(<InvitesPage />);
    expect(await screen.findByText("Não encontrado.")).toBeInTheDocument();
    expect(screen.getByText(/common\.not_found/)).toBeInTheDocument();
    expect(listed).not.toHaveBeenCalled();
  });

  it("treats auth.forbidden from the API as not found too", async () => {
    signIn();
    server.use(http.get("*/api/v1/admin/invitations", () => HttpResponse.json({ error: { code: "auth.forbidden" } }, { status: 403 })));
    renderApp(<InvitesPage />);
    expect(await screen.findByText("Não encontrado.")).toBeInTheDocument();
  });

  it("lists invitations with their status and the empty state", async () => {
    signIn();
    server.use(http.get("*/api/v1/admin/invitations", () => HttpResponse.json([invitation])));
    renderApp(<InvitesPage />);
    expect((await screen.findAllByText("new@example.com")).length).toBeGreaterThan(0);
    expect(screen.getAllByText("Pendente").length).toBeGreaterThan(0);
  });

  it("shows the empty state", async () => {
    signIn();
    server.use(http.get("*/api/v1/admin/invitations", () => HttpResponse.json([])));
    renderApp(<InvitesPage />);
    expect(await screen.findByText("Nenhum convite ainda")).toBeInTheDocument();
  });

  it("creates an invitation and shows the link once, masked until revealed", async () => {
    signIn();
    server.use(http.get("*/api/v1/admin/invitations", () => HttpResponse.json([])));
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/admin/invitations", async ({ request }) => {
      bodies.push(await request.json());
      return HttpResponse.json({ invitation, invite_url: inviteUrl, email_sent: true }, { status: 201 });
    }));
    const user = userEvent.setup();
    renderApp(<InvitesPage />);
    await user.click((await screen.findAllByRole("button", { name: "Convidar" }))[0] as HTMLElement);
    await user.type(await screen.findByLabelText("E-mail"), "new@example.com");
    await user.click(screen.getByRole("button", { name: "Criar convite" }));
    expect(await screen.findByText("Enviamos o convite para new@example.com.")).toBeInTheDocument();
    expect(bodies).toEqual([{ email: "new@example.com", locale: "pt-BR", send_email: true }]);
    const dialog = screen.getByRole("dialog");
    expect(within(dialog).queryByText(/SECRET-TOKEN-VALUE/)).not.toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Revelar" }));
    expect(within(dialog).getByText(inviteUrl)).toBeInTheDocument();
  });

  it("asks before closing when the e-mail failed and the link was not copied", async () => {
    signIn();
    server.use(http.get("*/api/v1/admin/invitations", () => HttpResponse.json([])));
    server.use(http.post("*/api/v1/admin/invitations", () => HttpResponse.json({ invitation, invite_url: inviteUrl, email_sent: false }, { status: 201 })));
    const user = userEvent.setup();
    renderApp(<InvitesPage />);
    await user.click((await screen.findAllByRole("button", { name: "Convidar" }))[0] as HTMLElement);
    await user.type(await screen.findByLabelText("E-mail"), "new@example.com");
    await user.click(screen.getByRole("button", { name: "Criar convite" }));
    await screen.findByText(/Não foi possível enviar o e-mail/);
    await user.click(screen.getByRole("button", { name: "Concluir" }));
    expect(await screen.findByText("Fechar sem copiar o link?")).toBeInTheDocument();
  });

  it("shows the translated error when the e-mail is taken", async () => {
    signIn();
    server.use(http.get("*/api/v1/admin/invitations", () => HttpResponse.json([])));
    server.use(http.post("*/api/v1/admin/invitations", () => HttpResponse.json({ error: { code: "invites.email_taken" } }, { status: 409 })));
    const user = userEvent.setup();
    renderApp(<InvitesPage />);
    await user.click((await screen.findAllByRole("button", { name: "Convidar" }))[0] as HTMLElement);
    await user.type(await screen.findByLabelText("E-mail"), "new@example.com");
    await user.click(screen.getByRole("button", { name: "Criar convite" }));
    expect(await screen.findByText("Já existe uma conta ou convite para este e-mail.")).toBeInTheDocument();
  });

  it("revokes a pending invitation after confirmation", async () => {
    signIn();
    server.use(http.get("*/api/v1/admin/invitations", () => HttpResponse.json([invitation])));
    const revoked = vi.fn();
    server.use(http.post("*/api/v1/admin/invitations/:id/revoke", ({ params }) => { revoked(params.id); return new HttpResponse(null, { status: 204 }); }));
    const user = userEvent.setup();
    renderApp(<InvitesPage />);
    await user.click((await screen.findAllByRole("button", { name: "Revogar o convite de new@example.com" }))[0] as HTMLElement);
    await user.click(await screen.findByRole("button", { name: "Revogar" }));
    await waitFor(() => expect(revoked).toHaveBeenCalledWith(invitation.id));
  });
});
