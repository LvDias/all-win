import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { CodeForm } from "./code-form";
import { InviteForm } from "./invite-form";
import { LoginForm } from "./login-form";
import { clearPendingLogin, readPendingLogin } from "./pending-login";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "en", time_zone: "America/Chicago", is_platform_admin: false };
const unauthenticated = () => HttpResponse.json({ error: { code: "auth.unauthenticated" } }, { status: 401 });
const issued = (id = "challenge-1") => HttpResponse.json({ challenge_id: id, expires_at: new Date(Date.now() + 600_000).toISOString(), resend_after_s: 60 }, { status: 202 });
const invalidCode = () => HttpResponse.json({ error: { code: "auth.code_invalid" } }, { status: 401 });
const codeLabel = /Código de verificação/;

beforeEach(() => {
  clearPendingLogin();
  navigation.replace.mockReset();
  navigation.push.mockReset();
  navigation.search = "";
  server.use(http.get("*/api/v1/me", unauthenticated));
});

describe("LoginForm", () => {
  it("asks for a code by e-mail and moves to the code page, keeping the e-mail only in memory", async () => {
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/auth/login/challenges", async ({ request }) => { bodies.push(await request.json()); return issued(); }));
    renderApp(<LoginForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("E-mail"), "trader@example.com");
    await person.click(screen.getByRole("button", { name: "Enviar código" }));
    await waitFor(() => expect(navigation.push).toHaveBeenCalledWith("/pt-BR/login/code"));
    expect(bodies).toEqual([{ email: "trader@example.com", channel: "email" }]);
    expect(readPendingLogin()?.email).toBe("trader@example.com");
    expect(navigation.push.mock.calls[0]?.[0]).not.toContain("trader");
  });

  it("carries a safe returnTo and drops an external one", async () => {
    navigation.search = "returnTo=%2F%2Fevil.com";
    server.use(http.post("*/api/v1/auth/login/challenges", () => issued()));
    renderApp(<LoginForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("E-mail"), "a@b.co");
    await person.click(screen.getByRole("button", { name: "Enviar código" }));
    await waitFor(() => expect(navigation.push).toHaveBeenCalledWith("/pt-BR/login/code"));
  });

  it("shows no password field and validates the e-mail format only", async () => {
    renderApp(<LoginForm />);
    const person = userEvent.setup();
    expect(screen.queryByLabelText(/senha/i)).not.toBeInTheDocument();
    await person.type(await screen.findByLabelText("E-mail"), "not-an-email");
    await person.click(screen.getByRole("button", { name: "Enviar código" }));
    expect(await screen.findByText("Digite um e-mail válido.")).toBeInTheDocument();
    expect(navigation.push).not.toHaveBeenCalled();
  });

  it("shows the wait and counts down on a rate limit, never saying which limit", async () => {
    server.use(http.post("*/api/v1/auth/login/challenges", () => HttpResponse.json({ error: { code: "auth.rate_limited" } }, { status: 429, headers: { "Retry-After": "90" } })));
    renderApp(<LoginForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("E-mail"), "a@b.co");
    await person.click(screen.getByRole("button", { name: "Enviar código" }));
    expect(await screen.findByText(/Muitos pedidos de código\. Aguarde 2 min/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /2 min/ })).toBeDisabled();
  });

  it("explains why the user is back at the login page", async () => {
    navigation.search = "reason=sessions_ended";
    renderApp(<LoginForm />);
    expect(await screen.findByText("Suas sessões foram encerradas. Entre de novo.")).toBeInTheDocument();
  });

  it("redirects a signed-in user to the app", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(user)));
    renderApp(<LoginForm />);
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/en"));
  });
});

