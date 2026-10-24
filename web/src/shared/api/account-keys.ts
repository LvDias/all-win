/**
 * The one TanStack Query key family for accounts, shared by every feature that reads or changes an account
 * (S5 list and detail, S13 restart-evaluation). A feature that changes an account invalidates `accountKeys.all`,
 * so every screen gets the new `row_version` and none sends a stale one.
 */
export const accountKeys = {
  all: ["accounts"] as const,
  list: () => [...accountKeys.all, "list"] as const,
  detail: (id: string) => [...accountKeys.all, "detail", id] as const,
};
