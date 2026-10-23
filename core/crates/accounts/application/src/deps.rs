//! Everything the accounts use cases need, injected by `crates/app`.

use std::sync::Arc;

use access::StepUpGate;
use shared_kernel::Clock;

use crate::ports::{
    AccountRoutes, AccountStore, InitialProfilePlanner, PresetCatalog, RuleVersions, UserTimeZone,
    VenueAccounts,
};

#[derive(Clone)]
pub struct AccountsDeps {
    pub store: Arc<dyn AccountStore>,
    pub rules: Arc<dyn RuleVersions>,
    pub presets: Arc<dyn PresetCatalog>,
    pub venues: Arc<dyn VenueAccounts>,
    pub routes: Arc<dyn AccountRoutes>,
    pub time_zones: Arc<dyn UserTimeZone>,
    pub profiles: Arc<dyn InitialProfilePlanner>,
    pub step_up: Arc<dyn StepUpGate>,
    pub clock: Arc<dyn Clock>,
    /// Production environment: only then may a `live`/`unknown` platform account be bound
    /// (ADR-0004 §8). Never true on a development or test host.
    pub production: bool,
}
