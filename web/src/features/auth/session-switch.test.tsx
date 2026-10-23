import { navigation, renderApp } from "@/test/render";
import { QueryClient } from "@tanstack/react-query";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { useLogout } from "@/shared/session/logout";
import { server } from "@/test/server";
import { ForgotPasswordForm } from "./forgot-password-form";
import { InviteForm } from "./invite-form";
import { LoginForm } from "./login-form";
import { ResetPasswordForm } from "./reset-password-form";
import { VerifyForm } from "./verify-form";

const userB = { id: "00000000-0000-4000-8000-0000000000bb", email: "b@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const unauthenticated = () => HttpResponse.json({ error: { code: "auth.unauthenticated" } }, { status: 401 });
const client = () => new QueryClient({ defaultOptions: { queries: { retry: false }, mutations: { retry: false } } });

beforeEach(() => {
  navigation.replace.mockReset();
  navigation.push.mockReset();
  navigation.search = "";
  server.use(http.get("*/api/v1/me", unauthenticated));
});

describe("a new session never sees the previous one's data", () => {
  it("login as B empties what user A left in the cache", async () => {
    const queryClient = client();
    queryClient.setQueryData(["sources", { includeRevoked: false }], [{ name: "A's source" }]);
    queryClient.setQueryData(["connections", { includeUnpaired: false }], [{ name: "A's connection" }]);
    server.use(http.post("*/api/v1/auth/login", () => HttpResponse.json({ state: "authenticated", user: userB })));
    renderApp(<LoginForm />, "pt-BR", queryClient);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("E-mail"), "b@example.com");
    await person.type(screen.getByLabelText("Senha", { exact: true }), "pw");
    await person.click(screen.getByRole("button", { name: "Entrar" }));
    await waitFor(() => expect(navigation.replace).toHaveBeenCalled());
    expect(queryClient.getQueryData(["sources", { includeRevoked: false }])).toBeUndefined();
    expect(queryClient.getQueryData(["connections", { includeUnpaired: false }])).toBeUndefined();
    expect(queryClient.getQueryData(["me"])).toEqual(userB);
  });

  it("verifying the second factor and accepting an invitation also start from an empty cache", async () => {
    const queryClient = client();
    queryClient.setQueryData(["sources"], [{ name: "A" }]);
    server.use(http.post("*/api/v1/auth/login/totp", () => HttpResponse.json({ state: "authenticated", user: userB })));
    renderApp(<VerifyForm />, "pt-BR", queryClient);
    await userEvent.setup().type(await screen.findByRole("textbox"), "123456");
    await waitFor(() => expect(navigation.replace).toHaveBeenCalled());
    expect(queryClient.getQueryData(["sources"])).toBeUndefined();
  });

  it("invitation accept clears the cache", async () => {
    window.history.replaceState(null, "", "/pt-BR/invite#t=T");
    const queryClient = client();
    queryClient.setQueryData(["sources"], [{ name: "A" }]);
    server.use(
      http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ email: "b@example.com", expires_at: "2026-10-30T12:00:00Z" })),
      http.post("*/api/v1/auth/invitations/accept", () => HttpResponse.json({ state: "authenticated", user: userB }, { status: 201 })),
    );
    renderApp(<InviteForm />, "pt-BR", queryClient);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("Senha", { exact: true }), "a long passphrase here");
    await person.click(screen.getByRole("button", { name: "Criar conta" }));
    await waitFor(() => expect(navigation.replace).toHaveBeenCalled());
    expect(queryClient.getQueryData(["sources"])).toBeUndefined();
  });
});

function SignOut() {
  const logout = useLogout();
  return (
    <button type="button" onClick={() => logout.mutate()}>
      sair
    </button>
  );
}

