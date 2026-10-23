"use client";

import { useEffect, useState } from "react";

type FragmentToken = { checked: boolean; token: string | null };

/**
 * Reads the one-time token from `#t=<token>` and removes the fragment at once, so it is not in history,
 * bookmarks or screenshots of the address bar (S3 spec §1). The token lives only in component state.
 */
export function useFragmentToken(): FragmentToken {
  const [state, setState] = useState<FragmentToken>({ checked: false, token: null });
  useEffect(() => {
    /* The fragment exists only in the browser, so it is read once after mount, then removed. */
    /* eslint-disable react-hooks/set-state-in-effect */
    const match = /^#t=([^&]+)/.exec(window.location.hash);
    if (match?.[1]) {
      let token: string | null = null;
      try {
        token = decodeURIComponent(match[1]);
      } catch {
        // A malformed fragment is an invalid link; it is still removed from the address bar.
      }
      window.history.replaceState(window.history.state, "", `${window.location.pathname}${window.location.search}`);
      setState({ checked: true, token });
    } else {
      // A second effect run (React strict mode) finds the fragment already gone: keep the token read by the first.
      setState((current) => ({ checked: true, token: current.token }));
    }
    /* eslint-enable react-hooks/set-state-in-effect */
  }, []);
  return state;
}
