//! S3 request and response bodies (`contracts/http/openapi.yaml`). Shared components (`Error`,
//! `Uuid`, `Timestamp`, `Locale`, `TimeZone`, `DisplayName`) come from `contracts::http`.
//! Request types are validated against their own schema by [`super::guard::ValidJson`].

use chrono::{DateTime, SecondsFormat, Utc};
use contracts::http::{DisplayName, Locale, TimeZone, Timestamp, Uuid};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub fn ts(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub fn locale_out(l: identity_enterprise::profile::Locale) -> Locale {
    match l {
        identity_enterprise::profile::Locale::PtBr => Locale::PtBr,
        identity_enterprise::profile::Locale::En => Locale::En,
    }
}

pub fn locale_in(l: Locale) -> identity_enterprise::profile::Locale {
    match l {
        Locale::PtBr => identity_enterprise::profile::Locale::PtBr,
        Locale::En => identity_enterprise::profile::Locale::En,
    }
}

/// Integer without a `format` (the contract declares none), with optional bounds.
macro_rules! plain_integer {
    ($(#[$meta:meta])* $name:ident, $min:expr, $max:expr) => {
        $(#[$meta])*
        pub struct $name;

        impl utoipa::PartialSchema for $name {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                utoipa::openapi::schema::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .minimum($min)
                    .maximum($max)
                    .into()
            }
        }

        impl ToSchema for $name {}
    };
}

plain_integer!(
    /// A duration in whole seconds.
    Seconds,
    None::<i64>,
    None::<i64>
);

// ---------- requests ----------

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct TokenBody {
    #[schema(max_length = 128)]
    pub token: String,
}

/// Sealed challenge token (auth policy §3); opaque to clients. Never logged.
pub struct ChallengeId;

impl utoipa::PartialSchema for ChallengeId {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .pattern(Some("^[A-Za-z0-9_-]{16,256}$"))
            .into()
    }
}

impl ToSchema for ChallengeId {}

/// The code as typed; the Core normalizes it (U16 format 1). Never logged.
pub struct OneTimeCode;

impl utoipa::PartialSchema for OneTimeCode {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .min_length(Some(1))
            .max_length(Some(32))
            .into()
    }
}

impl ToSchema for OneTimeCode {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CodeChannel {
    Email,
    Whatsapp,
}

impl CodeChannel {
    pub fn domain(self) -> identity_enterprise::channel::Channel {
        match self {
            Self::Email => identity_enterprise::channel::Channel::Email,
            Self::Whatsapp => identity_enterprise::channel::Channel::Whatsapp,
        }
    }

