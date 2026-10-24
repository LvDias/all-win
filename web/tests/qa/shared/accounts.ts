// QA-owned test support for the accounts flows (does not import feature internals; contract types come from the generated schema).
// Mirrors the draft Core stand-in so the QA tests judge the UI against the contract, not against Frontend fixtures.
import { navigation, renderApp } from "@/test/render";
import { server } from "@/test/server";
import { http, HttpResponse } from "msw";
import type { ReactElement } from "react";
import type { StepUpAction, StepUpProof } from "@/shared/api/types";

export { navigation as nav };
export type StepUpRunner = <T>(request: { action: StepUpAction; target?: string; description: string; send: (proof?: StepUpProof) => Promise<T> }) => Promise<T>;

const me = { id: "00000000-0000-4000-8000-000000000001", email: "trader@example.com", locale: "pt-BR", time_zone: "UTC", is_platform_admin: false };
export function renderAccounts(ui: ReactElement, locale: "pt-BR" | "en" = "pt-BR") {
  server.use(http.get("/api/v1/me", () => HttpResponse.json(me)));
  return renderApp(ui, locale);
}

import type { components } from "@/shared/api/schema.gen";

type S = components["schemas"];
export type Account = S["Account"];
export type OptInRules = S["OptInRules"];
export type OptInState = S["OptInState"];
export type RuleVersion = S["RuleVersion"];
export type FirmStance = S["FirmStance"];
export type AccountCreateBody = S["AccountCreateRequest"];
export type AccountUpdateBody = S["AccountUpdateRequest"];

export const ids = {
  conn: "5a2b8c3d-1e4f-4a6b-8c7d-9e0f1a2b3c4d",
  acc1: "11111111-1111-4111-8111-111111111111",
  acc2: "22222222-2222-4222-8222-222222222222",
  ruleAuto: "1f2e3d4c-5b6a-4978-8a9b-0c1d2e3f4a5b",
  ruleAuto2: "2a3b4c5d-6e7f-4a8b-9c0d-1e2f3a4b5c6d",
  ruleCopy: "3b4c5d6e-7f8a-4b9c-8d1e-2f3a4b5c6d7e",
};

const notRequired = (kind: OptInState["kind"], stance?: OptInState["firm_stance"]): OptInState => ({ kind, state: "not_required", ...(stance ? { firm_stance: stance } : {}) });

export function realAccount(over: Partial<Account> = {}): Account {
  return {
    id: ids.acc2,
    kind: "real",
    bracket_mode: "individual",
    sync: { state: "in_sync", version: 1, observable: true, reasons: [], reasons_total: 0 },
    display_name: "Conta real",
    size_amount: "25000.00",
    currency: "USD",
    multiplier: 1,
    max_position_qty: 3,
    lifecycle_state: "active",
    opt_ins: { automation: notRequired("automation"), copy: notRequired("copy") },
    has_open_position: false,
    row_version: 1,
    created_at: "2026-10-20T12:00:00Z",
    updated_at: "2026-10-20T12:00:00Z",
    ...over,
  };
}

export function propAccount(over: Partial<Account> = {}): Account {
  return {
    ...realAccount(),
    id: ids.acc1,
    kind: "prop",
    display_name: "Apex 50K 1",
    firm_name: "Apex Trader Funding",
    plan_name: "50K Rithmic",
    phase: "Evaluation",
    size_amount: "50000.00",
    max_position_qty: 2,
    opt_ins: {
      automation: { kind: "automation", firm_stance: "forbidden", state: "chosen", choice: "declined", chosen_at: "2026-10-21T10:00:00Z", rule_version_id: ids.ruleAuto, current_rule_version_id: ids.ruleAuto },
      copy: notRequired("copy", "forbidden"),
    },
    ...over,
  };
}

export function ruleVersion(over: Partial<RuleVersion> = {}): RuleVersion {
  return {
    id: ids.ruleAuto,
    kind: "automation",
    firm_stance: "forbidden",
    is_generic: false,
    firm_id: "9c0d1e2f-3a4b-4c5d-8e6f-7a8b9c0d1e2f",
    firm_name: "Apex Trader Funding",
    version_number: 3,
    published_at: "2026-10-01T00:00:00Z",
    quotes: [{ text: "No Automation or Algorithm Usage allowed.", language: "en", page_title: "Prohibited Activities", url: "https://example.com/rules", accessed_at: "2026-10-23" }],
    ...over,
  };
}

export const genericRules: OptInRules = {
  automation: ruleVersion({ id: ids.ruleAuto, firm_stance: "unknown", is_generic: true, firm_id: undefined, firm_name: undefined, quotes: [] }),
  copy: ruleVersion({ id: ids.ruleCopy, kind: "copy", firm_stance: "unknown", is_generic: true, firm_id: undefined, firm_name: undefined, quotes: [] }),
};


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
