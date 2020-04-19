//! The core library
#![warn(missing_docs)]

pub mod protocol;

#[cfg(feature = "backend")]
#[macro_use]
extern crate diesel;

// macro_rules! apis {
//     ($($name:ident => $content:expr,)*) => (
//         $(#[allow(missing_docs)] pub const $name: &str = $content;)*
//     )
// }

#[allow(missing_docs)]
pub const API_URL_V1_NEW_STEP_PAGE: &str = "v1/step/new";
pub const API_URL_V1_SET_STEP: &str = "v1/step/set/{id}";
pub const API_URL_V1_VIEW_PAGE: &str = "v1/{id}";
