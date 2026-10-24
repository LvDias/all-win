import { vi } from "vitest";
// QA (Lente) — S3 AC-10 step-up, Q30 (fresh code, no reuse window), M3 (e-mail on a young device), Q35 actions.
// Exercises the shared step-up runner with every action of the contract's StepUpAction enum.
import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { useState } from "react";
import { beforeEach, describe, expect, it } from "vitest";
import { apiFetch } from "@/shared/api/client";
import { StepUpCancelled, useStepUp } from "@/shared/session/step-up";
import type { StepUpAction } from "@/shared/api/types";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";

const me = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const target = "5a2b8c3d-1e4f-4a6b-8c7d-9e0f1a2b3c4d";
const codeLabel = /Código de verificação/;
const needs = () => HttpResponse.json({ error: { code: "auth.step_up_required" } }, { status: 403 });
const issued = (n: number) => HttpResponse.json({ challenge_id: `challenge-00000000${n}`, expires_at: "2099-01-01T00:00:00Z", resend_after_s: 60 }, { status: 202 });

/** contract §requestStepUp: target_id required for these, forbidden for the others. */
const withTarget: StepUpAction[] = ["source_rotate", "pairing_code_new", "connection_unpair", "account_opt_in_accept", "route_enable", "account_binding_change"];
const withoutTarget: StepUpAction[] = ["source_create", "connection_create", "account_create_opt_in_accept", "admin_invitation_create"];

function Runner({ action, targetId, out }: { action: StepUpAction; targetId?: string; out: (v: string) => void }) {
  const stepUp = useStepUp();
  const [n, setN] = useState(0);
  return (
    <button
      type="button"
      onClick={() => {
        setN(n + 1);
        stepUp
          .run({ action, targetId, description: "ação" }, (proof) => apiFetch("/api/v1/qa/action", { method: "POST", body: { step_up: proof } }))
          .then(() => out("done"))
          .catch((e: unknown) => out(e instanceof StepUpCancelled ? "cancelled" : "error"));
      }}
    >
      run
    </button>
  );
}

beforeEach(() => server.use(http.get("*/api/v1/me", () => HttpResponse.json(me))));

describe("QA S3 AC-10 — request shape per action", () => {
  it.each([...withTarget, ...withoutTarget])("%s: asks e-mail code with the right target_id, then ONE original request with the proof", async (action) => {
    const stepUpBodies: unknown[] = [];
    const actionBodies: unknown[] = [];
    server.use(
      http.post("*/api/v1/qa/action", async ({ request }) => { actionBodies.push(await request.json()); return actionBodies.length === 1 ? needs() : HttpResponse.json({}, { status: 201 }); }),
      http.post("*/api/v1/auth/step-up", async ({ request }) => { stepUpBodies.push(await request.json()); return issued(1); }),
    );
    const out = vi.fn();
    const person = userEvent.setup();
    renderApp(<Runner action={action} targetId={withTarget.includes(action) ? target : undefined} out={out} />);
    await person.click(screen.getByRole("button", { name: "run" }));
    await person.type(await screen.findByLabelText(codeLabel), "ABCD2345");
    await waitFor(() => expect(out).toHaveBeenCalledWith("done"));
    expect(stepUpBodies).toEqual([withTarget.includes(action) ? { action, target_id: target, channel: "email" } : { action, channel: "email" }]);
    expect(actionBodies).toHaveLength(2);
    expect(actionBodies[1]).toEqual({ step_up: { challenge_id: "challenge-000000001", code: "ABCD2345" } });
  });
});

