// Include bindgen headers
// Reference: https://rust-lang.github.io/rust-bindgen/tutorial-4.html
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_use]
extern crate lazy_static;

extern crate config;
extern crate rust_decimal;

mod accounting;
mod logging;
mod safe_helpers;

use config::Config;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::os::raw::c_char;
use std::sync::Mutex;

static PRICES_CONFIG_FILE_PATH: &str = "/etc/slurm/prices";
static PLUGIN_NAME: &str = "jobcomp_bank";

lazy_static! {
    static ref settings: Mutex<Config> = Mutex::new(Config::default());
}

// Static strings reference: https://stackoverflow.com/a/33883281
#[repr(C)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

// Begin static values required by Slurm
#[no_mangle]
pub static plugin_name: StaticCString = StaticCString(b"Slurm bank completion\0" as *const u8);

#[no_mangle]
pub static plugin_type: StaticCString = StaticCString(b"jobcomp/bank\0" as *const u8);

#[no_mangle]
pub static plugin_version: u32 = SLURM_VERSION_NUMBER;
// End public static values

fn log(message: &str) {
    logging::safe_info(&format!("{}: {}", PLUGIN_NAME, message));
}

// Slurm
#[no_mangle]
pub extern "C" fn init() -> u32 {
    let mut conf = settings.lock().unwrap();
    log(&format!(
        "Looking for config file at {}",
        PRICES_CONFIG_FILE_PATH
    ));
    match conf.merge(config::File::with_name(PRICES_CONFIG_FILE_PATH)) {
        Ok(_) => {}
        Err(_) => {
            log("Could not find config file");
            return ESLURM_INTERNAL;
        }
    };
    log(&format!(
        "Using url {:?}",
        conf.get::<HashMap<String, String>>("Prices")
    ));
    log(&format!(
        "Plugin initialized using the prices config file from {}",
        PRICES_CONFIG_FILE_PATH
    ));
    return SLURM_SUCCESS;
}

#[no_mangle]
pub extern "C" fn fini() -> u32 {
    return SLURM_SUCCESS;
}

#[no_mangle]
pub extern "C" fn slurm_jobcomp_set_location(location: *const c_char) -> u32 {
    return SLURM_SUCCESS;
}

#[no_mangle]
pub extern "C" fn slurm_jobcomp_log_record(job_ptr: *const job_record) -> u32 {
    return SLURM_SUCCESS;
}

#[no_mangle]
pub extern "C" fn slurm_jobcomp_get_errno() -> u32 {
    return ESLURM_JOBCOMP_MIN;
}

#[no_mangle]
pub extern "C" fn slurm_jobcomp_strerror(errnum: u32) -> *const c_char {
    return std::ptr::null as *const i8;
}

#[no_mangle]
pub extern "C" fn slurm_jobcomp_get_jobs(job_cond: *const slurmdb_job_cond_t) -> List {
    return std::ptr::null;
}

#[no_mangle]
pub extern "C" fn slurm_jobcomp_archive(arch_cond: *const slurmdb_archive_cond_t) -> u32 {
    return SLURM_SUCCESS;
}