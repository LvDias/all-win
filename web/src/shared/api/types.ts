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

type Json<T> = T extends { content: { "application/json": infer B } } ? B : never;
/** Response bodies and request bodies taken from the generated paths, so contract drift breaks the build. */
export type TotpSetup = Json<paths["/api/v1/me/totp/setup"]["post"]["responses"][200]>;
export type InvitationCreated = Json<paths["/api/v1/admin/invitations"]["post"]["responses"][201]>;
export type ResetLink = Json<paths["/api/v1/admin/users/{user_id}/password-reset-link"]["post"]["responses"][201]>;
export type CreatedConnection = Json<paths["/api/v1/connections"]["post"]["responses"][201]>;
export type InvitationLookup = Json<paths["/api/v1/auth/invitations/lookup"]["post"]["responses"][200]>;
export type AcceptInvitationBody = Json<NonNullable<paths["/api/v1/auth/invitations/accept"]["post"]["requestBody"]>>;
export type ApiLocale = components["schemas"]["Locale"];
