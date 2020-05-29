//! Response specific implementations
use serde::{Deserialize, Serialize};
use chrono::{
    DateTime, Utc,
    serde::ts_seconds
};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The new page response
pub struct PageAccess {
    /// The password for the page should be included by the user whenever there is 
    /// a change the page state
    pub admin_secret: String,
    /// unique ID for the page
    pub link: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The progress status response
pub struct StepPageProgress {
    /// last step to complete
    pub steps: u64,
    /// progress indocation 
    pub completed: u64,
    /// latest update time (seconds since epoch)
    #[serde(with = "ts_seconds")]
    pub updated: DateTime<Utc>,
}
