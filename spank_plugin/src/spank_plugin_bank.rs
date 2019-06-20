#[macro_use]
extern crate lazy_static;

extern crate config;
extern crate rust_decimal;
extern crate slurm_banking;

use slurm_banking::accounting;
use slurm_banking::bindings::*;
use slurm_banking::logging;
use slurm_banking::safe_helpers;

use config::Config;
use chrono::prelude::Utc;
use std::collections::HashMap;
use std::os::raw::{c_char, c_int};
use std::sync::Mutex;

static PLUGIN_NAME: &str = "spank_bank";

lazy_static! {
    static ref SETTINGS: Mutex<Config> = Mutex::new(Config::default());
}

// Static strings reference: https://stackoverflow.com/a/33883281
#[repr(C)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

// Begin static values required by Slurm
#[no_mangle]
pub static plugin_name: StaticCString = StaticCString(b"Slurm bank spank\0" as *const u8);

#[no_mangle]
pub static plugin_type: StaticCString = StaticCString(b"spank\0" as *const u8);

#[no_mangle]
pub static plugin_version: u32 = SLURM_VERSION_NUMBER;
// End public static values

fn log(message: &str) {
    logging::safe_spank_info(&format!("{}: {}", PLUGIN_NAME, message));
}

fn error(message: &str) {
    logging::safe_spank_error(&format!("{}: {}", PLUGIN_NAME, message));
}

// Slurm
#[no_mangle]
pub extern "C" fn slurm_spank_init(sp: spank_t, ac: c_int, argv: *const *const c_char) -> c_int {
    let mut job_id: u32 = 0;
    unsafe {
        log(&format!("{:?}", *sp));
        let result = spank_get_item(sp, spank_item_S_JOB_ID, &mut job_id as *mut u32); 
        if result != 0 {
            // No job ID available in this context
            return 0
        }
    }
    log(&format!("slurm_spank_init(). Result: {}; Job ID: {}", result, job_id));
    0
}