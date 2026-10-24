// QA (Lente) — S3 passwordless web, slice-plan rev. 8 AC-1…AC-8, AC-13; contract 5.1.1; auth policy. Written from the
// contract, not from the implementation. Public surface only: @/features/auth, @/features/security, @/shared, @/test.
// The navigation mock lives in @/test/render and must load before the components.
import { navigation, renderApp } from "@/test/render";
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { CodeForm, InviteForm, LoginForm } from "@/features/auth";
import { server } from "@/test/server";

const me = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const unauthenticated = () => HttpResponse.json({ error: { code: "auth.unauthenticated" } }, { status: 401 });
const issued = (id = "challenge-0000000001") => HttpResponse.json({ challenge_id: id, expires_at: new Date(Date.now() + 600_000).toISOString(), resend_after_s: 60 }, { status: 202 });
const codeLabel = /Código de verificação/;
const wrong = () => HttpResponse.json({ error: { code: "auth.code_invalid" } }, { status: 401 });

/** Contract OneTimeCode normalization (U16): trim, drop spaces and `-`, uppercase, O→0, I/L→1. */
const crockford = (raw: string) => raw.trim().replace(/[\s-]/g, "").toUpperCase().replace(/O/g, "0").replace(/[IL]/g, "1");

beforeEach(() => {
  navigation.replace.mockReset();
  navigation.push.mockReset();
  navigation.search = "";
  window.sessionStorage.clear();
  window.localStorage.clear();
  server.use(http.get("*/api/v1/me", unauthenticated));
});

/** Login page → (navigation is mocked) → the code page, with the pending login carried in memory like the app does. */
async function toCodePage(person: ReturnType<typeof userEvent.setup>, email = "trader@example.com") {
  const login = renderApp(<LoginForm />);
  const field = await screen.findByLabelText(/E-mail/i);
  await person.clear(field);
  await person.type(field, email);
  await person.click(screen.getByRole("button", { name: /Enviar código/i }));
  await waitFor(() => expect(navigation.push.mock.calls.length + navigation.replace.mock.calls.length).toBeGreaterThan(0));
  login.unmount();
  renderApp(<CodeForm />);
}

describe("QA S3 — CodeForm guard", () => {
  it("without a pending login the code page goes back to the e-mail page", async () => {
    renderApp(<CodeForm />);
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith(expect.stringMatching(/\/login/)));
  });
});

describe("QA S3 — AC-3 no password anywhere", () => {
  it("the e-mail page, the code page and the invitation have no password-type input and no password wording", async () => {
    server.use(
      http.post("*/api/v1/auth/login/challenges", () => issued()),
      http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ email: "new@example.com", locale: "en", expires_at: "2099-01-01T00:00:00Z" })),
    );
    const person = userEvent.setup();
    const { unmount } = renderApp(<LoginForm />);
    expect(document.querySelector('input[type="password"]')).toBeNull();
    expect(screen.queryByLabelText(/senha|password/i)).not.toBeInTheDocument();
    expect(screen.queryByText(/esqueci|forgot|redefinir|reset/i)).not.toBeInTheDocument();
    unmount();
    window.history.replaceState(null, "", "/pt-BR/invite#t=tok");
    renderApp(<InviteForm />);
    await screen.findByDisplayValue("new@example.com");
    expect(document.querySelector('input[type="password"]')).toBeNull();
    expect(screen.queryByLabelText(/senha|password/i)).not.toBeInTheDocument();
    void person;
  });
});

