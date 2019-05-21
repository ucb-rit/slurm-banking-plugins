// Include bindgen headers
// Source: https://rust-lang.github.io/rust-bindgen/tutorial-4.html
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;
extern crate reqwest;

mod logging;

use std::ffi::CStr;
use std::fs::File;
use std::io::prelude::*;

// Static strings based on: https://stackoverflow.com/a/33883281
#[repr(C)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

#[no_mangle]
pub static plugin_name: StaticCString =
    StaticCString(b"Slurm bank submit\0" as *const u8);

#[no_mangle]
pub static plugin_type: StaticCString =
    StaticCString(b"job_submit/bank\0" as *const u8);

#[no_mangle]
pub static plugin_version: u32 = SLURM_VERSION_NUMBER;

static myname: &str = "job_submit_bank";

pub fn log(message: &str) {
    logging::safe_info(&format!("{}: {}", myname, message));
}

// Slurm
#[no_mangle]
pub extern fn init() -> u32 {
    let mut file = File::create("/tmp/bank.txt").expect("could not create file");
    file.write_all(b"bank!").expect("could not write to file");
    log("hello from the log function");
    return SLURM_SUCCESS;
}

#[no_mangle]
pub extern fn job_submit(job_desc: *const job_descriptor, _submit_uid: u32, _error_msg: *mut *const u8) -> u32 {
    println!("Job submitted");
    let mut file = File::create("/tmp/submit.txt").expect("could not create file");
    let alloc_node: &CStr = unsafe { CStr::from_ptr((*job_desc).alloc_node) };
    file.write_all(alloc_node.to_bytes()).expect("could not write to file");
    return SLURM_SUCCESS;
}

#[no_mangle]
pub extern fn job_modify() -> u32 {
    println!("Job modified");
    return SLURM_SUCCESS;
}