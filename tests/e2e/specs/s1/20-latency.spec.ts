// V1-S1 latency smoke — contract §2: p95 < 300 ms from the first byte at the Core to the response.
// Measured at the client through Caddy on loopback, which is an upper bound of the contract's measure.
import { test, expect } from '@playwright/test';
import { config } from '../../src/config.ts';
import { validInstruction } from '../../src/instructions.ts';
import { ctx, expectAccepted, ingress, isFull, post } from '../../src/harness.ts';

test('S1 p95 latency of accepted instructions', async () => {
  test.skip(!isFull(), 'needs the Core (edge-only mode)');
  const { samples, pauseMs, p95Ms } = config.latency;
  // Warm-up: TLS handshake and connection pool are not part of the budget.
  expectAccepted('warm-up', await post(ingress(ctx().d.value), validInstruction()), 'accepted');
  const ms: number[] = [];
  for (let i = 0; i < samples; i++) {
    const res = await post(ingress(ctx().d.value), validInstruction(), undefined, { retry429: false });
    expect(res.status, `sample ${i}: rate limited (raise E2E_LATENCY_PAUSE_MS to the configured limit)`).not.toBe(429);
    expectAccepted(`sample ${i}`, res, 'accepted');
    ms.push(res.ms);
    await new Promise((r) => setTimeout(r, pauseMs));
  }
  ms.sort((x, y) => x - y);
  const p95 = ms[Math.ceil(0.95 * ms.length) - 1]!;
  const p50 = ms[Math.ceil(0.5 * ms.length) - 1]!;
  test.info().annotations.push({ type: 'latency', description: `n=${ms.length} p50=${p50.toFixed(1)}ms p95=${p95.toFixed(1)}ms max=${ms.at(-1)!.toFixed(1)}ms` });
  expect(p95, `p95 ${p95.toFixed(1)} ms`).toBeLessThan(p95Ms);
});
