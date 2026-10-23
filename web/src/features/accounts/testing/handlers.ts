import { http, HttpResponse } from "msw";
import type { Account, AccountCreateBody, AccountUpdateBody, OptInRules } from "../api/types";
import { genericRules, propAccount, realAccount } from "./fixtures";

export const apiError = (status: number, code: string, extra: Record<string, unknown> = {}) =>
  HttpResponse.json({ error: { code, ...extra } }, { status });

/** In-memory Core stand-in following the draft contract 2.1.0; state is per call of `accountHandlers`. */
export function accountHandlers(initial: Account[] = [propAccount(), realAccount()], rules: OptInRules = genericRules) {
  const accounts = new Map(initial.map((a) => [a.id, a]));
  const seenKeys = new Map<string, Account>();
  const seenBodies = new Map<string, string>();
  const calls = { create: [] as { body: AccountCreateBody; key: string | null }[], update: [] as AccountUpdateBody[], lifecycle: [] as unknown[], optIn: [] as unknown[] };

  const handlers = [
    http.get("/api/v1/accounts", () => HttpResponse.json({ accounts: [...accounts.values()] })),
    http.post("/api/v1/accounts", async ({ request }) => {
      const body = (await request.json()) as AccountCreateBody;
      const key = request.headers.get("Idempotency-Key");
      calls.create.push({ body, key });
      if (key && !seenBodies.has(key)) seenBodies.set(key, JSON.stringify({ ...body, step_up: undefined }));
      const replay = key ? seenKeys.get(key) : undefined;
      if (!replay && key && seenBodies.has(key) && seenBodies.get(key) !== JSON.stringify({ ...body, step_up: undefined })) return apiError(409, "accounts.idempotency_key_reused");
      if (replay) return HttpResponse.json(replay, { status: 200 });
      if ([...accounts.values()].some((a) => a.display_name.toLowerCase() === body.display_name.toLowerCase()))
        return apiError(409, "accounts.name_taken", { pointer: "/display_name" });
      const created: Account = {
        ...realAccount({ id: crypto.randomUUID(), kind: body.kind, display_name: body.display_name, size_amount: body.size_amount?.includes(".") ? (body.size_amount as string) : `${body.size_amount}.00`, max_position_qty: body.max_position_qty, multiplier: body.multiplier ?? 1 }),
        ...(body.kind === "prop" ? { firm_name: body.firm_name, plan_name: body.plan_name, phase: body.phase } : {}),
      };
      accounts.set(created.id, created);
      if (key) seenKeys.set(key, created);
      return HttpResponse.json(created, { status: 201 });
    }),
    http.get("/api/v1/accounts/:id", ({ params }) => {
      const a = accounts.get(String(params.id));
      return a ? HttpResponse.json(a) : apiError(404, "common.not_found");
    }),
    http.put("/api/v1/accounts/:id", async ({ params, request }) => {
      const a = accounts.get(String(params.id));
      if (!a) return apiError(404, "common.not_found");
      const body = (await request.json()) as AccountUpdateBody;
      calls.update.push(body);
      if (body.row_version !== a.row_version) return apiError(409, "common.version_conflict");
      const next = { ...a, display_name: body.display_name, max_position_qty: body.max_position_qty, multiplier: body.multiplier ?? 1, row_version: a.row_version + 1 };
      accounts.set(a.id, next);
      return HttpResponse.json(next);
    }),
    http.delete("/api/v1/accounts/:id", ({ params }) => {
      const a = accounts.get(String(params.id));
      if (!a) return apiError(404, "common.not_found");
      if (a.has_open_position) return apiError(409, "accounts.delete_open_position");
      accounts.delete(a.id);
      return new HttpResponse(null, { status: 204 });
    }),
    http.get("/api/v1/accounts/:id/archive-impact", ({ params }) => {
      const a = accounts.get(String(params.id));
      return a ? HttpResponse.json({ has_open_position: a.has_open_position, routes: [], follower_links: [] }) : apiError(404, "common.not_found");
    }),
    http.put("/api/v1/accounts/:id/lifecycle", async ({ params, request }) => {
      const a = accounts.get(String(params.id));
      if (!a) return apiError(404, "common.not_found");
      const body = (await request.json()) as { state: Account["lifecycle_state"]; row_version: number };
      calls.lifecycle.push(body);
      if (body.row_version !== a.row_version) return apiError(409, "common.version_conflict");
      const next = { ...a, lifecycle_state: body.state, row_version: a.row_version + 1 };
      accounts.set(a.id, next);
      return HttpResponse.json(next);
    }),
    http.get("/api/v1/opt-in-rules", () => HttpResponse.json(rules)),
  ];
  return { handlers, accounts, calls };
}
