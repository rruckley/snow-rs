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

/// # Description
/// Generate `SnowString` from anything that implements Into<String>
/// ```
/// use snow_rs::{snow_string,SnowString};
/// let a_string = snow_string("a str");
/// assert!(a_string.is_some());
/// ```
pub fn snow_string(input : impl Into<String>) -> SnowString {
    Some(input.into())
}