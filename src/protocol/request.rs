//! Request specific implementations
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The new page request
pub struct NewPage {
    steps: u32,
}
