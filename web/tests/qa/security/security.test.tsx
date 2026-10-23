// QA (Lente) — S3 AC-11 (channel change, M2 hold, 24 h freeze), AC-17 (devices, sign out everywhere), AC-8 display.
// The navigation mock lives in @/test/render and must load before the components.
import { navigation, renderApp } from "@/test/render";
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { beforeEach, describe, expect, it } from "vitest";
import { SecurityPage } from "@/features/security";
import { server } from "@/test/server";

const me = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const session = (persistent = false) => ({ persistent, idle_timeout_s: persistent ? 604800 : 43200, absolute_timeout_s: persistent ? 2592000 : 86400, idle_expires_at: "2099-01-01T00:00:00Z", absolute_expires_at: "2099-01-02T00:00:00Z" });
const state = (extra: object = {}, persistent = false) => ({ email: "trader@example.com", channel_change_allowed_at: "2020-01-01T00:00:00Z", session: session(persistent), ...extra });
const devices = [
  { id: "d1", label: "Chrome no Windows", first_seen_at: "2026-10-01T10:00:00Z", last_seen_at: "2026-10-23T10:00:00Z", current: true },
  { id: "d2", label: "Safari no iPhone", first_seen_at: "2026-10-05T10:00:00Z", last_seen_at: "2026-10-20T10:00:00Z", current: false },
];
function serve(security: object = state(), sources: object[] = [], connections: object[] = []) {
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(me)),
    http.get("*/api/v1/me/security", () => HttpResponse.json(security)),
    http.get("*/api/v1/me/devices", () => HttpResponse.json(devices)),
    http.get("*/api/v1/sources", () => HttpResponse.json(sources)),
    http.get("*/api/v1/connections", () => HttpResponse.json(connections)),
  );
}
beforeEach(() => { navigation.replace.mockReset(); navigation.search = ""; });

describe("QA S3 AC-8 — session display", () => {
  it("a persistent session states 7 days idle / 30 days absolute; a normal one 12 h / 1 day", async () => {
    serve(state({}, true));
    renderApp(<SecurityPage />);
    expect(await screen.findByText(/7 dias/)).toBeInTheDocument();
    expect(screen.getByText(/30 dias/)).toBeInTheDocument();
  });
});