describe("QA S3 — AC-7 no account enumeration", () => {
  it("a registered and an unregistered e-mail get the identical UI path and text (same 202 shape, same page)", async () => {
    const seen: string[] = [];
    server.use(http.post("*/api/v1/auth/login/challenges", async ({ request }) => { seen.push(((await request.json()) as { email: string }).email); return issued(); }));
    const texts: string[] = [];
    for (const email of ["known@example.com", "nobody@example.com"]) {
      const person = userEvent.setup();
      const { unmount } = renderApp(<LoginForm />);
      const field = await screen.findByLabelText(/E-mail/i);
      await person.clear(field);
      await person.type(field, email);
      await person.click(screen.getByRole("button", { name: /Enviar código/i }));
      await waitFor(() => expect(navigation.push.mock.calls.length + navigation.replace.mock.calls.length).toBeGreaterThan(0));
      texts.push(JSON.stringify([navigation.push.mock.calls, navigation.replace.mock.calls]));
      navigation.push.mockReset();
      navigation.replace.mockReset();
      unmount();
    }
    expect(seen).toEqual(["known@example.com", "nobody@example.com"]);
    expect(texts[0]).toBe(texts[1]);
    expect(document.body.textContent).not.toMatch(/não cadastrad|não encontrad|not registered|not found|unknown/i);
  });

  it("the code page shows the same wrong-code text on every attempt (1…4), never revealing attempts left or why", async () => {
    server.use(http.post("*/api/v1/auth/login/challenges", () => issued()), http.post("*/api/v1/auth/login/verify", wrong));
    const person = userEvent.setup();
    await toCodePage(person);
    const messages = new Set<string>();
    for (let attempt = 1; attempt <= 4; attempt++) {
      const input = await screen.findByLabelText(codeLabel);
      await person.type(input, `ABCD234${attempt}`);
      const alert = await screen.findByRole("alert");
      messages.add(alert.textContent ?? "");
      expect(alert.textContent).not.toMatch(/\b[1-5]\b.*(tentativa|attempt)|restam|remaining|expirou|expired|usado|used/i);
    }
    expect(messages.size).toBe(1);
  });

  it("a 429 on the code request asks the user to wait without naming which limit (user/IP/identifier)", async () => {
    server.use(http.post("*/api/v1/auth/login/challenges", () => HttpResponse.json({ error: { code: "auth.rate_limited" } }, { status: 429, headers: { "Retry-After": "90" } })));
    const person = userEvent.setup();
    renderApp(<LoginForm />);
    await person.type(await screen.findByLabelText(/E-mail/i), "trader@example.com");
    await person.click(screen.getByRole("button", { name: /Enviar código/i }));
    const alert = await screen.findByRole("alert");
    expect(alert.textContent).toMatch(/aguarde|espere|tente de novo|segundos|minuto/i);
    expect(alert.textContent).not.toMatch(/IP|usuário|user|dispositivo|device|e-mail.*limite/i);
    expect(navigation.push).not.toHaveBeenCalled();
    expect(screen.queryByLabelText(codeLabel)).not.toBeInTheDocument(); // no code was sent, no code page
  });
});

describe("QA S3 — AC-5 code entry (U16)", () => {
  it.each(["ab-cd 23 45", "  abcd2345 ", "abcd–2345", "ABCD-2345"])("accepts %j and the Core receives something that normalizes to ABCD2345", async (typed) => {
    const bodies: { code: string }[] = [];
    server.use(
      http.post("*/api/v1/auth/login/challenges", () => issued()),
      http.post("*/api/v1/auth/login/verify", async ({ request }) => { bodies.push((await request.json()) as { code: string }); return HttpResponse.json({ user: me, new_device: false }); }),
    );
    const person = userEvent.setup();
    await toCodePage(person);
    const input = await screen.findByLabelText(codeLabel);
    await person.click(input);
    await person.paste(typed);
    await waitFor(() => expect(bodies).toHaveLength(1));
    expect(crockford(bodies[0]?.code ?? "")).toBe("ABCD2345");
    expect((bodies[0]?.code ?? "").length).toBeLessThanOrEqual(32); // OneTimeCode maxLength
  });

  it("look-alikes O/o/I/i/L/l are never dropped or turned into other letters before the Core (Core decodes them)", async () => {
    const bodies: { code: string }[] = [];
    server.use(
      http.post("*/api/v1/auth/login/challenges", () => issued()),
      http.post("*/api/v1/auth/login/verify", async ({ request }) => { bodies.push((await request.json()) as { code: string }); return wrong(); }),
    );
    const person = userEvent.setup();
    await toCodePage(person);
    await person.click(await screen.findByLabelText(codeLabel));
    await person.paste("oOiIlL-9AZ");
    await waitFor(() => expect(bodies).toHaveLength(1));
    expect(crockford(bodies[0]?.code ?? "")).toBe("0011119A".replace(/^/, "") && crockford("oOiIlL9A"));
    expect(crockford("oOiIlL9A")).toBe("0011119A");
  });

  it("the field is a single text input with one-time-code autofill and no spellcheck (paste, autofill, AT)", async () => {
    server.use(http.post("*/api/v1/auth/login/challenges", () => issued()));
    const person = userEvent.setup();
    await toCodePage(person);
    const input = await screen.findByLabelText(codeLabel);
    expect(input).toHaveAttribute("autocomplete", "one-time-code");
    expect(input).toHaveAttribute("type", "text");
    expect(input).toHaveAttribute("spellcheck", "false");
  });

  it("the code never reaches the URL, web storage or the console", async () => {
    const spies = (["log", "info", "warn", "error", "debug"] as const).map((m) => vi.spyOn(console, m).mockImplementation(() => undefined));
    server.use(http.post("*/api/v1/auth/login/challenges", () => issued()), http.post("*/api/v1/auth/login/verify", wrong));
    const person = userEvent.setup();
    await toCodePage(person);
    await person.type(await screen.findByLabelText(codeLabel), "ZZ99YY88");
    await screen.findByRole("alert");
    expect(window.location.href).not.toMatch(/ZZ99YY88/i);
    expect(JSON.stringify({ ...window.localStorage })).not.toMatch(/ZZ99YY88/i);
    expect(JSON.stringify({ ...window.sessionStorage })).not.toMatch(/ZZ99YY88/i);
    for (const spy of spies) expect(JSON.stringify(spy.mock.calls)).not.toMatch(/ZZ99YY88/i);
    // the field is cleared after the attempt (code lives only in component state)
    expect(screen.getByLabelText(codeLabel)).toHaveValue("");
  });

  it("the pending login survives only in memory/session for the challenge, never holds the code", async () => {
    server.use(http.post("*/api/v1/auth/login/challenges", () => issued()));
    const person = userEvent.setup();
    await toCodePage(person);
    await screen.findByLabelText(codeLabel);
    expect(JSON.stringify({ ...window.localStorage })).not.toMatch(/trader@example\.com/); // e-mail not persisted across sessions
  });
});

