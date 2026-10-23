import { screen } from "@testing-library/react";
import { http, HttpResponse } from "msw";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
import { GettingStarted } from "./getting-started";

const source = { id: "1", name: "S", source_type: "tradingview", is_automated: true, status: "active", key_id: "AbCdEfGhIjKl", masked_url: "x", created_at: "2026-10-20T12:00:00Z" };
const connection = (status: string) => ({ id: "2", name: "C", status, status_changed_at: "2026-10-20T12:00:00Z", created_at: "2026-10-20T12:00:00Z" });

function api(sources: unknown[], connections: unknown[]) {
  server.use(http.get("*/api/v1/sources", () => HttpResponse.json(sources)), http.get("*/api/v1/connections", () => HttpResponse.json(connections)));
}

describe("GettingStarted", () => {
  it("marks nothing done for a new user and links each step", async () => {
    api([], []);
    renderApp(<GettingStarted />);
    expect(await screen.findByRole("link", { name: "1. Crie uma fonte de sinal" })).toHaveAttribute("href", "/pt-BR/connections/sources");
    expect(screen.getByRole("link", { name: "2. Pareie o NinjaTrader" })).toHaveAttribute("href", "/pt-BR/connections/ninjatrader");
    expect(screen.getAllByText("Pendente")).toHaveLength(2);
  });

  it("derives done from the list counts; a connection still awaiting pairing is not done", async () => {
    api([source], [connection("awaiting_pairing")]);
    renderApp(<GettingStarted />);
    await screen.findByRole("link", { name: "1. Crie uma fonte de sinal" });
    expect(screen.getAllByText("Concluído")).toHaveLength(1);
    expect(screen.getAllByText("Pendente")).toHaveLength(1);
  });

  it("marks both done once a connection is paired, and keeps the guides inactive", async () => {
    api([source], [connection("connected")]);
    renderApp(<GettingStarted />);
    await screen.findByRole("link", { name: "1. Crie uma fonte de sinal" });
    expect(screen.getAllByText("Concluído")).toHaveLength(2);
    expect(screen.getByText(/Guia de alertas do TradingView · em breve/)).toHaveAttribute("aria-disabled", "true");
  });
});
