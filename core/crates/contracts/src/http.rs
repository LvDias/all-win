//! Schemas of the HTTP contract (`contracts/http/openapi.yaml`, `components.schemas`)
//! shared by handlers. Handlers serialize these types, and `utoipa` generates the
//! Core's own OpenAPI from them, so responses and spec cannot diverge silently.

use serde::Serialize;
use utoipa::openapi::RefOr;
use utoipa::openapi::schema::{ObjectBuilder, Schema, Type};
use utoipa::{PartialSchema, ToSchema};

/// Declares a string component with a pattern (no Rust value type needed).
macro_rules! pattern_string {
    ($(#[$meta:meta])* $name:ident, $pattern:expr) => {
        $(#[$meta])*
        pub struct $name;

        impl PartialSchema for $name {
            fn schema() -> RefOr<Schema> {
                ObjectBuilder::new()
                    .schema_type(Type::String)
                    .pattern(Some($pattern))
                    .into()
            }
        }

        impl ToSchema for $name {}
    };
}

pattern_string!(
    /// Lowercase UUID (`components.schemas.Uuid`).
    Uuid,
    "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
);
pattern_string!(
    /// Stable error code; clients translate it (`components.schemas.ErrorCode`).
    ErrorCode,
    r"^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$"
);

/// `Retry-After` header value: whole seconds, at least 1 (`components.headers.RetryAfter`).
pub struct RetryAfterSeconds;

impl PartialSchema for RetryAfterSeconds {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .schema_type(Type::Integer)
            .minimum(Some(1))
            .into()
    }
}

impl ToSchema for RetryAfterSeconds {}

/// `200` body of the ingress (`components.schemas.IngressAccepted`).
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct IngressAccepted {
    pub status: IngressAcceptedStatus,
    #[schema(value_type = Uuid)]
    pub instruction_id: String,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum IngressAcceptedStatus {
    Accepted,
    Duplicate,
}

/// Rejection body of the ingress (`components.schemas.IngressRejected`).
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct IngressRejected {
    /// Always `rejected` (`const` in the contract).
    pub status: IngressRejectedStatus,
    pub error: IngressError,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum IngressRejectedStatus {
    Rejected,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct IngressError {
    #[schema(value_type = ErrorCode)]
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(max_length = 256, nullable = false)]
    pub pointer: Option<String>,
}

impl IngressRejected {
    pub fn new(code: &str, pointer: Option<&str>) -> Self {
        Self {
            status: IngressRejectedStatus::Rejected,
            error: IngressError {
                code: code.to_owned(),
                pointer: pointer.map(str::to_owned),
            },
        }
    }
}

/// Ingress JSON body: the instruction contract v1. The generated spec embeds the exact
/// bytes of `contracts/instruction/v1/instruction.schema.json`
/// ([`crate::instruction::v1::SCHEMA_JSON`]); `utoipa` cannot express JSON Schema 2020-12
/// (`if`/`then`, `not`, `$defs`), so this placeholder is replaced when the document is built.
pub struct InstructionV1;

impl PartialSchema for InstructionV1 {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new().into()
    }
}

impl ToSchema for InstructionV1 {}

/// Ingress `text/plain` body: parsed as JSON and validated like `application/json`.
pub struct PlainInstruction;

impl PartialSchema for PlainInstruction {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .schema_type(Type::String)
            .max_length(Some(16 * 1024))
            .into()
    }
}

impl ToSchema for PlainInstruction {}
