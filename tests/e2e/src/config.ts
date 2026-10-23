// E2E configuration, read from the environment. Every value has a safe local default.
// See tests/e2e/README.md for the meaning of each variable.
import { resolve } from 'node:path';
import { tmpdir } from 'node:os';

const env = process.env;
// Fixed once in the runner process; workers inherit it, so every process shares one project and run directory.
env.E2E_COMPOSE_PROJECT ??= `allwin-e2e-${Date.now().toString(36)}`;

export type Mode = 'full' | 'edge-only';

function int(name: string, fallback: number): number {
  const raw = env[name];
  if (raw === undefined || raw === '') return fallback;
  const n = Number.parseInt(raw, 10);
  if (!Number.isFinite(n)) throw new Error(`${name} must be an integer`);
  return n;
}

const e2eDir = resolve(import.meta.dirname, '..');

export const config = {
  /** `full` = Caddy + Core + PostgreSQL; `edge-only` = Caddy + a stub upstream (no Core yet). */
  mode: (env.E2E_MODE === 'edge-only' ? 'edge-only' : 'full') as Mode,
  /** Root of the checkout that holds `infra/` (and `core/` in full mode). Defaults to this repository. */
  stackRoot: resolve(env.E2E_STACK_ROOT ?? resolve(e2eDir, '..', '..')),
  e2eDir,
  /** `managed` = the suite brings the stack up and down; `external` = already running (E2E_ENV_FILE required). */
  stack: (env.E2E_STACK === 'external' ? 'external' : 'managed') as 'managed' | 'external',
  /** Keep the managed stack running after the suite (for debugging). */
  keepStack: env.E2E_KEEP_STACK === '1',
  /** Compose project name. A fresh one per run keeps runs isolated from developer stacks. */
  project: env.E2E_COMPOSE_PROJECT,
  /** Env file for compose. Managed mode generates one in the run directory when unset. */
  envFile: env.E2E_ENV_FILE,
  /** Run directory for state and captured logs; outside the repository, mode 0700, removed at the end. */
  runDir: resolve(env.E2E_RUN_DIR ?? resolve(tmpdir(), env.E2E_COMPOSE_PROJECT)),
  /** Prefix docker calls with `sg docker -c` when the user is not in the docker group (auto|1|0). */
  dockerSg: env.E2E_DOCKER_SG ?? 'auto',

  /**
   * Seed command: prints one token `awk_<key_id>_<secret>` on stdout. Gets SEED_LABEL (a distinct user per label)
   * and E2E_COMPOSE. Default: the Core's dev command (needs ALLWIN_ENABLE_DEV_COMMANDS=true in the compose).
   */
  seedCmd: env.E2E_SEED_CMD ?? '$E2E_COMPOSE exec -T core allwin-core seed "$SEED_LABEL"',
  /** Revoke command for M5. Gets KEY_ID and E2E_COMPOSE. */
  revokeCmd: env.E2E_REVOKE_CMD ?? '$E2E_COMPOSE exec -T core allwin-core revoke-key "$KEY_ID"',
  /** Metrics path on the Core's direct loopback port. Empty string = metrics not searched (reported). */
  metricsPath: env.E2E_METRICS_PATH ?? '/metrics',
  /** Instrument symbols enabled in the seeded catalog (AC-7). */
  enabledSymbols: (env.E2E_ENABLED_SYMBOLS ?? 'MNQ').split(',').map((s) => s.trim()).filter(Boolean),
  /** Route templates the Core may report in `http.route` besides the ingress ones (M10). */
  extraRoutes: (env.E2E_EXTRA_ROUTES ?? '').split(',').map((s) => s.trim()).filter(Boolean),

  /** SQL relations (wave1-data-model §6). Overridable if the Core lands different names. */
  sql: {
    instruction: env.E2E_SQL_INSTRUCTION ?? 'routing.instruction',
    receipt: env.E2E_SQL_RECEIPT ?? 'routing.message_receipt',
    audit: env.E2E_SQL_AUDIT ?? 'audit.audit_entry',
  },
  /** How long to wait for asynchronous audit rows (outbox dispatcher). */
  auditWaitMs: int('E2E_AUDIT_WAIT_MS', 15_000),

  latency: {
    // Below the per-key burst (60/min) of the dedicated latency key, warm-up included.
    samples: int('E2E_LATENCY_SAMPLES', 50),
    pauseMs: int('E2E_LATENCY_PAUSE_MS', 50),
    p95Ms: int('E2E_LATENCY_P95_MS', 300),
  },
  rateLimit: {
    /** Upper bound of requests in an M8 burst before declaring "no 429". */
    maxBurst: int('E2E_RATE_MAX_BURST', 600),
    /** Longest Retry-After the suite is willing to wait for between disruptive steps. */
    maxRetryAfterS: int('E2E_RATE_MAX_RETRY_AFTER_S', 120),
  },
  timeouts: {
    stackUpMs: int('E2E_STACK_UP_MS', 15 * 60_000),
    readyMs: int('E2E_READY_MS', 180_000),
  },
};

export const INGRESS_PREFIX = '/ingress/v1/tradingview/';
export const TOKEN_RE = /^awk_([A-Za-z0-9]{12})_([A-Za-z0-9_-]{43})$/;
export const INGRESS_ROUTE = '/ingress/v1/tradingview/{token}';
