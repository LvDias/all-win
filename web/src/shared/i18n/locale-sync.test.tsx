import { waitFor } from "@testing-library/react";
import { http, HttpResponse } from "msw";
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { LocaleSync } from "./locale-sync";

const me = { id: "00000000-0000-4000-8000-000000000001", email: "a@b.co", locale: "en", time_zone: "UTC", is_platform_admin: false };

beforeEach(() => {
  navigation.replace.mockReset();
  window.history.replaceState(null, "", "/pt-BR/settings/preferences?x=1");
});

describe("LocaleSync", () => {
  it("moves a signed-in user to the profile language, keeping path and query", async () => {
    navigation.pathname = "/pt-BR/settings/preferences";
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(me)));
    renderApp(<LocaleSync />);
    await waitFor(() => expect(navigation.replace).toHaveBeenCalledWith("/en/settings/preferences?x=1", { scroll: false }));
  });

  it("leaves invite links in their embedded locale", async () => {
    navigation.pathname = "/pt-BR/invite";
    server.use(http.get("*/api/v1/me", () => HttpResponse.json(me)));
    renderApp(<LocaleSync />);
    await new Promise((resolve) => setTimeout(resolve, 50));
    expect(navigation.replace).not.toHaveBeenCalled();
  });

  it("does nothing when signed out", async () => {
    navigation.pathname = "/pt-BR";
    server.use(http.get("*/api/v1/me", () => HttpResponse.json({ error: { code: "auth.unauthenticated" } }, { status: 401 })));
    renderApp(<LocaleSync />);
    await new Promise((resolve) => setTimeout(resolve, 50));
    expect(navigation.replace).not.toHaveBeenCalled();
  });
});
