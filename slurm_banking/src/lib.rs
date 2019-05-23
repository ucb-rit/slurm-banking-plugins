// Include bindgen headers
// Reference: https://rust-lang.github.io/rust-bindgen/tutorial-4.html
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod accounting;
pub mod prices_config;
pub mod logging;
pub mod safe_helpers;