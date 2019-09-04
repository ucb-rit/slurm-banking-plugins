extern crate bindgen;

use std::env;
use std::path::PathBuf;

// Based upon: https://rust-lang.github.io/rust-bindgen/tutorial-3.html
fn main() {
    let bindings = bindgen::Builder::default()
        .blacklist_item("IPPORT_RESERVED") // this is defined twice and causes an error
        .clang_arg("-I".to_owned() + &env::var("SLURM_SOURCE_CODE_DIR")
            .expect("Set environment variable SLURM_SOURCE_CODE_DIR to point to the Slurm source code"))
        .header("wrapper.h")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
