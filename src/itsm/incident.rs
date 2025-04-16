//! ServiceNow/ITSM/Incident - Incident Module
//! 

use serde::{Deserialize, Serialize};
use crate::{snow_string,SnowString};

const CLASS_NAME : &str = "incident";
const RUST_RS : &str = "rust_rs";

#[derive(Clone,Debug,Default,Serialize, Deserialize)]
pub struct Incident {
    /// The sys_id of the incident
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_due: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_history: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_group: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_service: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_code: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_notes: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_by: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_display: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_plan: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_task: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_time: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_state: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub made_sla: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_id: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_by: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_code: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_notes: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_due: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_class_name: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_id: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_mod_count: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_updated_by: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_updated_on: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_created_by: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_created_on: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_domain: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_domain_path: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_worked: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_input: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_list: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_end: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_notes: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_notes_list: SnowString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_start: SnowString,
}

impl Incident {
    /// Create a new incident with a given title
    /// Sets meta-data
    /// ```
    /// # use snow_rs::snow_string;
    /// # use snow_rs::itsm::incident::Incident;
    /// let incident = Incident::new("An Incident");
    /// assert!(incident.short_description.is_some());
    /// ```
    pub fn new(title : impl Into<String>) -> Incident {
        Incident {
            short_description: snow_string(title),
            // Technically server now will set these
            sys_class_name : snow_string(CLASS_NAME),
            // Ideally this will be the user used to interact with the SNOW APIs
            sys_created_by : snow_string(RUST_RS),
            ..Default::default()
        }
    }
    /// Set the short_description in a builder pattern
    /// ```
    /// # use snow_rs::snow_string;
    /// # use snow_rs::itsm::incident::Incident;
    /// let incident = Incident::default()
    /// .short_description("Short Description");
    /// assert!(incident.short_description.is_some());
    /// ```
    pub fn short_description(mut self, desc : impl Into<String>) -> Incident {
        self.short_description = snow_string(desc);
        self
    }

    /// Set the description in a builder pattern
    /// ```
    /// # use snow_rs::snow_string;
    /// # use snow_rs::itsm::incident::Incident;
    /// let incident = Incident::default()
    /// .description("A Description");
    /// assert!(incident.description.is_some());
    /// ```
    pub fn description(mut self, desc : impl Into<String>) -> Incident {
        self.description = snow_string(desc);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::snow_string;
    use crate::itsm::incident::Incident;

    const DESCRIPTION : &str = "A Description";

    #[test]
    fn test_new_snow_string() {
        let snow_string = snow_string("test");
        assert_eq!(snow_string, Some("test".to_string()));
    }

    #[test]
    fn test_new_incident() {
        let incident = Incident::default();
        assert!(incident.short_description.is_none());
    }

    #[test]
    fn test_new_incident_description() {
        let incident = Incident::default()
            .description(DESCRIPTION);
        assert!(incident.description.is_some());
        assert_eq!(incident.description.unwrap().as_str(),DESCRIPTION);
    }
}   
