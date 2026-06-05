//! Contains the models for [`WakaTimeClient::meta`](crate::WakaTimeClient::meta).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Information about WakaTime's infrastructure, e.g. for IP whitelisting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    /// Descriptions of each IP list, keyed like [`Meta::ips`].
    pub ip_descriptions: Option<BTreeMap<String, String>>,
    /// Public IP addresses used by WakaTime servers.
    pub ips: Option<MetaIps>,
    /// Time when these IPs were last changed in ISO 8601 format.
    pub last_modified_at: Option<String>,
}

/// Public IP addresses used by WakaTime servers, grouped by role.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaIps {
    /// Public IPs of the api.wakatime.com servers.
    pub api: Option<MetaIpList>,
    /// Public IPs of the wakatime.com website servers.
    pub website: Option<MetaIpList>,
    /// Public IPs of the worker servers which make outgoing requests.
    pub worker: Option<MetaIpList>,
}

/// A list of IPv4 and IPv6 addresses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaIpList {
    /// IPv4 addresses.
    pub v4: Option<Vec<String>>,
    /// IPv6 addresses.
    pub v6: Option<Vec<String>>,
}
