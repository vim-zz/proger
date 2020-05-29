//! Request specific implementations
use serde::{Deserialize, Serialize};
use chrono::{
    DateTime, Utc,
    serde::ts_seconds
};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Step {
    pub step: u64,
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

/// The new page model
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct StepPageModel {
    /// The page unique link, it is used as the index
    pub link: String,
    /// This field is a result of the secret, it is sotred in the database
    /// and allows to verify the seceret access credentials
    pub secret: String,
    /// Total amount of expected steps
    pub steps: u64,
    /// Total of steps actaully completed
    pub completed: u64,
    /// Progess details
    pub progress: Vec<Step>,
    /// Createtion date and time at epoch format (seconds)
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
    /// Last update date and time at epoch format (seconds)
    #[serde(with = "ts_seconds")]
    pub updated: DateTime<Utc>,
}
