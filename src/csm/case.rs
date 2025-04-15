//! ServiceNow/CSM/Case - Case Module

use serde::{Deserialize, Serialize};

/// Case Record
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Case {
    /// Case Number
    pub case_number: String,
    /// Case State
    pub case_state: String,
    /// Case Priority
    pub case_priority: String,
    /// Case Assignment Group
    pub case_assignment_group: String,
    /// Case Assigned To
    pub case_assigned_to: String,
    /// Case Created Date
    pub case_created_date: String,
}