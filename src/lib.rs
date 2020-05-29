//! The core library
#![warn(missing_docs)]

pub mod protocol;

#[cfg(feature = "backend")]
#[macro_use]
extern crate diesel;

macro_rules! apis {
    ($($name:ident => $content:expr,)*) => (
        $(#[allow(missing_docs)] pub const $name: &str = $content;)*
    )
}

apis!{
    API_URL_V1_CREATE_STEP_PAGE => "v1/step/create",
    API_URL_V1_READ_STEP_PAGE => "v1/step/{id}",
    API_URL_V1_UPDATE_STEP_PAGE => "v1/step/update/{id}",
    API_URL_V1_DELETE_PAGE => "v1/step/delete/{id}",
}