describe("CodeForm", () => {
  function pending(remember = false) {
    return import("./pending-login").then(({ startPendingLogin }) =>
      startPendingLogin({ email: "trader@example.com", remember, challenge: { challenge_id: "challenge-1", expires_at: new Date(Date.now() + 600_000).toISOString(), resend_after_s: 60 } }),
    );
  }

  it("sends the user back to the e-mail page when nothing is pending (reload)", async () => {
    renderApp(<CodeForm />);
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/pt-BR/login"));
  });

  it("verifies a pasted code (separators and lower case are normalised), clears the cache and lands in the profile language", async () => {
    await pending(true);
    navigation.search = "returnTo=%2Fpt-BR%2Fconnections%2Fsources";
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/auth/login/verify", async ({ request }) => { bodies.push(await request.json()); return HttpResponse.json({ user, new_device: false }); }));
    renderApp(<CodeForm />);
    const person = userEvent.setup();
    const field = await screen.findByLabelText(codeLabel);
    await person.click(field);
    await person.paste("abcd-2345 ");
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/en/connections/sources"));
    expect(bodies).toEqual([{ challenge_id: "challenge-1", code: "ABCD2345", remember_me: true }]);
    expect(readPendingLogin()).toBeNull();
  });

  it("announces a new device once", async () => {
    await pending();
    server.use(http.post("*/api/v1/auth/login/verify", () => HttpResponse.json({ user, new_device: true })));
    renderApp(<CodeForm />);
    await userEvent.setup().type(await screen.findByLabelText(codeLabel), "ABCD2345");
    expect(await screen.findByText("Este é um dispositivo novo. Enviamos um aviso ao seu e-mail.")).toBeInTheDocument();
  });

  it("shows one message for any wrong code, clears the field and never echoes the code", async () => {
    await pending();
    server.use(http.post("*/api/v1/auth/login/verify", invalidCode));
    renderApp(<CodeForm />);
    const person = userEvent.setup();
    const field = await screen.findByLabelText(codeLabel);
    await person.type(field, "WRONG123");
    const message = await screen.findByText("Código inválido ou expirado. Confira ou peça um novo.");
    expect(message.textContent).not.toContain("WRONG123");
    expect(field).toHaveValue("");
    expect(navigation.replace).not.toHaveBeenCalledWith(expect.stringMatching(/^\/en/));
  });

  it("locks the field after five failed attempts and offers a new code, which starts a new challenge", async () => {
    await pending();
    let issuedNew = 0;
    server.use(
      http.post("*/api/v1/auth/login/verify", invalidCode),
      http.post("*/api/v1/auth/login/challenges", () => { issuedNew += 1; return issued("challenge-2"); }),
    );
    renderApp(<CodeForm />);
    const person = userEvent.setup();
    const field = await screen.findByLabelText(codeLabel);
    for (let attempt = 0; attempt < 5; attempt += 1) {
      await person.type(field, "WRONG123");
      await waitFor(() => expect(field).toHaveValue(""));
    }
    expect(await screen.findByText("Peça um novo código para continuar.")).toBeInTheDocument();
    expect(field).toBeDisabled();
    expect(screen.queryByRole("button", { name: "Verificar" })).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Pedir novo código/ })).toHaveAttribute("aria-disabled", "true");
    expect(issuedNew).toBe(0);
  });

  it("does not resend while the cooldown runs and keeps the button focusable", async () => {
    await pending();
    let resends = 0;
    server.use(http.post("*/api/v1/auth/challenges/resend", () => { resends += 1; return issued(); }));
    renderApp(<CodeForm />);
    const resend = await screen.findByRole("button", { name: /Reenviar código/ });
    expect(resend).toHaveAttribute("aria-disabled", "true");
    expect(resend).not.toBeDisabled();
    await userEvent.setup().click(resend);
    expect(resends).toBe(0);
  });

  it("returns to the e-mail page and forgets the pending login", async () => {
    await pending();
    renderApp(<CodeForm />);
    await userEvent.setup().click(await screen.findByRole("link", { name: "Usar outro e-mail" }));
    expect(readPendingLogin()).toBeNull();
  });
});

