import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { ApiError, apiFetch, networkErrorCode } from "./client";
import { errorMessage } from "./error-message";

describe("apiFetch", () => {
  it("returns the JSON body", async () => {
    server.use(http.get("*/api/v1/x", () => HttpResponse.json({ ok: true })));
    await expect(apiFetch("/api/v1/x")).resolves.toEqual({ ok: true });
  });

  it("raises ApiError with code, pointer and retry-after from the error body", async () => {
    server.use(http.post("*/api/v1/x", () => HttpResponse.json({ error: { code: "auth.rate_limited", pointer: "/a", retry_after_s: 30 } }, { status: 429 })));
    const error = await apiFetch("/api/v1/x", { method: "POST", body: {} }).catch((e: unknown) => e);
    expect(error).toBeInstanceOf(ApiError);
    expect(error).toMatchObject({ code: "auth.rate_limited", status: 429, pointer: "/a", retryAfter: 30 });
  });

  it("falls back to the Retry-After header and to an http.<status> code for non-JSON errors", async () => {
    server.use(http.get("*/api/v1/x", () => new HttpResponse("bad gateway", { status: 502, headers: { "Retry-After": "7" } })));
    await expect(apiFetch("/api/v1/x")).rejects.toMatchObject({ code: "http.502", status: 502, retryAfter: 7 });
  });

  it("maps a network failure to a stable code", async () => {
    server.use(http.get("*/api/v1/x", () => HttpResponse.error()));
    await expect(apiFetch("/api/v1/x")).rejects.toMatchObject({ code: networkErrorCode, status: 0 });
  });
});

describe("errorMessage", () => {
  const messages: Record<string, string> = { "errors.auth.invalid_credentials": "Wrong", "errors.unknown": "Unexpected ({code})", "common.errors.network": "Offline", "common.errors.failed": "Failed" };
  const t = Object.assign((key: string, values?: Record<string, string>) => (messages[key] ?? key).replace("{code}", values?.code ?? ""), { has: (key: string) => key in messages });

  it("maps a known Core code 1:1 to errors.<code>", () => {
    expect(errorMessage(t, new ApiError({ code: "auth.invalid_credentials", status: 401 }))).toEqual({ message: "Wrong", code: "auth.invalid_credentials" });
  });

  it("shows the generic message with the code for unknown codes", () => {
    expect(errorMessage(t, new ApiError({ code: "x.new_code", status: 400 }))).toEqual({ message: "Unexpected (x.new_code)", code: "x.new_code" });
  });

  it("handles network and non-API errors", () => {
    expect(errorMessage(t, new ApiError({ code: networkErrorCode, status: 0 })).message).toBe("Offline");
    expect(errorMessage(t, new Error("boom")).message).toBe("Failed");
  });
});
