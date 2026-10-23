import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { InviteForm } from "./invite-form";
import { LoginForm } from "./login-form";
import { ResetPasswordForm } from "./reset-password-form";
import { normalizeRecoveryCode, VerifyForm } from "./verify-form";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "en", time_zone: "America/Chicago", is_platform_admin: false };
const unauthenticated = () => HttpResponse.json({ error: { code: "auth.unauthenticated" } }, { status: 401 });

beforeEach(() => {
  navigation.replace.mockReset();
  navigation.push.mockReset();
  navigation.search = "";
  server.use(http.get("*/api/v1/me", unauthenticated));
});

describe("LoginForm", () => {
  it("signs in and lands on returnTo in the profile language", async () => {
    navigation.search = "returnTo=%2Fpt-BR%2Fconnections%2Fsources";
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/auth/login", async ({ request }) => { bodies.push(await request.json()); return HttpResponse.json({ state: "authenticated", user }); }));
    renderApp(<LoginForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("E-mail"), "trader@example.com");
    await person.type(screen.getByLabelText("Senha"), "correct horse battery");
    await person.click(screen.getByRole("checkbox", { name: "Manter conectado" }));
    await person.click(screen.getByRole("button", { name: "Entrar" }));
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/en/connections/sources"));
    expect(bodies).toEqual([{ email: "trader@example.com", password: "correct horse battery", remember_me: true }]);
  });

  it("ignores an external returnTo", async () => {
    navigation.search = "returnTo=%2F%2Fevil.com";
    server.use(http.post("*/api/v1/auth/login", () => HttpResponse.json({ state: "authenticated", user })));
    renderApp(<LoginForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("E-mail"), "trader@example.com");
    await person.type(screen.getByLabelText("Senha"), "x");
    await person.click(screen.getByRole("button", { name: "Entrar" }));
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/en"));
  });

  it("goes to the verification step when TOTP is on", async () => {
    server.use(http.post("*/api/v1/auth/login", () => HttpResponse.json({ state: "pending_2fa", pending_expires_at: "2026-10-23T12:05:00Z" })));
    renderApp(<LoginForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("E-mail"), "trader@example.com");
    await person.type(screen.getByLabelText("Senha"), "x");
    await person.click(screen.getByRole("button", { name: "Entrar" }));
    await waitFor(() => expect(navigation.push).toHaveBeenCalledWith("/pt-BR/login/verify"));
  });

  it("shows one message for bad credentials, clears the password and keeps the e-mail", async () => {
    server.use(http.post("*/api/v1/auth/login", () => HttpResponse.json({ error: { code: "auth.invalid_credentials" } }, { status: 401 })));
    renderApp(<LoginForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("E-mail"), "trader@example.com");
    await person.type(screen.getByLabelText("Senha"), "wrong");
    await person.click(screen.getByRole("button", { name: "Entrar" }));
    expect(await screen.findByText("E-mail ou senha incorretos.")).toBeInTheDocument();
    expect(screen.getByLabelText("Senha")).toHaveValue("");
    expect(screen.getByLabelText("E-mail")).toHaveValue("trader@example.com");
  });

  it("disables the button with a wait message when rate limited", async () => {
    server.use(http.post("*/api/v1/auth/login", () => HttpResponse.json({ error: { code: "auth.rate_limited", retry_after_s: 120 } }, { status: 429, headers: { "Retry-After": "120" } })));
    renderApp(<LoginForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("E-mail"), "trader@example.com");
    await person.type(screen.getByLabelText("Senha"), "x");
    await person.click(screen.getByRole("button", { name: "Entrar" }));
    expect(await screen.findByText(/Aguarde 2 min/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /min/ })).toBeDisabled();
  });

  it("explains why the session ended", async () => {
    navigation.search = "reason=sessions_ended";
    renderApp(<LoginForm />);
    expect(await screen.findByText(/encerrada por uma mudança de segurança/)).toBeInTheDocument();
  });

  it("redirects a signed-in visitor away from login", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(user)));
    renderApp(<LoginForm />);
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/en"));
  });
});

