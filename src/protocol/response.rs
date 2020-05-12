//! Response specific implementations
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The new page response
pub struct PageAccess {
    pub admin_secret: String,
    pub link: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The progress status response
pub struct Progress {
    steps: u32,
    start: u32,
    completed: u32,
}