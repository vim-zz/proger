//! Response specific implementations
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The logout response
pub struct PageAccess {
    pub admin_secret: String,
    pub private_link: String,
}