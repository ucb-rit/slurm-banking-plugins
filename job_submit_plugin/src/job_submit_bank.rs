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
use std::collections::HashMap;
use std::os::raw::c_char;
use std::sync::Mutex;

static PRICES_CONFIG_FILE_PATH: &str = "/etc/slurm/prices";
static PLUGIN_NAME: &str = "job_submit_bank";

lazy_static! {
    static ref SETTINGS: Mutex<Config> = Mutex::new(Config::default());
}

// Static strings reference: https://stackoverflow.com/a/33883281
#[repr(C)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

// Begin static values required by Slurm
#[no_mangle]
pub static plugin_name: StaticCString = StaticCString(b"Slurm bank submit\0" as *const u8);

#[no_mangle]
pub static plugin_type: StaticCString = StaticCString(b"job_submit/bank\0" as *const u8);

#[no_mangle]
pub static plugin_version: u32 = SLURM_VERSION_NUMBER;
// End public static values

fn log(message: &str) {
    logging::safe_info(&format!("{}: {}", PLUGIN_NAME, message));
}

// Slurm
#[no_mangle]
pub extern "C" fn init() -> u32 {
    let mut conf = SETTINGS.lock().unwrap();
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
pub extern "C" fn job_submit(
    job_desc: *const job_descriptor,
    submit_uid: u32,
    _error_msg: *mut *const c_char,
) -> u32 {
    log("job_submit invoke");
    let account: String = match safe_helpers::deref_cstr(unsafe { (*job_desc).account }) {
        Some(account) => account,
        None => return ESLURM_INVALID_ACCOUNT,
    };
    let partition: String = match safe_helpers::deref_cstr(unsafe { (*job_desc).partition }) {
        Some(partition) => partition,
        None => return ESLURM_INVALID_PARTITION_NAME,
    };
    let max_cpus: u32 = unsafe { (*job_desc).max_cpus };
    let time_limit_minutes: u32 = unsafe { (*job_desc).time_limit }; // in minutes
    let max_nodes: u32 = unsafe { (*job_desc).max_nodes };

    log(&format!(
        "account: {}, partition: {}, max_cpus: {}, time_limit: {}, max_nodes: {}",
        account, partition, max_cpus, time_limit_minutes, max_nodes
    ));

    let conf = SETTINGS.lock().unwrap();
    let prices: HashMap<String, String> = conf.get::<HashMap<String, String>>("Prices").unwrap();
    let expected_cost =
        match accounting::expected_cost(&partition, max_cpus, time_limit_minutes, &prices) {
            Some(cost) => cost,
            None => return ESLURM_INTERNAL,
        };
    let deduction = accounting::deduct_service_units(&account, submit_uid, expected_cost);

    log(&format!("expected cost: {:?}", expected_cost));
    log(&format!("deduction {:?}", deduction));

    SLURM_SUCCESS
}

#[no_mangle]
pub extern "C" fn job_modify() -> u32 {
    println!("Job modified");
    return SLURM_SUCCESS;
}
