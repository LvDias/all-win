// Shared test helpers: run context, masked leak checks, Caddy log structure checks, response assertions.
// Rule for every helper: no request target, token or secret is ever placed in a test title or error message.
import { expect } from '@playwright/test';
import { config, INGRESS_PREFIX } from './config.ts';
import { formatHits, fragmentTerms, M6O_CUTS, maskAll, parseToken, scan, secretTerms, type Hit, type SearchTerm, type Token } from './canary.ts';
import { JSON_HEADERS, send, type RawResponse } from './http.ts';
import { readFileSync } from 'node:fs';
import { composeFiles, loadState, pgDump, serviceLogs, type StackState } from './stack.ts';

export interface Ctx {
  s: StackState;
  a: Token;
  b: Token;
  terms: SearchTerm[];
  coreService: string;
}

let cached: Ctx | undefined;
export function ctx(): Ctx {
  if (cached) return cached;
  const s = loadState();
  const a = parseToken('a', s.tokens.a!);
  const b = parseToken('b', s.tokens.b!);
  const terms = [...secretTerms(a), ...fragmentTerms(a, M6O_CUTS), ...secretTerms(b), ...fragmentTerms(b, M6O_CUTS)];
  cached = { s, a, b, terms, coreService: config.mode === 'full' ? 'core' : 'fake-core' };
  return cached;
}

export const isFull = () => config.mode === 'full';

export function ingress(token: string): string {
  return `${INGRESS_PREFIX}${token}`;
}

/**
 * POST through the edge. A 429 is retried after its Retry-After (bounded) so functional tests are not
 * coupled to the rate-limit configuration; rate-limit tests pass `retry429: false`.
 */
export async function post(
  target: string,
  body: unknown,
  headers: Record<string, string> = JSON_HEADERS,
  opts: { retry429?: boolean; method?: string } = {},
): Promise<RawResponse> {
  const payload = body === undefined ? undefined : typeof body === 'string' || Buffer.isBuffer(body) ? body : JSON.stringify(body);
  for (let attempt = 0; ; attempt++) {
    const res = await send(ctx().s.origin, { method: opts.method ?? 'POST', target, headers, body: payload });
    if (res.status !== 429 || opts.retry429 === false || attempt >= 3) return res;
    await waitRetryAfter(res);
  }
}

/** ISO timestamp slightly in the past, for `docker compose logs --since`. */
export function mark(): string {
  return new Date(Date.now() - 1500).toISOString();
}

export async function settle(ms = 1500): Promise<void> {
  await new Promise((r) => setTimeout(r, ms));
}

export interface TextSource {
  name: string;
  text: string;
}

/** Caddy and Core container output since `since` (Core only in full mode). */
export function logSources(since?: string): TextSource[] {
  const c = ctx();
  const out: TextSource[] = [{ name: 'caddy', text: serviceLogs(c.s, 'caddy', since) }];
  if (isFull()) out.push({ name: 'core', text: serviceLogs(c.s, 'core', since) });
  return out;
}

export async function metricsSource(): Promise<TextSource | undefined> {
  if (!isFull() || !config.metricsPath) return undefined;
  const r = await send(ctx().s.coreOrigin, { method: 'GET', target: config.metricsPath });
  return { name: `metrics(${r.status})`, text: r.body };
}

export function responseSources(responses: Array<{ label: string; res: RawResponse }>): TextSource[] {
  return responses.map(({ label, res }) => ({
    name: `response:${label}`,
    text: `${res.status}\n${Object.entries(res.headers).map(([k, v]) => `${k}: ${String(v)}`).join('\n')}\n\n${res.body}`,
  }));
}

export function findLeaks(sources: TextSource[]): Hit[] {
  const terms = ctx().terms;
  return sources.flatMap((src) => scan(src.name, src.text, terms));
}

/** Fails with a masked report when any search term appears in any source. */
export function expectNoLeak(scenario: string, sources: TextSource[]): void {
  const hits = findLeaks(sources);
  expect(hits.length, `${scenario}: canary found in ${hits.length} line(s)\n${formatHits(hits)}`).toBe(0);
}

/** Every place §4.3 names for M1: Caddy + Core output, database (audit rows included), metrics, responses. */
export async function allSources(since: string | undefined, responses: Array<{ label: string; res: RawResponse }> = []): Promise<TextSource[]> {
  await settle();
  const sources = [...logSources(since), ...responseSources(responses)];
  if (isFull()) sources.push({ name: 'postgres:data', text: pgDump(ctx().s) });
  const m = await metricsSource();
  if (m) sources.push(m);
  return sources;
}

// ---- Caddy log structure (R2/R3) -------------------------------------------------------------------

/** True when the infra under test has the Caddy log-level switch and the suite asked for DEBUG. */
export function caddyDebugExpected(): boolean {
  const level = process.env.E2E_CADDY_LOG_LEVEL ?? 'DEBUG';
  const composeText = readFileSync(composeFiles()[0]!, 'utf8');
  return level.toUpperCase() === 'DEBUG' && composeText.includes('CADDY_LOG_LEVEL');
}

