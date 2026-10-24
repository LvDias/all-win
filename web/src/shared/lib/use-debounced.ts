import { useEffect, useState } from "react";

/** The value after it stopped changing for `delayMs` (for example the 300 ms preview of a form being typed). */
export function useDebounced<T>(value: T, delayMs: number): T {
  const [settled, setSettled] = useState(value);
  useEffect(() => {
    const timer = window.setTimeout(() => setSettled(value), delayMs);
    return () => window.clearTimeout(timer);
  }, [value, delayMs]);
  return settled;
}
