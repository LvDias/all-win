import { resetInUserZone } from "./zones";

describe("resetInUserZone", () => {
  const at = new Date("2026-10-24T12:00:00Z");
  it("shows the firm's reset on the user's clock with the offset (US Central in October is UTC-5, Sao Paulo UTC-3)", () => {
    const zones = resetInUserZone("17:00", "America/Chicago", "America/Sao_Paulo", "pt-BR", at);
    expect(zones?.user).toBe("19:00 GMT-3");
    expect(zones?.firm).toMatch(/^17:00/);
  });
  it("is null for an unknown zone or a malformed time instead of guessing", () => {
    expect(resetInUserZone("17:00", "Mars/Olympus", "UTC", "en", at)).toBeNull();
    expect(resetInUserZone("5pm", "America/Chicago", "UTC", "en", at)).toBeNull();
  });
});
