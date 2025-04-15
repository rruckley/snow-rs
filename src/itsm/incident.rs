//! ServiceNow/ITSM/Incident - Incident Module
//! 

use serde::{Deserialize, Serialize};


#[derive(Clone,Debug,Default,Serialize, Deserialize)]
pub struct Incident {
    /// The sys_id of the incident
    pub sys_id: String,
    /// The number of the incident
    pub number: String,
    /// The short description of the incident
    pub short_description: String,
    /// The description of the incident
    pub description: String,
    /// The state of the incident
    pub state: String,
    /// The priority of the incident
    pub priority: String,
    /// The assignment group of the incident
    pub assignment_group: String,
    /// The assigned to of the incident
    pub assigned_to: String,
}