describe("VerifyForm", () => {
  it("submits the sixth digit by itself and lands in the app", async () => {
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/auth/login/totp", async ({ request }) => { bodies.push(await request.json()); return HttpResponse.json({ state: "authenticated", user }); }));
    renderApp(<VerifyForm />);
    await userEvent.setup().type(await screen.findByRole("textbox"), "123456");
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/en"));
    expect(bodies).toEqual([{ code: "123456" }]);
  });

  it("shows one message for a wrong code and clears the slots", async () => {
    server.use(http.post("*/api/v1/auth/login/totp", () => HttpResponse.json({ error: { code: "auth.invalid_code" } }, { status: 401 })));
    renderApp(<VerifyForm />);
    const input = await screen.findByRole("textbox");
    await userEvent.setup().type(input, "000000");
    expect(await screen.findByText("Código inválido. Tente de novo.")).toBeInTheDocument();
    expect(input).toHaveValue("");
  });

  it("goes back to login when the pending state expired", async () => {
    server.use(http.post("*/api/v1/auth/login/totp", () => HttpResponse.json({ error: { code: "auth.pending_expired" } }, { status: 401 })));
    renderApp(<VerifyForm />);
    await userEvent.setup().type(await screen.findByRole("textbox"), "123456");
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/pt-BR/login?reason=pending_expired"));
  });

  it("accepts a recovery code with or without hyphen, any case", async () => {
    expect(normalizeRecoveryCode(" ab12c-de34f ")).toBe("AB12CDE34F");
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/auth/login/recovery-code", async ({ request }) => { bodies.push(await request.json()); return HttpResponse.json({ state: "authenticated", user, recovery_codes_remaining: 2 }); }));
    renderApp(<VerifyForm />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Usar um código de recuperação" }));
    await person.type(screen.getByLabelText("Código de recuperação"), "ab12c-de34f");
    await person.click(screen.getByRole("button", { name: "Verificar" }));
    await waitFor(() => expect(navigation.replace).toHaveBeenCalled());
    expect(bodies).toEqual([{ code: "AB12CDE34F" }]);
    expect(sessionStorage.getItem("aw.recovery-remaining")).toBe("2");
    sessionStorage.clear();
  });
});

describe("InviteForm", () => {
  it("removes the fragment, sends the token in the body and accepts with locale and time zone", async () => {
    window.history.replaceState(null, "", "/pt-BR/invite#t=SECRET-TOKEN");
    const lookups: unknown[] = [];
    const accepts: unknown[] = [];
    server.use(
      http.post("*/api/v1/auth/invitations/lookup", async ({ request }) => { lookups.push(await request.json()); return HttpResponse.json({ email: "new@example.com", locale: "en", expires_at: "2026-10-30T12:00:00Z" }); }),
      http.post("*/api/v1/auth/invitations/accept", async ({ request }) => { accepts.push(await request.json()); return HttpResponse.json({ state: "authenticated", user }, { status: 201 }); }),
    );
    renderApp(<InviteForm />);
    expect(await screen.findByDisplayValue("new@example.com")).toBeInTheDocument();
    expect(window.location.hash).toBe("");
    expect(window.location.href).not.toContain("SECRET-TOKEN");
    expect(lookups).toEqual([{ token: "SECRET-TOKEN" }]);

    const person = userEvent.setup();
    await person.type(screen.getByLabelText("Senha"), "a long passphrase here");
    await person.click(screen.getByRole("button", { name: "Criar conta" }));
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/en"));
    expect(accepts).toEqual([{ token: "SECRET-TOKEN", password: "a long passphrase here", locale: "en", time_zone: expect.any(String) }]);
  });

  it("shows one invalid message without a token", async () => {
    window.history.replaceState(null, "", "/pt-BR/invite");
    renderApp(<InviteForm />);
    expect(await screen.findByText(/Este convite não é válido/)).toBeInTheDocument();
  });

  it("shows the same invalid message when the Core rejects the token", async () => {
    window.history.replaceState(null, "", "/pt-BR/invite#t=USED");
    server.use(http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ error: { code: "auth.invitation_invalid" } }, { status: 404 })));
    renderApp(<InviteForm />);
    expect(await screen.findByText(/Este convite não é válido/)).toBeInTheDocument();
  });

  it("keeps the submit disabled until the password has 12 characters", async () => {
    window.history.replaceState(null, "", "/pt-BR/invite#t=T");
    server.use(http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ email: "new@example.com", expires_at: "2026-10-30T12:00:00Z" })));
    renderApp(<InviteForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("Senha"), "short");
    expect(screen.getByRole("button", { name: "Criar conta" })).toBeDisabled();
  });
});

describe("ResetPasswordForm", () => {
  it("removes the fragment, posts the token in the body and shows the success page", async () => {
    window.history.replaceState(null, "", "/pt-BR/reset-password#t=RESET-TOKEN");
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/auth/password-reset/confirm", async ({ request }) => { bodies.push(await request.json()); return new HttpResponse(null, { status: 204 }); }));
    renderApp(<ResetPasswordForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("Nova senha"), "a long passphrase here");
    expect(window.location.href).not.toContain("RESET-TOKEN");
    await person.click(screen.getByRole("button", { name: "Redefinir senha" }));
    expect(await screen.findByText(/todas as suas sessões foram encerradas/)).toBeInTheDocument();
    expect(bodies).toEqual([{ token: "RESET-TOKEN", new_password: "a long passphrase here" }]);
  });

  it("shows the generic invalid message with a link to request a new one", async () => {
    window.history.replaceState(null, "", "/pt-BR/reset-password#t=OLD");
    server.use(http.post("*/api/v1/auth/password-reset/confirm", () => HttpResponse.json({ error: { code: "auth.reset_token_invalid" } }, { status: 404 })));
    renderApp(<ResetPasswordForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("Nova senha"), "a long passphrase here");
    await person.click(screen.getByRole("button", { name: "Redefinir senha" }));
    expect(await screen.findByText(/Este link de redefinição não é válido/)).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Pedir novo link" })).toHaveAttribute("href", "/pt-BR/forgot-password");
  });
});
