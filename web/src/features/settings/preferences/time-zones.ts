export type TimeZoneOption = {
  id: string;
  region: string;
  label: string;
  /** Lowercased, accent-free text matched by the search box. */
  haystack: string;
};

export type TimeZoneGroup = { region: string; options: TimeZoneOption[] };

function normalize(text: string): string {
  return text.normalize("NFD").replace(/\p{Diacritic}/gu, "").toLowerCase();
}

/** `GMT-03:00` for display only; the browser computes the current offset. */
export function offsetLabel(timeZone: string, at: Date): string {
  const part = new Intl.DateTimeFormat("en-US", { timeZone, timeZoneName: "longOffset" }).formatToParts(at).find((p) => p.type === "timeZoneName");
  const value = part?.value ?? "GMT";
  return value === "GMT" ? "GMT+00:00" : value;
}

export function buildTimeZoneOptions(zones: readonly string[], at: Date): TimeZoneOption[] {
  return zones.map((id) => {
    const [region = id, ...rest] = id.split("/");
    const city = (rest.length > 0 ? rest.join(" / ") : id).replaceAll("_", " ");
    const label = `(${offsetLabel(id, at)}) ${city}`;
    return { id, region: rest.length > 0 ? region : id, label, haystack: normalize(`${label} ${id}`) };
  });
}

export function filterTimeZones(options: readonly TimeZoneOption[], query: string): TimeZoneOption[] {
  const needle = normalize(query.trim());
  return needle === "" ? [...options] : options.filter((option) => option.haystack.includes(needle));
}

export function groupTimeZones(options: readonly TimeZoneOption[]): TimeZoneGroup[] {
  const groups = new Map<string, TimeZoneOption[]>();
  for (const option of options) groups.set(option.region, [...(groups.get(option.region) ?? []), option]);
  return [...groups].map(([region, items]) => ({ region, options: items }));
}

export function supportedTimeZones(selected: string): string[] {
  const zones = Intl.supportedValuesOf("timeZone");
  return zones.includes(selected) ? zones : [...zones, selected].sort();
}
