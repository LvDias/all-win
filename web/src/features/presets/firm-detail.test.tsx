import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { renderApp } from "@/test/render";
import { server } from "@/test/server";
import { FirmDetailPage } from "./firm-detail";
import { apexDetail, ids, topstep, unconfirmedDrawdown, version } from "./testing/fixtures";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "t@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
const firstQuote = "Automated trading is permitted.";

function serve(detail: typeof apexDetail = apexDetail) {
  server.use(
    http.get("*/api/v1/me", () => HttpResponse.json(user)),
    http.get("*/api/v1/firms/:id", () => HttpResponse.json(detail)),
    http.get("*/api/v1/preset-versions/:id", ({ params }) =>
      HttpResponse.json(params.id === ids.v50 ? version(ids.v50, "50000.00", { drawdown: unconfirmedDrawdown }) : version(ids.v100, "100000.00")),
    ),
    http.get("*/api/v1/firms/:id/releases", () =>
      HttpResponse.json({
        firm_id: ids.apex,
        releases: [
          { version_number: 2, published_at: "2026-09-01T00:00:00Z", verified_on: "2026-08-30", changes: [], vps: { value: "allowed", quotes: [] }, personal_device: { value: "unknown", quotes: [] }, rule_versions: [] },
          { version_number: 3, published_at: "2026-10-20T00:00:00Z", verified_on: "2026-10-18", changes: [{ kind: "stance_text_changed", stance: "automation", material: true, reason: { "pt-BR": "Nova exigência.", en: "New requirement." } }, { kind: "stance_changed", stance: "automation", from: "allowed", to: "not_confirmed" }, { kind: "stance_changed", stance: "vps", from: "forbidden", to: "future_value" }], vps: { value: "allowed", quotes: [] }, personal_device: { value: "unknown", quotes: [] }, rule_versions: [] },
        ],
      }),
    ),
  );
}

describe("FirmDetailPage", () => {
  it("shows the four stance blocks with verbatim quotes; conflicting quotes are all listed", async () => {
    serve();
    renderApp(<FirmDetailPage firmId={ids.apex} />);
    expect(await screen.findByRole("heading", { level: 1, name: "Apex" })).toBeInTheDocument();
    expect(screen.getAllByText(firstQuote).length).toBeGreaterThan(0);
    const copy = screen.getByRole("region", { name: "Replicação" });
    expect(within(copy).getByText("As páginas oficiais divergem:")).toBeInTheDocument();
    expect(within(copy).getByText("Copy trading is not allowed.")).toBeInTheDocument();
    expect(within(copy).getByText(firstQuote)).toBeInTheDocument();
    const device = screen.getByRole("region", { name: "Dispositivo pessoal" });
    expect(within(device).getByText(/Pesquisamos as páginas oficiais em 18\/10\/2026 e não encontramos uma regra\./)).toBeInTheDocument();
    expect(screen.getByRole("region", { name: "Servidor / VPS" })).toHaveTextContent("Permitido");
  });

  it("lists sizes with exact money, never choosing a value for a not-confirmed field", async () => {
    serve();
    renderApp(<FirmDetailPage firmId={ids.apex} />);
    await userEvent.click(await screen.findByRole("tab", { name: "Planos e regras" }));
    const row50 = (await screen.findByRole("rowheader", { name: "50K" })).closest("tr");
    if (!row50) throw new Error("row missing");
    expect(await within(row50).findByText("Não confirmado")).toBeInTheDocument();
    expect(within(row50).getByText(/US\$\s*2\.500,00 · Trailing fim do dia/)).toBeInTheDocument();
    expect(within(row50).getByText(/US\$\s*2\.000,00 · Estático/)).toBeInTheDocument();
    const row100 = screen.getByRole("rowheader", { name: "100K" }).closest("tr");
    if (!row100) throw new Error("row missing");
    expect(await within(row100).findByText(/US\$\s*2\.500,00 · Trailing fim do dia/)).toBeInTheDocument();
    expect(within(row100).queryByText("Não confirmado")).not.toBeInTheDocument();
    expect(within(row100).getByText("Sem limite")).toBeInTheDocument();
    expect(within(row100).getByText(/US\$\s*3\.000,00/)).toBeInTheDocument();
    expect(screen.getByText("Estes planos não estão disponíveis como preset: Legacy. Cadastre-os manualmente.")).toBeInTheDocument();
    expect(screen.getByRole("columnheader", { name: "Meta da avaliação" })).toBeInTheDocument();
  });

  it("shows the source popover for a verified value", async () => {
    serve();
    renderApp(<FirmDetailPage firmId={ids.apex} />);
    await userEvent.click(await screen.findByRole("tab", { name: "Planos e regras" }));
    const row100 = (await screen.findByRole("rowheader", { name: "100K" })).closest("tr");
    if (!row100) throw new Error("row missing");
    await within(row100).findByText(/Trailing fim do dia/);
    await userEvent.click(within(row100).getByRole("button", { name: "Fontes de Drawdown" }));
    const link = within(await screen.findByRole("dialog")).getByRole("link", { name: /Rules · acessado em 18\/10\/2026/ });
    expect(link).toHaveAttribute("href", "https://firm.example/rules");
    expect(link).toHaveAttribute("rel", "noopener noreferrer");
  });

  it("lists versions newest first and flags a material change", async () => {
    serve();
    renderApp(<FirmDetailPage firmId={ids.apex} />);
    await userEvent.click(await screen.findByRole("tab", { name: "Versões" }));
    const titles = await screen.findAllByRole("heading", { level: 3 });
    expect(titles.map((h) => h.textContent)).toEqual(["Versão 3", "Versão 2"]);
    const newest = titles[0]?.closest("li");
    expect(newest).toHaveTextContent("Mudança material");
    expect(newest).toHaveTextContent("Nova exigência.");
    // The contract sends stance codes; the sentence uses the badge words for that subject (feminine for automation, masculine for VPS).
    expect(newest).toHaveTextContent("A posição sobre automação mudou: Permitida → Não confirmada");
    expect(newest).not.toHaveTextContent("allowed");
    expect(newest).not.toHaveTextContent("not_confirmed");
    // An unknown code is shown as received rather than hidden.
    expect(newest).toHaveTextContent("Proibido → future_value");
  });

  it("explains a manual-only firm with the preset's own text", async () => {
    serve({ ...apexDetail, summary: topstep });
    renderApp(<FirmDetailPage firmId={ids.topstep} />);
    expect(await screen.findByText("Só acompanhamento manual")).toBeInTheDocument();
    expect(screen.getByText("Orders must originate from your personal device.")).toBeInTheDocument();
    expect(screen.getByText("Roteamento de sinais e replicação.")).toBeInTheDocument();
  });

  it("shows not found for an unknown firm", async () => {
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(user)), http.get("*/api/v1/firms/:id", () => HttpResponse.json({ error: { code: "common.not_found" } }, { status: 404 })));
    renderApp(<FirmDetailPage firmId={ids.apex} />);
    expect(await screen.findByText("Firma não encontrada.")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Tentar novamente" })).not.toBeInTheDocument();
  });
});
