//! ServiceNow Schema Module
//! # Description
//! This module contains the schema for the ServiceNow APIs

#[warn(missing_docs)]

#[cfg(feature ="csm")]
pub mod csm;
#[cfg(feature ="itsm")]
pub mod itsm;

pub type SnowString = Option<String>;


pub mod service_now;