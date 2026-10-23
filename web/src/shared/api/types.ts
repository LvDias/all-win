import type { components } from "./schema.gen";

export type Me = components["schemas"]["Me"];
export type UpdateMeBody = { locale?: components["schemas"]["Locale"]; time_zone?: string };
