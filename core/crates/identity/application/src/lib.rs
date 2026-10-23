//! Identity use cases (V1-S3): sign-in, sessions, invitations, password reset, TOTP, admin,
//! instruction sources, adapter connections and pairing.
//!
//! Contract: contracts/http/openapi.yaml 1.1.0 and docs/architecture/contracts/s3-access-sources-pairing.md.
//! Every use case is a struct built with `new(deps)` and run with `handle(..)`. Persistence is
//! one port ([`ports::IdentityStore`]) whose writes are applied atomically as a [`changes::ChangeSet`]
//! together with the audit records and events they cause.
//!
//! Two throttling mechanisms are pending a user decision and sit behind swappable ports:
//! [`throttle::LoginThrottle`] (Q2 / board U10) and [`throttle::PairingThrottle`] (Q3 / board U11).

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
