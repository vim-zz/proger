//! The core library
#![deny(missing_docs)]
//#![allow(unknown_lints, proc_macro_derive_resolution_fallback)]

#[cfg(feature = "backend")]
#[macro_use]
extern crate diesel;

// macro_rules! apis {
//     ($($name:ident => $content:expr,)*) => (
//         $(#[allow(missing_docs)] pub const $name: &str = $content;)*
//     )
// }

#[allow(missing_docs)]
pub const API_URL_V1_NEW: &str = "v1/new";
