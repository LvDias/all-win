//! Client-address keying for the per-IP limits (s1-webhook-ingest §2 steps 1–2):
//! IPv4 by address, IPv6 by /64, so rotating addresses inside one /64 shares a bucket.

use std::net::IpAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IpKey {
    V4(u32),
    /// The /64 prefix.
    V6(u64),
}

impl From<IpAddr> for IpKey {
    fn from(ip: IpAddr) -> Self {
        match ip {
            IpAddr::V4(v4) => Self::V4(u32::from(v4)),
            IpAddr::V6(v6) => match v6.to_ipv4_mapped() {
                Some(v4) => Self::V4(u32::from(v4)),
                None => Self::V6((u128::from(v6) >> 64) as u64),
            },
        }
    }
}

/// An IPv4 or IPv6 network in CIDR notation (`52.89.214.238/32`, `2001:db8::/32`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cidr {
    network: IpAddr,
    prefix: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidCidr;

impl std::str::FromStr for Cidr {
    type Err = InvalidCidr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (addr, prefix) = match s.trim().split_once('/') {
            Some((a, p)) => (a, Some(p)),
            None => (s.trim(), None),
        };
        let network: IpAddr = addr.parse().map_err(|_| InvalidCidr)?;
        let max = if network.is_ipv4() { 32 } else { 128 };
        let prefix = match prefix {
            Some(p) => p.parse::<u8>().map_err(|_| InvalidCidr)?,
            None => max,
        };
        if prefix > max {
            return Err(InvalidCidr);
        }
        Ok(Self { network, prefix })
    }
}

impl Cidr {
    pub fn contains(&self, ip: IpAddr) -> bool {
        let ip = match ip {
            IpAddr::V6(v6) => v6.to_ipv4_mapped().map_or(ip, IpAddr::V4),
            v4 => v4,
        };
        match (self.network, ip) {
            (IpAddr::V4(net), IpAddr::V4(ip)) => {
                let mask = u32::MAX
                    .checked_shl(32 - u32::from(self.prefix))
                    .unwrap_or(0);
                u32::from(net) & mask == u32::from(ip) & mask
            }
            (IpAddr::V6(net), IpAddr::V6(ip)) => {
                let mask = u128::MAX
                    .checked_shl(128 - u32::from(self.prefix))
                    .unwrap_or(0);
                u128::from(net) & mask == u128::from(ip) & mask
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    #[test]
    fn ipv6_is_keyed_by_slash_64_and_mapped_v4_by_address() {
        assert_eq!(
            IpKey::from(ip("2001:db8:1:2::1")),
            IpKey::from(ip("2001:db8:1:2:ffff::9"))
        );
        assert_ne!(
            IpKey::from(ip("2001:db8:1:2::1")),
            IpKey::from(ip("2001:db8:1:3::1"))
        );
        assert_eq!(
            IpKey::from(ip("::ffff:203.0.113.7")),
            IpKey::from(ip("203.0.113.7"))
        );
        assert_ne!(
            IpKey::from(ip("203.0.113.7")),
            IpKey::from(ip("203.0.113.8"))
        );
    }

    #[test]
    fn cidr_membership() {
        let tv: Cidr = "52.89.214.0/24".parse().unwrap();
        assert!(tv.contains(ip("52.89.214.238")));
        assert!(tv.contains(ip("::ffff:52.89.214.1")));
        assert!(!tv.contains(ip("52.89.215.1")));
        let single: Cidr = "34.212.75.30".parse().unwrap();
        assert!(single.contains(ip("34.212.75.30")) && !single.contains(ip("34.212.75.31")));
        let v6: Cidr = "2001:db8::/32".parse().unwrap();
        assert!(v6.contains(ip("2001:db8:ffff::1")) && !v6.contains(ip("2001:db9::1")));
        let all: Cidr = "0.0.0.0/0".parse().unwrap();
        assert!(all.contains(ip("8.8.8.8")));
        for bad in ["1.2.3.4/33", "x/8", "::/129", ""] {
            assert!(bad.parse::<Cidr>().is_err(), "{bad}");
        }
    }
}
