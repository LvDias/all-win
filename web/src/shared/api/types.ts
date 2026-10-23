import type { components, paths } from "./schema.gen";

export type Me = components["schemas"]["Me"];
export type UpdateMeBody = NonNullable<paths["/api/v1/me"]["patch"]["requestBody"]>["content"]["application/json"];
