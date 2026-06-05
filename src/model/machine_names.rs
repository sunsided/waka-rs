//! Contains the models for [`WakaTimeClient::machine_names`](crate::WakaTimeClient::machine_names).

use crate::model::pagination::Pagination;
use serde::{Deserialize, Serialize};

/// Response for [`WakaTimeClient::machine_names`](crate::WakaTimeClient::machine_names).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineNames {
    /// The machines of the user.
    pub data: Vec<MachineName>,
    /// Pagination metadata.
    #[serde(flatten)]
    pub pagination: Pagination,
}

/// A machine the user has sent heartbeats from.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineName {
    /// Unique id of this machine.
    pub id: String,
    /// Display name of this machine, if customized.
    pub name: Option<String>,
    /// Hostname of this machine.
    pub value: Option<String>,
    /// IP address of this machine.
    pub ip: Option<String>,
    /// Time when this machine was last seen in ISO 8601 format.
    pub last_seen_at: Option<String>,
    /// Timezone of this machine in Olson Country/Region format.
    pub timezone: Option<String>,
    /// Time when this machine was first seen in ISO 8601 format.
    pub created_at: Option<String>,
}
