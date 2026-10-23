// Self-test of the masking scanner (no stack calls): proves the search terms of §4.3 catch what they must.
import { test, expect } from '@playwright/test';
import { fragmentTerms, M6O_CUTS, maskAll, scan, secretTerms, syntheticToken } from '../../src/canary.ts';

test.describe('scanner self-test', () => {
  const t = syntheticToken('t');
  const terms = [...secretTerms(t), ...fragmentTerms(t, M6O_CUTS)];

  test('43-character secret gives the full secret plus 28 windows of 16', () => {
    const windows = secretTerms(t).filter((x) => x.label.includes('window'));
    expect(t.secret).toHaveLength(43);
    expect(windows).toHaveLength(28);
    expect(windows.every((w) => w.text.length === 16)).toBe(true);
  });

  test('M6-o pieces of >= 8 characters are terms', () => {
    const f = fragmentTerms(t, M6O_CUTS).map((x) => x.text);
    expect(f).toContain(t.secret.slice(0, 10));
    expect(f).toContain(t.secret.slice(10, 20));
    expect(f).toContain(t.secret.slice(20));
  });

  test('a 16-character slice anywhere in a line is a hit, and the report is masked', () => {
    const line = `{"msg":"req","path":"/x/${t.secret.slice(5, 21)}"}`;
    const hits = scan('log', `ok\n${line}\nok`, terms);
    expect(hits).toHaveLength(1);
    expect(hits[0]!.line).toBe(2);
    expect(hits[0]!.masked.includes(t.secret.slice(5, 21))).toBe(false);
  });

  test('split with separators (M6-o shape) is caught by the fragment terms', () => {
    const line = `GET /pt-BR/${t.secret.slice(0, 10)}.${t.secret.slice(10, 20)}/x`;
    expect(scan('log', line, terms)).toHaveLength(1);
  });

  test('15 characters and the key_id alone are not hits', () => {
    expect(scan('log', `${t.secret.slice(0, 7)} ${t.keyId}`, secretTerms(t))).toHaveLength(0);
    expect(scan('log', t.secret.slice(3, 18), secretTerms(t))).toHaveLength(0);
  });

  test('maskAll removes every occurrence', () => {
    const masked = maskAll(`${t.value} and ${t.secret}`, terms);
    expect(masked.includes(t.secret.slice(0, 16))).toBe(false);
    expect(masked.includes(t.keyId)).toBe(true);
  });
});
