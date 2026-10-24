import type { components, paths } from "./schema.gen";

type Schemas = components["schemas"];
export type Me = Schemas["Me"];
export type UpdateMeBody = NonNullable<paths["/api/v1/me"]["patch"]["requestBody"]>["content"]["application/json"];
export type SecurityState = Schemas["SecurityState"];
export type LoginResult = Schemas["LoginResult"];
export type Source = Schemas["Source"];
export type SourceWithUrl = Schemas["SourceWithUrl"];
export type Connection = Schemas["Connection"];
export type PairingCode = Schemas["PairingCode"];
export type VisibleAccount = Schemas["VisibleAccount"];
export type Invitation = Schemas["Invitation"];
export type ApiEvent = Schemas["Event"];
export type ChallengeIssued = Schemas["ChallengeIssued"];
export type ChannelChange = Schemas["ChannelChange"];
export type Device = Schemas["Device"];
export type StepUpProof = Schemas["StepUpProof"];
export type StepUpAction = Schemas["StepUpAction"];
export type ApiLocale = Schemas["Locale"];

type Json<T> = T extends { content: { "application/json": infer B } } ? B : never;
/** Response bodies and request bodies taken from the generated paths, so contract drift breaks the build. */
export type InvitationCreated = Json<paths["/api/v1/admin/invitations"]["post"]["responses"][201]>;
export type CreatedConnection = Json<paths["/api/v1/connections"]["post"]["responses"][201]>;
export type InvitationLookup = Json<paths["/api/v1/auth/invitations/lookup"]["post"]["responses"][200]>;
export type ChallengeDelivery = Json<paths["/api/v1/auth/challenges/delivery"]["post"]["responses"][200]>["delivery"];
