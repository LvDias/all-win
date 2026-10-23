import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
import { recoveryFileText } from "./recovery-codes-view";
import { SecurityPage } from "./security-page";
import { TotpBanner } from "./totp-banner";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const session = { persistent: false, idle_timeout_s: 43200, absolute_timeout_s: 86400, idle_expires_at: "2026-10-24T00:00:00Z", absolute_expires_at: "2026-10-24T12:00:00Z", step_up_valid_until: "2026-10-23T12:15:00Z" };
const codes = ["ABCDE-FGHJK", "MNPQR-STVWX", "01234-56789", "ABCDE-12345", "FGHJK-12345", "MNPQR-12345", "STVWX-12345", "ABCDE-67890", "FGHJK-67890", "MNPQR-67890"];

function security(totp: boolean, remaining = 10) {
  server.use(http.get("*/api/v1/me/security", () => HttpResponse.json({ totp_enabled: totp, recovery_codes_remaining: remaining, session })));
}

beforeEach(() => {
  localStorage.clear();
  server.use(http.get("*/api/v1/me", () => HttpResponse.json(user)));
});

describe("SecurityPage", () => {
  it("shows the status, remaining codes and the session limits from the Core", async () => {
    security(true, 3);
    renderApp(<SecurityPage />);
    expect(await screen.findByText("Ativada")).toBeInTheDocument();
    expect(screen.getByText("Códigos de recuperação restantes: 3 de 10")).toHaveClass("text-status-warning");
    expect(screen.getByText(/12 horas sem uso ou 1 dia depois de entrar/)).toBeInTheDocument();
  });

  it("enables TOTP in three steps and shows the recovery codes once", async () => {
    security(false);
    server.use(
      http.post("*/api/v1/me/totp/setup", () => HttpResponse.json({ otpauth_uri: "otpauth://totp/All%20Win:a@b.co?secret=JBSWY3DPEHPK3PXP&issuer=All%20Win", secret: "JBSWY3DPEHPK3PXP", expires_at: "2026-10-23T12:10:00Z" })),
      http.post("*/api/v1/me/totp/confirm", async ({ request }) => {
        expect(await request.json()).toEqual({ code: "123456" });
        return HttpResponse.json({ recovery_codes: codes });
      }),
    );
    renderApp(<SecurityPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ativar" }));

    const dialog = await screen.findByRole("dialog");
    expect(await within(dialog).findByRole("img", { name: "QR code para o app autenticador" })).toBeInTheDocument();
    expect(within(dialog).getByText("Etapa 1 de 3")).toBeInTheDocument();
    expect(dialog.innerHTML).not.toContain("JBSWY3DPEHPK3PXP");
    await person.click(within(dialog).getByRole("button", { name: "Continuar" }));

    expect(within(dialog).getByText("Etapa 2 de 3")).toBeInTheDocument();
    await person.type(within(dialog).getByRole("textbox"), "123456");
    expect(await within(dialog).findByText("Etapa 3 de 3")).toBeInTheDocument();
    expect(within(dialog).getByText("ABCDE-FGHJK")).toBeInTheDocument();
    expect(within(dialog).getByRole("button", { name: "Concluir" })).toBeDisabled();
  });

  it("asks before closing the codes without confirming they were saved", async () => {
    security(false);
    server.use(
      http.post("*/api/v1/me/totp/setup", () => HttpResponse.json({ otpauth_uri: "otpauth://totp/x?secret=AAAA", secret: "AAAA", expires_at: "2026-10-23T12:10:00Z" })),
      http.post("*/api/v1/me/totp/confirm", () => HttpResponse.json({ recovery_codes: codes })),
    );
    renderApp(<SecurityPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ativar" }));
    const dialog = await screen.findByRole("dialog");
    await person.click(await within(dialog).findByRole("button", { name: "Continuar" }));
    await person.type(within(dialog).getByRole("textbox"), "123456");
    await within(dialog).findByText("ABCDE-FGHJK");

    await person.click(within(dialog).getByRole("button", { name: "Fechar" }));
    expect(await screen.findByText("Você ainda não confirmou que guardou os códigos. Fechar mesmo assim?")).toBeInTheDocument();
    await person.click(screen.getByRole("button", { name: "Voltar" }));
    await person.click(within(dialog).getByRole("checkbox", { name: "Guardei meus códigos" }));
    await person.click(within(dialog).getByRole("button", { name: "Concluir" }));
    await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());
  });

  it("offers a restart when the setup expired", async () => {
    security(false);
    server.use(
      http.post("*/api/v1/me/totp/setup", () => HttpResponse.json({ otpauth_uri: "otpauth://totp/x?secret=AAAA", secret: "AAAA", expires_at: "2026-10-23T12:10:00Z" })),
      http.post("*/api/v1/me/totp/confirm", () => HttpResponse.json({ error: { code: "auth.totp_setup_expired" } }, { status: 422 })),
    );
    renderApp(<SecurityPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Ativar" }));
    const dialog = await screen.findByRole("dialog");
    await person.click(await within(dialog).findByRole("button", { name: "Continuar" }));
    await person.type(within(dialog).getByRole("textbox"), "123456");
    expect(await within(dialog).findByText("A configuração expirou. Comece de novo.")).toBeInTheDocument();
    expect(within(dialog).getByRole("button", { name: "Recomeçar" })).toBeInTheDocument();
  });

  it("disables TOTP with the password and a current code", async () => {
    security(true);
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/me/totp/disable", async ({ request }) => { bodies.push(await request.json()); return new HttpResponse(null, { status: 204 }); }));
    renderApp(<SecurityPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Desativar" }));
    const dialog = await screen.findByRole("alertdialog");
    await person.type(within(dialog).getByLabelText("Senha"), "my password");
    await person.type(within(dialog).getByRole("textbox"), "654321");
    await person.click(within(dialog).getByRole("button", { name: "Desativar" }));
    await waitFor(() => expect(bodies).toEqual([{ password: "my password", code: "654321" }]));
  });

  it("generates new recovery codes after re-authentication", async () => {
    security(true);
    server.use(http.post("*/api/v1/me/recovery-codes/regenerate", () => HttpResponse.json({ recovery_codes: codes })));
    renderApp(<SecurityPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Gerar novos códigos" }));
    const dialog = await screen.findByRole("dialog");
    await person.type(within(dialog).getByLabelText("Senha"), "pw");
    await person.type(within(dialog).getByRole("textbox"), "111111");
    await person.click(within(dialog).getByRole("button", { name: "Gerar códigos" }));
    expect(await within(dialog).findByText("MNPQR-STVWX")).toBeInTheDocument();
    expect(within(dialog).getByText(/Os códigos anteriores deixaram de funcionar/)).toBeInTheDocument();
  });

  it("changes the password and ends other sessions (step-up is retried once)", async () => {
    security(false);
    let attempts = 0;
    server.use(
      http.post("*/api/v1/me/password", () => (++attempts === 1 ? HttpResponse.json({ error: { code: "auth.step_up_required" } }, { status: 403 }) : new HttpResponse(null, { status: 204 }))),
      http.post("*/api/v1/auth/step-up", () => new HttpResponse(null, { status: 204 })),
    );
    renderApp(<SecurityPage />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Alterar senha" }));
    const dialog = await screen.findByRole("dialog");
    await person.type(within(dialog).getByLabelText("Senha atual"), "old password");
    await person.type(within(dialog).getByLabelText("Nova senha"), "a brand new passphrase");
    await person.click(within(dialog).getByRole("button", { name: "Alterar senha" }));
    const stepUp = await screen.findByText("Confirme que é você");
    await person.type(within(stepUp.closest("[role=dialog]") as HTMLElement).getByLabelText("Senha"), "old password");
    await person.click(within(stepUp.closest("[role=dialog]") as HTMLElement).getByRole("button", { name: "Confirmar" }));
    await waitFor(() => expect(attempts).toBe(2));
    expect(await screen.findByText(/Senha alterada/)).toBeInTheDocument();
  });
});

describe("TotpBanner", () => {
  it("shows while TOTP is off and stays dismissed on this device", async () => {
    security(false);
    const { unmount } = renderApp(<TotpBanner />);
    expect(await screen.findByText("Proteja sua conta com a verificação em duas etapas.")).toBeInTheDocument();
    await userEvent.setup().click(screen.getByRole("button", { name: "Dispensar aviso" }));
    expect(screen.queryByText("Proteja sua conta com a verificação em duas etapas.")).not.toBeInTheDocument();
    unmount();
    renderApp(<TotpBanner />);
    await waitFor(() => expect(screen.queryByText("Proteja sua conta com a verificação em duas etapas.")).not.toBeInTheDocument());
  });

  it("is hidden when TOTP is on", async () => {
    security(true);
    renderApp(<TotpBanner />);
    await waitFor(() => expect(screen.queryByRole("status")).not.toBeInTheDocument());
  });
});

describe("recovery file", () => {
  it("has one code per line and the generation date", () => {
    expect(recoveryFileText(["AAAAA-BBBBB", "CCCCC-DDDDD"], new Date("2026-10-23T12:00:00Z"))).toBe("AAAAA-BBBBB\nCCCCC-DDDDD\n\n2026-10-23T12:00:00.000Z\n");
  });
});
