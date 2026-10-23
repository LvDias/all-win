"use client";

import { useQueries, useQuery } from "@tanstack/react-query";
import { apiFetch } from "@/shared/api/client";
import { segment } from "@/shared/api/ids";
import type { components } from "@/shared/api/schema.gen";
import type { FirmDetail, PresetVersion } from "./types";

/** Presets are shared reference data: the keys carry no user id, the Core scopes nothing by user here. */
export const presetKeys = {
  all: ["presets"] as const,
  firms: () => [...presetKeys.all, "firms"] as const,
  firm: (id: string) => [...presetKeys.all, "firm", id] as const,
  releases: (id: string) => [...presetKeys.all, "releases", id] as const,
  version: (id: string) => [...presetKeys.all, "version", id] as const,
};

export const useFirms = () =>
  useQuery({ queryKey: presetKeys.firms(), queryFn: ({ signal }) => apiFetch<components["schemas"]["FirmList"]>("/api/v1/firms", { signal }) });

export const useFirm = (firmId: string) =>
  useQuery({ queryKey: presetKeys.firm(firmId), queryFn: ({ signal }) => apiFetch<FirmDetail>(`/api/v1/firms/${segment(firmId)}`, { signal }) });

export const useFirmReleases = (firmId: string, enabled: boolean) =>
  useQuery({
    queryKey: presetKeys.releases(firmId),
    enabled,
    queryFn: ({ signal }) => apiFetch<components["schemas"]["FirmReleaseList"]>(`/api/v1/firms/${segment(firmId)}/releases`, { signal }),
  });

/** One preset version per size row (the firm detail lists only ids). */
export const usePresetVersions = (ids: string[]) =>
  useQueries({
    queries: ids.map((id) => ({ queryKey: presetKeys.version(id), queryFn: ({ signal }: { signal: AbortSignal }) => apiFetch<PresetVersion>(`/api/v1/preset-versions/${segment(id)}`, { signal }) })),
  });
