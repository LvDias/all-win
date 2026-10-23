import type { components, paths } from "./schema.gen";

export type Me = components["schemas"]["Me"];
export type UpdateMeBody = NonNullable<paths["/api/v1/me"]["patch"]["requestBody"]>["content"]["application/json"];

type Schemas = components["schemas"];
export type SecurityState = Schemas["SecurityState"];
export type LoginResult = Schemas["LoginResult"];
export type Source = Schemas["Source"];
export type SourceWithUrl = Schemas["SourceWithUrl"];
export type Connection = Schemas["Connection"];
export type PairingCode = Schemas["PairingCode"];
export type VisibleAccount = Schemas["VisibleAccount"];
export type Invitation = Schemas["Invitation"];
export type AdminUser = Schemas["AdminUser"];
export type RecoveryCodes = Schemas["RecoveryCodes"];
export type ApiEvent = Schemas["Event"];