    pub fn of(c: identity_enterprise::channel::Channel) -> Self {
        match c {
            identity_enterprise::channel::Channel::Email => Self::Email,
            identity_enterprise::channel::Channel::Whatsapp => Self::Whatsapp,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StepUpAction {
    SourceCreate,
    SourceRotate,
    ConnectionCreate,
    PairingCodeNew,
    ConnectionUnpair,
    AccountOptInAccept,
    AccountCreateOptInAccept,
    AdminInvitationCreate,
    RouteEnable,
    AccountBindingChange,
}

impl StepUpAction {
    pub fn domain(self) -> access::StepUpAction {
        use access::StepUpAction as A;
        match self {
            Self::SourceCreate => A::SourceCreate,
            Self::SourceRotate => A::SourceRotate,
            Self::ConnectionCreate => A::ConnectionCreate,
            Self::PairingCodeNew => A::PairingCodeNew,
            Self::ConnectionUnpair => A::ConnectionUnpair,
            Self::AccountOptInAccept => A::AccountOptInAccept,
            Self::AccountCreateOptInAccept => A::AccountCreateOptInAccept,
            Self::AdminInvitationCreate => A::AdminInvitationCreate,
            Self::RouteEnable => A::RouteEnable,
            Self::AccountBindingChange => A::AccountBindingChange,
        }
    }
}

/// Step-up proof carried in the body of a step-up operation; consumed in the action's transaction.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct StepUpProof {
    #[schema(value_type = ChallengeId)]
    pub challenge_id: String,
    #[schema(value_type = OneTimeCode)]
    pub code: String,
}

impl StepUpProof {
    pub fn domain(self) -> access::StepUpProof {
        access::StepUpProof {
            challenge_id: self.challenge_id,
            code: self.code,
        }
    }
}

/// Body of a step-up operation that has no other input.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct StepUpBody {
    #[schema(nullable = false)]
    pub step_up: Option<StepUpProof>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RequestLoginCodeRequest {
    #[schema(format = Email, max_length = 254)]
    pub email: String,
    pub channel: CodeChannel,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct VerifyLoginCodeRequest {
    #[schema(value_type = ChallengeId)]
    pub challenge_id: String,
    #[schema(value_type = OneTimeCode)]
    pub code: String,
    pub remember_me: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ChallengeBody {
    #[schema(value_type = ChallengeId)]
    pub challenge_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RequestStepUpRequest {
    pub action: StepUpAction,
    /// Required for source_rotate, pairing_code_new, connection_unpair, account_opt_in_accept, route_enable, account_binding_change; forbidden otherwise.
    #[schema(value_type = Option<Uuid>, nullable = false)]
    pub target_id: Option<String>,
    pub channel: CodeChannel,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CompleteSignupRequest {
    #[schema(value_type = ChallengeId)]
    pub challenge_id: String,
    #[schema(value_type = OneTimeCode)]
    pub code: String,
    pub locale: Locale,
    #[schema(value_type = TimeZone)]
    pub time_zone: String,
    pub remember_me: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct StartChannelChangeRequest {
    pub channel: CodeChannel,
    /// An e-mail address for `email`; an E.164 number (`+5511…`) for `whatsapp`.
    #[schema(max_length = 254)]
    pub new_value: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct VerifyChannelChangeCodeRequest {
    #[schema(value_type = ChallengeId)]
    pub challenge_id: String,
    #[schema(value_type = OneTimeCode)]
    pub code: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[schema(min_properties = 1)]
pub struct UpdateMeRequest {
    #[schema(nullable = false)]
    pub locale: Option<Locale>,
    #[schema(value_type = Option<TimeZone>, nullable = false)]
    pub time_zone: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateInvitationRequest {
    #[schema(format = Email, max_length = 254)]
    pub email: String,
    pub locale: Locale,
    #[schema(default = true, nullable = false)]
    pub send_email: Option<bool>,
    #[schema(nullable = false)]
    pub step_up: Option<StepUpProof>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SourceTypeDto {
    Tradingview,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateSourceRequest {
    #[schema(value_type = DisplayName)]
    pub name: String,
    pub source_type: SourceTypeDto,
    #[schema(nullable = false)]
    pub step_up: Option<StepUpProof>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateConnectionRequest {
    #[schema(value_type = DisplayName)]
    pub name: String,
    #[schema(nullable = false)]
    pub step_up: Option<StepUpProof>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RenameRequest {
    #[schema(value_type = DisplayName)]
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UnpairRequest {
    /// Must be true when any mapped account has an open position (design S3-10 checkbox).
    pub acknowledge_open_positions: bool,
    #[schema(nullable = false)]
    pub step_up: Option<StepUpProof>,
}

#[derive(Debug, Clone, Copy, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AdapterBuildDto {
    Release,
    Development,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdapterDto {
    #[serde(rename = "type")]
    #[schema(pattern = "^[a-z][a-z0-9_]{0,31}$")]
    pub adapter_type: String,
    #[schema(max_length = 32)]
    pub version: String,
    pub build: AdapterBuildDto,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PairAdapterRequest {
    /// 8 Crockford Base32 characters; case, spaces and hyphens ignored; O→0, I/L→1.
    #[schema(max_length = 16)]
    pub pairing_code: String,
    #[schema(value_type = Uuid)]
    pub instance_id: String,
    #[schema(min_length = 1, max_length = 64)]
    pub instance_name: String,
    pub adapter: AdapterDto,
}

// ---------- responses ----------

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Me {
    #[schema(value_type = Uuid)]
    pub id: String,
    #[schema(format = Email)]
    pub email: String,
    pub locale: Locale,
    #[schema(value_type = TimeZone)]
    pub time_zone: String,
    pub is_platform_admin: bool,
}

impl From<&identity_enterprise::user::User> for Me {
    fn from(u: &identity_enterprise::user::User) -> Self {
        Self {
            id: u.id.to_string(),
            email: u.email.as_entered().to_owned(),
            locale: locale_out(u.locale),
            time_zone: u.time_zone.as_str().to_owned(),
            is_platform_admin: u.is_platform_admin,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct LoginResult {
    pub user: Me,
    /// True when this sign-in registered a new device (the notice e-mail was sent; never on sign-up).
    pub new_device: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ChallengeIssued {
    #[schema(value_type = ChallengeId)]
    pub challenge_id: String,
    #[schema(value_type = Timestamp)]
    pub expires_at: String,
    /// Seconds until `resendCode` is accepted (60 after a send).
    #[schema(value_type = ResendAfter)]
    pub resend_after_s: i64,
}

plain_integer!(
    /// Seconds until a resend is accepted.
    ResendAfter,
    Some(0),
    None::<i64>
);

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryStateDto {
    Pending,
    Sent,
    Failed,
    NotReported,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ChallengeDelivery {
    pub delivery: DeliveryStateDto,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Device {
    #[schema(value_type = Uuid)]
    pub id: String,
    /// Coarse browser and OS family from the User-Agent, e.g. "Chrome on Windows".
    #[schema(max_length = 128)]
    pub label: String,
    #[schema(value_type = Timestamp)]
    pub first_seen_at: String,
    #[schema(value_type = Timestamp)]
    pub last_seen_at: String,
    /// True for the device of this request.
    pub current: bool,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChangeStatusDto {
    AwaitingCodes,
    OnHold,
    Completed,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CodeRoleDto {
    CurrentValue,
    NewValue,
    OtherChannel,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ChangeCode {
    pub role: CodeRoleDto,
    #[schema(value_type = ChallengeId)]
    pub challenge_id: String,
    pub confirmed: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ChannelChange {
    #[schema(value_type = Uuid)]
    pub id: String,
    pub channel: CodeChannel,
    /// `on_hold` = every code confirmed, the change waits until `hold_until` (auth policy §9 hold; §7 M2) and can be cancelled.
    pub status: ChangeStatusDto,
    #[schema(max_length = 64)]
    pub masked_new_value: String,
    /// One entry per code of the flow.
    pub codes: Vec<ChangeCode>,
    #[schema(value_type = Timestamp)]
    pub expires_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Timestamp>, nullable = false)]
    pub hold_until: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SessionInfo {
    pub persistent: bool,
    /// 43200 (12 h) or 604800 (7 d) — Q21.
    #[schema(value_type = Seconds)]
    pub idle_timeout_s: i64,
    /// 86400 (24 h) or 2592000 (30 d) — Q21.
    #[schema(value_type = Seconds)]
    pub absolute_timeout_s: i64,
    #[schema(value_type = Timestamp)]
    pub idle_expires_at: String,
    #[schema(value_type = Timestamp)]
    pub absolute_expires_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SecurityState {
    #[schema(format = Email)]
    pub email: String,
    /// Confirmed WhatsApp number (E.164), absent until added (S3b).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(pattern = "^\\+[1-9][0-9]{7,14}$", nullable = false)]
    pub whatsapp: Option<String>,
    /// When a new channel change is accepted (24 h after the last one; auth policy §7). In the past when allowed now.
    #[schema(value_type = Timestamp)]
    pub channel_change_allowed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub pending_channel_change: Option<ChannelChange>,
    pub session: SessionInfo,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum InvitationStatusDto {
    Pending,
    Accepted,
    Revoked,
    Expired,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Invitation {
    #[schema(value_type = Uuid)]
    pub id: String,
    #[schema(format = Email)]
    pub email: String,
    pub status: InvitationStatusDto,
    #[schema(value_type = Timestamp)]
    pub created_at: String,
    #[schema(value_type = Timestamp)]
    pub expires_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Timestamp>, nullable = false)]
    pub accepted_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Timestamp>, nullable = false)]
    pub revoked_at: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreatedInvitation {
    pub invitation: Invitation,
    /// https://<host>/<locale>/invite#t=<token>
    pub invite_url: String,
    /// False when e-mail is disabled or failed (`invites.email_send_failed` is then logged); the admin delivers the link manually.
    pub email_sent: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct InvitationLookup {
    #[schema(format = Email)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub locale: Option<Locale>,
    #[schema(value_type = Timestamp)]
    pub expires_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserStatusDto {
    Active,
    Closed,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminUser {
    #[schema(value_type = Uuid)]
    pub id: String,
    #[schema(format = Email)]
    pub email: String,
    pub status: UserStatusDto,
    pub whatsapp_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub is_platform_admin: Option<bool>,
    #[schema(value_type = Timestamp)]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Timestamp>, nullable = false)]
    pub last_sign_in_at: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SourceStatusDto {
    Active,
    Revoked,
}

/// `[A-Za-z0-9]{12}`, the public part of a token.
pub struct KeyIdString;

impl utoipa::PartialSchema for KeyIdString {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .pattern(Some("^[A-Za-z0-9]{12}$"))
            .into()
    }
}

impl ToSchema for KeyIdString {}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Source {
    #[schema(value_type = Uuid)]
    pub id: String,
    #[schema(value_type = DisplayName)]
    pub name: String,
    pub source_type: SourceTypeDto,
    /// Derived by the Core from `source_type` (never stored, never set by the client): true when
    /// instructions from this source are subject to the per-account `automation` opt-in (O1).
    /// `tradingview` is automated. Clients must use this flag, not their own list of types.
    pub is_automated: bool,
    pub status: SourceStatusDto,
    #[schema(value_type = KeyIdString)]
    pub key_id: String,
    /// 'https://<host>/ingress/v1/tradingview/awk_<key_id>_…' — never the secret.
    pub masked_url: String,
    #[schema(value_type = Timestamp)]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Timestamp>, nullable = false)]
    pub rotated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Timestamp>, nullable = false)]
    pub revoked_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Timestamp>, nullable = false)]
    pub last_received_at: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SourceWithUrl {
    pub source: Source,
    /// Full URL with the secret token. Returned only by create/rotate; never logged.
    #[schema(
        pattern = "^https://[^/]+/ingress/v1/tradingview/awk_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}$"
    )]
    pub webhook_url: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PairingCode {
    /// Display form, returned once per code.
    #[schema(pattern = "^[0-9A-HJKMNP-TV-Z]{4}-[0-9A-HJKMNP-TV-Z]{4}$")]
    pub pairing_code: String,
    #[schema(value_type = Timestamp)]
    pub expires_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionStatusDto {
    AwaitingPairing,
    Connected,
    Disconnected,
    Outdated,
    Unpaired,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct VisibleAccount {
    #[schema(max_length = 128)]
    pub platform_account: String,
    pub kind: VisibleAccountKind,
    pub oco_location: OcoLocation,
    /// The All Win account bound to it, if any (S5).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Uuid>, nullable = false)]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum VisibleAccountKind {
    Simulation,
    Playback,
    Live,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum OcoLocation {
    Local,
    Server,
    Unknown,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Connection {
    #[schema(value_type = Uuid)]
    pub id: String,
    #[schema(value_type = DisplayName)]
    pub name: String,
    pub status: ConnectionStatusDto,
    /// When `status` last changed (equals `created_at` until the first change).
    #[schema(value_type = Timestamp)]
    pub status_changed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub adapter_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub adapter_version: Option<String>,
    /// Plugin instance id reported at pairing (D6). Absent until paired.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Uuid>, nullable = false)]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub instance_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Timestamp>, nullable = false)]
    pub paired_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Timestamp>, nullable = false)]
    pub last_seen_at: Option<String>,
    /// Present while a code is active (the code itself is never re-shown).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Timestamp>, nullable = false)]
    pub pairing_code_expires_at: Option<String>,
    /// True while a "move to another machine" code is active and the current instance still works.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub repairing_pending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub visible_accounts: Option<Vec<VisibleAccount>>,
    /// Platform accounts with an open position (for the unpair confirmation).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub open_position_accounts: Option<Vec<String>>,
    #[schema(value_type = Timestamp)]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Timestamp>, nullable = false)]
    pub unpaired_at: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreatedConnection {
    pub connection: Connection,
    pub pairing: PairingCode,
}

/// `awd_<key_id>_<secret>`.
pub struct DeviceTokenString;

impl utoipa::PartialSchema for DeviceTokenString {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .pattern(Some("^awd_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}$"))
            .into()
    }
}

impl ToSchema for DeviceTokenString {}

#[derive(Debug, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PairingResult {
    #[schema(value_type = Uuid)]
    pub connection_id: String,
    #[schema(value_type = DeviceTokenString)]
    pub device_token: String,
}

/// `Cache-Control` value on responses that carry a one-time secret (`components.headers.NoStore`).
pub struct NoStore;

impl utoipa::PartialSchema for NoStore {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .enum_values(Some(["no-store"]))
            .into()
    }
}

impl ToSchema for NoStore {}
