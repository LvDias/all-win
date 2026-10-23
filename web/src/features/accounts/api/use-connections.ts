"use client";

import { useQuery } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import type { components } from "@/shared/api/schema.gen";

export type Connection = components["schemas"]["Connection"];

/** The user's NinjaTrader connections with their reported platform accounts (read-only here; pairing is S3). */
export const useConnections = () =>
  useQuery({ queryKey: ["connections"], queryFn: ({ signal }) => apiFetch<Connection[]>("/api/v1/connections", { signal }) });
