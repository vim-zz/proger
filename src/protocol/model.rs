//! Request specific implementations
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The new page model
pub struct PageModel {
    /// The page unique link, it is used as the index
    pub link: String,
    /// This field is a result of the secret, it is sotred in the database 
    /// and allows to verify the seceret access credentials 
    pub hashed_secret: String,
    /// Amount of steps
    pub steps: u32,
    /// First step
    pub start: u32,
    /// The progress as completed steps
    pub completed: u32,
    /// Createtion date at epoch format (seconds)
    pub epoch_time: u64, 
}
