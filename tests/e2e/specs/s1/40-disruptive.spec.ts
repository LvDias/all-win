// V1-S1 masking, disruptive scenarios — contract §4.3 M8 (rate limit) and M9 (database down; Core down → Caddy 502).
// Runs last: it exhausts rate limits and stops services, then restores them.
import { test, expect } from '@playwright/test';
import { config, INGRESS_PREFIX } from '../../src/config.ts';
import { maskAll, syntheticToken } from '../../src/canary.ts';
import { JSON_HEADERS, type RawResponse } from '../../src/http.ts';
import { validInstruction } from '../../src/instructions.ts';
import {
  allSources,
  caddyStructureFindings,
  ctx,
  expectNoLeak,
  expectRejected,
  ingress,
  isFull,
  logSources,
  mark,
  post,
  settle,
  waitRetryAfter,
} from '../../src/harness.ts';
import { send } from '../../src/http.ts';
import { serviceLogs, startService, stopService } from '../../src/stack.ts';

async function burst(target: () => string, body: string): Promise<{ last: RawResponse; sent: number }> {
  let last: RawResponse | undefined;
  for (let i = 1; i <= config.rateLimit.maxBurst; i++) {
    last = await post(target(), body, JSON_HEADERS, { retry429: false });
    if (last.status === 429) return { last, sent: i };
  }
  return { last: last!, sent: config.rateLimit.maxBurst };
}

async function waitEdge(expectStatus: number, timeoutMs = 120_000): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const r = await send(ctx().s.origin, { target: `${INGRESS_PREFIX}not-a-token`, body: '{}', headers: JSON_HEADERS });
      if (r.status === expectStatus) return;
    } catch {
      /* edge restarting */
    }
    await settle(1000);
  }
  throw new Error(`edge did not answer ${expectStatus} within ${timeoutMs} ms`);
}

test.describe('S1 masking — disruptive (contract §4.3)', () => {
  test('M8 rate limit with a valid token and with invalid tokens', async () => {
    test.skip(!isFull(), 'rate limiting is a Core feature (edge-only mode)');
    const since = mark();
    // Valid token, invalid JSON body: the per-key limit (step 3) precedes parsing, so no instruction is created.
    const valid = await burst(() => ingress(ctx().b.value), '{');
    expect(valid.last.status, `M8 valid token: no 429 within ${valid.sent} requests`).toBe(429);
    expectRejected('M8 valid token 429', valid.last, 429, 'ingress.rate_limited');
    expect(valid.last.headers['retry-after'], 'M8: Retry-After header').toBeDefined();
    await waitRetryAfter(valid.last);

    const invalid = await burst(() => ingress(syntheticToken('x').value), '{}');
    expect(invalid.last.status, `M8 invalid tokens: no 429 within ${invalid.sent} requests`).toBe(429);
    expectRejected('M8 invalid tokens 429', invalid.last, 429, 'ingress.rate_limited');
    expect(invalid.last.headers['retry-after'], 'M8: Retry-After header').toBeDefined();

    expectNoLeak('M8', await allSources(since, [{ label: 'M8-valid', res: valid.last }, { label: 'M8-invalid', res: invalid.last }]));
    await waitRetryAfter(invalid.last);
  });

  test('M9 database down → 500 common.internal without path, token or SQL', async () => {
    test.skip(!isFull(), 'needs the Core (edge-only mode)');
    const { s, b } = ctx();
    const since = mark();
    stopService(s, 'postgres');
    let res: RawResponse;
    let nearMiss: RawResponse;
    try {
      res = await post(ingress(b.value), validInstruction());
      nearMiss = await post(`/ingress/v1/tradingview//${b.value}`, validInstruction());
    } finally {
      startService(s, 'postgres');
    }
    expect(res.status, 'M9 db down: status').toBe(500);
    expect(JSON.parse(res.body), 'M9 db down: exact body').toEqual({ status: 'rejected', error: { code: 'common.internal' } });
    expect(nearMiss.status, 'M9 db down, near-miss path: still the empty 404 (no DB needed)').toBe(404);
    expectNoLeak('M9 db down', [...logSources(since), { name: 'response:M9-db', text: res.body }, { name: 'response:M9-near-miss', text: nearMiss.body }]);
    await waitEdge(404);
  });

  test('M9 Core down → Caddy 502 without path; Caddy logs clean', async () => {
    const { s, a, b, coreService } = ctx();
    const since = mark();
    stopService(s, coreService);
    const responses: Array<{ label: string; res: RawResponse }> = [];
    try {
      responses.push({ label: 'M9-502 canonical', res: await post(ingress(b.value), validInstruction(), JSON_HEADERS, { retry429: false }) });
      responses.push({ label: 'M9-502 near-miss', res: await post(`/ingress/v1/tradingview//${a.value}`, validInstruction(), JSON_HEADERS, { retry429: false }) });
      responses.push({ label: 'M9-502 api', res: await post(`/api/v1/${a.value}`, validInstruction(), JSON_HEADERS, { retry429: false }) });
    } finally {
      startService(s, coreService);
    }
    for (const { label, res } of responses) {
      const body = maskAll(res.body, ctx().terms);
      expect(res.status, `${label}: status`).toBeGreaterThanOrEqual(500);
      expect(body.toLowerCase().includes('ingress') || body.includes('/api/') || body.includes('awk_'), `${label}: 5xx body carries the path: ${body.slice(0, 200)}`).toBe(false);
    }
    expect(responses[0]!.res.status, 'M9 canonical: Caddy answers 502').toBe(502);
    const caddy = serviceLogs(s, 'caddy', since);
    expect(caddyStructureFindings(caddy).findings, 'R3: forbidden keys in Caddy lines while upstream is down').toEqual([]);
    expectNoLeak('M9 core down', [{ name: 'caddy', text: caddy }, ...responses.map(({ label, res }) => ({ name: `response:${label}`, text: res.body }))]);
    await waitEdge(404);
  });

  test('whole run after disruptive scenarios: no canary anywhere', async () => {
    expectNoLeak('whole run (final)', await allSources(undefined));
  });
});
