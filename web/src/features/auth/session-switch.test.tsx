import { navigation, renderApp } from "@/test/render";
import { QueryClient } from "@tanstack/react-query";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { useLogout } from "@/shared/session/logout";
import { server } from "@/test/server";
import { CodeForm } from "./code-form";
import { InviteForm } from "./invite-form";
import { clearPendingLogin, startPendingLogin } from "./pending-login";

const userB = { id: "00000000-0000-4000-8000-0000000000bb", email: "b@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const unauthenticated = () => HttpResponse.json({ error: { code: "auth.unauthenticated" } }, { status: 401 });
const client = () => new QueryClient({ defaultOptions: { queries: { retry: false }, mutations: { retry: false } } });
const challenge = { challenge_id: "c1", expires_at: "2099-01-01T00:00:00Z", resend_after_s: 60 };

beforeEach(() => {
  clearPendingLogin();
  navigation.replace.mockReset();
  navigation.push.mockReset();
  navigation.search = "";
  server.use(http.get("*/api/v1/me", unauthenticated));
});

describe("a new session never sees the previous one's data", () => {
  it("signing in as B empties what user A left in the cache", async () => {
    const queryClient = client();
    queryClient.setQueryData(["sources", { includeRevoked: false }], [{ name: "A's source" }]);
    queryClient.setQueryData(["connections", { includeUnpaired: false }], [{ name: "A's connection" }]);
    startPendingLogin({ email: "b@example.com", remember: false, challenge });
    server.use(http.post("*/api/v1/auth/login/verify", () => HttpResponse.json({ user: userB, new_device: false })));
    renderApp(<CodeForm />, "pt-BR", queryClient);
    await userEvent.setup().type(await screen.findByLabelText(/Código de verificação/), "ABCD2345");
    await waitFor(() => expect(navigation.replace).toHaveBeenCalled());
    expect(queryClient.getQueryData(["sources", { includeRevoked: false }])).toBeUndefined();
    expect(queryClient.getQueryData(["connections", { includeUnpaired: false }])).toBeUndefined();
    expect(queryClient.getQueryData(["me"])).toEqual(userB);
  });

  it("confirming an invitation also starts from an empty cache", async () => {
    window.history.replaceState(null, "", "/pt-BR/invite#t=T");
    const queryClient = client();
    queryClient.setQueryData(["sources"], [{ name: "A" }]);
    server.use(
      http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ email: "b@example.com", expires_at: "2099-01-01T00:00:00Z" })),
      http.post("*/api/v1/auth/signup/challenges", () => HttpResponse.json(challenge, { status: 202 })),
      http.post("*/api/v1/auth/challenges/delivery", () => HttpResponse.json({ delivery: "sent" })),
      http.post("*/api/v1/auth/signup/verify", () => HttpResponse.json({ user: userB, new_device: false }, { status: 201 })),
    );
    renderApp(<InviteForm />, "pt-BR", queryClient);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("button", { name: "Enviar código" }));
    await person.type(await screen.findByLabelText(/Código de verificação/), "ABCD2345");
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

  it("invite: language and time zone can be changed before the code is requested", async () => {
    window.history.replaceState(null, "", "/pt-BR/invite#t=T");
    const bodies: unknown[] = [];
    server.use(
      http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ email: "n@example.com", expires_at: "2099-01-01T00:00:00Z" })),
      http.post("*/api/v1/auth/signup/challenges", () => HttpResponse.json(challenge, { status: 202 })),
      http.post("*/api/v1/auth/challenges/delivery", () => HttpResponse.json({ delivery: "sent" })),
      http.post("*/api/v1/auth/signup/verify", async ({ request }) => {
        bodies.push(await request.json());
        return HttpResponse.json({ user: userB, new_device: false }, { status: 201 });
      }),
    );
    renderApp(<InviteForm />);
    const person = userEvent.setup();
    await person.click(await screen.findByRole("combobox", { name: "Idioma" }));
    await person.click(await screen.findByRole("option", { name: "English" }));
    await person.click(screen.getByRole("button", { name: "Alterar" }));
    await person.click(await screen.findByRole("combobox", { name: "Fuso horário" }));
    await person.type(await screen.findByRole("combobox", { name: "Buscar fuso horário" }), "Sao Paulo");
    await person.click(await screen.findByRole("option", { name: /Sao Paulo/ }));
    await person.click(screen.getByRole("button", { name: "Enviar código" }));
    await person.type(await screen.findByLabelText(/Código de verificação/), "ABCD2345");
    await waitFor(() => expect(bodies).toHaveLength(1));
    expect(bodies[0]).toMatchObject({ locale: "en", time_zone: "America/Sao_Paulo" });
  }, 20_000);
});
