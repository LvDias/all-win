//! Adapter connections and pairing (S3 AC-4, ADR-0004 §8, Q18).

use chrono::{DateTime, Duration, Utc};
use shared_kernel::UserId;
use thiserror::Error;
use uuid::Uuid;

use crate::ids::{ConnectionId, PairingCodeId};
use crate::source::DisplayName;
use crate::tokens::CodeHmac;

pub const PAIRING_CODE_LIFETIME: Duration = Duration::minutes(10);

/// Adapter identity reported by the plugin at pairing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdapterInfo {
    /// Adapter type string (`^[a-z][a-z0-9_]{0,31}$`); the Core does not interpret it.
    pub adapter_type: String,
    pub version: String,
    pub build: AdapterBuild,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdapterBuild {
    Release,
    Development,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairedInstance {
    pub instance_id: Uuid,
    pub instance_name: String,
    pub adapter: AdapterInfo,
    pub paired_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionStatus {
    AwaitingPairing,
    Connected,
    Disconnected,
    Outdated,
    Unpaired,
}

impl ConnectionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AwaitingPairing => "awaiting_pairing",
            Self::Connected => "connected",
            Self::Disconnected => "disconnected",
            Self::Outdated => "outdated",
            Self::Unpaired => "unpaired",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "awaiting_pairing" => Some(Self::AwaitingPairing),
            "connected" => Some(Self::Connected),
            "disconnected" => Some(Self::Disconnected),
            "outdated" => Some(Self::Outdated),
            "unpaired" => Some(Self::Unpaired),
            _ => None,
        }
    }
}

/// Live state of a paired connection, reported by the gateway's adapter sessions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiveState {
    Connected,
    Disconnected,
    /// Connected with an adapter version the Core no longer supports.
    Outdated,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Connection {
    pub id: ConnectionId,
    pub user_id: UserId,
    pub name: DisplayName,
    pub created_at: DateTime<Utc>,
    pub instance: Option<PairedInstance>,
    pub status: ConnectionStatus,
    /// When `status` last changed (= `created_at` until the first change).
    pub status_changed_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
    /// Unpaired (`identity.adapter_connection.archived_at`): read-only history row.
    pub unpaired_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("the connection is unpaired")]
pub struct ConnectionUnpaired;

impl Connection {
    pub fn create(
        id: ConnectionId,
        user_id: UserId,
        name: DisplayName,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            name,
            created_at: now,
            instance: None,
            status: ConnectionStatus::AwaitingPairing,
            status_changed_at: now,
            last_seen_at: None,
            unpaired_at: None,
        }
    }

    pub fn ensure_not_unpaired(&self) -> Result<(), ConnectionUnpaired> {
        match self.unpaired_at {
            None => Ok(()),
            Some(_) => Err(ConnectionUnpaired),
        }
    }

    pub fn is_paired(&self) -> bool {
        self.unpaired_at.is_none() && self.instance.is_some()
    }

    /// A new instance redeemed a code. It is disconnected until its WebSocket says `hello`; a
    /// previous instance loses its device token at this point (re-pairing, ADR-0004 §8).
    pub fn pair(&mut self, instance: PairedInstance) -> Result<(), ConnectionUnpaired> {
        self.ensure_not_unpaired()?;
        self.set_status(ConnectionStatus::Disconnected, instance.paired_at);
        self.instance = Some(instance);
        Ok(())
    }

    /// Applies a gateway report. Ignored for unpaired or never-paired connections and for
    /// reports older than the current status. Returns whether the status changed.
    pub fn record_live_state(&mut self, state: LiveState, at: DateTime<Utc>) -> bool {
        if !self.is_paired() || at < self.status_changed_at {
            return false;
        }
        if state != LiveState::Disconnected {
            self.last_seen_at = Some(self.last_seen_at.map_or(at, |seen| seen.max(at)));
        }
        let status = match state {
            LiveState::Connected => ConnectionStatus::Connected,
            LiveState::Disconnected => ConnectionStatus::Disconnected,
            LiveState::Outdated => ConnectionStatus::Outdated,
        };
        self.set_status(status, at)
    }

    /// Idempotent. Returns whether the connection was unpaired by this call.
    pub fn unpair(&mut self, now: DateTime<Utc>) -> bool {
        if self.unpaired_at.is_some() {
            return false;
        }
        self.unpaired_at = Some(now);
        self.set_status(ConnectionStatus::Unpaired, now)
    }

    fn set_status(&mut self, status: ConnectionStatus, at: DateTime<Utc>) -> bool {
        if self.status == status {
            return false;
        }
        self.status = status;
        self.status_changed_at = at;
        true
    }
}

