const uuid = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

/**
 * A route parameter that must be a resource id. Returns the id when it is a UUID, else null, so a page can show
 * not-found instead of sending a crafted value (`../`, `?`, `%2e`) to the API.
 */
export function parseRouteId(value: string | undefined): string | null {
  return value !== undefined && uuid.test(value) ? value : null;
}

/** One path segment of an API URL, always percent-encoded (defence in depth next to `parseRouteId`). */
export const segment = (value: string): string => encodeURIComponent(value);
