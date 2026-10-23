//! TradingView webhook Input adapter (ADR-0003 §6–7, s1-webhook-ingest).
//!
//! Everything TradingView-specific lives here: the token in the URL path, the
//! source policy and the HTTP answers. The Core modules only see a validated
//! instruction (`routing`) and the instrument catalog.

pub mod http;
pub mod ip;
pub mod metrics;
pub mod negative_cache;
pub mod policy;
pub mod ports;
pub mod rate_limit;
pub mod token;

pub use http::{INGRESS_ROUTE, IngressState, IpLimits, router, unmatched};
pub use ip::{Cidr, IpKey};
pub use metrics::IngressMetrics;
pub use negative_cache::UnknownKeyCache;
pub use ports::{InstrumentSupport, PortError, ResolvedSourceKey, SourceKeyResolver};
pub use rate_limit::{KeyedRateLimiter, RateLimit};
pub use token::IngressToken;

/// `source.type` this adapter authenticates (ADR-0003 §3).
pub const SOURCE_TYPE: &str = "tradingview";