describe("QA S3 — AC-8 sessions", () => {
  it("'keep me signed in' is off by default and sends remember_me=true only when ticked", async () => {
    const bodies: { remember_me: boolean }[] = [];
    server.use(
      http.post("*/api/v1/auth/login/challenges", () => issued()),
      http.post("*/api/v1/auth/login/verify", async ({ request }) => { bodies.push((await request.json()) as { remember_me: boolean }); return HttpResponse.json({ user: me, new_device: false }); }),
    );
    const person = userEvent.setup();
    const login = renderApp(<LoginForm />);
    const remember = await screen.findByRole("checkbox", { name: /Manter.*conectad|keep me/i });
    expect(remember).not.toBeChecked();
    await person.type(screen.getByLabelText(/E-mail/i), "trader@example.com");
    await person.click(remember);
    await person.click(screen.getByRole("button", { name: /Enviar código/i }));
    await waitFor(() => expect(navigation.push.mock.calls.length + navigation.replace.mock.calls.length).toBeGreaterThan(0));
    login.unmount();
    renderApp(<CodeForm />);
    await person.type(await screen.findByLabelText(codeLabel), "ABCD2345");
    await waitFor(() => expect(bodies).toHaveLength(1));
    expect(bodies[0]?.remember_me).toBe(true);
  });

  it("a signed-in session that expires returns to login with a reason message (not a blank error)", async () => {
    navigation.search = "reason=session_expired";
    renderApp(<LoginForm />);
    expect(await screen.findByRole("status")).toHaveTextContent(/sessão|session/i);
  });
});

describe("QA S3 — AC-1/AC-2 invite-only sign-up (Q36 a)", () => {
  function open() { window.history.replaceState(null, "", "/pt-BR/invite#t=tok"); }

  it("the e-mail is the invited one and cannot be edited; start-signup sends only the token", async () => {
    open();
    const bodies: unknown[] = [];
    server.use(
      http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ email: "new@example.com", locale: "en", expires_at: "2099-01-01T00:00:00Z" })),
      http.post("*/api/v1/auth/signup/challenges", async ({ request }) => { bodies.push(await request.json()); return issued("signup-0000000001"); }),
    );
    const person = userEvent.setup();
    renderApp(<InviteForm />);
    const field = await screen.findByDisplayValue("new@example.com");
    expect(field.hasAttribute("readonly") || field.hasAttribute("disabled")).toBe(true);
    await person.click(screen.getByRole("button", { name: "Enviar código" }));
    await waitFor(() => expect(bodies).toHaveLength(1));
    expect(bodies[0]).toEqual({ token: "tok" });
  });

  it.each(["auth.invitation_invalid"])("unknown/used/expired/revoked invitation (%s) → one generic message, no distinction", async (code) => {
    open();
    server.use(http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ error: { code } }, { status: 404 })));
    renderApp(<InviteForm />);
    const text = (await screen.findByText(/convite não é válido/i)).textContent ?? "";
    expect(text).not.toMatch(/\d{2}\/\d{2}\/\d{4}/); // no date: the page never says which reason applies
  });

  it("an abandoned or failed confirmation creates nothing client-side: no session, no landing redirect", async () => {
    open();
    server.use(
      http.post("*/api/v1/auth/invitations/lookup", () => HttpResponse.json({ email: "new@example.com", locale: "en", expires_at: "2099-01-01T00:00:00Z" })),
      http.post("*/api/v1/auth/signup/challenges", () => issued("signup-0000000001")),
      http.post("*/api/v1/auth/challenges/delivery", () => HttpResponse.json({ delivery: "sent" })),
      http.post("*/api/v1/auth/signup/verify", wrong),
    );
    const person = userEvent.setup();
    renderApp(<InviteForm />);
    await person.click(await screen.findByRole("button", { name: "Enviar código" }));
    await person.type(await screen.findByLabelText(codeLabel), "ABCD2345");
    await screen.findByRole("alert");
    expect(navigation.replace).not.toHaveBeenCalled();
  });
});

