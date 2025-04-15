//! ServiceNow/ITSM/Incident - Incident Module
//! 

use serde::{Deserialize, Serialize};
use crate::SnowString;


#[derive(Clone,Debug,Default,Serialize, Deserialize)]
pub struct Incident {
    /// The sys_id of the incident
    pub active: bool,
    pub activity_due: SnowString,
    pub approval: SnowString,
    pub approval_history: SnowString,
    pub assigned_to: SnowString,
    pub assignment_group: SnowString,
    pub business_service: SnowString,
    pub category: SnowString,
    pub close_code: SnowString,
    pub close_notes: SnowString,
    pub closed_at: SnowString,
    pub closed_by: SnowString,
    pub correlation_display: SnowString,
    pub correlation_id: SnowString,
    pub delivery_plan: SnowString,
    pub delivery_task: SnowString,
    pub due_date: SnowString,
    pub escalation: SnowString,
    pub escalation_time: SnowString,
    pub follow_up: SnowString,
    pub impact: SnowString,
    pub incident_state: SnowString,
    pub knowledge: SnowString,
    pub location: SnowString,
    pub made_sla: SnowString,
    pub number: SnowString,
    pub priority: SnowString,
    pub problem_id: SnowString,
    pub resolved_at: SnowString,
    pub resolved_by: SnowString,
    pub resolution_code: SnowString,
    pub resolution_notes: SnowString,
    pub severity: SnowString,
    pub short_description: SnowString,
    pub sla_due: SnowString,
    pub state: SnowString,
    pub sys_class_name: SnowString,
    pub sys_id: SnowString,
    pub sys_mod_count: SnowString,
    pub sys_updated_by: SnowString,
    pub sys_updated_on: SnowString,
    pub sys_created_by: SnowString,
    pub sys_created_on: SnowString,
    pub sys_domain: SnowString,
    pub sys_domain_path: SnowString,
    pub time_worked: SnowString,
    pub urgency: SnowString,
    pub user_input: SnowString,
    pub watch_list: SnowString,
    pub work_end: SnowString,
    pub work_notes: SnowString,
    pub work_notes_list: SnowString,
    pub work_start: SnowString,
}

#[cfg(test)]
mod test {
    use crate::snow_string;
    #[test]
    fn test_new_snow_string() {
        let snow_string = snow_string("test");
        assert_eq!(snow_string, Some("test".to_string()));
    }
}