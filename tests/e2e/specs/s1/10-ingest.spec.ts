// V1-S1 ingest behaviour — contract s1-webhook-ingest.md §2 and §5, S1 AC-1..AC-5, AC-7.
// Black-box through the edge (Caddy → Core → PostgreSQL); persistence checked in the database.
import { test, expect } from '@playwright/test';
import { config } from '../../src/config.ts';
import { syntheticToken } from '../../src/canary.ts';
import { JSON_HEADERS, type RawResponse } from '../../src/http.ts';
import {
  asTradingView,
  expectedForInvalid,
  expectedForValid,
  loadFixtures,
  uniqueMessageId,
  validInstruction,
  withFreshMessageId,
} from '../../src/instructions.ts';
import { ctx, expectAccepted, expectEmpty404, expectRejected, ingress, isFull, post } from '../../src/harness.ts';
import { lit, psql } from '../../src/stack.ts';

function count(sql: string): number {
  return Number.parseInt(psql(ctx().s, sql).trim(), 10);
}
const instructionRows = (msg: string) => count(`SELECT count(*) FROM ${config.sql.instruction} WHERE message_id = ${lit(msg)};`);
const receiptRows = (msg: string) => count(`SELECT count(*) FROM ${config.sql.receipt} WHERE message_id = ${lit(msg)};`);

async function waitAuditCode(msg: string, code: string): Promise<boolean> {
  const deadline = Date.now() + config.auditWaitMs;
  const sql = `SELECT count(*) FROM ${config.sql.audit} x WHERE row_to_json(x)::text LIKE ${lit(`%${msg}%`)} AND row_to_json(x)::text LIKE ${lit(`%${code}%`)};`;
  while (Date.now() < deadline) {
    if (count(sql) > 0) return true;
    await new Promise((r) => setTimeout(r, 500));
  }
  return false;
}

