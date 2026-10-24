//! Browsers a user has signed in from (`known_device`, auth policy rev. 2 §5.2). The device
//! cookie `__Host-aw_device` is separate from the session cookie, so logout or expiry does not make
//! a known browser "new".

use chrono::{DateTime, Duration, Utc};
use shared_kernel::UserId;

use crate::ids::DeviceId;
use crate::tokens::TokenHash;

/// A device younger than this is "new" for M2 (channel-change hold) and M3 (persistence step-ups).
pub const NEW_DEVICE_AGE: Duration = Duration::days(7);
pub const MAX_LABEL_CHARS: usize = 120;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KnownDevice {
    pub id: DeviceId,
    pub user_id: UserId,
    pub key_hash: TokenHash,
    pub label: Option<String>,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub last_ip: Option<std::net::IpAddr>,
    pub revoked_at: Option<DateTime<Utc>>,
}

impl KnownDevice {
    pub fn new(
        id: DeviceId,
        user_id: UserId,
        key_hash: TokenHash,
        user_agent: Option<&str>,
        ip: Option<std::net::IpAddr>,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            key_hash,
            label: user_agent.map(label_from_user_agent),
            first_seen_at: now,
            last_seen_at: now,
            last_ip: ip,
            revoked_at: None,
        }
    }

    pub fn is_revoked(&self) -> bool {
        self.revoked_at.is_some()
    }

    pub fn is_new(&self, now: DateTime<Utc>) -> bool {
        now - self.first_seen_at < NEW_DEVICE_AGE
    }

    pub fn display_label(&self) -> &str {
        self.label.as_deref().unwrap_or("Unknown browser")
    }
}

/// Coarse "Browser on OS" from a User-Agent (no version, no device model).
pub fn label_from_user_agent(ua: &str) -> String {
    let browser = if ua.contains("Edg/") {
        "Edge"
    } else if ua.contains("OPR/") || ua.contains("Opera") {
        "Opera"
    } else if ua.contains("Firefox/") {
        "Firefox"
    } else if ua.contains("Chrome/") || ua.contains("CriOS/") {
        "Chrome"
    } else if ua.contains("Safari/") {
        "Safari"
    } else {
        "Browser"
    };
    let os = if ua.contains("Windows") {
        "Windows"
    } else if ua.contains("iPhone") || ua.contains("iPad") {
        "iOS"
    } else if ua.contains("Android") {
        "Android"
    } else if ua.contains("Mac OS X") || ua.contains("Macintosh") {
        "macOS"
    } else if ua.contains("Linux") {
        "Linux"
    } else {
        "unknown OS"
    };
    let label = format!("{browser} on {os}");
    label.chars().take(MAX_LABEL_CHARS).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn labels_are_coarse() {
        let chrome = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0 Safari/537.36";
        assert_eq!(label_from_user_agent(chrome), "Chrome on Windows");
        let safari = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1";
        assert_eq!(label_from_user_agent(safari), "Safari on iOS");
        assert_eq!(label_from_user_agent("curl/8"), "Browser on unknown OS");
    }

    #[test]
    fn new_for_seven_days() {
        let t0 = Utc.with_ymd_and_hms(2026, 10, 1, 0, 0, 0).unwrap();
        let d = KnownDevice::new(
            DeviceId::new_v7(),
            UserId::new_v7(),
            TokenHash::from_bytes([1; 32]),
            None,
            None,
            t0,
        );
        assert!(d.is_new(t0 + Duration::days(6)));
        assert!(!d.is_new(t0 + Duration::days(7)));
        assert_eq!(d.display_label(), "Unknown browser");
    }
}
