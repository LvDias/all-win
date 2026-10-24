//! Temporary allowlist of emitted outbox event types that have no consumer yet (Esquadro
//! decision (b), exception to rule (A)1: startup otherwise fails for a type without consumer).
//!
//! Rules:
//! - only a type whose future consumer starts `from_now` may be listed: it never needs the rows
//!   written before it is registered. A type that needs a backfill (anything that becomes an audit
//!   entry, X-5) is never listed, since the rows are purged;
//! - a row of a listed type is excluded from the unconsumed alert (`/metrics`) and purged 7 days
//!   after `occurred_at`, like a published row. It is never marked published: published means
//!   consumed;
//! - startup fails when a listed type gets a consumer ([`check`]), and a test fails when a listed
//!   type is emitted by no module (`listed_types_are_emitted`).

use crate::outbox::Consumers;

/// Where a future consumer starts reading. Only `FromNow` exists: a backfilling consumer cannot
/// be allowlisted (rule 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartPolicy {
    FromNow,
}

/// One allowlisted event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unconsumed {
    pub event_type: &'static str,
    /// Slice that will register the consumer (e.g. "S13 risk", "gateway", "SSE").
    pub consumer_slice: &'static str,
    pub start: StartPolicy,
}

/// The allowlist. Remove an entry in the same change that registers its consumer.
pub const UNCONSUMED: &[Unconsumed] = &[
    Unconsumed {
        event_type: "account.limits_changed",
        consumer_slice: "S13 risk",
        start: StartPolicy::FromNow,
    },
    Unconsumed {
        event_type: "account.binding_changed",
        consumer_slice: "S2 gateway",
        start: StartPolicy::FromNow,
    },
    Unconsumed {
        event_type: "account.changed",
        consumer_slice: "SSE (accounts)",
        start: StartPolicy::FromNow,
    },
    Unconsumed {
        event_type: "account.archived",
        consumer_slice: "SSE (accounts)",
        start: StartPolicy::FromNow,
    },
];

/// The listed event types.
pub fn types(list: &[Unconsumed]) -> Vec<String> {
    list.iter().map(|u| u.event_type.to_owned()).collect()
}

#[derive(Debug, thiserror::Error)]
#[error(
    "allowlisted outbox event types now have a consumer; remove them from the allowlist: {0:?}"
)]
pub struct ConsumedButListed(pub Vec<String>);

/// Fails when a listed type has a registered consumer.
pub fn check(consumers: &Consumers, list: &[Unconsumed]) -> Result<(), ConsumedButListed> {
    let registered = consumers.types();
    let both: Vec<String> = list
        .iter()
        .filter(|u| registered.iter().any(|t| t == u.event_type))
        .map(|u| u.event_type.to_owned())
        .collect();
    if both.is_empty() {
        Ok(())
    } else {
        Err(ConsumedButListed(both))
    }
}

/// Every event type a module writes to the outbox, whatever the configuration (identity included
/// even when it is not mounted). Add each new emitting module here.
pub fn emitted_event_types() -> Vec<&'static str> {
    let mut all: Vec<&'static str> = routing_application::enterprise::IngressEvent::TYPES.to_vec();
    all.extend(identity_application::changes::emitted_event_types());
    all.extend(accounts_application::EMITTED_EVENT_TYPES);
    all.extend(accounts_application::profiles::PROFILE_EVENT_TYPES);
    all
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::*;
    use crate::outbox::{EventConsumer, OutboxEvent};

    struct Noop;

    #[async_trait]
    impl EventConsumer for Noop {
        fn name(&self) -> &'static str {
            "noop"
        }

        async fn handle(&self, _: &OutboxEvent) -> Result<(), String> {
            Ok(())
        }
    }

    const SAMPLE: &[Unconsumed] = &[Unconsumed {
        event_type: "account.limits_changed",
        consumer_slice: "S13 risk",
        start: StartPolicy::FromNow,
    }];

    #[test]
    fn listed_types_are_emitted() {
        let emitted = emitted_event_types();
        for u in UNCONSUMED {
            assert!(
                emitted.contains(&u.event_type),
                "{} is allowlisted but no module emits it",
                u.event_type
            );
        }
    }

    #[test]
    fn listed_types_are_unique() {
        let mut seen = std::collections::BTreeSet::new();
        for u in UNCONSUMED {
            assert!(seen.insert(u.event_type), "{} listed twice", u.event_type);
        }
    }

    #[test]
    fn a_listed_type_with_a_consumer_fails_startup() {
        let mut consumers = Consumers::default();
        consumers.register("ingress.received", Arc::new(Noop));
        assert!(check(&consumers, SAMPLE).is_ok());
        consumers.register("account.limits_changed", Arc::new(Noop));
        let err = check(&consumers, SAMPLE).unwrap_err();
        assert_eq!(err.0, ["account.limits_changed"]);
    }

    #[test]
    fn a_listed_type_needs_no_consumer_at_startup() {
        let consumers = Consumers::default();
        assert!(
            consumers
                .require(&["account.limits_changed"], SAMPLE)
                .is_ok()
        );
        let err = consumers
            .require(&["account.limits_changed", "account.changed"], SAMPLE)
            .unwrap_err();
        assert_eq!(err.0, ["account.changed"]);
    }
}
