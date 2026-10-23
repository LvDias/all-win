// Instruction fixtures (contracts/instruction/v1/examples) and the outcome the S1 contract expects for each.
import { readdirSync, readFileSync } from 'node:fs';
import { randomUUID } from 'node:crypto';
import { resolve } from 'node:path';
import { config } from './config.ts';

export type Instruction = Record<string, unknown> & { message_id?: unknown };

export interface Fixture {
  name: string;
  json: Instruction;
}

const EXAMPLES = resolve(config.e2eDir, '..', '..', 'contracts/instruction/v1/examples');
const MESSAGE_ID_RE = /^[A-Za-z0-9._:-]{1,128}$/;

export function loadFixtures(kind: 'valid' | 'invalid'): Fixture[] {
  const dir = resolve(EXAMPLES, kind);
  return readdirSync(dir)
    .filter((f) => f.endsWith('.json'))
    .sort()
    .map((f) => ({ name: f.replace(/\.json$/, ''), json: JSON.parse(readFileSync(resolve(dir, f), 'utf8')) as Instruction }));
}

export function uniqueMessageId(prefix = 'e2e'): string {
  return `${prefix}-${randomUUID()}`;
}

/**
 * Fixtures share message_ids, and the database outlives a single test, so each post gets a fresh id.
 * A message_id that is itself the invalid part of a fixture is left untouched.
 */
export function withFreshMessageId(json: Instruction, prefix?: string): Instruction {
  if (typeof json.message_id !== 'string' || !MESSAGE_ID_RE.test(json.message_id)) return structuredClone(json);
  return { ...structuredClone(json), message_id: uniqueMessageId(prefix) };
}

export function validInstruction(overrides: Partial<Instruction> = {}): Instruction {
  const base = loadFixtures('valid').find((f) => f.name === 'tv-trigger-buy');
  if (!base) throw new Error('fixture tv-trigger-buy missing');
  return { ...withFreshMessageId(base.json), ...overrides };
}

export interface Expected {
  status: number;
  code?: string;
}

/** Contract §2 step 7 (source policy, in its listed order) applied to a schema-valid instruction. */
export function expectedForValid(json: Instruction): Expected {
  const source = json.source as { type?: string } | undefined;
  const order = json.order as { type?: string } | undefined;
  const instrument = json.instrument as { symbol?: string } | undefined;
  if (source?.type !== 'tradingview') return { status: 422, code: 'ingress.source_mismatch' };
  if (json.action === 'modify' || json.action === 'cancel' || json.position_effect === 'reduce') {
    return { status: 422, code: 'ingress.action_not_allowed_for_source' };
  }
  if (order?.type !== undefined && order.type !== 'market') return { status: 422, code: 'ingress.order_type_not_allowed_for_source' };
  if (!instrument?.symbol || !config.enabledSymbols.includes(instrument.symbol)) return { status: 422, code: 'ingress.unsupported_instrument' };
  return { status: 200 };
}

/** Invalid fixtures: 400 with schema_violation, except an unsupported major version (contract §2 step 6). */
export function expectedForInvalid(name: string): Expected {
  return name === 'unsupported-major-version' ? { status: 400, code: 'ingress.unsupported_version' } : { status: 400, code: 'ingress.schema_violation' };
}
