import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useState } from "react";
import { renderApp } from "@/test/render";
import { ConflictPicker } from "./conflict-picker";
import { StanceBadge } from "./stance-badge";

const options = [
  { id: "a", label: "US$ 2.500,00", sources: [{ url: "https://firm.example/a", accessed_at: "2026-10-18", page_title: "Rules" }] },
  { id: "b", label: "US$ 2.000,00", sources: [{ url: "javascript:alert(1)", accessed_at: "2026-10-17", page_title: "Bad" }] },
];

function Harness() {
  const [value, setValue] = useState("");
  return <ConflictPicker legend="Drawdown" options={options} value={value} onValueChange={setValue} otherInput={<input aria-label="Outro valor digitado" />} error={value === "" ? "Resolva este valor para salvar." : undefined} />;
}

describe("ConflictPicker", () => {
  it("preselects nothing, links sources safely and reveals the input for another value", async () => {
    renderApp(<Harness />);
    for (const radio of screen.getAllByRole("radio")) expect(radio).not.toBeChecked();
    expect(screen.getByText("Resolva este valor para salvar.")).toBeInTheDocument();
    expect(screen.getByRole("link", { name: /Rules · acessado em 18\/10\/2026/ })).toHaveAttribute("href", "https://firm.example/a");
    expect(screen.queryByRole("link", { name: /Bad/ })).not.toBeInTheDocument();
    expect(screen.getByText(/Bad · acessado em 17\/10\/2026/)).toBeInTheDocument();
    expect(screen.queryByLabelText("Outro valor digitado")).not.toBeInTheDocument();
    await userEvent.click(screen.getByRole("radio", { name: "Outro valor" }));
    expect(screen.getByLabelText("Outro valor digitado")).toBeInTheDocument();
    expect(screen.queryByText("Resolva este valor para salvar.")).not.toBeInTheDocument();
  });
});

describe("StanceBadge", () => {
  it("uses subject-specific wording with text and an icon", () => {
    renderApp(
      <>
        <StanceBadge subject="automation" value="allowed" />
        <StanceBadge subject="vps" value="allowed" />
        <StanceBadge subject="personalDevice" value="not_confirmed" />
        <StanceBadge subject="vps" value="not_confirmed" />
      </>,
    );
    expect(screen.getByText("Permitida")).toBeInTheDocument();
    expect(screen.getByText("Permitido")).toBeInTheDocument();
    expect(screen.getByText("Desconhecido")).toBeInTheDocument();
    expect(screen.getByText("Não confirmado")).toBeInTheDocument();
  });
});

describe("ConflictPicker sources without a title", () => {
  it("falls back to the URL for a missing or empty title and shows the note beside it", () => {
    renderApp(
      <ConflictPicker
        legend="Drawdown"
        options={[{ id: "a", label: "US$ 2.500,00", sources: [{ url: "https://firm.example/x", accessed_at: "2026-10-18", page_title: "", title_note: "Página sem título" }] }]}
        value=""
        onValueChange={() => undefined}
      />,
    );
    expect(screen.getByRole("link", { name: /https:\/\/firm\.example\/x · acessado em 18\/10\/2026/ })).toBeInTheDocument();
    expect(screen.getByText(/Página sem título/)).toBeInTheDocument();
  });
});