describe("sign out", () => {
  it("goes to login with signed_out on success and empties the cache", async () => {
    const queryClient = client();
    queryClient.setQueryData(["sources"], [{ name: "A" }]);
    server.use(http.post("*/api/v1/auth/logout", () => new HttpResponse(null, { status: 204 })));
    renderApp(<SignOut />, "pt-BR", queryClient);
    await userEvent.setup().click(screen.getByRole("button", { name: "sair" }));
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/pt-BR/login?reason=signed_out"));
    expect(queryClient.getQueryData(["sources"])).toBeUndefined();
  });

  it("does not claim a sign-out when the request failed: error toast with retry, cache still cleared", async () => {
    const queryClient = client();
    queryClient.setQueryData(["sources"], [{ name: "A" }]);
    server.use(http.post("*/api/v1/auth/logout", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    renderApp(<SignOut />, "pt-BR", queryClient);
    await userEvent.setup().click(screen.getByRole("button", { name: "sair" }));
    expect(await screen.findByText("Não foi possível sair. Sua sessão pode continuar ativa.")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Tentar novamente" })).toBeInTheDocument();
    expect(navigation.replace).not.toHaveBeenCalled();
    expect(queryClient.getQueryData(["sources"])).toBeUndefined();
  });

  it("treats 401 as already signed out", async () => {
    server.use(http.post("*/api/v1/auth/logout", unauthenticated));
    renderApp(<SignOut />);
    await userEvent.setup().click(screen.getByRole("button", { name: "sair" }));
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/pt-BR/login?reason=signed_out"));
  });
});

describe("public pages", () => {
  it("a malformed fragment is the generic invalid state, and the fragment is still removed", async () => {
    window.history.replaceState(null, "", "/pt-BR/invite#t=%E0%A4%A");
    renderApp(<InviteForm />);
    expect(await screen.findByText(/Este convite não é válido/)).toBeInTheDocument();
    expect(window.location.hash).toBe("");
  });

  it("forgot password gives the same answer for any e-mail", async () => {
    server.use(http.post("*/api/v1/auth/password-reset/request", () => new HttpResponse(null, { status: 202 })));
    renderApp(<ForgotPasswordForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("E-mail"), "nobody@example.com");
    await person.click(screen.getByRole("button", { name: "Enviar link" }));
    expect(await screen.findByText(/Se houver uma conta com este e-mail, você receberá um link/)).toBeInTheDocument();
  });

  it("reset success has a single heading and moves focus to it", async () => {
    window.history.replaceState(null, "", "/pt-BR/reset-password#t=T");
    server.use(http.post("*/api/v1/auth/password-reset/confirm", () => new HttpResponse(null, { status: 204 })));
    renderApp(<ResetPasswordForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("Nova senha"), "a long passphrase here");
    await person.click(screen.getByRole("button", { name: "Redefinir senha" }));
    const headings = await screen.findAllByRole("heading");
    expect(headings).toHaveLength(1);
    expect(headings[0]).toHaveTextContent("Senha redefinida");
    expect(headings[0]).toHaveFocus();
  });

  it("keeps the e-mail typed at login when the verification expired", async () => {
    server.use(http.post("*/api/v1/auth/login", () => HttpResponse.json({ state: "pending_2fa", pending_expires_at: "2026-10-23T12:05:00Z" })));
    const first = renderApp(<LoginForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("E-mail"), "kept@example.com");
    await person.type(screen.getByLabelText("Senha", { exact: true }), "pw");
    await person.click(screen.getByRole("button", { name: "Entrar" }));
    await waitFor(() => expect(navigation.push).toHaveBeenCalled());
    first.unmount();
    navigation.search = "reason=pending_expired";
    renderApp(<LoginForm />);
    expect(await screen.findByLabelText("E-mail")).toHaveValue("kept@example.com");
  });

  it("invite: a signed-in visitor is asked to sign out first", async () => {
    window.history.replaceState(null, "", "/pt-BR/invite#t=T");
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(userB)));
    renderApp(<InviteForm />);
    expect(await screen.findByText(/Você está conectado como b@example.com/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Sair" })).toBeInTheDocument();
  });

  it("invite: language and time zone can be changed before creating the account", async () => {
    window.history.replaceState(null, "", "/pt-BR/invite#t=T");
    const bodies: unknown[] = [];
    server.use(
      http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ email: "n@example.com", expires_at: "2026-10-30T12:00:00Z" })),
      http.post("*/api/v1/auth/invitations/accept", async ({ request }) => { bodies.push(await request.json()); return HttpResponse.json({ state: "authenticated", user: userB }, { status: 201 }); }),
    );
    renderApp(<InviteForm />);
    const person = userEvent.setup();
    await person.type(await screen.findByLabelText("Senha", { exact: true }), "a long passphrase here");
    await person.click(screen.getByRole("combobox", { name: "Idioma" }));
    await person.click(await screen.findByRole("option", { name: "English" }));
    await person.click(screen.getByRole("button", { name: "Alterar" }));
    await person.click(await screen.findByRole("combobox", { name: "Fuso horário" }));
    await person.type(await screen.findByRole("combobox", { name: "Buscar fuso horário" }), "Sao Paulo");
    await person.click(await screen.findByRole("option", { name: /Sao Paulo/ }));
    await person.click(screen.getByRole("button", { name: "Criar conta" }));
    await waitFor(() => expect(bodies).toHaveLength(1));
    expect(bodies[0]).toMatchObject({ locale: "en", time_zone: "America/Sao_Paulo" });
  });
});