/// Stored pairing code (the code itself is never stored, only its HMAC).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairingCode {
    pub id: PairingCodeId,
    pub user_id: UserId,
    pub connection_id: ConnectionId,
    pub code_hmac: CodeHmac,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub invalidated_at: Option<DateTime<Utc>>,
}

impl PairingCode {
    pub fn issue(
        id: PairingCodeId,
        user_id: UserId,
        connection_id: ConnectionId,
        code_hmac: CodeHmac,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            connection_id,
            code_hmac,
            created_at: now,
            expires_at: now + PAIRING_CODE_LIFETIME,
            used_at: None,
            invalidated_at: None,
        }
    }

    /// Single use, 10 minutes, not superseded. Unknown/expired/used/superseded are one error.
    pub fn is_redeemable(&self, now: DateTime<Utc>) -> bool {
        self.used_at.is_none() && self.invalidated_at.is_none() && now < self.expires_at
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairingCodeInvalidation {
    /// A newer code was issued for the connection.
    Superseded,
    /// The connection was unpaired.
    Unpaired,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn t0() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 23, 0, 0, 0).unwrap()
    }

    fn conn() -> Connection {
        Connection::create(
            ConnectionId::new_v7(),
            UserId::new_v7(),
            DisplayName::parse("Desk A").unwrap(),
            t0(),
        )
    }

    fn instance(at: DateTime<Utc>) -> PairedInstance {
        PairedInstance {
            instance_id: Uuid::now_v7(),
            instance_name: "PC".into(),
            adapter: AdapterInfo {
                adapter_type: "ninjatrader8".into(),
                version: "1.0.0".into(),
                build: AdapterBuild::Release,
            },
            paired_at: at,
        }
    }

    #[test]
    fn lifecycle_awaiting_paired_live_unpaired() {
        let mut c = conn();
        assert_eq!(
            (c.status, c.status_changed_at),
            (ConnectionStatus::AwaitingPairing, t0())
        );
        assert!(
            !c.record_live_state(LiveState::Connected, t0()),
            "not paired yet"
        );
        let paired = t0() + Duration::minutes(3);
        c.pair(instance(paired)).unwrap();
        assert_eq!(
            (c.status, c.status_changed_at),
            (ConnectionStatus::Disconnected, paired)
        );
        let up = paired + Duration::seconds(5);
        assert!(c.record_live_state(LiveState::Connected, up));
        assert_eq!(
            (c.status, c.status_changed_at, c.last_seen_at),
            (ConnectionStatus::Connected, up, Some(up))
        );
        assert!(
            !c.record_live_state(LiveState::Disconnected, paired),
            "stale report ignored"
        );
        assert!(c.record_live_state(LiveState::Outdated, up + Duration::seconds(1)));
        let gone = up + Duration::minutes(1);
        assert!(c.unpair(gone));
        assert!(!c.unpair(gone + Duration::minutes(1)));
        assert_eq!(
            (c.status, c.unpaired_at),
            (ConnectionStatus::Unpaired, Some(gone))
        );
        assert!(!c.record_live_state(LiveState::Connected, gone + Duration::minutes(2)));
        assert_eq!(c.pair(instance(gone)), Err(ConnectionUnpaired));
        assert!(!c.is_paired());
    }

    #[test]
    fn pairing_code_is_single_use_for_ten_minutes() {
        let c = conn();
        let hmac = CodeHmac {
            bytes: [0; 32],
            pepper_version: 1,
        };
        let mut p = PairingCode::issue(PairingCodeId::new_v7(), c.user_id, c.id, hmac, t0());
        assert!(p.is_redeemable(t0() + Duration::minutes(9)));
        assert!(!p.is_redeemable(t0() + Duration::minutes(10)));
        p.used_at = Some(t0());
        assert!(!p.is_redeemable(t0()));
    }
}