export interface StructureFinding {
  line: number;
  logger: string;
  issue: string;
}

function keyPaths(v: unknown, prefix = ''): string[] {
  if (v === null || typeof v !== 'object') return [];
  return Object.entries(v as Record<string, unknown>).flatMap(([k, child]) => {
    const p = prefix ? `${prefix}>${k}` : k;
    return [p, ...keyPaths(child, p)];
  });
}

/**
 * R2: no access line carries request>uri or a Referer header; R3: no other Caddy line carries request>uri or
 * request>headers. Any `uri` key at any depth is reported too. Only key paths are reported, never values.
 */
export function caddyStructureFindings(caddyLog: string): { findings: StructureFinding[]; accessLines: number; jsonLines: number; debugLines: number } {
  const findings: StructureFinding[] = [];
  let accessLines = 0;
  let jsonLines = 0;
  let debugLines = 0;
  caddyLog.split('\n').forEach((raw, i) => {
    const line = raw.trim();
    if (!line.startsWith('{')) return;
    let obj: Record<string, unknown>;
    try {
      obj = JSON.parse(line) as Record<string, unknown>;
    } catch {
      return;
    }
    jsonLines++;
    if (obj.level === 'debug') debugLines++;
    const logger = typeof obj.logger === 'string' ? obj.logger : '(none)';
    const isAccess = logger.startsWith('http.log.access');
    if (isAccess) accessLines++;
    const paths = keyPaths(obj);
    for (const p of paths) {
      const leaf = p.split('>').pop()!.toLowerCase();
      if (leaf === 'uri') findings.push({ line: i + 1, logger, issue: `key ${p}` });
      if (isAccess && /^request>headers>referer$/i.test(p)) findings.push({ line: i + 1, logger, issue: `key ${p}` });
      if (!isAccess && p === 'request>headers') findings.push({ line: i + 1, logger, issue: `key ${p}` });
    }
  });
  return { findings, accessLines, jsonLines, debugLines };
}

// ---- Responses -------------------------------------------------------------------------------------

/** Contract §2 step 2 / AC-2: 404 with an empty body (label only in messages). */
export function expectEmpty404(label: string, res: RawResponse): void {
  const masked = maskAll(res.body, ctx().terms).slice(0, 200);
  expect(res.status, `${label}: expected 404, got ${res.status} (body: ${JSON.stringify(masked)})`).toBe(404);
  expect(res.body.length, `${label}: 404 body must be empty`).toBe(0);
}

export interface Rejected {
  status: 'rejected';
  error: { code: string; pointer?: string };
}

export function parseJson(label: string, res: RawResponse): unknown {
  try {
    return JSON.parse(res.body);
  } catch {
    throw new Error(`${label}: response body is not JSON (status ${res.status})`);
  }
}

/** IngressRejected (OpenAPI): exactly status + error{code, pointer?}. */
export function expectRejected(label: string, res: RawResponse, status: number, code?: string): Rejected {
  const masked = maskAll(res.body, ctx().terms).slice(0, 300);
  expect(res.status, `${label}: status (body: ${masked})`).toBe(status);
  const body = parseJson(label, res) as Record<string, unknown>;
  expect(Object.keys(body).sort(), `${label}: top-level keys`).toEqual(['error', 'status']);
  expect(body.status).toBe('rejected');
  const error = body.error as Record<string, unknown>;
  expect(typeof error?.code, `${label}: error.code`).toBe('string');
  for (const k of Object.keys(error)) expect(['code', 'pointer'], `${label}: unexpected error key "${k}"`).toContain(k);
  if (code) expect(error.code, `${label}: error code`).toBe(code);
  return body as unknown as Rejected;
}

const UUID_RE = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

/** IngressAccepted (OpenAPI): exactly status + instruction_id (uuid). */
export function expectAccepted(label: string, res: RawResponse, status: 'accepted' | 'duplicate'): string {
  const masked = maskAll(res.body, ctx().terms).slice(0, 300);
  expect(res.status, `${label}: status (body: ${masked})`).toBe(200);
  const body = parseJson(label, res) as Record<string, unknown>;
  expect(Object.keys(body).sort(), `${label}: keys`).toEqual(['instruction_id', 'status']);
  expect(body.status, `${label}: status field`).toBe(status);
  expect(String(body.instruction_id), `${label}: instruction_id is a uuid`).toMatch(UUID_RE);
  return String(body.instruction_id);
}

/** Waits for a Retry-After window (bounded) so a rate-limit test does not bleed into the next one. */
export async function waitRetryAfter(res: RawResponse): Promise<void> {
  const raw = Number.parseInt(String(res.headers['retry-after'] ?? '1'), 10);
  const s = Math.min(Number.isFinite(raw) ? raw : 1, config.rateLimit.maxRetryAfterS);
  await settle((s + 1) * 1000);
}
