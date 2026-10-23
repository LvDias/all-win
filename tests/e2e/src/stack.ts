// Composed stack control (docker compose) for the E2E suite: lifecycle, logs, service stop/start, SQL.
import { spawnSync, type SpawnSyncReturns } from 'node:child_process';
import { chmodSync, existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { createServer } from 'node:net';
import { randomBytes } from 'node:crypto';
import { resolve } from 'node:path';
import { config } from './config.ts';

export interface StackState {
  project: string;
  envFile: string;
  /** Caddy origin, e.g. https://localhost:8443 */
  origin: string;
  /** Core direct origin (loopback debug port), e.g. http://127.0.0.1:8080 */
  coreOrigin: string;
  postgresUser: string;
  postgresDb: string;
  /** Canonical tokens by label (a, b). Kept only in the 0600 state file of the run directory. */
  tokens: Record<string, string>;
  startedAt: string;
}

const STATE_FILE = () => resolve(config.runDir, 'state.json');

export function ensureRunDir(): void {
  mkdirSync(config.runDir, { recursive: true, mode: 0o700 });
  chmodSync(config.runDir, 0o700);
}

export function saveState(s: StackState): void {
  ensureRunDir();
  writeFileSync(STATE_FILE(), JSON.stringify(s, null, 2), { mode: 0o600 });
}

export function loadState(): StackState {
  return JSON.parse(readFileSync(STATE_FILE(), 'utf8')) as StackState;
}

function shq(s: string): string {
  return /^[A-Za-z0-9_@%+=:,./-]+$/.test(s) ? s : `'${s.replace(/'/g, `'\\''`)}'`;
}

let sgDecision: boolean | undefined;
function useSg(): boolean {
  if (sgDecision !== undefined) return sgDecision;
  if (config.dockerSg === '1') return (sgDecision = true);
  if (config.dockerSg === '0') return (sgDecision = false);
  const groups = spawnSync('id', ['-Gn'], { encoding: 'utf8' }).stdout ?? '';
  return (sgDecision = !groups.split(/\s+/).includes('docker'));
}

export function composeFiles(): string[] {
  return [resolve(config.stackRoot, 'infra/compose/docker-compose.dev.yml'), resolve(config.e2eDir, 'stack/compose.e2e.yml')];
}

export function composeArgs(envFile: string): string[] {
  const args = ['compose', '-p', config.project];
  for (const f of composeFiles()) args.push('-f', f);
  args.push('--env-file', envFile, '--profile', config.mode === 'full' ? 'core' : 'edge-only');
  return args;
}

/** Shell prefix handed to seed/revoke commands as $E2E_COMPOSE. */
export function composeShellPrefix(envFile: string): string {
  return ['docker', ...composeArgs(envFile)].map(shq).join(' ');
}

export function run(cmd: string, args: string[], opts: { input?: string; env?: NodeJS.ProcessEnv; timeoutMs?: number } = {}): SpawnSyncReturns<string> {
  const r = spawnSync(cmd, args, {
    encoding: 'utf8',
    input: opts.input,
    env: opts.env ?? process.env,
    timeout: opts.timeoutMs ?? 120_000,
    maxBuffer: 1024 * 1024 * 1024,
  });
  if (r.error) throw r.error;
  return r;
}

export function docker(args: string[], opts: { input?: string; timeoutMs?: number; allowFail?: boolean } = {}): SpawnSyncReturns<string> {
  const r = useSg() ? run('sg', ['docker', '-c', ['docker', ...args].map(shq).join(' ')], opts) : run('docker', args, opts);
  if (r.status !== 0 && !opts.allowFail) {
    // stderr of compose/docker carries no request data; trimmed to keep failures readable.
    throw new Error(`docker ${args.slice(0, 6).join(' ')} … exited ${r.status}: ${(r.stderr ?? '').slice(-2000)}`);
  }
  return r;
}

/** Runs a user-supplied shell command (seed/revoke) with the docker group if needed. */
export function shell(command: string, env: NodeJS.ProcessEnv, timeoutMs = 300_000): SpawnSyncReturns<string> {
  return useSg() ? run('sg', ['docker', '-c', command], { env, timeoutMs }) : run('bash', ['-c', command], { env, timeoutMs });
}

export function compose(s: Pick<StackState, 'envFile'>, args: string[], opts: { input?: string; timeoutMs?: number; allowFail?: boolean } = {}) {
  return docker([...composeArgs(s.envFile), ...args], opts);
}

async function freePort(): Promise<number> {
  return new Promise((res, rej) => {
    const srv = createServer();
    srv.once('error', rej);
    srv.listen(0, '127.0.0.1', () => {
      const addr = srv.address();
      const port = typeof addr === 'object' && addr ? addr.port : 0;
      srv.close(() => res(port));
    });
  });
}

function parseEnv(text: string): Record<string, string> {
  const out: Record<string, string> = {};
  for (const line of text.split('\n')) {
    const m = /^\s*([A-Z0-9_]+)=(.*)$/.exec(line);
    if (m) out[m[1]!] = m[2]!.trim();
  }
  return out;
}

/** Generates a disposable env file (random password and loopback ports, RUST_LOG=trace) from .env.example. */
export async function generateEnvFile(): Promise<string> {
  ensureRunDir();
  const example = resolve(config.stackRoot, 'infra/compose/.env.example');
  const base = parseEnv(readFileSync(example, 'utf8'));
  const values: Record<string, string> = {
    ...base,
    POSTGRES_PASSWORD: randomBytes(18).toString('hex'),
    POSTGRES_HOST_PORT: String(await freePort()),
    CORE_HOST_PORT: String(await freePort()),
    CADDY_HTTPS_PORT: String(await freePort()),
    RUST_LOG: process.env.E2E_RUST_LOG ?? 'trace',
  };
  const file = resolve(config.runDir, 'compose.env');
  writeFileSync(file, Object.entries(values).map(([k, v]) => `${k}=${v}`).join('\n') + '\n', { mode: 0o600 });
  return file;
}

export function readEnvFile(file: string): Record<string, string> {
  return parseEnv(readFileSync(file, 'utf8'));
}

export function stackFromEnv(envFile: string): Omit<StackState, 'tokens' | 'startedAt'> {
  if (!existsSync(envFile)) throw new Error(`env file not found: ${envFile}`);
  const e = readEnvFile(envFile);
  return {
    project: config.project,
    envFile,
    origin: `https://localhost:${e.CADDY_HTTPS_PORT ?? '8443'}`,
    coreOrigin: `http://127.0.0.1:${e.CORE_HOST_PORT ?? '8080'}`,
    postgresUser: e.POSTGRES_USER ?? 'allwin',
    postgresDb: e.POSTGRES_DB ?? 'allwin',
  };
}

