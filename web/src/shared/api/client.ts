export class ApiError extends Error {
  readonly code: string;
  readonly status: number;
  readonly pointer: string | undefined;
  readonly retryAfter: number | undefined;

  constructor(init: { code: string; status: number; pointer?: string; retryAfter?: number }) {
    super(init.code);
    this.name = "ApiError";
    this.code = init.code;
    this.status = init.status;
    this.pointer = init.pointer;
    this.retryAfter = init.retryAfter;
  }
}

/** Raised when the request never reached the Core (offline, DNS, reset). */
export const networkErrorCode = "common.network";

type ErrorBody = { error?: { code?: unknown; pointer?: unknown; retry_after_s?: unknown } };

export async function apiFetch<T>(path: string, init?: { method?: string; body?: unknown; signal?: AbortSignal }): Promise<T> {
  const url = new URL(path, window.location.origin);
  let response: Response;
  try {
    response = await fetch(url, {
      method: init?.method ?? "GET",
      credentials: "same-origin",
      headers: init?.body === undefined ? { Accept: "application/json" } : { Accept: "application/json", "Content-Type": "application/json" },
      body: init?.body === undefined ? undefined : JSON.stringify(init.body),
      signal: init?.signal,
    });
  } catch (cause) {
    if (cause instanceof DOMException && cause.name === "AbortError") throw cause;
    throw new ApiError({ code: networkErrorCode, status: 0 });
  }

  if (response.ok) {
    // 202/204 answers carry no body (for example the always-202 password-reset request).
    const text = response.status === 204 ? "" : await response.text();
    return (text === "" ? undefined : JSON.parse(text)) as T;
  }

  let body: ErrorBody = {};
  try {
    body = (await response.json()) as ErrorBody;
  } catch {
    // Non-JSON error bodies (proxy errors) fall through to the generic code.
  }
  const retryHeader = Number(response.headers.get("Retry-After"));
  throw new ApiError({
    code: typeof body.error?.code === "string" ? body.error.code : `http.${response.status}`,
    status: response.status,
    pointer: typeof body.error?.pointer === "string" ? body.error.pointer : undefined,
    retryAfter: typeof body.error?.retry_after_s === "number" ? body.error.retry_after_s : retryHeader > 0 ? retryHeader : undefined,
  });
}
