import { screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { http, HttpResponse } from "msw";
import { useState } from "react";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { LanguageSwitcher } from "./language-switcher";
import { useUnsavedChanges, useViewOnceLock } from "./locale-switch";

const me = { id: "00000000-0000-4000-8000-000000000001", email: "a@b.co", locale: "pt-BR", time_zone: "America/Sao_Paulo", is_platform_admin: false };

function signedIn() {
  server.use(http.get("*/api/v1/me", () => HttpResponse.json(me)));
}
function signedOut() {
  server.use(http.get("*/api/v1/me", () => HttpResponse.json({ error: { code: "auth.unauthenticated" } }, { status: 401 })));
}

function Dirty() {
  useUnsavedChanges(true);
  return null;
}
function ViewOnce() {
  const [open, setOpen] = useState(true);
  useViewOnceLock(open);
  return (
    <button type="button" onClick={() => setOpen(false)}>
      close-secret
    </button>
  );
}

beforeEach(() => {
  navigation.replace.mockReset();
  document.cookie = "NEXT_LOCALE=; Max-Age=0; Path=/";
  window.history.replaceState(null, "", "/pt-BR/settings/preferences?tab=1#top");
});

async function openMenuAndPick(name: string) {
  const user = userEvent.setup();
  await user.click(await screen.findByRole("button", { name: /Idioma: Português/ }));
  await user.click(await screen.findByRole("menuitemradio", { name }));
  return user;
}

describe("LanguageSwitcher", () => {
  it("exposes the current language in the accessible name and lang on options", async () => {
    signedOut();
    const user = userEvent.setup();
    renderApp(<LanguageSwitcher />);
    await user.click(screen.getByRole("button", { name: "Idioma: Português (Brasil)" }));
    expect(await screen.findByRole("menuitemradio", { name: "English" })).toHaveAttribute("lang", "en");
    expect(screen.getByRole("menuitemradio", { name: "Português (Brasil)" })).toHaveAttribute("lang", "pt-BR");
  });

  it("signed out: sets the cookie and navigates keeping query and hash, without calling the API", async () => {
    signedOut();
    const patch = vi.fn();
    server.use(http.patch("*/api/v1/me", () => { patch(); return HttpResponse.json(me); }));
    renderApp(<LanguageSwitcher />);
    await openMenuAndPick("English");
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/en/settings/preferences?tab=1#top", { scroll: false }));
    expect(document.cookie).toContain("NEXT_LOCALE=en");
    expect(patch).not.toHaveBeenCalled();
  });

  it("signed in: saves the profile language before navigating", async () => {
    signedIn();
    const bodies: unknown[] = [];
    server.use(http.patch("*/api/v1/me", async ({ request }) => { bodies.push(await request.json()); return HttpResponse.json({ ...me, locale: "en" }); }));
    renderApp(<LanguageSwitcher />);
    await openMenuAndPick("English");
    await waitFor(() => expect(navigation.replace).toHaveBeenCalled());
    expect(bodies).toEqual([{ locale: "en" }]);
  });

  it("signed in: on failure stays put, shows a toast with the code and does not set the cookie", async () => {
    signedIn();
    server.use(http.patch("*/api/v1/me", () => HttpResponse.json({ error: { code: "auth.rate_limited" } }, { status: 429 })));
    renderApp(<LanguageSwitcher />);
    await openMenuAndPick("English");
    expect(await screen.findByText("Não foi possível salvar o idioma. Tente de novo.")).toBeInTheDocument();
    expect(screen.getByText(/auth\.rate_limited/)).toBeInTheDocument();
    expect(navigation.replace).not.toHaveBeenCalled();
    expect(document.cookie).not.toContain("NEXT_LOCALE=en");
  });

  it("asks for confirmation when a form is dirty, and cancelling changes nothing", async () => {
    signedOut();
    renderApp(<><Dirty /><LanguageSwitcher /></>);
    const user = await openMenuAndPick("English");
    expect(await screen.findByText("Trocar o idioma recarrega a página e descarta as alterações não salvas.")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Continuar editando" }));
    expect(navigation.replace).not.toHaveBeenCalled();
  });

  it("switches after confirming the dirty-form dialog", async () => {
    signedOut();
    renderApp(<><Dirty /><LanguageSwitcher /></>);
    const user = await openMenuAndPick("English");
    await user.click(await screen.findByRole("button", { name: "Trocar idioma" }));
    await waitFor(() => expect(navigation.replace).toHaveBeenCalled());
  });

  it("is disabled while a view-once dialog is open and enabled after it closes", async () => {
    signedOut();
    const user = userEvent.setup();
    renderApp(<><ViewOnce /><LanguageSwitcher /></>);
    const trigger = await screen.findByRole("button", { name: /Idioma: Português/ });
    expect(trigger).toBeDisabled();
    await user.click(screen.getByRole("button", { name: "close-secret" }));
    await waitFor(() => expect(screen.getByRole("button", { name: /Idioma: Português/ })).toBeEnabled());
  });
});
