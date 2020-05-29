//! Request specific implementations
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The new page request
pub struct CreateStepPage {
    /// describe the number of steps to complete
    pub steps: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Update steps
pub struct UpdateStepPage {
    pub step_completed: u64,
    pub admin_secret: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Delete steps page
pub struct DeleteStepPage {
    pub admin_secret: String,
}
