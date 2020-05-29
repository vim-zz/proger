//! Response specific implementations
use serde::{Deserialize, Serialize};

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
pub struct Progress {
    /// last step to complete
    pub steps: u32,
    /// first step
    pub start: u32,
    /// progress indocation 
    pub completed: u32,
    /// latest update time (seconds since epoch)
    pub updated: u64,
}