test.describe('S1 ingest', () => {
  test.skip(!isFull(), 'needs the Core (edge-only mode)');

  test.describe('AC-1 / AC-7 / policy: valid instruction fixtures', () => {
    for (const f of loadFixtures('valid')) {
      test(`valid fixture ${f.name}`, async () => {
        const body = withFreshMessageId(f.json);
        const exp = expectedForValid(body);
        const res = await post(ingress(ctx().a.value), body);
        if (exp.status === 200) {
          expectAccepted(f.name, res, 'accepted');
          expect(instructionRows(String(body.message_id)), `${f.name}: one instruction row`).toBe(1);
          const receivedAt = psql(ctx().s, `SELECT received_at IS NOT NULL FROM ${config.sql.instruction} WHERE message_id = ${lit(String(body.message_id))};`).trim();
          expect(receivedAt, `${f.name}: received_at stored`).toBe('t');
        } else {
          const r = expectRejected(f.name, res, exp.status, exp.code);
          if (exp.pointer) expect(r.error.pointer, `${f.name}: pointer`).toBe(exp.pointer);
          expect(receiptRows(String(body.message_id)), `${f.name}: rejection creates no receipt`).toBe(0);
        }
      });
    }

    test('AC-1 audit entry "received"', async () => {
      const body = validInstruction();
      expectAccepted('AC-1', await post(ingress(ctx().a.value), body), 'accepted');
      expect(await waitAuditCode(String(body.message_id), 'ingress.received'), 'AC-1: audit ingress.received').toBe(true);
    });

    test('text/plain body is accepted like application/json', async () => {
      const body = validInstruction();
      expectAccepted('text/plain', await post(ingress(ctx().a.value), body, { 'content-type': 'text/plain' }), 'accepted');
    });

    test('policy: TradingView alert with a limit order, reduce, cancel or modify is rejected (422)', async () => {
      const expected: Record<string, string> = {
        'master-buy-limit-entry': 'ingress.order_type_not_allowed_for_source',
        'master-sell-reduce': 'ingress.action_not_allowed_for_source',
        'master-cancel': 'ingress.action_not_allowed_for_source',
        'master-modify-stop-delta': 'ingress.action_not_allowed_for_source',
      };
      for (const [name, code] of Object.entries(expected)) {
        const body = asTradingView(loadFixtures('valid').find((x) => x.name === name)!.json);
        expect(expectedForValid(body).code, `${name}: suite expectation`).toBe(code);
        expectRejected(`policy ${name}`, await post(ingress(ctx().a.value), body), 422, code);
      }
    });

    test('policy: signal_type master_account from TradingView → 422 with pointer /signal_type', async () => {
      const r = expectRejected('signal_type', await post(ingress(ctx().a.value), validInstruction({ signal_type: 'master_account' })), 422, 'ingress.signal_type_not_allowed_for_source');
      expect(r.error.pointer).toBe('/signal_type');
    });

    test('AC-7 symbol not enabled in the catalog → 422 ingress.unsupported_instrument', async () => {
      const symbol = ['ES', 'NQ', 'MES', 'YM'].find((s) => !config.enabledSymbols.includes(s));
      test.skip(!symbol, 'every candidate symbol is enabled in this environment');
      const res = await post(ingress(ctx().a.value), validInstruction({ instrument: { symbol } }));
      expectRejected('AC-7', res, 422, 'ingress.unsupported_instrument');
    });
  });

  test.describe('AC-3: invalid instructions', () => {
    for (const f of loadFixtures('invalid')) {
      test(`invalid fixture ${f.name}`, async () => {
        const body = withFreshMessageId(f.json);
        const exp = expectedForInvalid(f.name);
        const r = expectRejected(f.name, await post(ingress(ctx().a.value), body), exp.status, exp.code);
        if (exp.code === 'ingress.schema_violation') expect(typeof r.error.pointer, `${f.name}: pointer present`).toBe('string');
        if (exp.pointer) expect(r.error.pointer, `${f.name}: pointer`).toBe(exp.pointer);
        if (typeof body.message_id === 'string') expect(receiptRows(body.message_id), `${f.name}: no receipt`).toBe(0);
      });
    }

    test('malformed JSON → 400 ingress.invalid_json', async () => {
      expectRejected('invalid json', await post(ingress(ctx().a.value), '{"schema_version":'), 400, 'ingress.invalid_json');
    });
  });

  test.describe('AC-2: wrong, malformed or unknown token, other paths and methods → empty 404', () => {
    const cases: Array<{ label: string; method?: string; target: () => string }> = [
      { label: 'unknown well-formed token', target: () => ingress(syntheticToken('x').value) },
      { label: 'known key_id, wrong secret', target: () => ingress(`awk_${ctx().a.keyId}_${syntheticToken('x').secret}`) },
      { label: 'secret one character short', target: () => ingress(ctx().a.value.slice(0, -1)) },
      { label: 'token of 129 characters', target: () => ingress(`awk_${ctx().a.keyId}_${'A'.repeat(129 - 17)}`) },
      { label: 'token of 4096 characters', target: () => ingress(`awk_${'B'.repeat(4092)}`) },
      { label: 'invalid character in secret', target: () => ingress(`awk_${ctx().a.keyId}_${ctx().a.secret.slice(0, 42)}.`) },
      { label: 'wrong prefix awd_', target: () => ingress(`awd_${ctx().a.value.slice(4)}`) },
      { label: 'missing token (trailing slash)', target: () => '/ingress/v1/tradingview/' },
      { label: 'missing token (no slash)', target: () => '/ingress/v1/tradingview' },
      { label: '/ingress', target: () => '/ingress' },
      { label: '/ingress/', target: () => '/ingress/' },
      // §4.1: ingress-like = path starting with /ingress (no segment boundary required).
      { label: '/ingress with token in query', target: () => `/ingress?t=${ctx().a.value}` },
      { label: '/ingressfoo prefix', target: () => `/ingressfoo/${ctx().a.value}` },
      { label: 'other version', target: () => `/ingress/v2/tradingview/${ctx().a.value}` },
      { label: 'other platform segment', target: () => `/ingress/v1/other/${ctx().a.value}` },
      { label: 'extra segment', target: () => `${ingress(ctx().a.value)}/extra` },
      ...['GET', 'PUT', 'PATCH', 'DELETE', 'OPTIONS'].map((m) => ({ label: `${m} on canonical URL`, method: m, target: () => ingress(ctx().a.value) })),
    ];
    for (const c of cases) {
      test(c.label, async () => {
        const body = validInstruction();
        const res = await post(c.target(), c.method === 'GET' ? undefined : body, c.method === 'GET' ? {} : JSON_HEADERS, { method: c.method });
        expectEmpty404(c.label, res);
        expect(receiptRows(String(body.message_id)), `${c.label}: nothing stored`).toBe(0);
      });
    }

    test('HEAD on canonical URL → 404', async () => {
      const res = await post(ingress(ctx().a.value), undefined, {}, { method: 'HEAD' });
      expect(res.status).toBe(404);
    });

    test('unknown key and wrong secret for a known key are indistinguishable', async () => {
      const unknown = await post(ingress(syntheticToken('x').value), validInstruction());
      const wrong = await post(ingress(`awk_${ctx().a.keyId}_${syntheticToken('x').secret}`), validInstruction());
      const names = (r: RawResponse) => Object.keys(r.headers).filter((h) => h !== 'date').sort();
      expect(wrong.status).toBe(unknown.status);
      expect(wrong.body).toBe(unknown.body);
      expect(names(wrong), 'same response header names').toEqual(names(unknown));
    });
  });

  test.describe('AC-4 idempotency', () => {
    test('duplicate → 200 duplicate with the original instruction_id, one row, audit "duplicate ignored"', async () => {
      const body = validInstruction();
      const msg = String(body.message_id);
      const first = expectAccepted('first', await post(ingress(ctx().a.value), body), 'accepted');
      const second = expectAccepted('second', await post(ingress(ctx().a.value), body), 'duplicate');
      expect(second, 'same instruction_id').toBe(first);
      expect(instructionRows(msg), 'one instruction row').toBe(1);
      expect(receiptRows(msg), 'one receipt row').toBe(1);
      // Contract §3 kind (wave1-data-model §6.1 prose says 'ingress.duplicate'; the contract wins).
      expect(await waitAuditCode(msg, 'ingress.duplicate_ignored'), 'audit ingress.duplicate_ignored').toBe(true);
    });

    test('a different body with the same message_id is still a duplicate', async () => {
      const body = validInstruction();
      const first = expectAccepted('first', await post(ingress(ctx().a.value), body), 'accepted');
      const changed = { ...body, quantity: 2 };
      expect(expectAccepted('changed', await post(ingress(ctx().a.value), changed), 'duplicate')).toBe(first);
      expect(instructionRows(String(body.message_id))).toBe(1);
    });

    test('20 parallel posts → exactly 1 accepted, 19 duplicate, one row', async () => {
      const body = validInstruction();
      const results = await Promise.all(Array.from({ length: 20 }, () => post(ingress(ctx().c.value), body, JSON_HEADERS, { retry429: false })));
      const statuses = results.map((r) => r.status);
      expect(statuses.filter((s) => s === 429), 'no 429 during the concurrency test (rate limit below 20 parallel posts?)').toHaveLength(0);
      const parsed = results.map((r) => JSON.parse(r.body) as { status: string; instruction_id: string });
      expect(parsed.filter((p) => p.status === 'accepted')).toHaveLength(1);
      expect(parsed.filter((p) => p.status === 'duplicate')).toHaveLength(19);
      expect(new Set(parsed.map((p) => p.instruction_id)).size, 'all responses carry one instruction_id').toBe(1);
      expect(instructionRows(String(body.message_id))).toBe(1);
    });
  });

  test.describe('AC-5 / isolation: per-user scoping', () => {
    test('same message_id from two users → two independent instructions', async () => {
      const msg = uniqueMessageId('xuser');
      const ia = expectAccepted('user A', await post(ingress(ctx().a.value), validInstruction({ message_id: msg })), 'accepted');
      const ib = expectAccepted('user B', await post(ingress(ctx().b.value), validInstruction({ message_id: msg })), 'accepted');
      expect(ib).not.toBe(ia);
      const rows = psql(ctx().s, `SELECT user_id, source_key_id FROM ${config.sql.instruction} WHERE message_id = ${lit(msg)} ORDER BY source_key_id;`)
        .split('\n')
        .filter(Boolean)
        .map((l) => l.split('|'));
      expect(rows, 'two rows').toHaveLength(2);
      expect(new Set(rows.map((r) => r[0])).size, 'distinct user_id').toBe(2);
    });

    test("a user's token only ever creates that user's instructions", async () => {
      const { a, b } = ctx();
      expectAccepted('A', await post(ingress(a.value), validInstruction()), 'accepted');
      expectAccepted('B', await post(ingress(b.value), validInstruction()), 'accepted');
      const usersOf = (keyId: string) =>
        psql(ctx().s, `SELECT DISTINCT user_id FROM ${config.sql.instruction} WHERE source_key_id = ${lit(keyId)};`).split('\n').filter(Boolean);
      const ua = usersOf(a.keyId);
      const ub = usersOf(b.keyId);
      expect(ua, "token A rows belong to one user").toHaveLength(1);
      expect(ub, "token B rows belong to one user").toHaveLength(1);
      expect(ua[0]).not.toBe(ub[0]);
    });
  });
});
