import { act, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { NextIntlClientProvider } from "next-intl";
import ptBR from "../../../messages/pt-BR.json";
import { SecretField } from "./secret-field";

const secret = "SECRET-VALUE-123";
function field(onCopied?: () => void) {
  return render(
    <NextIntlClientProvider locale="pt-BR" messages={ptBR} timeZone="UTC">
      <SecretField value={secret} maskedDisplay="https://host/••••••••" label="URL" onCopied={onCopied} />
    </NextIntlClientProvider>,
  );
}

describe("SecretField", () => {
  it("keeps the secret out of the DOM while masked", () => {
    const { container } = field();
    expect(container.innerHTML).not.toContain(secret);
    expect(screen.getByText("https://host/••••••••")).toBeInTheDocument();
  });

  it("reveals on demand with aria-pressed and masks again", async () => {
    const { container } = field();
    const person = userEvent.setup();
    const toggle = screen.getByRole("button", { name: "Revelar" });
    await person.click(toggle);
    expect(container.innerHTML).toContain(secret);
    expect(screen.getByRole("button", { name: "Ocultar" })).toHaveAttribute("aria-pressed", "true");
    await person.click(screen.getByRole("button", { name: "Ocultar" }));
    expect(container.innerHTML).not.toContain(secret);
  });

  it("copies without revealing and reports the copy", async () => {
    const onCopied = vi.fn();
    const { container } = field(onCopied);
    const person = userEvent.setup();
    await person.click(screen.getByRole("button", { name: "Copiar" }));
    expect(await navigator.clipboard.readText()).toBe(secret);
    expect(onCopied).toHaveBeenCalledOnce();
    expect(container.innerHTML).not.toContain(secret);
    expect(screen.getByRole("button", { name: "Copiado" })).toBeInTheDocument();
  });

  it("re-masks after 60 seconds and when the tab is hidden", async () => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    try {
      const { container } = field();
      const person = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
      await person.click(screen.getByRole("button", { name: "Revelar" }));
      expect(container.innerHTML).toContain(secret);
      act(() => vi.advanceTimersByTime(60_000));
      expect(container.innerHTML).not.toContain(secret);

      await person.click(screen.getByRole("button", { name: "Revelar" }));
      Object.defineProperty(document, "visibilityState", { value: "hidden", configurable: true });
      act(() => document.dispatchEvent(new Event("visibilitychange")));
      expect(container.innerHTML).not.toContain(secret);
    } finally {
      Object.defineProperty(document, "visibilityState", { value: "visible", configurable: true });
      vi.useRealTimers();
    }
  });
});
