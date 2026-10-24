"use client";

import { useQueryClient } from "@tanstack/react-query";
import { useLocale } from "next-intl";
import { useRouter } from "next/navigation";
import { createContext, useContext, useEffect, useMemo, useRef, type ReactNode } from "react";
import type { ApiEvent } from "@/shared/api/types";
import { isLocale, defaultLocale } from "@/shared/i18n/config";
import { useMe } from "@/shared/session/me";
import { loginPath, loginReasonFor } from "@/shared/session/redirects";

type Listener = (event: ApiEvent) => void;
type Events = { subscribe: (listener: Listener) => () => void };

const EventsContext = createContext<Events | null>(null);

const eventTypes = ["connection.status_changed", "connection.paired", "connection.pairing_code_invalidated", "risk.state_changed", "trade.captured", "session.ended"] as const;

/** Server state shown on screen: refetched after every reconnect, since events are not replayed (OpenAPI `eventStream`). */
const liveQueryKeys = [["sources"], ["connections"], ["me", "security"], ["risk"], ["trades"]] as const;

function parse(data: unknown): ApiEvent | null {
  if (typeof data !== "string") return null;
  try {
    const value: unknown = JSON.parse(data);
    if (typeof value === "object" && value !== null && "type" in value && typeof value.type === "string" && (eventTypes as readonly string[]).includes(value.type)) return value as ApiEvent;
  } catch {
    // A malformed frame is ignored; the next refetch restores the state.
  }
  return null;
}

/** Opens the SSE stream while signed in, fans events out, and ends the session on `session.ended`. */
export function EventsProvider({ children }: { children: ReactNode }) {
  const me = useMe();
  const signedIn = me.data != null;
  const queryClient = useQueryClient();
  const router = useRouter();
  const current = useLocale();
  const locale = isLocale(current) ? current : defaultLocale;
  const listeners = useRef(new Set<Listener>());

  useEffect(() => {
    if (!signedIn) return;
    const refetch = () => {
      for (const queryKey of liveQueryKeys) void queryClient.invalidateQueries({ queryKey });
    };
    const source = new EventSource("/api/v1/events");
    let opened = false;
    source.onopen = () => {
      if (opened) refetch();
      opened = true;
    };
    const onVisible = () => {
      if (document.visibilityState === "visible") refetch();
    };
    document.addEventListener("visibilitychange", onVisible);
    for (const type of eventTypes) {
      source.addEventListener(type, (message) => {
        const event = parse((message as MessageEvent).data);
        if (!event) return;
        if (event.type === "session.ended") {
          source.close();
          queryClient.clear();
          router.replace(loginPath(locale, { reason: loginReasonFor(event.reason) }));
          return;
        }
        for (const listener of listeners.current) listener(event);
      });
    }
    return () => {
      document.removeEventListener("visibilitychange", onVisible);
      source.close();
    };
  }, [signedIn, queryClient, router, locale]);

  const value = useMemo<Events>(
    () => ({
      subscribe: (listener) => {
        listeners.current.add(listener);
        return () => listeners.current.delete(listener);
      },
    }),
    [],
  );
  return <EventsContext.Provider value={value}>{children}</EventsContext.Provider>;
}

/** Subscribes to stream events (except `session.ended`, which the provider handles). */
export function useEvents(handler: Listener): void {
  const context = useContext(EventsContext);
  const latest = useRef(handler);
  useEffect(() => {
    latest.current = handler;
  });
  useEffect(() => {
    if (!context) return;
    return context.subscribe((event) => latest.current(event));
  }, [context]);
}
