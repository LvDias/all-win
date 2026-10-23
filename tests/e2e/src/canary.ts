// Canary tokens and the masking search terms of contract s1-webhook-ingest.md §4.3.
// Secret values never reach test output: reports name a term by label and mask every match.
import { randomBytes } from 'node:crypto';
import { TOKEN_RE } from './config.ts';

export interface Token {
  label: string;
  keyId: string;
  secret: string;
  /** awk_<key_id>_<secret> */
  value: string;
}

export interface SearchTerm {
  /** Safe to print, e.g. "a:window[5..21)". */
  label: string;
  text: string;
}

/**
 * One seeded user per label (contract §2 M1 seed): a = masking and functional tests (revoked in M5),
 * b = second user (AC-5) and disruptive tests, c = concurrency, d = latency. Separate keys keep the per-key
 * rate limit (60/min) from coupling tests.
 */
export const TOKEN_LABELS = ['a', 'b', 'c', 'd'] as const;

export const WINDOW = 16;
export const FRAGMENT_MIN = 8;

export function parseToken(label: string, value: string): Token {
  const m = TOKEN_RE.exec(value);
  if (!m) throw new Error(`seed output for "${label}" is not a canonical token (value withheld)`);
  return { label, keyId: m[1]!, secret: m[2]!, value };
}

const ALNUM = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
const B64URL = `${ALNUM}-_`;

function pick(alphabet: string, n: number): string {
  const bytes = randomBytes(n);
  let out = '';
  for (const b of bytes) out += alphabet[b % alphabet.length];
  return out;
}

/** Synthetic canonical token (edge-only mode, and well-formed unknown tokens). */
export function syntheticToken(label: string): Token {
  // Letters at both ends keep variant h (percent-encoding one character) well defined.
  const secret = pick(ALNUM, 1) + pick(B64URL, 41) + pick(ALNUM, 1);
  const keyId = pick(ALNUM, 12);
  return { label, keyId, secret, value: `awk_${keyId}_${secret}` };
}

/** Full secret plus every 16-character sliding window (28 for a 43-character secret). */
export function secretTerms(t: Token): SearchTerm[] {
  const terms: SearchTerm[] = [{ label: `${t.label}:secret`, text: t.secret }];
  for (let i = 0; i + WINDOW <= t.secret.length; i++) {
    terms.push({ label: `${t.label}:window[${i}..${i + WINDOW})`, text: t.secret.slice(i, i + WINDOW) });
  }
  return terms;
}

/** Pieces introduced by M6-o; each piece of >= 8 characters becomes a term. */
export function fragmentTerms(t: Token, cuts: Array<[number, number]>): SearchTerm[] {
  return cuts
    .filter(([a, b]) => b - a >= FRAGMENT_MIN)
    .map(([a, b]) => ({ label: `${t.label}:fragment[${a}..${b})`, text: t.secret.slice(a, b) }));
}

/** M6-o pieces, as named in the contract. */
export const M6O_CUTS: Array<[number, number]> = [
  [0, 10],
  [10, 20],
  [0, 20],
  [20, 43],
];

export interface Hit {
  source: string;
  line: number;
  term: string;
  /** The line with every term occurrence replaced by «label». */
  masked: string;
}

/** Replaces every term occurrence in `s` (longest first) with a «label» marker. */
export function maskAll(s: string, terms: SearchTerm[]): string {
  let out = s;
  for (const t of [...terms].sort((a, b) => b.text.length - a.text.length)) {
    if (t.text.length === 0) continue;
    out = out.split(t.text).join(`«${t.label}»`);
  }
  return out;
}

export function scan(source: string, text: string, terms: SearchTerm[]): Hit[] {
  const hits: Hit[] = [];
  const lines = text.split('\n');
  lines.forEach((line, i) => {
    for (const t of terms) {
      if (line.includes(t.text)) {
        const masked = maskAll(line, terms);
        hits.push({ source, line: i + 1, term: t.label, masked: masked.length > 400 ? `${masked.slice(0, 400)}…` : masked });
        break;
      }
    }
  });
  return hits;
}

export function formatHits(hits: Hit[], max = 20): string {
  const lines = hits.slice(0, max).map((h) => `  ${h.source}:${h.line} [${h.term}] ${h.masked}`);
  if (hits.length > max) lines.push(`  … ${hits.length - max} more`);
  return lines.join('\n');
}
