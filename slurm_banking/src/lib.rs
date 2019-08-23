// Include bindgen headers
// Reference: https://rust-lang.github.io/rust-bindgen/tutorial-4.html
#[allow(warnings)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod accounting;
pub mod logging;
pub mod prices_config;
pub mod range_format;
pub mod safe_helpers;