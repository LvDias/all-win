// V1-S1 masking gate — contract docs/architecture/contracts/s1-webhook-ingest.md §4 (1.1.0), S1 AC-6.
// Search terms: each canary secret, its 28 sixteen-character windows and the M6-o pieces of >= 8 characters.
// Titles and messages never contain a request target; variants are named by their contract letter.
import { test, expect } from '@playwright/test';
import { config } from '../../src/config.ts';
import { formatHits, type Token } from '../../src/canary.ts';
import { JSON_HEADERS } from '../../src/http.ts';
import { asTradingView, loadFixtures, uniqueMessageId, validInstruction } from '../../src/instructions.ts';
import {
  allSources,
  caddyDebugExpected,
  caddyStructureFindings,
  ctx,
  expectAccepted,
  expectEmpty404,
  expectNoLeak,
  expectRejected,
  findLeaks,
  ingress,
  isFull,
  logSources,
  mark,
  post,
} from '../../src/harness.ts';
import { psql, lit, serviceLogs, shell, composeShellPrefix } from '../../src/stack.ts';

interface Variant {
  id: string;
  method: string;
  target: (t: Token) => string;
  /** a–m must get the empty 404; n and o are non-ingress paths (R2: absence only). */
  empty404: boolean;
}

function pct(ch: string): string {
  return `%${ch.charCodeAt(0).toString(16).toUpperCase().padStart(2, '0')}`;
}

const V: Variant[] = [
  { id: 'M6-a extra character', method: 'POST', empty404: true, target: (t) => ingress(`${t.value}X`) },
  { id: 'M6-b truncated secret', method: 'POST', empty404: true, target: (t) => ingress(`awk_${t.keyId}_${t.secret.slice(0, 30)}`) },
  { id: 'M6-c double slash before token', method: 'POST', empty404: true, target: (t) => `/ingress/v1/tradingview//${t.value}` },
  { id: 'M6-d leading double slash', method: 'POST', empty404: true, target: (t) => `//ingress/v1/tradingview/${t.value}` },
  { id: 'M6-e mixed case', method: 'POST', empty404: true, target: (t) => `/Ingress/v1/TradingView/${t.value}` },
  { id: 'M6-e upper case', method: 'POST', empty404: true, target: (t) => `/INGRESS/V1/TRADINGVIEW/${t.value}` },
  { id: 'M6-f %5F separator', method: 'POST', empty404: true, target: (t) => ingress(`awk_${t.keyId}%5F${t.secret}`) },
  { id: 'M6-g %2B inside secret', method: 'POST', empty404: true, target: (t) => ingress(`awk_${t.keyId}_${t.secret.slice(0, 20)}%2B${t.secret.slice(20)}`) },
  {
    id: 'M6-h one secret character percent-encoded',
    method: 'POST',
    empty404: true,
    target: (t) => ingress(`awk_${t.keyId}_${t.secret.slice(0, 21)}${pct(t.secret[21]!)}${t.secret.slice(22)}`),
  },
  { id: 'M6-i bare secret', method: 'POST', empty404: true, target: (t) => ingress(t.secret) },
  { id: 'M6-j token in query', method: 'POST', empty404: true, target: (t) => `/ingress/v1/tradingview?t=${t.value}` },
  { id: 'M6-k %69ngress', method: 'POST', empty404: true, target: (t) => `/%69ngress/v1/tradingview/${t.value}` },
  { id: 'M6-l trailing slash', method: 'POST', empty404: true, target: (t) => `${ingress(t.value)}/` },
  { id: 'M6-m GET canonical', method: 'GET', empty404: true, target: (t) => ingress(t.value) },
  { id: 'M6-n token in /api path', method: 'POST', empty404: false, target: (t) => `/api/v1/${t.value}` },
  { id: 'M6-o split in /x path', method: 'GET', empty404: false, target: (t) => `/x/awk_${t.keyId}/${t.secret.slice(0, 20)}/${t.secret.slice(20)}` },
  {
    id: 'M6-o split in /pt-BR path + query',
    method: 'GET',
    empty404: false,
    target: (t) => `/pt-BR/${t.secret.slice(0, 10)}.${t.secret.slice(10, 20)}/${t.secret.slice(20)}?q=${t.secret}`,
  },
];

async function auditRowsFor(messageId: string): Promise<string[]> {
  const sql = `SELECT row_to_json(x)::text FROM ${config.sql.audit} x WHERE row_to_json(x)::text LIKE ${lit(`%${messageId}%`)};`;
  return psql(ctx().s, sql).split('\n').filter(Boolean);
}

