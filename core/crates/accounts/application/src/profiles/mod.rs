//! Stop/target profiles (V1-S6): use cases, their ports, the initial-copy planner for the account
//! create, and the bracket query for execution.

pub mod bracket_query;
pub mod error;
pub mod events;
pub mod planner;
pub mod ports;
pub mod usecases;

pub use bracket_query::BracketQuery;
pub use error::ProfilesError;
pub use events::{PROFILE_EVENT_TYPES, ProfileAction, ProfileEvent};
pub use planner::ProfilePlanner;
pub use usecases::{
    AccountProfileEntry, AccountProfileView, DefaultEntry, NearestCandidate, ProfileDeps,
    ProfileUseCases,
};

#[cfg(test)]
mod tests;
