//! Request specific implementations
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The new page request
pub struct NewStepsPage {
    /// describe the number of steps to complete
    pub steps: u32,
    /// state the inital step, default is 0
    #[serde(default)]
    pub start: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Set steps
pub struct SetStepsPage {
    pub completed: u32,
    pub admin_secret: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Delete steps page
pub struct DeleteStepsPage {
    pub admin_secret: String,
}
