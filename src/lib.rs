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
pub const API_URL_V1_NEW_PAGE: &str = "v1/new_page";