async function waitAudit(messageId: string, predicate: (rows: string[]) => boolean): Promise<string[]> {
  const deadline = Date.now() + config.auditWaitMs;
  let rows: string[] = [];
  while (Date.now() < deadline) {
    rows = await auditRowsFor(messageId);
    if (predicate(rows)) return rows;
    await new Promise((r) => setTimeout(r, 500));
  }
  return rows;
}

test.describe('S1 masking (contract §4.3)', () => {
  // Stable per run: a worker restarted after a failure must post the same message in M2.
  const m1Body = validInstruction({ message_id: `e2e-m1-${config.project}` });

  test('control: log capture sees request data (absence checks are not vacuous)', async () => {
    // A non-secret marker in a header Caddy's access log keeps (User-Agent) must be found in the captured output.
    const marker = `e2e-control-${uniqueMessageId('m')}`;
    const since = mark();
    await post('/control-probe', undefined, { 'user-agent': marker }, { method: 'GET' });
    const caddy = (await allSources(since)).find((x) => x.name === 'caddy')!.text;
    expect(caddy.includes(marker), 'control: marker not found in captured Caddy output').toBe(true);
  });

  test('M1 accepted instruction', async () => {
    const { a } = ctx();
    const since = mark();
    const res = await post(ingress(a.value), m1Body);
    if (isFull()) {
      expectAccepted('M1', res, 'accepted');
      const rows = await waitAudit(String(m1Body.message_id), (r) => r.some((x) => x.includes('ingress.received')));
      expect(rows.some((x) => x.includes('ingress.received')), 'M1: audit entry "ingress.received" for the message').toBe(true);
      expect(rows.some((x) => x.includes(a.keyId)), 'M1: audit entry carries the key_id').toBe(true);
    }
    const sources = await allSources(since, [{ label: 'M1', res }]);
    if (isFull()) {
      // §4.2: the Core writes the ingress request log with the key_id; also the Core-side capture control.
      expect(sources.find((x) => x.name === 'core')!.text.includes(a.keyId), 'M1: Core request log carries the key_id').toBe(true);
    }
    expectNoLeak('M1', sources);
  });

  test('M2 duplicate', async () => {
    const { a } = ctx();
    const since = mark();
    const res = await post(ingress(a.value), m1Body);
    if (isFull()) expectAccepted('M2', res, 'duplicate');
    expectNoLeak('M2', await allSources(since, [{ label: 'M2', res }]));
  });

  test('M3 schema rejection', async () => {
    const { a } = ctx();
    const since = mark();
    const res = await post(ingress(a.value), validInstruction({ quantity: 0 }));
    if (isFull()) {
      const body = expectRejected('M3', res, 400, 'ingress.schema_violation');
      expect(body.error.pointer, 'M3: pointer').toBe('/quantity');
    }
    expectNoLeak('M3', await allSources(since, [{ label: 'M3', res }]));
  });

  test('M4 policy rejection (action modify from TradingView)', async () => {
    const { a } = ctx();
    const modify = asTradingView(loadFixtures('valid').find((f) => f.name === 'master-modify-entry-quantity')!.json);
    const since = mark();
    const res = await post(ingress(a.value), modify);
    if (isFull()) expectRejected('M4', res, 422, 'ingress.action_not_allowed_for_source');
    expectNoLeak('M4', await allSources(since, [{ label: 'M4', res }]));
  });

  test('M3b credential guard: another canary token inside the body is neither logged nor stored', async () => {
    const { a, b } = ctx();
    const since = mark();
    const res = await post(ingress(a.value), validInstruction({ details: { pasted: b.value } }));
    if (isFull()) {
      const r = expectRejected('M3b', res, 400, 'ingress.schema_violation');
      expect(r.error.pointer, 'M3b: pointer must be empty').toBe('');
    }
    expectNoLeak('M3b', await allSources(since, [{ label: 'M3b', res }]));
  });

  for (const v of V) {
    test(v.id, async () => {
      const { a } = ctx();
      const since = mark();
      const res = await post(v.target(a), v.method === 'GET' ? undefined : validInstruction(), v.method === 'GET' ? {} : JSON_HEADERS, {
        method: v.method,
      });
      // Assert absence first: it is the gate, and it stays meaningful even when routing is wrong.
      expectNoLeak(v.id, await allSources(since, [{ label: v.id, res }]));
      if (v.empty404) expectEmpty404(v.id, res);
    });
  }

  test('M7 oversized body and wrong content type', async () => {
    const { a } = ctx();
    const since = mark();
    const big = validInstruction({ details: { pad: 'x'.repeat(20 * 1024) } });
    const tooBig = await post(ingress(a.value), big);
    const xml = await post(ingress(a.value), '<instruction/>', { 'content-type': 'application/xml' });
    // Above Caddy's 64 KB transport backstop: answered by Caddy, its logs must stay clean too.
    const huge = await post(ingress(a.value), validInstruction({ details: { pad: 'x'.repeat(70 * 1024) } }));
    if (isFull()) {
      expectRejected('M7 20 KiB', tooBig, 413, 'ingress.payload_too_large');
      expectRejected('M7 xml', xml, 415, 'ingress.unsupported_media_type');
      expect(huge.status, 'M7 70 KiB: rejected as too large').toBe(413);
    }
    expectNoLeak(
      'M7',
      await allSources(since, [
        { label: 'M7-20KiB', res: tooBig },
        { label: 'M7-xml', res: xml },
        { label: 'M7-70KiB', res: huge },
      ]),
    );
  });

  test('M10 tracing spans use route templates', async () => {
    test.skip(!isFull(), 'needs the Core (edge-only mode)');
    const { a } = ctx();
    const since = mark();
    await post(ingress(a.value), validInstruction());
    await post(`/ingress/v1/tradingview//${a.value}`, validInstruction());
    const core = logSources(since).find((s) => s.name === 'core')!.text;
    // The Core logs JSON lines; route values contain braces, so only the closing quote ends a value.
    const routes = [...core.matchAll(/"http\.route"\s*:\s*"([^"]*)"/g)].map((m) => m[1]!);
    expect(routes.length, 'M10: no http.route field found in Core trace output').toBeGreaterThan(0);
    // /metrics and /healthz are the Core's own route templates (scraped by this suite).
    const allowed = new Set(['/ingress/v1/tradingview/{token}', '<unmatched>', '<unmatched-ingress>', '/metrics', '/healthz', ...config.extraRoutes]);
    const bad = [...new Set(routes.filter((r) => !allowed.has(r)))];
    // Values are reported only after masking; an unexpected route could itself be a raw URI.
    const hits = findLeaks([{ name: 'http.route', text: bad.join('\n') }]);
    expect(hits, `M10: http.route carries the canary\n${formatHits(hits)}`).toHaveLength(0);
    expect(bad, 'M10: unexpected http.route values (must be route templates)').toEqual([]);
    expectNoLeak('M10', [{ name: 'core', text: core }]);
  });

  test('R2/R3 Caddy log structure: no URI, no Referer, no headers outside access', async () => {
    const caddy = serviceLogs(ctx().s, 'caddy');
    const { findings, accessLines, debugLines } = caddyStructureFindings(caddy);
    // n/o variants are non-ingress requests and must produce access lines, otherwise this check is vacuous.
    expect(accessLines, 'R2: no Caddy access line captured (access logging inactive?)').toBeGreaterThan(0);
    test.info().annotations.push({ type: 'caddy', description: `access=${accessLines} debug=${debugLines}` });
    if (caddyDebugExpected()) expect(debugLines, 'M1: Caddy DEBUG requested but no debug line captured').toBeGreaterThan(0);
    else test.info().annotations.push({ type: 'untested', description: 'Caddy debug output (no CADDY_LOG_LEVEL switch in this infra revision)' });
    expect(findings, 'R2/R3: forbidden keys in Caddy log lines').toEqual([]);
  });

  test('M5 revoked token', async () => {
    test.skip(!isFull(), 'needs the Core (edge-only mode)');
    test.skip(!config.revokeCmd, 'E2E_REVOKE_CMD not set: M5 untested');
    const { a, s } = ctx();
    const r = shell(config.revokeCmd!, { ...process.env, KEY_ID: a.keyId, E2E_COMPOSE: composeShellPrefix(s.envFile) });
    expect(r.status, `M5: revoke command failed: ${(r.stderr ?? '').slice(-500)}`).toBe(0);
    const since = mark();
    const res = await post(ingress(a.value), validInstruction());
    expectEmpty404('M5', res);
    expectNoLeak('M5', await allSources(since, [{ label: 'M5', res }]));
  });

  test('whole run: no canary anywhere (all logs, database, metrics)', async () => {
    const sources = await allSources(undefined);
    if (isFull() && !config.metricsPath) test.info().annotations.push({ type: 'untested', description: 'metrics endpoint: E2E_METRICS_PATH not set' });
    expectNoLeak('whole run', sources);
  });
});
