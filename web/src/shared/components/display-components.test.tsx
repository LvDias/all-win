import { renderApp } from "@/test/render";
import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { useState } from "react";
import { server } from "@/test/server";
import { Combobox } from "@/shared/ui/combobox";
import { FirmRuleQuote } from "./firm-rule-quote";
import { PageHeader } from "./page-header";
import { StatusBadge } from "./status-badge";

const user = { id: "00000000-0000-4000-8000-000000000001", email: "a@b.co", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };

beforeEach(() => {
  server.use(http.get("*/api/v1/me", () => HttpResponse.json(user)));
});

describe("StatusBadge", () => {
  it("always carries text with its icon, and fills the time of a block", () => {
    renderApp(
      <>
        <StatusBadge kind="blocked" time="17:00 (GMT-3)" />
        <StatusBadge kind="outOfSync" />
        <StatusBadge kind="choicePending" />
      </>,
    );
    expect(screen.getByText("Bloqueada até 17:00 (GMT-3)")).toBeInTheDocument();
    expect(screen.getByText("Fora de sincronia")).toBeInTheDocument();
    expect(screen.getByText("Escolha pendente").querySelector("svg")).toHaveAttribute("aria-hidden", "true");
  });

  it("speaks English when the page does", () => {
    renderApp(<StatusBadge kind="acceptsAutomation" />, "en");
    expect(screen.getByText("Accepts automation")).toBeInTheDocument();
  });
});

describe("PageHeader", () => {
  it("renders the page's single h1 with description, actions and toggle", () => {
    renderApp(<PageHeader title="Contas" description="Suas contas." actions={<button type="button">Nova conta</button>} toggle={<label>Mostrar arquivadas</label>} />);
    expect(screen.getAllByRole("heading", { level: 1 })).toHaveLength(1);
    expect(screen.getByRole("heading", { level: 1, name: "Contas" })).toBeInTheDocument();
    expect(screen.getByText("Suas contas.")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Nova conta" })).toBeInTheDocument();
    expect(screen.getByText("Mostrar arquivadas")).toBeInTheDocument();
  });
});

describe("FirmRuleQuote", () => {
  const props = { firm: "Apex", pageTitle: "Rules", accessedAt: "2026-10-20T12:00:00Z", url: "https://firm.example/rules", lang: "en" };

  it("quotes the rule verbatim in the firm's language with source and an external link", () => {
    renderApp(<FirmRuleQuote {...props} text="Automated trading is prohibited." summary="A firma proíbe automação." />);
    const quote = screen.getByText("Automated trading is prohibited.");
    expect(quote.tagName).toBe("BLOCKQUOTE");
    expect(quote).toHaveAttribute("lang", "en");
    expect(quote).toHaveAttribute("cite", props.url);
    expect(screen.getByText(/Resumo do All Win \(não é aconselhamento jurídico\):/)).toBeInTheDocument();
    expect(screen.getByText(/Apex · Rules · acessado em 20\/10\/2026/)).toBeInTheDocument();
    const link = screen.getByRole("link", { name: /Ver página oficial/ });
    expect(link).toHaveAttribute("target", "_blank");
    expect(link).toHaveAttribute("rel", "noopener noreferrer");
  });

  it("renders only an https source as a link", () => {
    renderApp(<FirmRuleQuote {...props} url="javascript:alert(1)" text="Rule." />);
    expect(screen.queryByRole("link", { name: /Ver página oficial/ })).not.toBeInTheDocument();
    expect(screen.getByText("Rule.")).not.toHaveAttribute("cite");
  });

  it("keeps the whole text in the DOM and expands a long quote", async () => {
    const long = Array.from({ length: 12 }, (_, line) => `Line ${line + 1} of the rule.`).join("\n");
    renderApp(<FirmRuleQuote {...props} text={long} />);
    expect(screen.getByText(/Line 12 of the rule\./)).toBeInTheDocument();
    const toggle = screen.getByRole("button", { name: "Mostrar texto completo" });
    expect(toggle).toHaveAttribute("aria-expanded", "false");
    await userEvent.setup().click(toggle);
    expect(screen.getByRole("button", { name: "Mostrar menos" })).toHaveAttribute("aria-expanded", "true");
  });

  it("shows no toggle for a short quote", () => {
    renderApp(<FirmRuleQuote {...props} text="Short rule." />);
    expect(screen.queryByRole("button", { name: /Mostrar/ })).not.toBeInTheDocument();
  });
});

describe("Combobox", () => {
  const options = [
    { value: "apex", label: "Apex Trader Funding", group: "Firmas" },
    { value: "tpt", label: "Take Profit Trader", group: "Firmas" },
    { value: "topstep", label: "Topstep", group: "Firmas", keywords: "tsx" },
  ];
  function Harness({ onChange }: { onChange: (value: string) => void }) {
    const [value, setValue] = useState<string | null>(null);
    return (
      <Combobox
        id="firm"
        options={options}
        value={value}
        label="Firma"
        onChange={(next) => {
          setValue(next);
          onChange(next);
        }}
      />
    );
  }

  it("filters ignoring case, accents and keywords, and picks with the keyboard", async () => {
    const picked = vi.fn();
    renderApp(<Harness onChange={picked} />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("combobox", { name: "Firma: Escolher" }));
    await person.type(await screen.findByRole("combobox", { name: "Buscar: Firma" }), "TSX");
    const list = screen.getByRole("listbox", { name: "Firma" });
    expect(within(list).getAllByRole("option")).toHaveLength(1);
    await person.keyboard("{Enter}");
    expect(picked).toHaveBeenCalledWith("topstep");
    expect(screen.getByRole("combobox", { name: "Firma: Topstep" })).toBeInTheDocument();
  });

  it("builds option ids from the position, so values with spaces still make valid ids", async () => {
    function Spaced() {
      const [value, setValue] = useState<string | null>(null);
      return <Combobox id="firm" options={[{ value: "take profit", label: "Take Profit" }, { value: "top step", label: "Top Step" }]} value={value} label="Firma" onChange={setValue} />;
    }
    renderApp(<Spaced />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("combobox", { name: "Firma: Escolher" }));
    const search = await screen.findByRole("combobox", { name: "Buscar: Firma" });
    const active = search.getAttribute("aria-activedescendant") ?? "";
    expect(active).not.toMatch(/\s/);
    expect(document.getElementById(active)).toHaveAttribute("role", "option");
  });

  it("says when nothing matches", async () => {
    renderApp(<Harness onChange={() => {}} />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("combobox", { name: "Firma: Escolher" }));
    await person.type(await screen.findByRole("combobox", { name: "Buscar: Firma" }), "zzz");
    expect(screen.getByText("Nenhum resultado.")).toBeInTheDocument();
  });
});
