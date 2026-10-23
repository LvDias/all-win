// Audit rows written for ingress (contract s1-webhook-ingest §3, details per code in HTTP contract 4.3.0
// contracts/http/modules/audit.yaml DetailsIngress*). Details carry no message_id since 4.3.0, so rows are
// found through routing.instruction (received/duplicate) or by key_id and time (rejected).
import { expect } from '@playwright/test';
import { config } from './config.ts';
import { ctx } from './harness.ts';
import { lit, psql } from './stack.ts';

export interface AuditRow {
  code: string;
  instruction_id: string | null;
  source_key_id: string | null;
  details: Record<string, unknown>;
  /** The whole row as JSON text (for substring checks). */
  raw: string;
}

function rows(sql: string): AuditRow[] {
  return psql(ctx().s, sql)
    .split('\n')
    .filter(Boolean)
    .map((raw) => ({ ...(JSON.parse(raw) as Omit<AuditRow, 'raw'>), raw }));
}

/** Rows tied to the instruction `messageId` of key `keyId`: by instruction_id, or a duplicate's original_instruction_id. */
export function auditForMessage(keyId: string, messageId: string): AuditRow[] {
  const ids = `SELECT i.id FROM ${config.sql.instruction} i WHERE i.message_id = ${lit(messageId)} AND i.source_key_id = ${lit(keyId)}`;
  return rows(
    `SELECT row_to_json(x)::text FROM ${config.sql.audit} x WHERE x.source_key_id = ${lit(keyId)}` +
      ` AND (x.instruction_id IN (${ids}) OR x.details->>'original_instruction_id' IN (SELECT id::text FROM (${ids}) s));`,
  );
}

/** Database clock, so "since" comparisons do not depend on host/container clock skew. */
export function dbNow(): string {
  return psql(ctx().s, 'SELECT now()::text;').trim();
}

/** `ingress.rejected` rows of key `keyId` recorded at or after `since` (a dbNow() value). */
export function rejectedSince(keyId: string, since: string): AuditRow[] {
  return rows(
    `SELECT row_to_json(x)::text FROM ${config.sql.audit} x WHERE x.source_key_id = ${lit(keyId)}` +
      ` AND x.code = 'ingress.rejected' AND x.recorded_at >= ${lit(since)}::timestamptz ORDER BY x.recorded_at;`,
  );
}

/** Polls `read` until `predicate` holds or the audit wait (outbox dispatcher) runs out; returns the last read. */
export async function waitAudit(read: () => AuditRow[], predicate: (r: AuditRow[]) => boolean): Promise<AuditRow[]> {
  const deadline = Date.now() + config.auditWaitMs;
  let r = read();
  while (!predicate(r) && Date.now() < deadline) {
    await new Promise((res) => setTimeout(res, 500));
    r = read();
  }
  return r;
}

function expectKeys(label: string, details: Record<string, unknown>, required: string[], optional: string[] = []): void {
  for (const k of required) expect(Object.keys(details), `${label}: details.${k} required`).toContain(k);
  for (const k of Object.keys(details)) expect([...required, ...optional], `${label}: unexpected details key "${k}"`).toContain(k);
}

/** DetailsIngressReceived: action, instrument_symbol, sent_at (+ quantity), matching the posted instruction. */
export function expectReceivedDetails(label: string, row: AuditRow, body: Record<string, unknown>): void {
  const d = row.details;
  expectKeys(label, d, ['action', 'instrument_symbol', 'sent_at'], ['quantity']);
  expect(d.action, `${label}: details.action`).toBe(body.action);
  expect(d.instrument_symbol, `${label}: details.instrument_symbol`).toBe((body.instrument as { symbol: string }).symbol);
  expect(Date.parse(String(d.sent_at)), `${label}: details.sent_at is the instruction's sent_at`).toBe(Date.parse(String(body.sent_at)));
  if ('quantity' in d) expect(d.quantity, `${label}: details.quantity`).toBe(body.quantity);
}

/** DetailsIngressDuplicate: exactly original_instruction_id, the id of the first acceptance. */
export function expectDuplicateDetails(label: string, row: AuditRow, originalId: string): void {
  expectKeys(label, row.details, ['original_instruction_id']);
  expect(row.details.original_instruction_id, `${label}: details.original_instruction_id`).toBe(originalId);
}

/** DetailsIngressRejected: error_code, pointer, body_size; never the body (Q13 §8). */
export function expectRejectedDetails(label: string, row: AuditRow, exp: { code: string; pointer: string; bodySize: number }): void {
  expectKeys(label, row.details, ['error_code', 'pointer', 'body_size']);
  expect(row.details.error_code, `${label}: details.error_code`).toBe(exp.code);
  expect(row.details.pointer, `${label}: details.pointer`).toBe(exp.pointer);
  expect(row.details.body_size, `${label}: details.body_size is the request body length in bytes`).toBe(exp.bodySize);
}
