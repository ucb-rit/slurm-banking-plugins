#[macro_use]
extern crate lazy_static;

extern crate config;
extern crate openapi;
extern crate rust_decimal;
extern crate slurm_banking;

use slurm_banking::accounting;
use slurm_banking::bindings::*;
use slurm_banking::logging;
use slurm_banking::safe_helpers;

use config::Config;
use rust_decimal::prelude::Zero;
use std::collections::HashMap;
use std::os::raw::{c_char, c_int};

static PLUGIN_NAME: &str = "job_submit_bank";

lazy_static! {
    static ref SETTINGS: Config = {
        let mut conf = Config::default();
        match slurm_banking::prices_config::load_config_from_file(&mut conf) {
            Ok(_) => {}
            Err(_) => {}
        };
        conf
    };
}

// Static strings reference: https://stackoverflow.com/a/33883281
#[repr(C)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

// Begin static values required by Slurm
#[no_mangle]
pub static plugin_name: StaticCString = StaticCString(b"Slurm bank submit\0" as *const u8);

#[no_mangle]
pub static plugin_type: StaticCString = StaticCString(b"job_submit/slurm_banking\0" as *const u8);

#[no_mangle]
pub static plugin_version: u32 = SLURM_VERSION_NUMBER;
// End public static values

fn log(message: &str) {
    logging::safe_info(&format!("{}: {}", PLUGIN_NAME, message));
}

// Slurm
#[no_mangle]
pub extern "C" fn init() -> c_int {
    SLURM_SUCCESS as c_int
}

#[no_mangle]
pub extern "C" fn job_submit(
    job_desc: *mut job_descriptor,
    _submit_uid: u32,
    error_msg: *mut *const c_char,
) -> u32 {
    log("job_submit() invoked");
    // BEGIN: Check if this plugin should be enabled
    let conf = &SETTINGS;
    let plugin_enable_config = match conf.get::<HashMap<String, bool>>("Enable") {
        Ok(v) => v,
        Err(_) => return SLURM_SUCCESS,
    };
    let enabled = plugin_enable_config
        .get("enable_job_submit_plugin")
        .unwrap_or(&false);
    if !enabled {
        log("job_submit() not enabled; exiting");
        return SLURM_SUCCESS;
    }
    // END: Check if this plugin should be enabled

    log("job_submit() settings loaded");
    let partition: String = match safe_helpers::deref_cstr(unsafe { (*job_desc).partition }) {
        Some(partition) => partition,
        None => return ESLURM_INVALID_PARTITION_NAME,
    };
    log("job_submit() loaded partition");

    let partition_price = accounting::price_per_cpu_hour(&partition, conf);
    log(&format!(
        "Partition: {:?} Price: {:?}",
        partition, partition_price
    ));
    if partition_price.is_zero() {
        log("Partition price is 0 -- accepting job without checking API");
        return SLURM_SUCCESS;
    }

    let userid: u32 = unsafe { (*job_desc).user_id };
    log("job_submit() loaded userid");
    let account: String = match safe_helpers::deref_cstr(unsafe { (*job_desc).account }) {
        Some(account) => account,
        None => return ESLURM_INVALID_ACCOUNT,
    };
    log("job_submit() loaded account");

    let cpus_per_task = unsafe { (*job_desc).cpus_per_task };
    log("job_submit() loaded cpus_per_task");
    let num_tasks = unsafe { (*job_desc).num_tasks };
    log("job_submit() loaded num_tasks");
    // When not specified, cpus_per_task=65534, num_tasks=4294967294
    let max_cpus: u32 = if cpus_per_task == 65534 || num_tasks == 4294967294 {
        // TODO: In this calculation, assume 0 CPUs if unspecified
        // In the future, this should look at the partition and charge according to the default
        0
    } else {
        ((unsafe { (*job_desc).cpus_per_task }) as u32) * (unsafe { (*job_desc).num_tasks })
    };
    log("job_submit() loaded max_cpus");

    let time_limit_minutes: i64 = unsafe { (*job_desc).time_limit } as i64; // in minutes
    log("job_submit() loaded time_limit_minutes");
    let time_limit_seconds = time_limit_minutes * 60;
    log("job_submit() loaded time_limit_seconds");

    log(&format!(
        "Processing request from user_id {:?} with account {:?}: \
         partition: {:?}, time_limit_minutes: {:?}, max_cpus: {:?}",
        userid, account, partition, time_limit_seconds, max_cpus
    ));

    // Calculate the expected cost of the job
    let expected_cost =
        match accounting::expected_cost(&partition, max_cpus, time_limit_seconds, &conf) {
            Some(cost) => cost,
            None => return ESLURM_INTERNAL,
        };

    log(&format!(
        "Expected cost is {:?} SU for user_id {:?} with account {:?}: \
         partition: {:?}, time_limit_minutes: {:?}, max_cpus: {:?}",
        expected_cost, userid, account, partition, time_limit_minutes, max_cpus
    ));

    // Check if the account has sufficient funds for the job
    let base_path = slurm_banking::prices_config::get_base_path(&conf);
    let auth_token = slurm_banking::prices_config::get_auth_token(&conf);
    let has_funds = match accounting::check_sufficient_funds(
        base_path,
        &auth_token,
        expected_cost,
        &userid.to_string(),
        &account,
    ) {
        Ok(result) => result,
        Err(_err) => {
            log(&format!("API connection error on check_sufficient_funds. Job specifications are: \
            user_id: {:?}, account: {:?}, partition: {:?}, time_limit_minutes: {:?}, max_cpus: {:?}",
            userid, account, partition, time_limit_minutes, max_cpus));
            true
        }
    };

    // Return success if there are enough funds
    match has_funds {
        true => SLURM_SUCCESS,
        false => {
            let msg = std::ffi::CString::new("This user/account pair does not have enough service units to afford this job's estimated cost").unwrap();
            unsafe { *error_msg = xstrdup(msg.as_ptr()) }
            ESLURM_ACCOUNTING_POLICY
        }
    }
}

#[no_mangle]
pub extern "C" fn job_modify() -> u32 {
    return SLURM_SUCCESS;
}
