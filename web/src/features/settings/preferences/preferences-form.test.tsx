import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
import { PreferencesForm } from "./preferences-form";

const me = { id: "00000000-0000-4000-8000-000000000001", email: "a@b.co", locale: "pt-BR", time_zone: "America/Sao_Paulo", is_platform_admin: false };

const originalResolvedOptions = Intl.DateTimeFormat.prototype.resolvedOptions;

function mockBrowserZone(timeZone: string) {
  vi.spyOn(Intl.DateTimeFormat.prototype, "resolvedOptions").mockImplementation(function (this: Intl.DateTimeFormat) {
    return { ...originalResolvedOptions.call(this), timeZone };
  });
}

beforeEach(() => mockBrowserZone("America/Sao_Paulo"));

describe("PreferencesForm", () => {
  it("shows a skeleton while loading, then the fields", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(me)));
    renderApp(<PreferencesForm />);
    expect(screen.getByRole("status")).toHaveAttribute("aria-busy", "true");
    expect(await screen.findByLabelText("Idioma da interface")).toBeInTheDocument();
    expect(screen.getByText("Também usado pelo plugin do NinjaTrader a partir da próxima conexão.")).toBeInTheDocument();
    expect(screen.getByRole("combobox", { name: "Fuso horário" })).toHaveTextContent("(GMT-03:00) Sao Paulo");
  });

  it("shows ErrorState with the code and retries", async () => {
    let calls = 0;
    server.use(
      http.get("*/api/v1/me", () => {
        calls += 1;
        return calls === 1 ? HttpResponse.json({ error: { code: "x.broken" } }, { status: 500 }) : HttpResponse.json(me);
      }),
    );
    const user = userEvent.setup();
    renderApp(<PreferencesForm />);
    expect(await screen.findByRole("alert")).toHaveTextContent("Não foi possível carregar as preferências.");
    expect(screen.getByText("Código: x.broken")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Tentar novamente" }));
    expect(await screen.findByLabelText("Idioma da interface")).toBeInTheDocument();
  });

  it("asks to sign in when there is no session", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json({ error: { code: "auth.unauthenticated" } }, { status: 401 })));
    renderApp(<PreferencesForm />);
    expect(await screen.findByText("Entre na sua conta para alterar as preferências.")).toBeInTheDocument();
  });

  it("saves a chosen time zone and confirms with a toast", async () => {
    const bodies: unknown[] = [];
    server.use(
      http.get("*/api/v1/me", () => HttpResponse.json(me)),
      http.patch("*/api/v1/me", async ({ request }) => {
        const body = await request.json();
        bodies.push(body);
        return HttpResponse.json({ ...me, ...(body as object) });
      }),
    );
    const user = userEvent.setup();
    renderApp(<PreferencesForm />);
    await user.click(await screen.findByRole("combobox", { name: "Fuso horário" }));
    await user.type(await screen.findByRole("combobox", { name: "Buscar fuso horário" }), "chicago");
    await user.click(await screen.findByRole("option", { name: /Chicago/ }));
    expect(await screen.findByText("Preferência salva.")).toBeInTheDocument();
    expect(bodies).toEqual([{ time_zone: "America/Chicago" }]);
  }, 20_000);

  it("shows me.invalid_time_zone inline", async () => {
    server.use(
      http.get("*/api/v1/me", () => HttpResponse.json(me)),
      http.patch("*/api/v1/me", () => HttpResponse.json({ error: { code: "me.invalid_time_zone" } }, { status: 422 })),
    );
    const user = userEvent.setup();
    renderApp(<PreferencesForm />);
    await user.click(await screen.findByRole("combobox", { name: "Fuso horário" }));
    await user.type(await screen.findByRole("combobox", { name: "Buscar fuso horário" }), "paris{Enter}");
    const alert = await screen.findByText("Fuso horário inválido. Escolha outro.");
    expect(alert).toHaveAttribute("role", "alert");
    expect(screen.getByRole("combobox", { name: "Fuso horário" })).toBeInvalid();
  });

  it("offers to use the browser zone when it differs from the profile", async () => {
    mockBrowserZone("Europe/Paris");
    const bodies: unknown[] = [];
    server.use(
      http.get("*/api/v1/me", () => HttpResponse.json(me)),
      http.patch("*/api/v1/me", async ({ request }) => {
        const body = await request.json();
        bodies.push(body);
        return HttpResponse.json({ ...me, ...(body as object) });
      }),
    );
    const user = userEvent.setup();
    renderApp(<PreferencesForm />);
    const note = await screen.findByText("Seu navegador está em Europe/Paris. Usar este fuso?");
    await user.click(within(note.parentElement as HTMLElement).getByRole("button", { name: "Usar" }));
    await waitFor(() => expect(bodies).toEqual([{ time_zone: "Europe/Paris" }]));
  });
});
