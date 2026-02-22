use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Host {
    Ip(IpAddr),
    Domain(String),
}

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Host::Ip(ip) => ip.fmt(f),
            Host::Domain(host) => host.fmt(f),
        }
    }
}

impl From<String> for Host {
    fn from(v: String) -> Self {
        match IpAddr::from_str(&v) {
            Ok(ip) => Host::Ip(ip),
            _ => Host::Domain(v),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseHostError;

impl std::fmt::Display for ParseHostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: Unable to parse host!")
    }
}

impl FromStr for Host {
    type Err = ParseHostError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match IpAddr::from_str(s) {
            Ok(ip) => Ok(Host::Ip(ip)),
            _ => Ok(Host::Domain(s.to_string())),
        }
    }
}

impl Host {
    pub fn is_loopback(&self) -> bool {
        let common_loopbacks = [
            "localhost",
            "localhost.localdomain",
            "localhost4",
            "localhost4.localdomain4",
        ];

        match self {
            Host::Ip(ip) => ip.is_loopback(),
            Host::Domain(host) => {
                if common_loopbacks.contains(&host.as_str()) {
                    true
                } else {
                    false
                }
            }
        }
    }
}
