// Brings up the composed stack (managed mode), waits for the edge, and provisions the canary tokens.
import { config, INGRESS_PREFIX, TOKEN_RE } from './config.ts';
import { parseToken, syntheticToken, TOKEN_LABELS } from './canary.ts';
import { send } from './http.ts';
import { compose, composeShellPrefix, ensureRunDir, generateEnvFile, saveState, shell, stackFromEnv, type StackState } from './stack.ts';

async function waitForEdge(origin: string): Promise<void> {
  const deadline = Date.now() + config.timeouts.readyMs;
  let last = 'no response';
  while (Date.now() < deadline) {
    try {
      // A canonical-looking but unknown token: 404 means Caddy and the upstream both answer.
      const r = await send(origin, { target: `${INGRESS_PREFIX}not-a-token`, body: '{}', headers: { 'content-type': 'application/json' } });
      if (r.status === 404) return;
      last = `status ${r.status}`;
    } catch (e) {
      last = (e as Error).message;
    }
    await new Promise((r) => setTimeout(r, 1000));
  }
  throw new Error(`stack not ready after ${config.timeouts.readyMs} ms (last: ${last})`);
}

function seed(envFile: string, label: string): string {
  if (!config.seedCmd) {
    throw new Error('E2E_SEED_CMD is required in full mode (prints awk_<key_id>_<secret> once on stdout)');
  }
  const r = shell(config.seedCmd, { ...process.env, SEED_LABEL: label, E2E_COMPOSE: composeShellPrefix(envFile) });
  if (r.status !== 0) {
    // The seed's stdout may hold the token, so only its exit code and stderr tail are shown.
    throw new Error(`seed "${label}" exited ${r.status}; stderr tail: ${(r.stderr ?? '').slice(-800)}`);
  }
  const found = (r.stdout ?? '')
    .split(/\s+/)
    .filter((w) => TOKEN_RE.test(w))
    .pop();
  if (!found) throw new Error(`seed "${label}" printed no canonical token on stdout`);
  return found;
}

export default async function globalSetup(): Promise<void> {
  ensureRunDir();
  let envFile: string;
  if (config.stack === 'managed') {
    envFile = config.envFile ?? (await generateEnvFile());
  } else {
    if (!config.envFile) throw new Error('E2E_STACK=external needs E2E_ENV_FILE');
    envFile = config.envFile;
  }
  try {
    if (config.stack === 'managed') {
      compose({ envFile }, ['up', '-d', '--build', '--remove-orphans'], { timeoutMs: config.timeouts.stackUpMs });
    }
    const base = stackFromEnv(envFile);
    await waitForEdge(base.origin);

    const tokens: Record<string, string> = {};
    for (const label of TOKEN_LABELS) {
      tokens[label] = config.mode === 'full' ? parseToken(label, seed(envFile, label)).value : syntheticToken(label).value;
    }
    const state: StackState = { ...base, tokens, startedAt: new Date().toISOString() };
    saveState(state);
  } catch (e) {
    if (config.stack === 'managed' && !config.keepStack) {
      compose({ envFile }, ['down', '-v', '--remove-orphans'], { timeoutMs: 300_000, allowFail: true });
    }
    throw e;
  }
}
