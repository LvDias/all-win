/**
 * Page security headers. The Next.js server owns every page header; only HSTS lives at the reverse proxy.
 * `style-src` keeps `'unsafe-inline'` because Radix and Tailwind set inline styles; scripts are nonce-only.
 */
export function buildCsp(nonce: string, development: boolean): string {
  const directives = [
    "default-src 'self'",
    `script-src 'self' 'nonce-${nonce}' 'strict-dynamic'${development ? " 'unsafe-eval'" : ""}`,
    "style-src 'self' 'unsafe-inline'",
    "img-src 'self' data:",
    "font-src 'self'",
    "connect-src 'self'",
    "object-src 'none'",
    "base-uri 'none'",
    "frame-ancestors 'none'",
    "form-action 'self'",
  ];
  return directives.join("; ");
}

export function securityHeaders(nonce: string, development: boolean): Record<string, string> {
  return {
    "Content-Security-Policy": buildCsp(nonce, development),
    "X-Content-Type-Options": "nosniff",
    "Referrer-Policy": "strict-origin-when-cross-origin",
    "Permissions-Policy": "camera=(), microphone=(), geolocation=()",
  };
}

/** 128 random bits, base64. A new value for every request. */
export function createNonce(): string {
  return btoa(String.fromCharCode(...crypto.getRandomValues(new Uint8Array(16))));
}
