//! Request specific implementations
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The new page model
pub struct PageModel {
    /// The page unique key for the database 
    pub id: String,
    /// This field is a result of the secret, it is sotred in the database 
    /// and allows to verify the seceret access credentials 
    pub hashed_secret: String,
    /// The page unique link, it maybe same or different from the id
    pub link: String,
    /// Amount of steps
    pub steps: u32,
    /// First step
    pub start: u32,
    /// The progress as completed steps
    pub completed: u32,
}