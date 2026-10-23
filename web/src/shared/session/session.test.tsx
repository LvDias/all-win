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

const issued = { challenge_id: "challenge-1", expires_at: "2099-01-01T00:00:00Z", resend_after_s: 60 };
const issueChallenge = () => HttpResponse.json(issued, { status: 202 });

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
          .run({ action: "source_create", description: "criar a fonte" }, (proof) => apiFetch<{ ok: boolean }>("/api/v1/sources", { method: "POST", body: { name: "x", source_type: "tradingview", step_up: proof } }))
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
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(user)));
  });

  it("asks for a new code for the action, then sends the request once more with the proof", async () => {
    const bodies: unknown[] = [];
    let stepUpBody: unknown;
    server.use(
      http.post("*/api/v1/sources", async ({ request }) => {
        const body = await request.json();
        bodies.push(body);
        return bodies.length === 1 ? stepUpRequired() : HttpResponse.json({ ok: true }, { status: 201 });
      }),
      http.post("*/api/v1/auth/step-up", async ({ request }) => {
        stepUpBody = await request.json();
        return issueChallenge();
      }),
    );
    const result = vi.fn();
    renderApp(<Action onResult={result} />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("button", { name: "go" }));
    await person.type(await screen.findByLabelText(/Código de verificação/), "ABCD2345");
    await waitFor(() => expect(result).toHaveBeenCalledWith("done"));
    expect(stepUpBody).toEqual({ action: "source_create", channel: "email" });
    expect(bodies).toHaveLength(2);
    expect(bodies[0]).toEqual({ name: "x", source_type: "tradingview" });
    expect(bodies[1]).toEqual({ name: "x", source_type: "tradingview", step_up: { challenge_id: "challenge-1", code: "ABCD2345" } });
  });

  it("keeps the dialog open on a wrong code with one generic message, then accepts a good code", async () => {
    let attempts = 0;
    server.use(
      http.post("*/api/v1/sources", async ({ request }) => {
        const body = (await request.json()) as { step_up?: { code: string } };
        if (!body.step_up) return stepUpRequired();
        attempts += 1;
        return body.step_up.code === "AAAA1111" ? HttpResponse.json({ error: { code: "auth.code_invalid" } }, { status: 401 }) : HttpResponse.json({ ok: true }, { status: 201 });
      }),
      http.post("*/api/v1/auth/step-up", issueChallenge),
    );
    const result = vi.fn();
    renderApp(<Action onResult={result} />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("button", { name: "go" }));
    const field = await screen.findByLabelText(/Código de verificação/);
    await person.type(field, "AAAA1111");
    expect(await screen.findByText("Código inválido ou expirado. Confira ou peça um novo.")).toBeInTheDocument();
    expect(result).not.toHaveBeenCalled();
    expect(field).toHaveValue("");
    await person.type(field, "BBBB2222");
    await waitFor(() => expect(result).toHaveBeenCalledWith("done"));
    expect(attempts).toBe(2);
  });

  it("does not loop when the request with proof asks for step-up again", async () => {
    let attempts = 0;
    server.use(
      http.post("*/api/v1/sources", () => {
        attempts += 1;
        return stepUpRequired();
      }),
      http.post("*/api/v1/auth/step-up", issueChallenge),
    );
    const result = vi.fn();
    renderApp(<Action onResult={result} />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("button", { name: "go" }));
    await person.type(await screen.findByLabelText(/Código de verificação/), "ABCD2345");
    await waitFor(() => expect(result).toHaveBeenCalledWith("auth.step_up_required"));
    expect(attempts).toBe(2);
  });

  it("cancelling performs nothing", async () => {
    let attempts = 0;
    server.use(
      http.post("*/api/v1/sources", () => {
        attempts += 1;
        return stepUpRequired();
      }),
      http.post("*/api/v1/auth/step-up", issueChallenge),
    );
    const result = vi.fn();
    renderApp(<Action onResult={result} />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("button", { name: "go" }));
    await person.click(await screen.findByRole("button", { name: "Cancelar" }));
    await waitFor(() => expect(result).toHaveBeenCalledWith("cancelled"));
    expect(attempts).toBe(1);
  });

  it("shows the Core's error when the code cannot be requested", async () => {
    server.use(
      http.post("*/api/v1/sources", stepUpRequired),
      http.post("*/api/v1/auth/step-up", () => HttpResponse.json({ error: { code: "auth.channel_unavailable" } }, { status: 422 })),
    );
    renderApp(<Action onResult={() => {}} />);
    await userEvent.setup().click(screen.getByRole("button", { name: "go" }));
    expect(await screen.findByText("Você não tem um canal confirmado para isso.")).toBeInTheDocument();
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
    ["signed_out_everywhere", "sessions_ended"],
    ["device_revoked", "sessions_ended"],
    ["channel_changed", "sessions_ended"],
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