describe("QA S3 AC-10 — no reuse window (Q30 a)", () => {
  it("two consecutive actions ask two different codes; the second never reuses the first proof", async () => {
    let call = 0;
    let challenge = 0;
    const proofs: unknown[] = [];
    server.use(
      http.post("*/api/v1/qa/action", async ({ request }) => {
        const body = (await request.json()) as { step_up?: unknown };
        call++;
        if (body.step_up) { proofs.push(body.step_up); return HttpResponse.json({}, { status: 201 }); }
        return needs();
      }),
      http.post("*/api/v1/auth/step-up", () => issued(++challenge)),
    );
    const out = vi.fn();
    const person = userEvent.setup();
    renderApp(<Runner action="source_create" out={out} />);
    await person.click(screen.getByRole("button", { name: "run" }));
    await person.type(await screen.findByLabelText(codeLabel), "AAAA1111");
    await waitFor(() => expect(out).toHaveBeenCalledTimes(1));
    await person.click(screen.getByRole("button", { name: "run" }));
    await person.type(await screen.findByLabelText(codeLabel), "BBBB2222");
    await waitFor(() => expect(out).toHaveBeenCalledTimes(2));
    expect(challenge).toBe(2);
    expect(proofs).toEqual([
      { challenge_id: "challenge-000000001", code: "AAAA1111" },
      { challenge_id: "challenge-000000002", code: "BBBB2222" },
    ]);
    expect(call).toBe(4); // each run: attempt without proof + attempt with proof
  });

  it("cancelling performs nothing: the action is not retried with a proof", async () => {
    const bodies: unknown[] = [];
    server.use(http.post("*/api/v1/qa/action", async ({ request }) => { bodies.push(await request.json()); return needs(); }), http.post("*/api/v1/auth/step-up", () => issued(1)));
    const out = vi.fn();
    const person = userEvent.setup();
    renderApp(<Runner action="connection_create" out={out} />);
    await person.click(screen.getByRole("button", { name: "run" }));
    await screen.findByLabelText(codeLabel);
    await person.keyboard("{Escape}");
    await waitFor(() => expect(out).toHaveBeenCalledWith("cancelled"));
    expect(bodies).toHaveLength(1);
  });
});

describe("QA S3 AC-10 — failures", () => {
  it("a wrong proof keeps the dialog open with ONE generic message (same for expired/used/invalidated), nothing performed", async () => {
    server.use(
      http.post("*/api/v1/qa/action", async ({ request }) => ((await request.json()) as { step_up?: unknown }).step_up ? HttpResponse.json({ error: { code: "auth.code_invalid" } }, { status: 401 }) : needs()),
      http.post("*/api/v1/auth/step-up", () => issued(1)),
    );
    const out = vi.fn();
    const person = userEvent.setup();
    renderApp(<Runner action="source_create" out={out} />);
    await person.click(screen.getByRole("button", { name: "run" }));
    await person.type(await screen.findByLabelText(codeLabel), "AAAA1111");
    const first = (await screen.findByRole("alert")).textContent;
    await person.type(screen.getByLabelText(codeLabel), "BBBB2222");
    await waitFor(() => expect(screen.getByRole("alert").textContent).toBe(first));
    expect(out).not.toHaveBeenCalled();
  });

  it("M3: 422 auth.step_up_channel_not_allowed on the code request is shown as an error, never silently swallowed", async () => {
    server.use(
      http.post("*/api/v1/qa/action", needs),
      http.post("*/api/v1/auth/step-up", () => HttpResponse.json({ error: { code: "auth.step_up_channel_not_allowed" } }, { status: 422 })),
    );
    const out = vi.fn();
    const person = userEvent.setup();
    renderApp(<Runner action="source_create" out={out} />);
    await person.click(screen.getByRole("button", { name: "run" }));
    expect(await screen.findByRole("alert")).toBeInTheDocument();
    expect(out).not.toHaveBeenCalledWith("done");
  });

  it("429 on the code request: wait message, no code field, nothing performed", async () => {
    server.use(
      http.post("*/api/v1/qa/action", needs),
      http.post("*/api/v1/auth/step-up", () => HttpResponse.json({ error: { code: "auth.rate_limited" } }, { status: 429, headers: { "Retry-After": "60" } })),
    );
    const out = vi.fn();
    const person = userEvent.setup();
    renderApp(<Runner action="pairing_code_new" targetId={target} out={out} />);
    await person.click(screen.getByRole("button", { name: "run" }));
    expect(await screen.findByRole("alert")).toBeInTheDocument();
    expect(screen.queryByLabelText(codeLabel)).not.toBeInTheDocument();
    expect(out).not.toHaveBeenCalledWith("done");
  });
});
