let handler: (() => void) | null = null;

/** The app registers what to do when any request answers `auth.unauthenticated`; returns the unregister function. */
export function onSessionExpired(next: () => void): () => void {
  handler = next;
  return () => {
    if (handler === next) handler = null;
  };
}

export function notifySessionExpired(): void {
  handler?.();
}
