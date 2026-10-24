import { screen, within } from "@testing-library/react";
import { renderApp } from "@/test/render";
import { StanceBlock, type StanceQuote } from "./stance-block";
import type { StanceValues } from "./stance-badge";

const quote = (text: string, url = "https://firm.example/rules"): StanceQuote => ({ text, language: "en", page_title: "Rules", url, accessed_at: "2026-10-18" });

const valid: { [S in keyof StanceValues]: Exclude<StanceValues[S], "varies_by_platform">[] } = {
  automation: ["allowed", "forbidden", "conditional", "unknown", "not_confirmed"],
  copy: ["allowed", "forbidden", "conditional", "unknown", "not_confirmed"],
  vps: ["allowed", "forbidden", "conditional", "unknown", "not_confirmed"],
  personalDevice: ["required", "not_required", "unknown", "not_confirmed"],
};

describe("StanceBlock", () => {
  it.each(Object.entries(valid).flatMap(([subject, values]) => values.map((value) => [subject, value] as const)))("has a fixed summary sentence for %s %s (text, not a key)", (subject, value) => {
    renderApp(<StanceBlock {...({ subject, value } as { subject: "automation"; value: "allowed" })} firm="Apex" quotes={[]} />);
    const summary = screen.getByText(/Resumo do All Win/).closest("p");
    expect(summary?.textContent ?? "").not.toMatch(/stanceBlock|summary\./);
    // Aquarela's "pages disagree on this point" sentence for the personal device names no firm.
    if (!(subject === "personalDevice" && value === "not_confirmed")) expect(summary?.textContent ?? "").toMatch(/Apex/);
    expect(summary?.textContent?.length ?? 0).toBeGreaterThan(50);
  });

  it("quotes the firm for a known position and lists every conflicting text when not confirmed", () => {
    renderApp(<StanceBlock subject="copy" value="not_confirmed" firm="Apex" quotes={[quote("Copying is banned."), quote("Copying is fine.", "https://firm.example/faq")]} verifiedOn="2026-10-18" />);
    const block = screen.getByRole("region", { name: "Replicação" });
    expect(within(block).getByText("As páginas oficiais divergem:")).toBeInTheDocument();
    expect(within(block).getByText("Copying is banned.")).toBeInTheDocument();
    expect(within(block).getByText("Copying is fine.")).toBeInTheDocument();
    expect(within(block).getByText("Verificado em 18/10/2026")).toBeInTheDocument();
  });

  it("shows the search date and no quote for an unknown position", () => {
    renderApp(<StanceBlock subject="vps" value="unknown" firm="Apex" quotes={[quote("ignored")]} searchedOn="2026-10-17" />);
    expect(screen.getByText("Pesquisamos as páginas oficiais em 17/10/2026 e não encontramos uma regra.")).toBeInTheDocument();
    expect(screen.queryByText("ignored")).not.toBeInTheDocument();
  });
});
