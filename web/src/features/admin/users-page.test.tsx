import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
import { UsersPage } from "./users-page";

const admin = { id: "00000000-0000-4000-8000-000000000001", email: "admin@example.com", locale: "pt-BR", time_zone: "America/Sao_Paulo", is_platform_admin: true };
const other = { id: "00000000-0000-4000-8000-000000000002", email: "ana@example.com", status: "active", totp_enabled: true, created_at: "2026-10-01T12:00:00Z" };
const self = { id: admin.id, email: admin.email, status: "active", totp_enabled: false, is_platform_admin: true, created_at: "2026-09-01T12:00:00Z" };
const resetUrl = "https://allwin.example/pt-BR/reset-password#t=RESET-SECRET-VALUE";

function setup() {
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(admin)),
    http.get("*/api/v1/admin/users", () => HttpResponse.json([self, other])),
  );
}

describe("UsersPage", () => {
  it("shows the generic not-found screen to a non-admin", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json({ ...admin, is_platform_admin: false })));
    renderApp(<UsersPage />);
    expect(await screen.findByText("Não encontrado.")).toBeInTheDocument();
  });

  it("hides the actions on the admin's own row and filters by e-mail", async () => {
    setup();
    const user = userEvent.setup();
    renderApp(<UsersPage />);
    await screen.findAllByText("ana@example.com");
    expect(screen.queryByRole("button", { name: "Ações para admin@example.com" })).not.toBeInTheDocument();
    expect(screen.getAllByRole("button", { name: "Ações para ana@example.com" }).length).toBeGreaterThan(0);
    await user.type(screen.getByLabelText("Buscar por e-mail"), "zzz");
    expect(await screen.findByText("Nenhum usuário encontrado.")).toBeInTheDocument();
  });

  it("generates a reset link behind step-up, retrying the action once, and shows it once", async () => {
    setup();
    let attempts = 0;
    server.use(
      http.post("*/api/v1/admin/users/:id/password-reset-link", () => {
        attempts += 1;
        if (attempts === 1) return HttpResponse.json({ error: { code: "auth.step_up_required" } }, { status: 403 });
        return HttpResponse.json({ reset_url: resetUrl, expires_at: "2026-10-23T13:00:00Z" }, { status: 201 });
      }),
      http.get("*/api/v1/me/security", () => HttpResponse.json({ totp_enabled: false, recovery_codes_remaining: 0, session: {} })),
      http.post("*/api/v1/auth/step-up", () => new HttpResponse(null, { status: 204 })),
    );
    const user = userEvent.setup();
    renderApp(<UsersPage />);
    await user.click((await screen.findAllByRole("button", { name: "Ações para ana@example.com" }))[0] as HTMLElement);
    await user.click(await screen.findByRole("menuitem", { name: "Gerar link de redefinição de senha" }));
    await user.click(await screen.findByRole("button", { name: "Gerar link" }));
    await user.type(await screen.findByLabelText("Senha"), "correct horse battery");
    await user.click(screen.getByRole("button", { name: "Confirmar" }));
    expect(await screen.findByText("Link de redefinição")).toBeInTheDocument();
    expect(attempts).toBe(2);
    const dialog = screen.getByRole("dialog");
    expect(within(dialog).queryByText(/RESET-SECRET-VALUE/)).not.toBeInTheDocument();
    await user.click(within(dialog).getByRole("button", { name: "Revelar" }));
    expect(within(dialog).getByText(resetUrl)).toBeInTheDocument();
  });

  it("resets TOTP only after the identity checkbox is ticked", async () => {
    setup();
    const reset = vi.fn();
    server.use(http.post("*/api/v1/admin/users/:id/totp-reset", ({ params }) => { reset(params.id); return new HttpResponse(null, { status: 204 }); }));
    const user = userEvent.setup();
    renderApp(<UsersPage />);
    await user.click((await screen.findAllByRole("button", { name: "Ações para ana@example.com" }))[0] as HTMLElement);
    await user.click(await screen.findByRole("menuitem", { name: "Desativar verificação em duas etapas" }));
    const confirm = await screen.findByRole("button", { name: "Desativar verificação" });
    expect(confirm).toBeDisabled();
    await user.click(screen.getByRole("checkbox", { name: "Confirmei a identidade de ana@example.com" }));
    await user.click(confirm);
    await waitFor(() => expect(reset).toHaveBeenCalledWith(other.id));
  });
});
