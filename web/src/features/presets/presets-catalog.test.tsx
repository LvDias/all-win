import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
import { PresetsCatalog } from "./presets-catalog";
import { apex, topstep } from "./testing/fixtures";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "t@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
beforeEach(() => server.use(http.get("*/api/v1/me", () => HttpResponse.json(user))));
const firms = (list: unknown[]) => server.use(http.get("*/api/v1/firms", () => HttpResponse.json({ firms: list })));

describe("PresetsCatalog", () => {
  it("lists firms sorted by name with stances, manual-only badge and not-confirmed count", async () => {
    firms([topstep, apex]);
    renderApp(<PresetsCatalog />);
    const table = await screen.findByRole("table", { name: "Presets de firmas", hidden: true });
    const rows = within(table).getAllByRole("row", { hidden: true }).slice(1);
    if (!rows[0] || !rows[1]) throw new Error("rows missing");
    expect(rows[0]).toHaveTextContent("Apex");
    expect(rows[0]).toHaveTextContent("Plan A, Plan B +1");
    expect(rows[0]).toHaveTextContent("2 itens não confirmados");
    expect(rows[0]).toHaveTextContent("Permitida");
    expect(rows[0]).toHaveTextContent("Não confirmada");
    expect(rows[1]).toHaveTextContent("Só acompanhamento manual");
    expect(rows[1]).toHaveTextContent("Proibida");
    expect(within(rows[0]).getByRole("link", { name: "Apex", hidden: true })).toHaveAttribute("href", expect.stringMatching(/\/presets\/00000000-0000-4000-8000-000000000001$/));
  });

  it("filters by firm or plan name and offers to clear", async () => {
    firms([apex, topstep]);
    renderApp(<PresetsCatalog />);
    await screen.findByRole("table", { hidden: true });
    await userEvent.type(screen.getByRole("searchbox", { name: "Buscar firma ou plano" }), "combine");
    expect(screen.queryByText("Apex")).not.toBeInTheDocument();
    await userEvent.clear(screen.getByRole("searchbox"));
    await userEvent.type(screen.getByRole("searchbox"), "zzz");
    await userEvent.click(await screen.findByRole("button", { name: "Limpar busca" }));
    expect((await screen.findAllByText("Apex")).length).toBeGreaterThan(0);
  });

  it("shows the error with its code and a retry", async () => {
    server.use(http.get("*/api/v1/firms", () => HttpResponse.json({ error: { code: "common.internal" } }, { status: 500 })));
    renderApp(<PresetsCatalog />);
    expect(await screen.findByText("Código: common.internal")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Tentar novamente" })).toBeInTheDocument();
  });

  it("shows the empty catalog message", async () => {
    firms([]);
    renderApp(<PresetsCatalog />);
    expect(await screen.findByText("Os presets ainda não estão disponíveis.")).toBeInTheDocument();
  });
});