/** Container output (stdout + stderr) of one service, optionally since an ISO timestamp. */
export function serviceLogs(s: Pick<StackState, 'envFile'>, service: string, since?: string): string {
  const args = ['logs', '--no-color', '--no-log-prefix'];
  if (since) args.push('--since', since);
  args.push(service);
  return compose(s, args, { allowFail: true, timeoutMs: 300_000 }).stdout ?? '';
}

export function stopService(s: Pick<StackState, 'envFile'>, service: string): void {
  compose(s, ['stop', service], { timeoutMs: 180_000 });
}

export function startService(s: Pick<StackState, 'envFile'>, service: string): void {
  compose(s, ['start', service], { timeoutMs: 180_000 });
}

/** Runs SQL as the database owner inside the postgres container; returns unaligned tuples. */
export function psql(s: StackState, sql: string): string {
  const r = compose(s, ['exec', '-T', 'postgres', 'psql', '-v', 'ON_ERROR_STOP=1', '-At', '-U', s.postgresUser, '-d', s.postgresDb, '-f', '-'], {
    input: sql,
    allowFail: true,
  });
  if (r.status !== 0) throw new Error(`psql failed: ${(r.stderr ?? '').slice(-1500)}`);
  return r.stdout ?? '';
}

/** Full data-only dump, searched for the canary (covers audit rows and every other table). */
export function pgDump(s: StackState): string {
  const r = compose(s, ['exec', '-T', 'postgres', 'pg_dump', '--data-only', '--inserts', '-U', s.postgresUser, '-d', s.postgresDb], {
    allowFail: true,
    timeoutMs: 300_000,
  });
  if (r.status !== 0) throw new Error(`pg_dump failed: ${(r.stderr ?? '').slice(-1500)}`);
  return r.stdout ?? '';
}

/** Quotes a SQL literal (test-generated values only). */
export function lit(v: string): string {
  return `'${v.replace(/'/g, "''")}'`;
}