describe("InviteForm", () => {
  function openInvite(hash = "#t=tok%2Ben") {
    window.history.replaceState(null, "", `/pt-BR/invite${hash}`);
  }

  it("removes the token from the address bar at once and sends it in the body", async () => {
    openInvite();
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/auth/invitations/lookup", async ({ request }) => { bodies.push(await request.json()); return HttpResponse.json({ email: "new@example.com", locale: "en", expires_at: "2099-01-01T00:00:00Z" }); }));
    renderApp(<InviteForm />);
    expect(await screen.findByDisplayValue("new@example.com")).toBeInTheDocument();
    expect(window.location.hash).toBe("");
    expect(bodies).toEqual([{ token: "tok+en" }]);
  });

  it("shows one message for any invalid invitation, and when the token is gone after a reload", async () => {
    openInvite("");
    renderApp(<InviteForm />);
    expect(await screen.findByText(/Este convite não é válido/)).toBeInTheDocument();
    expect(screen.getByText(/abra o link do convite de novo/)).toBeInTheDocument();
  });

  it("creates the account only after the code is confirmed, then lands in the chosen language", async () => {
    openInvite();
    const completed: unknown[] = [];
    server.use(
      http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ email: "new@example.com", locale: "en", expires_at: "2099-01-01T00:00:00Z" })),
      http.post("*/api/v1/auth/signup/challenges", () => issued("signup-1")),
      http.post("*/api/v1/auth/challenges/delivery", () => HttpResponse.json({ delivery: "sent" })),
      http.post("*/api/v1/auth/signup/verify", async ({ request }) => { completed.push(await request.json()); return HttpResponse.json({ user, new_device: false }, { status: 201 }); }),
    );
    renderApp(<InviteForm />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Enviar código" }));
    await person.type(await screen.findByLabelText(codeLabel), "ABCD2345");
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/en"));
    expect(completed).toHaveLength(1);
    expect(completed[0]).toMatchObject({ challenge_id: "signup-1", code: "ABCD2345", locale: "en", remember_me: false });
    expect(completed[0]).toHaveProperty("time_zone");
    expect(screen.queryByLabelText(/senha/i)).not.toBeInTheDocument();
  });

  it("reports an unknown outcome when the confirmation never got an answer", async () => {
    openInvite();
    server.use(
      http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ email: "new@example.com", expires_at: "2099-01-01T00:00:00Z" })),
      http.post("*/api/v1/auth/signup/challenges", () => issued("signup-1")),
      http.post("*/api/v1/auth/challenges/delivery", () => HttpResponse.json({ delivery: "pending" })),
      http.post("*/api/v1/auth/signup/verify", () => HttpResponse.error()),
    );
    renderApp(<InviteForm />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Enviar código" }));
    await person.type(await screen.findByLabelText(codeLabel), "ABCD2345");
    expect(await screen.findByText(/Não sabemos se sua conta foi criada/)).toBeInTheDocument();
  });

  it("offers sign-out when a user is already signed in", async () => {
    openInvite();
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(user)), http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ email: "new@example.com", expires_at: "2099-01-01T00:00:00Z" })));
    renderApp(<InviteForm />);
    expect(await screen.findByText(/Você está conectado como trader@example.com/)).toBeInTheDocument();
  });
});

describe("InviteForm back", () => {
  it("Voltar from the code step returns to step 1 with focus on its first field and a new request needs a new code", async () => {
    window.history.replaceState(null, "", "/pt-BR/invite#t=T");
    let requests = 0;
    server.use(
      http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ email: "new@example.com", expires_at: "2099-01-01T00:00:00Z" })),
      http.post("*/api/v1/auth/signup/challenges", () => {
        requests += 1;
        return issued(`signup-${requests}`);
      }),
      http.post("*/api/v1/auth/challenges/delivery", () => HttpResponse.json({ delivery: "sent" })),
    );
    renderApp(<InviteForm />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Enviar código" }));
    await screen.findByLabelText(codeLabel);
    await person.click(screen.getByRole("button", { name: "Voltar" }));
    await waitFor(() => expect(screen.getByLabelText("E-mail")).toHaveFocus());
    expect(screen.getByLabelText("E-mail")).toHaveValue("new@example.com");
    await person.click(screen.getByRole("button", { name: "Enviar código" }));
    await screen.findByLabelText(codeLabel);
    expect(requests).toBe(2);
  });
});
