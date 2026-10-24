//! Identity use cases (V1-S3): sessions, invitations, admin, instruction sources, adapter
//! connections and pairing. Passwordless sign-in (HTTP contract 2.0.0) replaces the removed
//! password, TOTP, recovery-code and reset flows.
//!
//! Contract: contracts/http/openapi.yaml 1.1.0 and docs/architecture/contracts/s3-access-sources-pairing.md.
//! Every use case is a struct built with `new(deps)` and run with `handle(..)`. Persistence is
//! one port ([`ports::IdentityStore`]) whose writes are applied atomically as a [`changes::ChangeSet`]
//! together with the audit records and events they cause.

pub mod auth;
pub mod changes;
pub mod deps;
pub mod error;
pub mod ports;
pub mod throttle;
pub mod usecases;
pub mod views;

#[cfg(test)]
mod testing;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_admin_recovery;
#[cfg(test)]
mod tests_auth;
