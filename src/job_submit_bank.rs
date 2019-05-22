// Include bindgen headers
// Source: https://rust-lang.github.io/rust-bindgen/tutorial-4.html
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate config;

mod accounting;
mod logging;

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::prelude::*;
use std::os::raw::c_char;

static PRICES_CONFIG_FILE_PATH: &str = "/etc/slurm/prices.toml";
static PLUGIN_NAME: &str = "job_submit_bank";


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

pub fn log(message: &str) {
    logging::safe_info(&format!("{}: {}", PLUGIN_NAME, message));
}

// Slurm
#[no_mangle]
pub extern fn init() -> u32 {
    // let mut settings = config::Config::default();
    // settings.merge(config::File::with_name(PRICES_CONFIG_FILE_PATH)).unwrap();
    // log(&format!("{:?}", settings.try_into::<HashMap<String, String>>().unwrap()));
    let mut file = File::create("/tmp/bank.txt").expect("could not create file");
    file.write_all(b"bank!").expect("could not write to file");
    log(&format!("Plugin initialized using the prices config file from {}", PRICES_CONFIG_FILE_PATH));
    return SLURM_SUCCESS;
}

#[no_mangle]
pub extern fn job_submit(job_desc: *const job_descriptor, submit_uid: u32, error_msg: *mut *const c_char) -> u32 {
    log("job_submit invoke");
    let account_buf: *const c_char = unsafe { (*job_desc).alloc_node };
    let account_str: &CStr = unsafe { CStr::from_ptr(account_buf) };
    let account: &str = account_str.to_str().unwrap();
    // let max_cpus: u32 = unsafe { (*job_desc).max_cpus };
    // let time_limit_minutes: u32 = unsafe  { (*job_desc).time_limit }; // in minutes
    // let partition: &str = unsafe { CStr::from_ptr((*job_desc).partition).to_str().unwrap() };

    log(&("pre-calculation".to_owned() + account));

/*
    let expected_cost = accounting::expected_cost(partition, max_cpus, time_limit_minutes);
    let deduction = accounting::deduct_service_units(account, submit_uid, expected_cost);

    log(&format!("deduction {:?}", deduction));

    let mut file = File::create("/tmp/submit.txt").expect("could not create file");
    file.write_all(account.as_bytes()).expect("could not write to file");
    file.write_all(max_cpus.to_string().as_bytes()).expect("could not write to file");
    file.write_all(time_limit_minutes.to_string().as_bytes()).expect("could not write to file");
    file.write_all(partition.as_bytes()).expect("could not write to file");

    match deduction {
        Ok(()) => return SLURM_SUCCESS,
        Err(e) => {
            let error_message = "Not enough service units";
            let error_message_cstring = CString::new(error_message).unwrap();
            unsafe { *error_msg = error_message_cstring.as_ptr() }
            return ESLURM_INTERNAL;
        }
    }*/
    return ESLURM_ACCOUNTING_POLICY;
}

#[no_mangle]
pub extern fn job_modify() -> u32 {
    println!("Job modified");
    return SLURM_SUCCESS;
}