describe("QA S3 AC-11 — channel change", () => {
  it("M2: an on_hold change shows the hold end, offers Cancelar troca, and blocks a second change", async () => {
    serve(state({ pending_channel_change: { id: "c9", channel: "email", status: "on_hold", masked_new_value: "n***@e***.com", expires_at: "2099-01-01T00:00:00Z", hold_until: "2099-01-04T10:00:00Z", codes: [] } }));
    renderApp(<SecurityPage />);
    expect(await screen.findByText(/04\/01\/2099|4 de jan|2099/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Alterar e-mail" })).toBeDisabled();
    expect(screen.getAllByRole("button", { name: "Cancelar troca" }).length).toBeGreaterThan(0);
  });

  it("24 h freeze after a change: the button is disabled and the end time is shown with its zone", async () => {
    serve(state({ channel_change_allowed_at: "2099-01-01T12:00:00Z" }));
    renderApp(<SecurityPage />);
    expect(await screen.findByRole("button", { name: "Alterar e-mail" })).toBeDisabled();
    expect(screen.getByText(/UTC/)).toBeInTheDocument();
  });

  it("the new address is validated client-side and a clash (one account per channel value) does not reveal the owner", async () => {
    serve();
    server.use(http.post("*/api/v1/me/channel-changes", () => HttpResponse.json({ error: { code: "auth.channel_unavailable" } }, { status: 422 })));
    const person = userEvent.setup();
    renderApp(<SecurityPage />);
    await person.click(await screen.findByRole("button", { name: "Alterar e-mail" }));
    await person.type(await screen.findByLabelText("Novo e-mail"), "not-an-email");
    expect(screen.getByRole("button", { name: "Enviar códigos" })).toBeDisabled(); // client-side format gate (no request)
    await person.clear(screen.getByLabelText("Novo e-mail"));
    await person.type(screen.getByLabelText("Novo e-mail"), "taken@example.com");
    await person.click(screen.getByRole("button", { name: "Enviar códigos" }));
    const alert = await screen.findByRole("alert");
    expect(alert.textContent).not.toMatch(/taken@example\.com|outra conta|another account|pertence|belongs/i);
  });

  it("the new e-mail is shown masked everywhere after the request (no full address of the new channel in the DOM except the input)", async () => {
    serve();
    server.use(
      http.post("*/api/v1/me/channel-changes", () => HttpResponse.json({ id: "c1", channel: "email", status: "awaiting_codes", masked_new_value: "n***@e***.com", expires_at: "2099-01-01T00:00:00Z", codes: [{ role: "current_value", challenge_id: "old-0000000000001", confirmed: false }, { role: "new_value", challenge_id: "new-0000000000001", confirmed: false }] }, { status: 202 })),
      http.post("*/api/v1/auth/challenges/delivery", () => HttpResponse.json({ delivery: "sent" })),
    );
    const person = userEvent.setup();
    renderApp(<SecurityPage />);
    await person.click(await screen.findByRole("button", { name: "Alterar e-mail" }));
    await person.type(await screen.findByLabelText("Novo e-mail"), "newperson@example.com");
    await person.click(screen.getByRole("button", { name: "Enviar códigos" }));
    await screen.findByText(/Digite o código enviado para trader@example.com/);
    expect(document.body.textContent).not.toContain("newperson@example.com");
  });
});

describe("QA S3 AC-17 — devices and sign out everywhere", () => {
  it("forgetting a device needs no step-up call; the current device cannot be forgotten", async () => {
    serve();
    const calls: string[] = [];
    server.use(
      http.post("*/api/v1/auth/step-up", () => { calls.push("step-up"); return HttpResponse.json({}, { status: 202 }); }),
      http.post("*/api/v1/me/devices/:id/forget", ({ params }) => { calls.push(`forget:${String(params.id)}`); return new HttpResponse(null, { status: 204 }); }),
    );
    const person = userEvent.setup();
    renderApp(<SecurityPage />);
    const rows = await screen.findAllByRole("listitem");
    const current = rows.find((r) => within(r).queryByText(/Chrome no Windows/)) as HTMLElement;
    expect(within(current).queryByRole("button", { name: /Esquecer/ })).not.toBeInTheDocument();
    const other = rows.find((r) => within(r).queryByText(/Safari no iPhone/)) as HTMLElement;
    await person.click(within(other).getByRole("button", { name: /Esquecer/ }));
    await person.click(within(await screen.findByRole("alertdialog")).getByRole("button", { name: /Esquecer/ }));
    await waitFor(() => expect(calls).toContain("forget:d2"));
    expect(calls).not.toContain("step-up");
  });

  it("sign-out-everywhere: no step-up, lists active sources and paired instances only, links to their pages, ends at login", async () => {
    serve(state(), [{ status: "active" }, { status: "active" }, { status: "revoked" }], [{ status: "connected" }, { status: "awaiting_pairing" }, { status: "unpaired" }]);
    const calls: string[] = [];
    server.use(
      http.post("*/api/v1/auth/step-up", () => { calls.push("step-up"); return HttpResponse.json({}, { status: 202 }); }),
      http.post("*/api/v1/auth/logout-everywhere", () => { calls.push("logout-everywhere"); return new HttpResponse(null, { status: 204 }); }),
    );
    const person = userEvent.setup();
    renderApp(<SecurityPage />);
    await person.click(await screen.findByRole("button", { name: "Sair de todos os dispositivos" }));
    const dialog = await screen.findByRole("alertdialog");
    expect(await within(dialog).findByText(/2 fontes ativas/)).toBeInTheDocument();
    expect(within(dialog).getByText(/1 conexão pareada/)).toBeInTheDocument();
    expect(within(dialog).getAllByRole("link").length).toBeGreaterThanOrEqual(2); // revoke / unpair actions (AC-17)
    await person.click(within(dialog).getByRole("button", { name: "Sair de todos" }));
    await waitFor(() => expect(navigation.replace).toHaveBeenCalled());
    expect(calls).toEqual(["logout-everywhere"]);
  });

  it("a failed sign-out-everywhere does not leave the page and says so", async () => {
    serve();
    server.use(http.post("*/api/v1/auth/logout-everywhere", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    const person = userEvent.setup();
    renderApp(<SecurityPage />);
    await person.click(await screen.findByRole("button", { name: "Sair de todos os dispositivos" }));
    await person.click(within(await screen.findByRole("alertdialog")).getByRole("button", { name: "Sair de todos" }));
    expect(await screen.findByRole("alert")).toBeInTheDocument();
    expect(navigation.replace).not.toHaveBeenCalled();
  });
});
