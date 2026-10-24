//! `accounts` module — application layer (V1-S5): account use cases and the ports they need.

pub mod commands;
pub mod deps;
pub mod error;
pub mod events;
pub mod ports;
pub mod usecases;
pub mod views;

pub use accounts_enterprise as enterprise;
pub use deps::AccountsDeps;
pub use error::AccountsError;
pub use events::{AUDIT_EVENT_TYPES, AccountsEvent, EMITTED_EVENT_TYPES};
pub use usecases::audit::AUDITED_FIELDS;
pub use usecases::create::{CreateAccountUseCase, CreateOutcome};
pub use usecases::update::UpdateAccountUseCase;
pub use usecases::{
    ArchiveAccount, BoundAccount, ReadAccounts, RestartAccountEvaluation, SetAccountLifecycle,
    SetAccountOptIn,
};

#[cfg(any(test, feature = "testing"))]
pub mod testing;
#[cfg(test)]
mod tests;
