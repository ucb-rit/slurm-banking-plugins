#[macro_use]
extern crate lazy_static;

extern crate config;
extern crate rust_decimal;
extern crate slurm_banking;
extern crate swagger;

use slurm_banking::accounting;
use slurm_banking::bindings::*;
use slurm_banking::logging;
use slurm_banking::safe_helpers;

use config::Config;
use chrono::prelude::Utc;
use std::collections::HashMap;
use std::os::raw::c_char;
use std::sync::Mutex;

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
pub extern "C" fn init() -> i32 {
    let mut conf = SETTINGS.lock().unwrap();
    match slurm_banking::prices_config::load_config_from_file(&mut conf) {
        Ok(()) => SLURM_SUCCESS as i32,
        Err(_) => SLURM_ERROR
    }
}

#[no_mangle]
pub extern "C" fn job_submit(
    job_desc: *mut job_descriptor,
    submit_uid: u32,
    _error_msg: *mut *const c_char,
) -> u32 {
    log("job_submit invoke");
    let max_cpus: u32 = unsafe { (*job_desc).max_cpus };
    let time_limit_minutes: i64 = unsafe { (*job_desc).time_limit } as i64; // in minutes
    let max_nodes: u32 = unsafe { (*job_desc).max_nodes };
    let partition: String = match safe_helpers::deref_cstr(unsafe { (*job_desc).partition }) {
        Some(partition) => partition,
        None => return ESLURM_INVALID_PARTITION_NAME,
    };

    log(&format!("job_id_str: {:?}", safe_helpers::deref_cstr(unsafe { (*job_desc).job_id_str })));

    let conf = SETTINGS.lock().unwrap();
    let prices: HashMap<String, String> = conf.get::<HashMap<String, String>>("Prices").unwrap();
    let expected_cost =
        match accounting::expected_cost(&partition, max_cpus, time_limit_minutes, &prices) {
            Some(cost) => cost,
            None => return ESLURM_INTERNAL,
        };

    log(&format!("expected cost: {:?}", expected_cost));

    let jobslurmid = unsafe { (*job_desc).job_id };
    let submitdate = Utc::now().to_rfc3339();
    let userid: u32 = unsafe { (*job_desc).user_id };
    let account: String = match safe_helpers::deref_cstr(unsafe { (*job_desc).account }) {
        Some(account) => account,
        None => return ESLURM_INVALID_ACCOUNT,
    };
    let amount: String = expected_cost.to_string();
    let job_status: String = "".to_string();
    let qos: String = match safe_helpers::deref_cstr(unsafe { (*job_desc).qos }) {
        Some(qos) => qos,
        None => return ESLURM_INVALID_QOS
    };

    let job = swagger::models::Job::new(
        jobslurmid.to_string(), 
        submitdate, 
        userid.to_string(), 
        account.clone(),
        amount, 
        job_status, 
        partition, 
        qos);

    accounting::deduct_service_units(&account, userid, expected_cost);

    log(&format!("{:?}", job));
    log(&format!("{:?}", accounting::post_job(job)));

    SLURM_SUCCESS
}

#[no_mangle]
pub extern "C" fn job_modify() -> u32 {
    println!("Job modified");
    return SLURM_SUCCESS;
}
