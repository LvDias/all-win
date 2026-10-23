// Raw HTTP(S) client. The request target is sent byte-for-byte (no URL normalization), so
// near-miss paths such as `//ingress`, `/%69ngress` or `/ingress/v1/tradingview//awk_…` reach the edge as written.
import https from 'node:https';
import http from 'node:http';
import { performance } from 'node:perf_hooks';

export interface RawResponse {
  status: number;
  headers: http.IncomingHttpHeaders;
  body: string;
  ms: number;
}

export interface RawRequest {
  method?: string;
  /** Request target exactly as it must appear on the request line. */
  target: string;
  headers?: Record<string, string>;
  body?: string | Buffer;
  timeoutMs?: number;
}

const agents = new Map<string, https.Agent | http.Agent>();

function agentFor(url: URL): https.Agent | http.Agent {
  const key = url.origin;
  let a = agents.get(key);
  if (!a) {
    // Self-signed internal CA of the loopback dev edge only; never used against a public host.
    if (url.protocol === 'https:' && !['localhost', '127.0.0.1'].includes(url.hostname)) {
      throw new Error('TLS verification is only relaxed for loopback targets');
    }
    a = url.protocol === 'https:' ? new https.Agent({ keepAlive: true, rejectUnauthorized: false }) : new http.Agent({ keepAlive: true });
    agents.set(key, a);
  }
  return a;
}

export function closeAgents(): void {
  for (const a of agents.values()) a.destroy();
  agents.clear();
}

export function send(origin: string, req: RawRequest): Promise<RawResponse> {
  const url = new URL(origin);
  const lib = url.protocol === 'https:' ? https : http;
  const body = req.body === undefined ? undefined : Buffer.isBuffer(req.body) ? req.body : Buffer.from(req.body, 'utf8');
  const headers: Record<string, string> = { ...(req.headers ?? {}) };
  if (body !== undefined) headers['content-length'] = String(body.length);
  return new Promise((resolvePromise, reject) => {
    const started = performance.now();
    const r = lib.request(
      {
        protocol: url.protocol,
        hostname: url.hostname,
        port: url.port,
        method: req.method ?? 'POST',
        path: req.target,
        headers,
        agent: agentFor(url),
        servername: url.protocol === 'https:' ? url.hostname : undefined,
      },
      (res) => {
        const chunks: Buffer[] = [];
        res.on('data', (c: Buffer) => chunks.push(c));
        res.on('end', () =>
          resolvePromise({
            status: res.statusCode ?? 0,
            headers: res.headers,
            body: Buffer.concat(chunks).toString('utf8'),
            ms: performance.now() - started,
          }),
        );
        res.on('error', reject);
      },
    );
    r.setTimeout(req.timeoutMs ?? 10_000, () => r.destroy(new Error('request timed out')));
    r.on('error', reject);
    if (body !== undefined) r.write(body);
    r.end();
  });
}

export const JSON_HEADERS = { 'content-type': 'application/json' };
