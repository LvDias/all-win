import { act, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { useState } from "react";
import { ApiError, apiFetch } from "@/shared/api/client";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { FakeEventSource } from "@/test/setup";
import { StepUpCancelled, useStepUp } from "./step-up";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const stepUpRequired = () => HttpResponse.json({ error: { code: "auth.step_up_required" } }, { status: 403 });

function Action({ onResult }: { onResult: (value: string) => void }) {
  const stepUp = useStepUp();
  const [busy, setBusy] = useState(false);
  return (
    <button
      type="button"
      disabled={busy}
      onClick={() => {
        setBusy(true);
        stepUp
          .run(() => apiFetch<{ ok: boolean }>("/api/v1/sources", { method: "POST", body: { name: "x", source_type: "tradingview" } }))
          .then(() => onResult("done"))
          .catch((error: unknown) => onResult(error instanceof StepUpCancelled ? "cancelled" : error instanceof ApiError ? error.code : "other"))
          .finally(() => setBusy(false));
      }}
    >
      go
    </button>
  );
}

describe("step-up", () => {
  beforeEach(() => {
    server.use(
      http.get("*/api/v1/me", () => HttpResponse.json(user)),
      http.get("*/api/v1/me/security", () => HttpResponse.json({ totp_enabled: false, recovery_codes_remaining: 0, session: {} })),
    );
  });

  it("asks for the password, then retries the original request exactly once", async () => {
    let attempts = 0;
    server.use(
      http.post("*/api/v1/sources", () => (++attempts === 1 ? stepUpRequired() : HttpResponse.json({ ok: true }, { status: 201 }))),
      http.post("*/api/v1/auth/step-up", async ({ request }) => {
        expect(await request.json()).toEqual({ password: "secret pass" });
        return new HttpResponse(null, { status: 204 });
      }),
    );
    const result = vi.fn();
    renderApp(<Action onResult={result} />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("button", { name: "go" }));
    await person.type(await screen.findByLabelText("Senha"), "secret pass");
    await person.click(screen.getByRole("button", { name: "Confirmar" }));
    await waitFor(() => expect(result).toHaveBeenCalledWith("done"));
    expect(attempts).toBe(2);
  });

  it("does not retry when the retry asks for step-up again (no loop)", async () => {
    let attempts = 0;
    server.use(
      http.post("*/api/v1/sources", () => { attempts += 1; return stepUpRequired(); }),
      http.post("*/api/v1/auth/step-up", () => new HttpResponse(null, { status: 204 })),
    );
    const result = vi.fn();
    renderApp(<Action onResult={result} />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("button", { name: "go" }));
    await person.type(await screen.findByLabelText("Senha"), "p");
    await person.click(screen.getByRole("button", { name: "Confirmar" }));
    await waitFor(() => expect(result).toHaveBeenCalledWith("auth.step_up_required"));
    expect(attempts).toBe(2);
  });

  it("cancelling performs nothing", async () => {
    let attempts = 0;
    server.use(http.post("*/api/v1/sources", () => { attempts += 1; return stepUpRequired(); }));
    const result = vi.fn();
    renderApp(<Action onResult={result} />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("button", { name: "go" }));
    await person.click(await screen.findByRole("button", { name: "Cancelar" }));
    await waitFor(() => expect(result).toHaveBeenCalledWith("cancelled"));
    expect(attempts).toBe(1);
  });

  it("shows one message for a wrong password or code", async () => {
    server.use(
      http.post("*/api/v1/sources", stepUpRequired),
      http.post("*/api/v1/auth/step-up", () => HttpResponse.json({ error: { code: "auth.reauth_failed" } }, { status: 401 })),
    );
    renderApp(<Action onResult={() => {}} />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("button", { name: "go" }));
    await person.type(await screen.findByLabelText("Senha"), "bad");
    await person.click(screen.getByRole("button", { name: "Confirmar" }));
    expect(await screen.findByText("Senha ou código incorretos.")).toBeInTheDocument();
  });

  it("asks for the authenticator code when TOTP is on", async () => {
    server.use(
      http.get("*/api/v1/me/security", () => HttpResponse.json({ totp_enabled: true, recovery_codes_remaining: 10, session: {} })),
      http.post("*/api/v1/sources", stepUpRequired),
    );
    renderApp(<Action onResult={() => {}} />);
    await userEvent.setup().click(screen.getByRole("button", { name: "go" }));
    expect(await screen.findByLabelText("Código do app autenticador")).toBeInTheDocument();
  });
});

describe("event stream", () => {
  beforeEach(() => {
    navigation.replace.mockReset();
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(user)));
  });

  it.each([
    ["expired", "session_expired"],
    ["logout", "signed_out"],
    ["password_reset", "sessions_ended"],
    ["password_changed", "sessions_ended"],
    ["totp_changed", "sessions_ended"],
    ["admin_totp_reset", "sessions_ended"],
  ])("session.ended (%s) goes to login with reason %s", async (ended, reason) => {
    renderApp(<p>app</p>);
    await waitFor(() => expect(FakeEventSource.latest()).toBeDefined());
    act(() => FakeEventSource.latest()?.emit("session.ended", { type: "session.ended", at: "2026-10-23T12:00:00Z", reason: ended }));
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith(`/pt-BR/login?reason=${reason}`));
    expect(FakeEventSource.latest()?.closed).toBe(true);
  });

  it("does not open the stream without a session", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json({ error: { code: "auth.unauthenticated" } }, { status: 401 })));
    renderApp(<p>app</p>);
    await screen.findByText("app");
    expect(FakeEventSource.instances).toHaveLength(0);
  });
});
